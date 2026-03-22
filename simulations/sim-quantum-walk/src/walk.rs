use sim_core::complex::Complex;
use sim_core::lattice;
use sim_core::rng::Rng;
use sim_core::traits::{ParamDef, SimMetadata, Simulation};

const NUM_DIRS: usize = 4;

/// Quantum random walk on a 2D lattice with Grover coin.
///
/// Outputs normalized f32 probabilities. All visuals handled by GPU shader.
pub struct QuantumWalk {
    width: u32,
    height: u32,
    state: Vec<Complex>,
    scratch: Vec<Complex>,
    /// Smoothed probability (EMA, unnormalized).
    smooth_prob: Vec<f32>,
    /// Normalized output [0,1] for the shader.
    output: Vec<f32>,
    rng: Rng,
    steps_since_inject: u32,
    inject_interval: u32,
    steps_per_frame: u32,
    total_steps: u32,
}

impl QuantumWalk {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            state: Vec::new(),
            scratch: Vec::new(),
            smooth_prob: Vec::new(),
            output: Vec::new(),
            rng: Rng::new(42),
            steps_since_inject: 0,
            inject_interval: 6,
            steps_per_frame: 2,
            total_steps: 0,
        }
    }

    fn coin_step(&mut self) {
        let n = lattice::num_sites(self.width, self.height);
        for site in 0..n {
            let base = site * NUM_DIRS;
            let a = self.state[base];
            let b = self.state[base + 1];
            let c = self.state[base + 2];
            let d = self.state[base + 3];
            let half_sum = (a + b + c + d).scale(0.5);
            self.state[base] = half_sum - a;
            self.state[base + 1] = half_sum - b;
            self.state[base + 2] = half_sum - c;
            self.state[base + 3] = half_sum - d;
        }
    }

    fn shift_step(&mut self) {
        let w = self.width;
        let h = self.height;
        for c in self.scratch.iter_mut() {
            *c = Complex::ZERO;
        }
        let n = lattice::num_sites(w, h);
        for site in 0..n {
            let x = (site as u32) % w;
            let y = (site as u32) / w;
            for dir in 0..NUM_DIRS {
                let amp = self.state[site * NUM_DIRS + dir];
                if amp.norm_sq() < 1e-30 {
                    continue;
                }
                let (dx, dy) = lattice::DIRS[dir];
                let nx = lattice::wrap(x as i32 + dx, w);
                let ny = lattice::wrap(y as i32 + dy, h);
                let target = lattice::idx(nx, ny, w);
                self.scratch[target * NUM_DIRS + dir] =
                    self.scratch[target * NUM_DIRS + dir] + amp;
            }
        }
        std::mem::swap(&mut self.state, &mut self.scratch);
    }

    fn inject_walker(&mut self) {
        let w = self.width;
        let h = self.height;
        let x = self.gauss_coord(w);
        let y = self.gauss_coord(h);
        let dir = self.rng.usize(NUM_DIRS);
        let site = lattice::idx(x, y, w);
        let phase_angle = self.total_steps as f64 * 0.37;
        let amp = Complex::new(0.3 * phase_angle.cos(), 0.3 * phase_angle.sin());
        self.state[site * NUM_DIRS + dir] = self.state[site * NUM_DIRS + dir] + amp;
    }

    fn gauss_coord(&mut self, size: u32) -> u32 {
        let a = self.rng.f64();
        let b = self.rng.f64();
        let c = self.rng.f64();
        ((a + b + c) / 3.0 * size as f64).min(size as f64 - 1.0) as u32
    }

    /// Update EMA, normalize into separate output buffer.
    fn update_display(&mut self) -> *const f32 {
        let n = lattice::num_sites(self.width, self.height);
        let smooth = 0.08_f32;
        let mut max_prob: f32 = 0.0;

        for site in 0..n {
            let base = site * NUM_DIRS;
            let prob: f32 = (0..NUM_DIRS)
                .map(|d| self.state[base + d].norm_sq() as f32)
                .sum();

            // EMA on the raw (unnormalized) probability
            self.smooth_prob[site] =
                self.smooth_prob[site] * (1.0 - smooth) + prob * smooth;

            if self.smooth_prob[site] > max_prob {
                max_prob = self.smooth_prob[site];
            }
        }

        // Write normalized values to separate output buffer
        let inv = if max_prob > 1e-10 { 1.0 / max_prob } else { 0.0 };
        for site in 0..n {
            self.output[site] = self.smooth_prob[site] * inv;
        }

        self.output.as_ptr()
    }
}

impl Simulation for QuantumWalk {
    fn init(&mut self, width: u32, height: u32, _config_json: &str) {
        self.width = width;
        self.height = height;
        let n_sites = lattice::num_sites(width, height);
        self.state = vec![Complex::ZERO; n_sites * NUM_DIRS];
        self.scratch = vec![Complex::ZERO; n_sites * NUM_DIRS];
        self.smooth_prob = vec![0.0_f32; n_sites];
        self.output = vec![0.0_f32; n_sites];
        self.total_steps = 0;

        for _ in 0..20 {
            self.inject_walker();
            self.total_steps += 1;
        }
        for _ in 0..60 {
            self.coin_step();
            self.shift_step();
            self.total_steps += 1;
        }
        // Prime display
        for site in 0..n_sites {
            let base = site * NUM_DIRS;
            let prob: f32 = (0..NUM_DIRS)
                .map(|d| self.state[base + d].norm_sq() as f32)
                .sum();
            self.smooth_prob[site] = prob;
        }
    }

    fn metadata(&self) -> SimMetadata {
        SimMetadata {
            name: "quantum-walk",
            renderer: "webgl-hero",
            grid_width: self.width,
            grid_height: self.height,
            channels: 1,
            continuous: true,
            params: &[
                ParamDef {
                    key: "inject_interval",
                    label: "Injection Interval",
                    min: 2.0,
                    max: 30.0,
                    default: 6.0,
                    step: 1.0,
                },
                ParamDef {
                    key: "steps_per_frame",
                    label: "Steps per Frame",
                    min: 1.0,
                    max: 8.0,
                    default: 2.0,
                    step: 1.0,
                },
            ],
        }
    }

    fn step(&mut self) -> *const u8 {
        for _ in 0..self.steps_per_frame {
            self.coin_step();
            self.shift_step();
            self.total_steps += 1;

            self.steps_since_inject += 1;
            if self.steps_since_inject >= self.inject_interval {
                self.inject_walker();
                self.steps_since_inject = 0;
            }
        }

        let decay = 0.998_f64;
        for amp in self.state.iter_mut() {
            *amp = amp.scale(decay);
        }

        self.update_display() as *const u8
    }

    fn buffer_len(&self) -> usize {
        self.output.len() * 4
    }

    fn set_param(&mut self, key: &str, value: f64) {
        match key {
            "inject_interval" => self.inject_interval = value as u32,
            "steps_per_frame" => self.steps_per_frame = (value as u32).max(1),
            "inject" => {
                if value > 0.5 {
                    self.inject_walker();
                }
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        for amp in self.state.iter_mut() {
            *amp = Complex::ZERO;
        }
        for p in self.smooth_prob.iter_mut() {
            *p = 0.0;
        }
        self.steps_since_inject = 0;
        self.total_steps = 0;
        for _ in 0..20 {
            self.inject_walker();
            self.total_steps += 1;
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.init(width, height, "");
    }
}
