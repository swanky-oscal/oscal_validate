#[derive(Debug, Clone)]
pub struct ValidationConfig {}

impl ValidationConfig {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for ValidationConfig {
    fn default() -> Self {
        Self::new()
    }
}
