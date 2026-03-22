/// Metadata describing a simulation's capabilities and requirements.
#[derive(Clone, Debug)]
pub struct SimMetadata {
    pub name: &'static str,
    pub renderer: &'static str,
    pub grid_width: u32,
    pub grid_height: u32,
    pub channels: u8,
    pub continuous: bool,
    pub params: &'static [ParamDef],
}

/// Definition of a tunable parameter exposed to JS.
#[derive(Clone, Debug)]
pub struct ParamDef {
    pub key: &'static str,
    pub label: &'static str,
    pub min: f64,
    pub max: f64,
    pub default: f64,
    pub step: f64,
}

/// The contract every simulation must implement.
pub trait Simulation {
    fn init(&mut self, width: u32, height: u32, config_json: &str);
    fn metadata(&self) -> SimMetadata;
    fn step(&mut self) -> *const u8;
    fn buffer_len(&self) -> usize;
    fn set_param(&mut self, key: &str, value: f64);
    fn reset(&mut self);
    fn resize(&mut self, width: u32, height: u32);
}
