use sim_core::lattice;
use sim_core::rng::Rng;
use sim_core::traits::{ParamDef, SimMetadata, Simulation};

/// 2D Ising model with Swendsen-Wang cluster algorithm.
///
/// Outputs 2 floats per site: [spin (0/1), glow (0..1)].
/// The glow tracks recently-flipped clusters for visual effect.
pub struct Ising {
    width: u32,
    height: u32,
    /// Spin configuration: +1 or -1.
    spins: Vec<i8>,
    /// Cluster labels (union-find parent array).
    parent: Vec<u32>,
    /// Cluster rank for union-find.
    rank: Vec<u8>,
    /// Glow per site: set when cluster flips, decays over frames.
    glow: Vec<f32>,
    /// Output buffer: [spin, glow] interleaved.
    output: Vec<f32>,
    rng: Rng,
    /// Inverse temperature β. Tc ≈ 2.269 → βc ≈ 0.4407.
    beta: f64,
    /// Bond probability p = 1 - exp(-2β) for the standard Ising.
    bond_prob: f64,
    /// Frames between SW sweeps.
    sweep_interval: u32,
    /// Frame counter.
    frame: u32,
    /// Glow decay rate per frame.
    glow_decay: f32,
    /// Whether beta oscillates around Tc.
    oscillate: bool,
    /// Phase of oscillation.
    phase: f64,
}

impl Ising {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            spins: Vec::new(),
            parent: Vec::new(),
            rank: Vec::new(),
            glow: Vec::new(),
            output: Vec::new(),
            rng: Rng::new(137),
            beta: 0.4407,
            bond_prob: 0.0,
            sweep_interval: 40,
            frame: 0,
            glow_decay: 0.03,
            oscillate: true,
            phase: 0.0,
        }
    }

    fn update_bond_prob(&mut self) {
        self.bond_prob = 1.0 - (-2.0 * self.beta).exp();
    }

    // ── Union-Find ──

    fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
            x = self.parent[x as usize];
        }
        x
    }

    fn union(&mut self, a: u32, b: u32) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.rank[ra as usize] < self.rank[rb as usize] {
            self.parent[ra as usize] = rb;
        } else if self.rank[ra as usize] > self.rank[rb as usize] {
            self.parent[rb as usize] = ra;
        } else {
            self.parent[rb as usize] = ra;
            self.rank[ra as usize] += 1;
        }
    }

    // ── Swendsen-Wang sweep ──

    fn sw_sweep(&mut self) {
        let w = self.width;
        let h = self.height;
        let n = lattice::num_sites(w, h);

        // Reset union-find: each site is its own root
        for i in 0..n {
            self.parent[i] = i as u32;
            self.rank[i] = 0;
        }

        // Build bonds: for each edge, if spins agree, bond with probability p
        for y in 0..h {
            for x in 0..w {
                let i = lattice::idx(x, y, w);
                let si = self.spins[i];

                // Right neighbor
                let nx = lattice::wrap(x as i32 + 1, w);
                let j = lattice::idx(nx, y, w);
                if si == self.spins[j] && self.rng.f64() < self.bond_prob {
                    self.union(i as u32, j as u32);
                }

                // Down neighbor
                let ny = lattice::wrap(y as i32 + 1, h);
                let j = lattice::idx(x, ny, w);
                if si == self.spins[j] && self.rng.f64() < self.bond_prob {
                    self.union(i as u32, j as u32);
                }
            }
        }

        // For each cluster root, decide to flip (50% chance)
        // Track which roots flip so we can set glow
        let mut flip_root = vec![false; n];
        let mut decided = vec![false; n];

        for i in 0..n {
            let root = self.find(i as u32) as usize;
            if !decided[root] {
                decided[root] = true;
                flip_root[root] = self.rng.f64() < 0.5;
            }
        }

        // Apply flips and set glow
        for i in 0..n {
            let root = self.find(i as u32) as usize;
            if flip_root[root] {
                self.spins[i] = -self.spins[i];
                self.glow[i] = 1.0; // light up
            }
        }
    }

    fn render(&mut self) {
        let n = lattice::num_sites(self.width, self.height);
        for i in 0..n {
            let base = i * 2;
            // Spin: 0.0 = down, 1.0 = up
            self.output[base] = if self.spins[i] > 0 { 1.0 } else { 0.0 };
            // Glow
            self.output[base + 1] = self.glow[i];
        }
    }
}

impl Simulation for Ising {
    fn init(&mut self, width: u32, height: u32, _config_json: &str) {
        self.width = width;
        self.height = height;
        let n = lattice::num_sites(width, height);

        // Random initial spins
        self.spins = (0..n)
            .map(|_| if self.rng.f64() < 0.5 { 1i8 } else { -1i8 })
            .collect();
        self.parent = (0..n as u32).collect();
        self.rank = vec![0u8; n];
        self.glow = vec![0.0f32; n];
        self.output = vec![0.0f32; n * 2];
        self.frame = 0;
        self.phase = 0.0;

        self.update_bond_prob();

        // Thermalize: run several SW sweeps to reach equilibrium
        for _ in 0..30 {
            self.sw_sweep();
        }
        // Clear glow after thermalization
        for g in self.glow.iter_mut() {
            *g = 0.0;
        }
    }

    fn metadata(&self) -> SimMetadata {
        SimMetadata {
            name: "ising",
            renderer: "webgl-ising",
            grid_width: self.width,
            grid_height: self.height,
            channels: 2, // spin + glow
            continuous: true,
            params: &[
                ParamDef {
                    key: "beta",
                    label: "Inverse Temperature",
                    min: 0.2,
                    max: 0.8,
                    default: 0.4407,
                    step: 0.01,
                },
                ParamDef {
                    key: "sweep_interval",
                    label: "Frames per Sweep",
                    min: 10.0,
                    max: 120.0,
                    default: 40.0,
                    step: 1.0,
                },
                ParamDef {
                    key: "oscillate",
                    label: "Oscillate Temperature",
                    min: 0.0,
                    max: 1.0,
                    default: 1.0,
                    step: 1.0,
                },
            ],
        }
    }

    fn step(&mut self) -> *const u8 {
        self.frame += 1;

        // Oscillate beta around Tc for visual interest
        if self.oscillate {
            self.phase += 0.002;
            // Slow oscillation: β = 0.4407 ± 0.08
            self.beta = 0.4407 + 0.08 * self.phase.sin();
            self.update_bond_prob();
        }

        // Run SW sweep at interval
        if self.frame % self.sweep_interval == 0 {
            self.sw_sweep();
        }

        // Decay glow
        for g in self.glow.iter_mut() {
            *g = (*g - self.glow_decay).max(0.0);
        }

        self.render();
        self.output.as_ptr() as *const u8
    }

    fn buffer_len(&self) -> usize {
        self.output.len() * 4 // f32 = 4 bytes
    }

    fn set_param(&mut self, key: &str, value: f64) {
        match key {
            "beta" => {
                self.beta = value;
                self.update_bond_prob();
            }
            "sweep_interval" => self.sweep_interval = (value as u32).max(1),
            "oscillate" => self.oscillate = value > 0.5,
            _ => {}
        }
    }

    fn reset(&mut self) {
        let w = self.width;
        let h = self.height;
        self.init(w, h, "");
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.init(width, height, "");
    }
}
