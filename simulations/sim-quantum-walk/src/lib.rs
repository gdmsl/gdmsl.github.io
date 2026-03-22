mod walk;

use sim_core::traits::Simulation;
use walk::QuantumWalk;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmSimulation {
    inner: QuantumWalk,
}

#[wasm_bindgen]
impl WasmSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: QuantumWalk::new(),
        }
    }

    pub fn init(&mut self, width: u32, height: u32, config_json: &str) {
        self.inner.init(width, height, config_json);
    }

    pub fn metadata_json(&self) -> String {
        let m = self.inner.metadata();
        format!(
            r#"{{"name":"{}","renderer":"{}","gridWidth":{},"gridHeight":{},"channels":{},"continuous":{}}}"#,
            m.name, m.renderer, m.grid_width, m.grid_height, m.channels, m.continuous
        )
    }

    /// Advance simulation, return normalized f32 probability array as Float32Array view.
    pub fn step(&mut self) -> js_sys::Float32Array {
        let ptr = self.inner.step();
        let byte_len = self.inner.buffer_len();
        let float_len = byte_len / 4;
        unsafe { js_sys::Float32Array::view_mut_raw(ptr as *mut f32, float_len) }
    }

    pub fn buffer_len(&self) -> usize {
        self.inner.buffer_len()
    }

    pub fn set_param(&mut self, key: &str, value: f64) {
        self.inner.set_param(key, value);
    }

    pub fn reset(&mut self) {
        self.inner.reset();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.inner.resize(width, height);
    }
}
