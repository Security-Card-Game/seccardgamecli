use log::warn;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct ResourceFixMultiplier(usize);

impl ResourceFixMultiplier {
    pub fn new(value: usize) -> Self {
        if value < 1 {
            warn!("Modifier must not be 0. Setting it to 1!")
        }
        ResourceFixMultiplier(value)
    }

    pub fn value(&self) -> &usize {
        &self.0
    }
}

impl Default for ResourceFixMultiplier {
    fn default() -> Self {
        ResourceFixMultiplier(1)
    }
}
