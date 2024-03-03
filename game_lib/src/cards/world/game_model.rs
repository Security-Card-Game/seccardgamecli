#[derive(Clone, Debug)]
pub struct Resources(pub(crate) usize);

impl Resources {
    pub fn new(value: usize) -> Self {
        Resources(value)
    }
}

impl Default for Resources {
    fn default() -> Self {
        Resources(0)
    }
}
