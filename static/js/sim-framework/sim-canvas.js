/**
 * Simulation canvas — WebGL2 renderer for multiple simulation types.
 *
 * Each simulation outputs a float texture from WASM.
 * A per-sim fragment shader renders color, vignette, grid — one draw call.
 */

// ─── Shared vertex shader ──────────────────────────────────────────────────

const VERT = `#version 300 es
in vec2 a_pos;
out vec2 v_uv;
void main() {
  v_uv = vec2(a_pos.x * 0.5 + 0.5, 0.5 - a_pos.y * 0.5);
  gl_Position = vec4(a_pos, 0.0, 1.0);
}`;

// ─── Quantum walk fragment shader (R32F: probability) ──────────────────────

const FRAG_QUANTUM = `#version 300 es
precision highp float;
in vec2 v_uv;
out vec4 fragColor;

uniform sampler2D u_data;
uniform vec2 u_gridSize;
uniform float u_isDark;
uniform float u_maxAlpha;
uniform float u_showGrid;

vec3 blueRamp(float t) {
  vec3 navy      = vec3(0.059, 0.090, 0.165);
  vec3 elecDark  = vec3(0.231, 0.510, 0.965);
  vec3 elecLight = vec3(0.145, 0.388, 0.922);
  vec3 hiDark    = vec3(0.376, 0.647, 0.980);
  vec3 hiLight   = vec3(0.290, 0.545, 0.950);
  vec3 mid  = mix(elecLight, elecDark, u_isDark);
  vec3 high = mix(hiLight,   hiDark,   u_isDark);
  return t < 0.5 ? mix(navy, mid, t * 2.0) : mix(mid, high, (t - 0.5) * 2.0);
}

float vignette(vec2 uv) {
  float r = length((uv - 0.5) * 2.0);
  return 1.0 - smoothstep(0.5, 1.35, r);
}

float grid(vec2 uv, vec2 gs) {
  vec2 cell = uv * gs;
  vec2 near = min(fract(cell), 1.0 - fract(cell));
  vec2 px = fwidth(cell);
  return max(
    1.0 - smoothstep(0.0, px.x * 1.5, near.x),
    1.0 - smoothstep(0.0, px.y * 1.5, near.y)
  );
}

float dots(vec2 uv, vec2 gs) {
  vec2 cell = uv * gs;
  vec2 near = min(fract(cell), 1.0 - fract(cell));
  float px = max(fwidth(cell.x), fwidth(cell.y));
  return 1.0 - smoothstep(0.0, px * 2.0, length(near));
}

void main() {
  float prob = texture(u_data, v_uv).r;
  float t = sqrt(clamp(prob, 0.0, 1.0));
  vec3 color = blueRamp(t);
  float vig = vignette(v_uv);
  float alpha = t * vig * u_maxAlpha;

  if (u_showGrid > 0.5) {
    float ga = u_isDark > 0.5 ? 0.05 : 0.04;
    float da = u_isDark > 0.5 ? 0.08 : 0.06;
    vec3 gc = mix(vec3(0.145,0.388,0.922), vec3(0.231,0.510,0.965), u_isDark);
    float overlay = (grid(v_uv, u_gridSize) * ga + dots(v_uv, u_gridSize) * da) * vig;
    color = mix(color, gc, clamp(overlay / max(alpha + overlay, 0.001), 0.0, 1.0));
    alpha += overlay;
  }

  fragColor = vec4(color * alpha, alpha);
}`;

// ─── Ising fragment shader (RG32F: spin + glow) ───────────────────────────

const FRAG_ISING = `#version 300 es
precision highp float;
in vec2 v_uv;
out vec4 fragColor;

uniform sampler2D u_data;
uniform vec2 u_gridSize;
uniform float u_isDark;
uniform float u_maxAlpha;
uniform float u_showGrid;

float vignette(vec2 uv) {
  float r = length((uv - 0.5) * 2.0);
  return 1.0 - smoothstep(0.5, 1.35, r);
}

void main() {
  // Nearest-neighbor sampling for crisp spin cells
  vec2 texel = (floor(v_uv * u_gridSize) + 0.5) / u_gridSize;
  vec2 data = texture(u_data, texel).rg;
  float spin = data.r;   // 0 = down, 1 = up
  float glow = data.g;   // 0..1 flash on flip

  // Base colors: spin-up = electric blue, spin-down = deep navy
  vec3 upColorDark   = vec3(0.231, 0.510, 0.965);
  vec3 upColorLight  = vec3(0.145, 0.388, 0.922);
  vec3 downColorDark  = vec3(0.040, 0.055, 0.110);
  vec3 downColorLight = vec3(0.070, 0.090, 0.160);

  vec3 upColor   = mix(upColorLight,   upColorDark,   u_isDark);
  vec3 downColor = mix(downColorLight, downColorDark, u_isDark);

  vec3 baseColor = mix(downColor, upColor, spin);

  // Glow: bright flash that fades — teal accent on recently flipped sites
  vec3 glowColor = vec3(0.078, 0.722, 0.651);  // teal
  vec3 color = mix(baseColor, glowColor, glow * 0.6);

  // Cell border: thin dark line between cells for visual structure
  vec2 cell = fract(v_uv * u_gridSize);
  vec2 px = fwidth(v_uv * u_gridSize);
  float borderX = smoothstep(0.0, px.x * 1.2, cell.x) * (1.0 - smoothstep(1.0 - px.x * 1.2, 1.0, cell.x));
  float borderY = smoothstep(0.0, px.y * 1.2, cell.y) * (1.0 - smoothstep(1.0 - px.y * 1.2, 1.0, cell.y));
  float border = borderX * borderY;
  color *= mix(0.7, 1.0, border);

  float vig = vignette(v_uv);
  float alpha = vig * u_maxAlpha;

  // Boost alpha where there's spin-up activity or glow
  alpha *= 0.6 + 0.4 * spin + glow * 0.3;

  fragColor = vec4(color * alpha, alpha);
}`;

// ─── Rydberg fragment shader (RG32F: ground field + excited field) ────────
// Particles rendered as gaussian blobs. Ground = dim blue, Rydberg = bright.

const FRAG_RYDBERG = `#version 300 es
precision highp float;
in vec2 v_uv;
out vec4 fragColor;

uniform sampler2D u_data;
uniform vec2 u_gridSize;
uniform float u_isDark;
uniform float u_maxAlpha;
uniform float u_showGrid;

float vignette(vec2 uv) {
  float r = length((uv - 0.5) * 2.0);
  return 1.0 - smoothstep(0.5, 1.35, r);
}

void main() {
  vec2 data = texture(u_data, v_uv).rg;
  float ground  = data.r;
  float excited = data.g;

  // Sharpen blobs with power curve
  ground  = pow(ground,  1.5);
  excited = pow(excited, 1.4);

  // Ground atoms: bright blue
  vec3 groundCol = mix(vec3(0.15, 0.40, 0.95), vec3(0.25, 0.55, 1.0), u_isDark);
  // Rydberg atoms: yellow
  vec3 excitedCol = mix(vec3(0.95, 0.85, 0.15), vec3(1.0, 0.92, 0.25), u_isDark);
  // Hot core of excited atom — bright white-yellow
  vec3 coreCol = vec3(1.0, 0.98, 0.75);
  // Glow halo: warm yellow
  vec3 glowCol = vec3(0.85, 0.75, 0.1);

  // Ground: bright and visible
  float gAlpha = ground * 1.2;
  // Excited: bright with hot core
  float eAlpha = excited * 1.6;
  vec3 excColor = mix(excitedCol, coreCol, smoothstep(0.3, 0.8, excited));

  vec3 color = groundCol * gAlpha + excColor * eAlpha;
  // Teal glow bloom around excited atoms
  color += glowCol * smoothstep(0.08, 0.4, excited) * 0.25;

  float totalAlpha = min(gAlpha + eAlpha, 1.0);
  float vig = vignette(v_uv);
  float alpha = vig * u_maxAlpha * totalAlpha;

  vec3 finalColor = totalAlpha > 0.001 ? color / totalAlpha : vec3(0.0);

  // Light mode: composite over dark background so simulation is visible
  fragColor = vec4(finalColor * alpha, alpha);
}`;

// ─── Tweezer fragment shader (RG32F: traps+atoms + tweezer blob) ──────────
// R channel: 0.0=empty, ~0.15=trap circle, ~1.0=atom
// G channel: 0.0=no tweezer, >0=tweezer blob

const FRAG_TWEEZER = `#version 300 es
precision highp float;
in vec2 v_uv;
out vec4 fragColor;

uniform sampler2D u_data;
uniform vec2 u_gridSize;
uniform float u_isDark;
uniform float u_maxAlpha;
uniform float u_showGrid;

float vignette(vec2 uv) {
  float r = length((uv - 0.5) * 2.0);
  return 1.0 - smoothstep(0.5, 1.35, r);
}

void main() {
  vec2 data = texture(u_data, v_uv).rg;
  float r = data.r;   // traps + atoms
  float g = data.g;   // tweezer blob

  vec3 color = vec3(0.0);
  float alpha = 0.0;

  // Trap circles: light grey semi-transparent
  float trapMask = smoothstep(0.03, 0.08, r) * (1.0 - smoothstep(0.3, 0.5, r));
  vec3 trapColDark  = vec3(0.55, 0.55, 0.60);
  vec3 trapColLight = vec3(0.65, 0.65, 0.70);
  vec3 trapCol = mix(trapColLight, trapColDark, u_isDark);
  float trapAlpha = trapMask * 0.35;

  // Atom: bright teal
  float atomMask = smoothstep(0.3, 0.5, r);
  vec3 atomColDark  = vec3(0.08, 0.82, 0.72);
  vec3 atomColLight = vec3(0.06, 0.70, 0.62);
  vec3 atomCol = mix(atomColLight, atomColDark, u_isDark);
  float atomAlpha = atomMask * min(r * 1.5, 1.0);

  // Tweezer: bright yellow semi-transparent
  float tweezMask = smoothstep(0.05, 0.15, g);
  vec3 tweezColDark  = vec3(1.0, 0.85, 0.10);
  vec3 tweezColLight = vec3(0.90, 0.75, 0.05);
  vec3 tweezCol = mix(tweezColLight, tweezColDark, u_isDark);
  float tweezAlpha = tweezMask * 0.4 * g;

  // Composite: trap -> tweezer -> atom (front)
  color = trapCol * trapAlpha;
  alpha = trapAlpha;

  // Blend tweezer over traps
  color = color * (1.0 - tweezAlpha) + tweezCol * tweezAlpha;
  alpha = alpha * (1.0 - tweezAlpha) + tweezAlpha;

  // Blend atoms on top
  color = color * (1.0 - atomAlpha) + atomCol * atomAlpha;
  alpha = alpha * (1.0 - atomAlpha) + atomAlpha;

  float vig = vignette(v_uv);
  alpha *= vig * u_maxAlpha;

  // Light mode: composite over dark background so simulation is visible
  fragColor = vec4(color * alpha, alpha);
}`;

// ─── Shader configs per simulation type ────────────────────────────────────

const SHADERS = {
  "quantum-walk": {
    frag: FRAG_QUANTUM,
    texFormat: "R32F",       // 1 float per site
    texFilter: "linear",     // smooth probability field
  },
  "ising": {
    frag: FRAG_ISING,
    texFormat: "RG32F",      // 2 floats per site: spin + glow
    texFilter: "nearest",    // crisp spin cells
  },
  "rydberg": {
    frag: FRAG_RYDBERG,
    texFormat: "RG32F",      // 2 floats per cell: ground + excited field
    texFilter: "linear",     // smooth particle blobs
  },
  "tweezer": {
    frag: FRAG_TWEEZER,
    texFormat: "RG32F",      // 2 floats per cell: traps+atoms + tweezer
    texFilter: "linear",     // smooth blob rendering
  },
};

// ─── WebGL helpers ─────────────────────────────────────────────────────────

function compileShader(gl, type, src) {
  const s = gl.createShader(type);
  gl.shaderSource(s, src);
  gl.compileShader(s);
  if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) {
    console.error("[sim] Shader compile:", gl.getShaderInfoLog(s));
    gl.deleteShader(s);
    return null;
  }
  return s;
}

function linkProgram(gl, vs, fs) {
  const p = gl.createProgram();
  gl.attachShader(p, vs);
  gl.attachShader(p, fs);
  gl.linkProgram(p);
  if (!gl.getProgramParameter(p, gl.LINK_STATUS)) {
    console.error("[sim] Program link:", gl.getProgramInfoLog(p));
    return null;
  }
  return p;
}

// ─── Simulation registry ───────────────────────────────────────────────────

const SIMS = {
  "quantum-walk": {
    wasm: "/wasm/quantum-walk/sim_quantum_walk.js",
    cls: "WasmSimulation",
    shader: "quantum-walk",
    desktopCell: 8,
    mobileCell: 12,
  },
  "ising": {
    wasm: "/wasm/ising/sim_ising.js",
    cls: "WasmSimulation",
    shader: "ising",
    desktopCell: 5,     // visible crisp cells
    mobileCell: 8,
  },
  "rydberg": {
    wasm: "/wasm/rydberg/sim_rydberg.js",
    cls: "WasmSimulation",
    shader: "rydberg",
    desktopCell: 2,     // very fine grid for free-space particle look
    mobileCell: 3,
  },
  "tweezer": {
    wasm: "/wasm/tweezer/sim_tweezer.js",
    cls: "WasmSimulation",
    shader: "tweezer",
    desktopCell: 3,     // fine grid for smooth blob rendering
    mobileCell: 4,
  },
};

// ─── Main entry point ──────────────────────────────────────────────────────

export async function initSimulations() {
  const canvases = document.querySelectorAll("[data-sim]");
  for (const canvas of canvases) {
    try {
      await launch(canvas);
    } catch (e) {
      console.warn("[sim] Init failed:", e);
      fallbackGradient(canvas);
    }
  }
}

async function launch(canvas) {
  if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
    canvas.style.display = "none";
    return;
  }

  const name = canvas.dataset.sim;
  const entry = SIMS[name];
  if (!entry) { console.warn("[sim] Unknown:", name); return; }

  const shaderCfg = SHADERS[entry.shader];
  if (!shaderCfg) { console.warn("[sim] Unknown shader:", entry.shader); return; }

  // ── WebGL2 ──
  const gl = canvas.getContext("webgl2", {
    alpha: true,
    premultipliedAlpha: true,
    antialias: false,
    powerPreference: "low-power",
  });
  if (!gl) { fallbackGradient(canvas); return; }

  // ── Compile program ──
  const vs = compileShader(gl, gl.VERTEX_SHADER, VERT);
  const fs = compileShader(gl, gl.FRAGMENT_SHADER, shaderCfg.frag);
  if (!vs || !fs) { fallbackGradient(canvas); return; }
  const prog = linkProgram(gl, vs, fs);
  if (!prog) { fallbackGradient(canvas); return; }

  // ── Fullscreen quad ──
  const vao = gl.createVertexArray();
  gl.bindVertexArray(vao);
  const buf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buf);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1,-1, 1,-1, -1,1, 1,1]), gl.STATIC_DRAW);
  const aPos = gl.getAttribLocation(prog, "a_pos");
  gl.enableVertexAttribArray(aPos);
  gl.vertexAttribPointer(aPos, 2, gl.FLOAT, false, 0, 0);

  // ── Uniforms ──
  gl.useProgram(prog);
  const uGridSize = gl.getUniformLocation(prog, "u_gridSize");
  const uIsDark   = gl.getUniformLocation(prog, "u_isDark");
  const uMaxAlpha = gl.getUniformLocation(prog, "u_maxAlpha");
  const uShowGrid = gl.getUniformLocation(prog, "u_showGrid");

  // ── Texture ──
  const tex = gl.createTexture();
  gl.activeTexture(gl.TEXTURE0);
  gl.bindTexture(gl.TEXTURE_2D, tex);

  const wantLinear = shaderCfg.texFilter === "linear";
  const hasFloatLinear = gl.getExtension("OES_texture_float_linear");
  const filter = (wantLinear && hasFloatLinear) ? gl.LINEAR : gl.NEAREST;
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, filter);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, filter);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

  // Texture format
  const isRG = shaderCfg.texFormat === "RG32F";
  const internalFmt = isRG ? gl.RG32F : gl.R32F;
  const format      = isRG ? gl.RG    : gl.RED;

  // ── Blending (premultiplied alpha) ──
  gl.enable(gl.BLEND);
  gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);

  // ── Load WASM ──
  const mod = await import(entry.wasm);
  await mod.default();
  const sim = new mod[entry.cls]();

  // ── Config ──
  let showGrid = true;
  let maxAlpha = 0.25;
  try {
    const cfg = JSON.parse(canvas.dataset.simConfig || "{}");
    if (cfg.overlayGrid === false) showGrid = false;
    if (typeof cfg.maxAlpha === "number") maxAlpha = cfg.maxAlpha;
  } catch (_) {}

  // ── Canvas + grid sizing ──
  let gridW = 0, gridH = 0;
  let texNeedsResize = true;

  function computeGrid() {
    const rect = canvas.parentElement.getBoundingClientRect();
    const cell = window.innerWidth < 768 ? entry.mobileCell : entry.desktopCell;
    return {
      w: Math.max(16, Math.round(rect.width / cell)),
      h: Math.max(16, Math.round(rect.height / cell)),
    };
  }

  function sizeCanvas() {
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    const rect = canvas.parentElement.getBoundingClientRect();
    canvas.width  = Math.round(rect.width * dpr);
    canvas.height = Math.round(rect.height * dpr);
    canvas.style.width  = rect.width + "px";
    canvas.style.height = rect.height + "px";
  }

  function initGrid() {
    const g = computeGrid();
    if (g.w !== gridW || g.h !== gridH) {
      gridW = g.w; gridH = g.h;
      sim.resize(gridW, gridH);
      texNeedsResize = true;
    }
  }

  sizeCanvas();
  const g0 = computeGrid();
  gridW = g0.w; gridH = g0.h;
  sim.init(gridW, gridH, "{}");

  // ── Visibility ──
  let isVisible = true;
  let pageVisible = true;
  new IntersectionObserver(
    (entries) => { isVisible = entries[0].isIntersecting; },
    { threshold: 0.1 }
  ).observe(canvas);
  document.addEventListener("visibilitychange", () => {
    pageVisible = document.visibilityState !== "hidden";
  });

  // ── Resize ──
  let resizeTimer;
  window.addEventListener("resize", () => {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => { sizeCanvas(); initGrid(); }, 200);
  });

  // ── Animation loop ──
  function tick() {
    if (!isVisible || !pageVisible) {
      requestAnimationFrame(tick);
      return;
    }

    const dataView = sim.step();
    const data = new Float32Array(dataView);

    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, tex);
    if (texNeedsResize) {
      gl.texImage2D(gl.TEXTURE_2D, 0, internalFmt, gridW, gridH, 0, format, gl.FLOAT, data);
      texNeedsResize = false;
    } else {
      gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, gridW, gridH, format, gl.FLOAT, data);
    }

    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(prog);
    gl.uniform2f(uGridSize, gridW, gridH);
    gl.uniform1f(uIsDark, document.documentElement.classList.contains("dark") ? 1.0 : 0.0);
    gl.uniform1f(uMaxAlpha, maxAlpha);
    gl.uniform1f(uShowGrid, showGrid ? 1.0 : 0.0);

    gl.bindVertexArray(vao);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

    requestAnimationFrame(tick);
  }

  // ── "What is this?" overlay link ──
  addSimLink(canvas, name);

  requestAnimationFrame(tick);
}

// Simulation name → post slug mapping
const SIM_SLUGS = {
  "quantum-walk": "not-just-an-animation-quantum-walk",
  "ising":        "not-just-an-animation-ising-model",
  "rydberg":      "not-just-an-animation-rydberg-atoms",
  "tweezer":      "not-just-an-animation-optical-tweezers",
};

function addSimLink(canvas, simName) {
  const parent = canvas.parentElement;
  if (!parent || parent.querySelector('.sim-explainer-link')) return;

  const phrases = [
    "What is going on here?",
    "This is not just an animation",
  ];
  const text = phrases[Math.floor(Math.random() * phrases.length)];

  const lang = document.documentElement.lang || "en";
  const prefix = (lang && lang !== "en") ? `/${lang}` : "";
  const slug = SIM_SLUGS[simName] || SIM_SLUGS["quantum-walk"];
  const href = `${prefix}/posts/${slug}/`;

  const a = document.createElement("a");
  a.className = "sim-explainer-link";
  a.href = href;
  a.innerHTML = `<span class="sim-explainer-icon" aria-hidden="true">&#x24D8;</span> ${text}`;
  parent.appendChild(a);
}

function fallbackGradient(canvas) {
  canvas.style.background =
    "radial-gradient(ellipse at 50% 30%, rgba(59,130,246,0.08) 0%, transparent 60%)";
}
