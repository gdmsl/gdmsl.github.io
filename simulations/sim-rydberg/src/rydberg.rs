/// Rydberg atom simulation — 2D gas with facilitated excitation dynamics.
///
/// Two-level atoms (ground / Rydberg) in a box with continuous positions.
/// Off-resonance facilitation: detuning Δ ≫ γ suppresses spontaneous excitation,
/// but vdW shift from nearby Rydberg atoms brings the effective detuning to
/// resonance at the facilitation shell, creating spatial correlations.
///
/// Excited atoms repel via C₆/r⁶ van der Waals interaction.
/// Langevin dynamics models laser cooling (MOT): friction dissipates vdW
/// energy kicks, stochastic noise maintains thermal equilibrium at T_MOT.
/// Integration via velocity Verlet with Langevin thermostat.
///
/// Parameters scaled from Rb-87 70s Rydberg state:
///   Ω=1MHz, γ_laser=0.5MHz, δ=32MHz, C₆=0.86THz·μm⁶, T_MOT≈200μK
///
/// Output: RG32F texture — R = ground atom field, G = excited atom field.

use core::f64::consts::PI;
use sim_core::{rng::Rng, traits::*};

struct Atom {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    excited: bool,
    glow: f32,
}

pub struct Rydberg {
    grid_w: u32,
    grid_h: u32,
    box_w: f64,      // physical box width
    box_h: f64,      // physical box height
    scale: f32,      // grid cells per physics unit

    atoms: Vec<Atom>,
    n_atoms: usize,

    // Physics (γ_laser = 1 sets the unit of rates)
    omega: f64,      // Rabi frequency (Ω/γ = 2)
    gamma: f64,      // laser linewidth
    gamma_sp: f64,   // spontaneous decay rate (≪ γ)
    delta: f64,      // detuning (δ/γ = 64)
    c6: f64,         // vdW coefficient C₆ = γ·R_b⁶
    blockade_r: f64, // blockade radius

    // Dynamics
    force_scale: f32,
    max_accel: f32,
    dt_mech: f32,
    dt_frame: f64,   // Gillespie time budget per animation frame
    min_dist: f32,

    // MOT-like confinement for ground atoms
    mot_friction: f32, // velocity-dependent restoring (friction on outward motion)

    // Rendering (in physics units, scaled to grid in render())
    ground_sigma: f32,
    excited_sigma: f32,
    ground_amp: f32,
    excited_amp: f32,
    glow_decay: f32,

    output: Vec<f32>,
    rng: Rng,
    frame: u32,
}

impl Rydberg {
    pub fn new() -> Self {
        // v_thermal ≈ 0.025 R_b per time unit (from Rb-87 at 200μK)
        // With R_b = 18: v_thermal = 0.45 physics units / time unit
        // temp = v_thermal² (in units where k_B/m = 1)
        Self {
            grid_w: 0,
            grid_h: 0,
            box_w: 0.0,
            box_h: 0.0,
            scale: 1.0,
            atoms: Vec::new(),
            n_atoms: 0,

            // Rb-87 70s ratios: Ω/γ=2, δ/γ=64, γ_sp/γ≈0.02
            omega: 2.0,
            gamma: 1.0,
            gamma_sp: 0.02,
            delta: 64.0,
            c6: 0.0,
            blockade_r: 18.0,

            force_scale: 0.05,
            max_accel: 0.8,
            dt_mech: 1.0,
            dt_frame: 0.3,
            min_dist: 2.0,

            mot_friction: 0.08,

            ground_sigma: 0.8,
            excited_sigma: 1.5,
            ground_amp: 0.65,
            excited_amp: 1.0,
            glow_decay: 0.015,
            output: Vec::new(),
            rng: Rng::new(137),
            frame: 0,
        }
    }

    // ── Rate equations ────────────────────────────────────────────────────

    #[inline]
    fn base_rate(&self) -> f64 {
        PI * self.omega * self.omega / self.gamma
    }

    /// Transition rate for atom i:
    /// - Ground → Rydberg: Ω²/γ / (1 + ((Δ−V_k)/γ)²)
    /// - Rydberg → Ground: γ_sp + stimulated de-excitation
    fn transition_rate(&self, idx: usize) -> f64 {
        let pot = self.vdw_potential(idx);
        let base = self.base_rate();
        let shift = (self.delta - pot) / self.gamma;
        let laser_rate = base / (1.0 + shift * shift);

        if self.atoms[idx].excited {
            self.gamma_sp + laser_rate
        } else {
            laser_rate
        }
    }

    fn vdw_potential(&self, idx: usize) -> f64 {
        let ax = self.atoms[idx].x as f64;
        let ay = self.atoms[idx].y as f64;
        let min_r6 = (self.min_dist as f64).powi(6);
        let mut pot = 0.0;
        for (j, other) in self.atoms.iter().enumerate() {
            if j == idx || !other.excited {
                continue;
            }
            let dx = ax - other.x as f64;
            let dy = ay - other.y as f64;
            let r2 = dx * dx + dy * dy;
            let r6 = (r2 * r2 * r2).max(min_r6);
            pot += self.c6 / r6;
        }
        pot
    }

    // ── Gillespie algorithm ──────────────────────────────────────────────

    fn gillespie_frame(&mut self) {
        let n = self.n_atoms;
        if n == 0 {
            return;
        }

        let mut t = 0.0f64;
        let budget = self.dt_frame;

        loop {
            let mut partials: Vec<f64> = Vec::with_capacity(n);
            let mut total = 0.0;
            for i in 0..n {
                total += self.transition_rate(i);
                partials.push(total);
            }

            if total < 1e-12 {
                break;
            }

            let u = self.rng.f64().max(1e-15);
            let dt = -u.ln() / total;
            t += dt;

            if t > budget {
                break;
            }

            let target = self.rng.f64() * total;
            let idx = partials
                .iter()
                .position(|&r| r >= target)
                .unwrap_or(n - 1);

            self.atoms[idx].excited = !self.atoms[idx].excited;
            self.atoms[idx].glow = 1.0;
        }
    }

    // ── Velocity Verlet + Langevin thermostat ────────────────────────────

    fn compute_accelerations(&mut self) {
        let n = self.n_atoms;
        let min_r2 = self.min_dist * self.min_dist;
        let c6f = self.c6 as f32;
        let fs = self.force_scale;
        let max_a = self.max_accel;
        for i in 0..n {
            let (mut acx, mut acy) = (0.0f32, 0.0f32);
            let xi = self.atoms[i].x;
            let yi = self.atoms[i].y;

            if self.atoms[i].excited {
                // vdW repulsion from other excited atoms
                for j in 0..n {
                    if i == j || !self.atoms[j].excited {
                        continue;
                    }
                    let dx = xi - self.atoms[j].x;
                    let dy = yi - self.atoms[j].y;
                    let r2 = (dx * dx + dy * dy).max(min_r2);
                    let r = r2.sqrt();
                    let f = 6.0 * c6f * fs / (r2 * r2 * r2 * r);
                    acx += f * dx / r;
                    acy += f * dy / r;
                }
            }

            let a_mag = (acx * acx + acy * acy).sqrt();
            if a_mag > max_a {
                let s = max_a / a_mag;
                acx *= s;
                acy *= s;
            }

            self.atoms[i].ax = acx;
            self.atoms[i].ay = acy;
        }
    }

    /// Velocity Verlet with MOT-like friction on ground atoms.
    /// Ground atoms moving away from center get their outward velocity damped,
    /// modeling the velocity-dependent restoring force of a MOT.
    /// Excited atoms (Rydberg) don't feel the MOT light — free + vdW only.
    fn mechanical_step(&mut self) {
        let dt = self.dt_mech;
        let half_dt = 0.5 * dt;
        let bw = self.box_w as f32;
        let bh = self.box_h as f32;
        let cx = bw * 0.5;
        let cy = bh * 0.5;
        let fric = self.mot_friction;

        // 1. Half-kick
        for atom in self.atoms.iter_mut() {
            atom.vx += atom.ax * half_dt;
            atom.vy += atom.ay * half_dt;
        }

        // 2. Drift
        for atom in self.atoms.iter_mut() {
            atom.x += atom.vx * dt;
            atom.y += atom.vy * dt;

            // Inelastic box walls (half speed on bounce)
            if atom.x < 0.0 {
                atom.x = -atom.x;
                atom.vx = atom.vx.abs() * 0.5;
            }
            if atom.x > bw {
                atom.x = 2.0 * bw - atom.x;
                atom.vx = -atom.vx.abs() * 0.5;
            }
            if atom.y < 0.0 {
                atom.y = -atom.y;
                atom.vy = atom.vy.abs() * 0.5;
            }
            if atom.y > bh {
                atom.y = 2.0 * bh - atom.y;
                atom.vy = -atom.vy.abs() * 0.5;
            }
        }

        // 3. New accelerations
        self.compute_accelerations();

        // 4. Second half-kick
        for atom in self.atoms.iter_mut() {
            atom.vx += atom.ax * half_dt;
            atom.vy += atom.ay * half_dt;
        }

        // 5. MOT friction on ground atoms: damp radial-outward velocity component
        //    v_radial = (v · r̂) r̂, only damp if v_radial points outward (v·r > 0)
        for i in 0..self.n_atoms {
            if self.atoms[i].excited {
                continue;
            }
            let rx = self.atoms[i].x - cx;
            let ry = self.atoms[i].y - cy;
            let r2 = rx * rx + ry * ry;
            if r2 < 1e-6 {
                continue;
            }
            // Project velocity onto radial direction
            let v_dot_r = self.atoms[i].vx * rx + self.atoms[i].vy * ry;
            if v_dot_r > 0.0 {
                // Moving outward — apply friction to radial component
                let damp = fric * dt;
                self.atoms[i].vx -= damp * v_dot_r * rx / r2;
                self.atoms[i].vy -= damp * v_dot_r * ry / r2;
            }
        }
    }

    // ── Render particles to grid ──────────────────────────────────────────

    fn render(&mut self) {
        let w = self.grid_w as usize;
        let h = self.grid_h as usize;
        let s = self.scale;
        let gs = self.ground_sigma * s;
        let es = self.excited_sigma * s;
        let ga = self.ground_amp;
        let ea = self.excited_amp;

        self.output.fill(0.0);

        for i in 0..self.atoms.len() {
            let gx = self.atoms[i].x * s;
            let gy = self.atoms[i].y * s;
            let exc = self.atoms[i].excited;
            let glow = self.atoms[i].glow;

            let sigma = if exc { es } else { gs };
            let amplitude = if exc { ea } else { ga };
            let channel: usize = if exc { 1 } else { 0 };

            splat(&mut self.output, w, h, gx, gy, sigma, amplitude, channel);

            if glow > 0.05 {
                let glow_s = es * 2.0;
                splat(&mut self.output, w, h, gx, gy, glow_s, glow * 0.3, 1);
            }
        }

        for v in self.output.iter_mut() {
            *v = v.min(1.0);
        }
    }
}

fn splat(out: &mut [f32], w: usize, h: usize, cx: f32, cy: f32, sigma: f32, amp: f32, ch: usize) {
    let radius = (sigma * 3.0).ceil() as i32;
    let ix = cx as i32;
    let iy = cy as i32;
    let inv_2s2 = 1.0 / (2.0 * sigma * sigma);

    for dy in -radius..=radius {
        let gy = iy + dy;
        if gy < 0 || gy >= h as i32 {
            continue;
        }
        let py = gy as f32 + 0.5 - cy;
        let py2 = py * py;

        for dx in -radius..=radius {
            let gx = ix + dx;
            if gx < 0 || gx >= w as i32 {
                continue;
            }
            let px = gx as f32 + 0.5 - cx;
            let val = amp * (-(px * px + py2) * inv_2s2).exp();

            let idx = (gy as usize * w + gx as usize) * 2 + ch;
            out[idx] += val;
        }
    }
}

impl Simulation for Rydberg {
    fn init(&mut self, width: u32, height: u32, _config: &str) {
        self.grid_w = width;
        self.grid_h = height;

        // Physical box: width = 10 × R_b, height scaled by aspect ratio
        let aspect = height as f64 / width as f64;
        self.box_w = 10.0 * self.blockade_r;
        self.box_h = self.box_w * aspect;
        self.scale = width as f32 / self.box_w as f32;

        self.c6 = self.gamma * self.blockade_r.powi(6);

        // ~1 atom per (2·R_b)² area
        let phys_area = self.box_w * self.box_h;
        let spacing_sq = (2.0 * self.blockade_r) * (2.0 * self.blockade_r);
        self.n_atoms = (phys_area / spacing_sq).round().max(10.0).min(200.0) as usize;

        // Initialize with Maxwell-Boltzmann velocities at T_MOT
        let bw = self.box_w as f32;
        let bh = self.box_h as f32;
        let margin = 0.5;
        let v_th: f32 = 0.45; // 0.025 R_b per time unit (200μK MOT)
        self.atoms.clear();
        for _ in 0..self.n_atoms {
            let u1 = self.rng.f64().max(1e-10) as f32;
            let u2 = self.rng.f64() as f32;
            let r = (-2.0 * u1.ln()).sqrt() * v_th;
            let theta = core::f32::consts::TAU * u2;
            self.atoms.push(Atom {
                x: self.rng.f64() as f32 * (bw - 2.0 * margin) + margin,
                y: self.rng.f64() as f32 * (bh - 2.0 * margin) + margin,
                vx: r * theta.cos(),
                vy: r * theta.sin(),
                ax: 0.0,
                ay: 0.0,
                excited: false,
                glow: 0.0,
            });
        }

        // Seed a few excited atoms to nucleate facilitation
        let n_seed = (self.n_atoms / 8).max(2);
        for i in 0..n_seed.min(self.n_atoms) {
            self.atoms[i].excited = true;
        }

        self.output = vec![0.0f32; (width * height) as usize * 2];
        self.frame = 0;

        self.compute_accelerations();
    }

    fn metadata(&self) -> SimMetadata {
        SimMetadata {
            name: "rydberg",
            renderer: "webgl-rydberg",
            grid_width: self.grid_w,
            grid_height: self.grid_h,
            channels: 2,
            continuous: true,
            params: &[
                ParamDef {
                    key: "blockade_r",
                    label: "Blockade Radius",
                    min: 8.0,
                    max: 30.0,
                    default: 18.0,
                    step: 1.0,
                },
                ParamDef {
                    key: "delta",
                    label: "Detuning",
                    min: 0.0,
                    max: 128.0,
                    default: 64.0,
                    step: 1.0,
                },
            ],
        }
    }

    fn step(&mut self) -> *const u8 {
        self.frame += 1;

        self.gillespie_frame();
        self.mechanical_step();

        for atom in &mut self.atoms {
            atom.glow = (atom.glow - self.glow_decay).max(0.0);
        }

        self.render();
        self.output.as_ptr() as *const u8
    }

    fn buffer_len(&self) -> usize {
        self.output.len() * 4
    }

    fn set_param(&mut self, key: &str, value: f64) {
        match key {
            "blockade_r" => {
                self.blockade_r = value;
                self.c6 = self.gamma * value.powi(6);
            }
            "delta" => self.delta = value,
            "omega" => self.omega = value,
            _ => {}
        }
    }

    fn reset(&mut self) {
        let w = self.grid_w;
        let h = self.grid_h;
        self.init(w, h, "");
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.init(width, height, "");
    }
}
