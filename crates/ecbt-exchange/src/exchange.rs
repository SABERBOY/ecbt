#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Environment {
    Production,
    Sandbox,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Endpoint {
    Com,
    Us,
    Other,
}
