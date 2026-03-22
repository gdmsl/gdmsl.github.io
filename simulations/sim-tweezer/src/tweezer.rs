/// Optical tweezer atom sorting simulation.
///
/// Atoms randomly loaded into a 2D grid of optical traps are rearranged into
/// a compact, defect-free target region. Two algorithms alternate each cycle:
///
/// 1. **Hungarian** (optimal assignment): single tweezer, interstitial paths,
///    minimizes total displacement via the Kuhn–Munkres algorithm.
/// 2. **Compression**: parallel tweezers compress columns then rows toward
///    center, one hop per step, mimicking real experimental protocols.
///
/// Output: RG32F texture — R = traps + atoms, G = tweezer blob(s).

use sim_core::{rng::Rng, traits::*};

// ── Types ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum TrapState {
    Empty,
    Occupied,
}

#[derive(Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Clone)]
struct Move {
    source: Pos,
    target: Pos,
}

#[derive(Clone, Copy, PartialEq)]
enum SortMode {
    Hungarian,
    Compression,
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Idle,
    PickingUp,
    Moving,
    Dropping,
    Done,
}

/// A single compression step: all moves happen in parallel.
struct CompressStep {
    moves: Vec<(Pos, Pos)>,
}

/// State of one tweezer during a parallel compression step.
struct ParTweezer {
    x: f32,
    y: f32,
    target_x: f32,
    target_y: f32,
    src_r: usize,
    src_c: usize,
    tgt_r: usize,
    tgt_c: usize,
}

// ── Main struct ─────────────────────────────────────────────────────────────

pub struct Tweezer {
    grid_w: u32,
    grid_h: u32,

    trap_rows: usize,
    trap_cols: usize,
    traps: Vec<TrapState>,

    sort_mode: SortMode,
    cycle_count: usize,
    phase: Phase,
    phase_timer: f32,
    done_timer: f32,

    // ── Hungarian mode ──
    h_moves: Vec<Move>,
    h_current: usize,
    h_waypoints: Vec<(f32, f32)>,
    h_wp_idx: usize,
    h_tx: f32,
    h_ty: f32,
    h_carrying: bool,

    // ── Compression mode ──
    c_steps: Vec<CompressStep>,
    c_current: usize,
    c_tweezers: Vec<ParTweezer>,
    c_carrying: bool,

    // ── Params ──
    load_prob: f64,
    tweezer_speed: f32,

    // ── Layout (pixel coords) ──
    trap_sigma: f32,
    atom_sigma: f32,
    tweezer_sigma: f32,
    trap_spacing: f32,
    trap_offset_x: f32,
    trap_offset_y: f32,
    desired_spacing: f32,

    output: Vec<f32>,
    rng: Rng,
}

// ── Helpers ─────────────────────────────────────────────────────────────────

impl Tweezer {
    pub fn new() -> Self {
        Self {
            grid_w: 0,
            grid_h: 0,
            trap_rows: 0,
            trap_cols: 0,
            traps: Vec::new(),
            sort_mode: SortMode::Hungarian,
            cycle_count: 0,
            phase: Phase::Idle,
            phase_timer: 0.0,
            done_timer: 0.0,
            h_moves: Vec::new(),
            h_current: 0,
            h_waypoints: Vec::new(),
            h_wp_idx: 0,
            h_tx: 0.0,
            h_ty: 0.0,
            h_carrying: false,
            c_steps: Vec::new(),
            c_current: 0,
            c_tweezers: Vec::new(),
            c_carrying: false,
            load_prob: 0.5,
            tweezer_speed: 2.0,
            trap_sigma: 0.0,
            atom_sigma: 0.0,
            tweezer_sigma: 0.0,
            trap_spacing: 0.0,
            trap_offset_x: 0.0,
            trap_offset_y: 0.0,
            desired_spacing: 12.0,
            output: Vec::new(),
            rng: Rng::new(42),
        }
    }

    #[inline]
    fn trap_to_pixel(&self, row: usize, col: usize) -> (f32, f32) {
        (
            self.trap_offset_x + col as f32 * self.trap_spacing,
            self.trap_offset_y + row as f32 * self.trap_spacing,
        )
    }

    #[inline]
    fn frac_to_pixel(&self, row: f32, col: f32) -> (f32, f32) {
        (
            self.trap_offset_x + col * self.trap_spacing,
            self.trap_offset_y + row * self.trap_spacing,
        )
    }

    #[inline]
    fn idx(&self, r: usize, c: usize) -> usize {
        idx(r, c, self.trap_cols)
    }

    // ── Randomize + plan ────────────────────────────────────────────────────

    fn randomize_and_plan(&mut self) {
        let n = self.trap_rows * self.trap_cols;
        self.traps.clear();
        self.traps.resize(n, TrapState::Empty);
        for t in self.traps.iter_mut() {
            if self.rng.f64() < self.load_prob {
                *t = TrapState::Occupied;
            }
        }

        if self.cycle_count % 2 == 0 {
            self.sort_mode = SortMode::Hungarian;
            self.plan_hungarian();
        } else {
            self.sort_mode = SortMode::Compression;
            self.plan_compression();
        }
        self.cycle_count += 1;
        self.phase = Phase::Idle;

        // Park Hungarian tweezer at center
        let (cx, cy) = self.trap_to_pixel(self.trap_rows / 2, self.trap_cols / 2);
        self.h_tx = cx;
        self.h_ty = cy;
        self.h_carrying = false;
        self.c_carrying = false;
    }

    // ── Hungarian (optimal assignment) ──────────────────────────────────────

    fn plan_hungarian(&mut self) {
        self.h_moves.clear();
        self.h_current = 0;

        let rows = self.trap_rows;
        let cols = self.trap_cols;
        let n_atoms = self.traps.iter().filter(|t| **t == TrapState::Occupied).count();
        if n_atoms == 0 {
            return;
        }

        // Target rectangle (same aspect as grid)
        let aspect = cols as f64 / rows as f64;
        let tr = (n_atoms as f64 / aspect).sqrt().ceil() as usize;
        let tc = (tr as f64 * aspect).ceil() as usize;
        let tr = tr.min(rows);
        let tc = tc.min(cols);
        let row_off = rows.saturating_sub(tr) / 2;
        let col_off = cols.saturating_sub(tc) / 2;

        let cr = (rows as f32 - 1.0) / 2.0;
        let cc = (cols as f32 - 1.0) / 2.0;

        let mut targets: Vec<Pos> = Vec::new();
        for r in row_off..row_off + tr {
            for c in col_off..col_off + tc {
                targets.push(Pos { row: r, col: c });
            }
        }
        targets.sort_by(|a, b| {
            let da = (a.row as f32 - cr).powi(2) + (a.col as f32 - cc).powi(2);
            let db = (b.row as f32 - cr).powi(2) + (b.col as f32 - cc).powi(2);
            da.partial_cmp(&db).unwrap()
        });
        targets.truncate(n_atoms);

        let mut tgt_set = vec![false; rows * cols];
        for p in &targets {
            tgt_set[self.idx(p.row, p.col)] = true;
        }

        let mut empty_tgts: Vec<Pos> = Vec::new();
        let mut sources: Vec<Pos> = Vec::new();
        for p in &targets {
            if self.traps[self.idx(p.row, p.col)] == TrapState::Empty {
                empty_tgts.push(*p);
            }
        }
        for r in 0..rows {
            for c in 0..cols {
                let i = self.idx(r, c);
                if self.traps[i] == TrapState::Occupied && !tgt_set[i] {
                    sources.push(Pos { row: r, col: c });
                }
            }
        }

        let n = sources.len();
        if n == 0 {
            return;
        }

        // Build cost matrix (squared distance)
        let mut cost = vec![vec![0i32; n]; n];
        for i in 0..n {
            for j in 0..n {
                let dr = sources[i].row as i32 - empty_tgts[j].row as i32;
                let dc = sources[i].col as i32 - empty_tgts[j].col as i32;
                cost[i][j] = dr * dr + dc * dc;
            }
        }

        let assignment = hungarian(&cost);

        for (si, ti) in assignment {
            self.h_moves.push(Move {
                source: sources[si],
                target: empty_tgts[ti],
            });
        }
    }

    fn hungarian_interstitial(&self, source: Pos, target: Pos) -> Vec<(f32, f32)> {
        let mut wps = Vec::new();
        let sr = source.row as f32;
        let sc = source.col as f32;
        let tr = target.row as f32;
        let tc = target.col as f32;

        let col_off: f32 = if tc >= sc { 0.5 } else { -0.5 };
        let row_off: f32 = if tr >= sr { 0.5 } else { -0.5 };

        let ic_src = sc + col_off;
        let ir_tgt = tr - row_off;
        let ic_tgt = tc - col_off;

        wps.push(self.frac_to_pixel(sr, ic_src));
        if (ir_tgt - sr).abs() > 0.01 {
            wps.push(self.frac_to_pixel(ir_tgt, ic_src));
        }
        if (ic_tgt - ic_src).abs() > 0.01 {
            wps.push(self.frac_to_pixel(ir_tgt, ic_tgt));
        }
        wps.push(self.frac_to_pixel(tr, tc));
        wps
    }

    // ── Compression (parallel tweezers) ─────────────────────────────────────

    fn plan_compression(&mut self) {
        self.c_steps.clear();
        self.c_current = 0;

        let rows = self.trap_rows;
        let cols = self.trap_cols;
        let mut grid = self.traps.clone();

        // Phase 1: column compression (horizontal 1-hop moves, all rows parallel)
        loop {
            let mut step = Vec::new();
            for r in 0..rows {
                let occ: Vec<usize> =
                    (0..cols).filter(|&c| grid[r * cols + c] == TrapState::Occupied).collect();
                let k = occ.len();
                if k == 0 {
                    continue;
                }
                let off = cols.saturating_sub(k) / 2;
                for (i, &oc) in occ.iter().enumerate() {
                    let tgt = off + i;
                    if oc < tgt {
                        step.push((
                            Pos { row: r, col: oc },
                            Pos { row: r, col: oc + 1 },
                        ));
                    } else if oc > tgt {
                        step.push((
                            Pos { row: r, col: oc },
                            Pos { row: r, col: oc - 1 },
                        ));
                    }
                }
            }
            if step.is_empty() {
                break;
            }
            // Apply to working grid
            for &(src, _) in &step {
                grid[src.row * cols + src.col] = TrapState::Empty;
            }
            for &(_, tgt) in &step {
                grid[tgt.row * cols + tgt.col] = TrapState::Occupied;
            }
            self.c_steps.push(CompressStep { moves: step });
        }

        // Phase 2: row compression (vertical 1-hop moves, all cols parallel)
        loop {
            let mut step = Vec::new();
            for c in 0..cols {
                let occ: Vec<usize> =
                    (0..rows).filter(|&r| grid[r * cols + c] == TrapState::Occupied).collect();
                let k = occ.len();
                if k == 0 {
                    continue;
                }
                let off = rows.saturating_sub(k) / 2;
                for (i, &or) in occ.iter().enumerate() {
                    let tgt = off + i;
                    if or < tgt {
                        step.push((
                            Pos { row: or, col: c },
                            Pos { row: or + 1, col: c },
                        ));
                    } else if or > tgt {
                        step.push((
                            Pos { row: or, col: c },
                            Pos { row: or - 1, col: c },
                        ));
                    }
                }
            }
            if step.is_empty() {
                break;
            }
            for &(src, _) in &step {
                grid[src.row * cols + src.col] = TrapState::Empty;
            }
            for &(_, tgt) in &step {
                grid[tgt.row * cols + tgt.col] = TrapState::Occupied;
            }
            self.c_steps.push(CompressStep { moves: step });
        }
    }

    // ── Move helper ─────────────────────────────────────────────────────────

    /// Returns true when (x,y) has reached (tx,ty).
    fn advance(x: &mut f32, y: &mut f32, tx: f32, ty: f32, speed: f32) -> bool {
        let dx = tx - *x;
        let dy = ty - *y;
        let d = (dx * dx + dy * dy).sqrt();
        if d <= speed {
            *x = tx;
            *y = ty;
            true
        } else {
            let s = speed / d;
            *x += dx * s;
            *y += dy * s;
            false
        }
    }

    // ── Tick ────────────────────────────────────────────────────────────────

    fn tick(&mut self) {
        match self.sort_mode {
            SortMode::Hungarian => self.tick_hungarian(),
            SortMode::Compression => self.tick_compression(),
        }
    }

    fn tick_hungarian(&mut self) {
        match self.phase {
            Phase::Idle => {
                if self.h_current >= self.h_moves.len() {
                    self.phase = Phase::Done;
                    self.done_timer = 120.0;
                    return;
                }
                // Jump to source
                let m = &self.h_moves[self.h_current];
                let (sx, sy) = self.trap_to_pixel(m.source.row, m.source.col);
                self.h_tx = sx;
                self.h_ty = sy;
                self.phase = Phase::PickingUp;
                self.phase_timer = 15.0;
            }
            Phase::PickingUp => {
                self.phase_timer -= 1.0;
                if self.phase_timer <= 0.0 {
                    let m = self.h_moves[self.h_current].clone();
                    let cols = self.trap_cols;
                    self.traps[idx(m.source.row, m.source.col, cols)] = TrapState::Empty;
                    self.h_carrying = true;
                    self.h_waypoints = self.hungarian_interstitial(m.source, m.target);
                    self.h_wp_idx = 0;
                    self.phase = Phase::Moving;
                }
            }
            Phase::Moving => {
                if self.h_wp_idx < self.h_waypoints.len() {
                    let (tx, ty) = self.h_waypoints[self.h_wp_idx];
                    if Self::advance(&mut self.h_tx, &mut self.h_ty, tx, ty, self.tweezer_speed) {
                        self.h_wp_idx += 1;
                    }
                }
                if self.h_wp_idx >= self.h_waypoints.len() {
                    self.phase = Phase::Dropping;
                    self.phase_timer = 15.0;
                }
            }
            Phase::Dropping => {
                self.phase_timer -= 1.0;
                if self.phase_timer <= 0.0 {
                    let m = &self.h_moves[self.h_current];
                    let cols = self.trap_cols;
                    self.traps[idx(m.target.row, m.target.col, cols)] = TrapState::Occupied;
                    self.h_carrying = false;
                    self.h_current += 1;
                    self.phase = Phase::Idle;
                }
            }
            Phase::Done => {
                self.done_timer -= 1.0;
                if self.done_timer <= 0.0 {
                    self.randomize_and_plan();
                }
            }
        }
    }

    fn tick_compression(&mut self) {
        match self.phase {
            Phase::Idle => {
                if self.c_current >= self.c_steps.len() {
                    self.phase = Phase::Done;
                    self.done_timer = 120.0;
                    return;
                }
                // Spawn parallel tweezers at source positions
                let step = &self.c_steps[self.c_current];
                self.c_tweezers.clear();
                for &(src, tgt) in &step.moves {
                    let (sx, sy) = self.trap_to_pixel(src.row, src.col);
                    let (tx, ty) = self.trap_to_pixel(tgt.row, tgt.col);
                    self.c_tweezers.push(ParTweezer {
                        x: sx,
                        y: sy,
                        target_x: tx,
                        target_y: ty,
                        src_r: src.row,
                        src_c: src.col,
                        tgt_r: tgt.row,
                        tgt_c: tgt.col,
                    });
                }
                self.phase = Phase::PickingUp;
                self.phase_timer = 8.0;
            }
            Phase::PickingUp => {
                self.phase_timer -= 1.0;
                if self.phase_timer <= 0.0 {
                    // Remove atoms from sources
                    let cols = self.trap_cols;
                    for tw in &self.c_tweezers {
                        self.traps[idx(tw.src_r, tw.src_c, cols)] = TrapState::Empty;
                    }
                    self.c_carrying = true;
                    self.phase = Phase::Moving;
                }
            }
            Phase::Moving => {
                let spd = self.tweezer_speed;
                let mut all_done = true;
                for tw in &mut self.c_tweezers {
                    if !Self::advance(&mut tw.x, &mut tw.y, tw.target_x, tw.target_y, spd) {
                        all_done = false;
                    }
                }
                if all_done {
                    self.phase = Phase::Dropping;
                    self.phase_timer = 8.0;
                }
            }
            Phase::Dropping => {
                self.phase_timer -= 1.0;
                if self.phase_timer <= 0.0 {
                    // Place atoms at targets
                    let cols = self.trap_cols;
                    for tw in &self.c_tweezers {
                        self.traps[idx(tw.tgt_r, tw.tgt_c, cols)] = TrapState::Occupied;
                    }
                    self.c_carrying = false;
                    self.c_current += 1;
                    self.phase = Phase::Idle;
                }
            }
            Phase::Done => {
                self.done_timer -= 1.0;
                if self.done_timer <= 0.0 {
                    self.randomize_and_plan();
                }
            }
        }
    }

    // ── Render ──────────────────────────────────────────────────────────────

    fn render(&mut self) {
        let w = self.grid_w as usize;
        let h = self.grid_h as usize;
        self.output.fill(0.0);

        let rows = self.trap_rows;
        let cols = self.trap_cols;

        // Splat all traps (dim, R channel)
        for r in 0..rows {
            for c in 0..cols {
                let (px, py) = self.trap_to_pixel(r, c);
                splat(&mut self.output, w, h, px, py, self.trap_sigma, 0.15, 0);
            }
        }

        // Splat occupied traps (bright, R channel)
        for r in 0..rows {
            for c in 0..cols {
                if self.traps[self.idx(r, c)] == TrapState::Occupied {
                    let (px, py) = self.trap_to_pixel(r, c);
                    splat(&mut self.output, w, h, px, py, self.atom_sigma, 1.0, 0);
                }
            }
        }

        match self.sort_mode {
            SortMode::Hungarian => {
                // Carried atom
                if self.h_carrying {
                    splat(
                        &mut self.output, w, h,
                        self.h_tx, self.h_ty,
                        self.atom_sigma, 1.0, 0,
                    );
                }
                // Single tweezer blob
                splat(
                    &mut self.output, w, h,
                    self.h_tx, self.h_ty,
                    self.tweezer_sigma, 1.0, 1,
                );
            }
            SortMode::Compression => {
                for tw in &self.c_tweezers {
                    // Carried atom
                    if self.c_carrying {
                        splat(
                            &mut self.output, w, h,
                            tw.x, tw.y,
                            self.atom_sigma, 1.0, 0,
                        );
                    }
                    // Tweezer blob
                    splat(
                        &mut self.output, w, h,
                        tw.x, tw.y,
                        self.tweezer_sigma, 1.0, 1,
                    );
                }
            }
        }

        for v in self.output.iter_mut() {
            *v = v.min(1.0);
        }
    }
}

#[inline]
fn idx(r: usize, c: usize, cols: usize) -> usize {
    r * cols + c
}

// ── Gaussian splat ──────────────────────────────────────────────────────────

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

// ── Hungarian algorithm (Kuhn–Munkres, O(n³)) ──────────────────────────────

/// Returns Vec<(source_index, target_index)> for minimum-cost assignment.
fn hungarian(cost: &[Vec<i32>]) -> Vec<(usize, usize)> {
    let n = cost.len();
    if n == 0 {
        return vec![];
    }

    let mut u = vec![0i64; n + 1];
    let mut v = vec![0i64; n + 1];
    let mut p = vec![0usize; n + 1]; // p[j] = row assigned to col j
    let mut way = vec![0usize; n + 1];

    for i in 1..=n {
        p[0] = i;
        let mut j0: usize = 0;
        let mut minv = vec![i64::MAX; n + 1];
        let mut used = vec![false; n + 1];

        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = i64::MAX;
            let mut j1: usize = 0;

            for j in 1..=n {
                if used[j] {
                    continue;
                }
                let cur = cost[i0 - 1][j - 1] as i64 - u[i0] - v[j];
                if cur < minv[j] {
                    minv[j] = cur;
                    way[j] = j0;
                }
                if minv[j] < delta {
                    delta = minv[j];
                    j1 = j;
                }
            }

            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }

        loop {
            let prev = way[j0];
            p[j0] = p[prev];
            j0 = prev;
            if j0 == 0 {
                break;
            }
        }
    }

    let mut result = Vec::with_capacity(n);
    for j in 1..=n {
        if p[j] != 0 {
            result.push((p[j] - 1, j - 1));
        }
    }
    result
}

// ── Simulation trait ────────────────────────────────────────────────────────

impl Simulation for Tweezer {
    fn init(&mut self, width: u32, height: u32, _config: &str) {
        self.grid_w = width;
        self.grid_h = height;

        let spacing = self.desired_spacing;
        let margin = spacing * 0.5;
        let usable_w = width as f32 - 2.0 * margin;
        let usable_h = height as f32 - 2.0 * margin;

        self.trap_cols = ((usable_w / spacing).floor() as usize + 1).max(3);
        self.trap_rows = ((usable_h / spacing).floor() as usize + 1).max(3);
        self.trap_spacing = spacing;

        let gw = spacing * (self.trap_cols - 1) as f32;
        let gh = spacing * (self.trap_rows - 1) as f32;
        self.trap_offset_x = (width as f32 - gw) * 0.5;
        self.trap_offset_y = (height as f32 - gh) * 0.5;

        self.atom_sigma = spacing * 0.10;
        self.trap_sigma = spacing * 0.22;
        self.tweezer_sigma = spacing * 0.35;
        self.tweezer_speed = spacing * 0.15;

        self.output = vec![0.0f32; (width * height) as usize * 2];

        self.randomize_and_plan();
    }

    fn metadata(&self) -> SimMetadata {
        SimMetadata {
            name: "tweezer",
            renderer: "webgl-tweezer",
            grid_width: self.grid_w,
            grid_height: self.grid_h,
            channels: 2,
            continuous: true,
            params: &[
                ParamDef {
                    key: "load_prob",
                    label: "Load Probability",
                    min: 0.2,
                    max: 0.8,
                    default: 0.5,
                    step: 0.05,
                },
                ParamDef {
                    key: "speed",
                    label: "Speed",
                    min: 0.5,
                    max: 5.0,
                    default: 2.0,
                    step: 0.25,
                },
            ],
        }
    }

    fn step(&mut self) -> *const u8 {
        self.tick();
        self.render();
        self.output.as_ptr() as *const u8
    }

    fn buffer_len(&self) -> usize {
        self.output.len() * 4
    }

    fn set_param(&mut self, key: &str, value: f64) {
        match key {
            "load_prob" => self.load_prob = value,
            "speed" => {
                self.tweezer_speed = self.trap_spacing * 0.15 * value as f32 / 2.0;
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        let (w, h) = (self.grid_w, self.grid_h);
        self.init(w, h, "");
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.init(width, height, "");
    }
}
