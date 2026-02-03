#[derive(Clone, Debug)]
pub struct Steps {
    /// The steps in minutes.
    steps: Vec<u32>,
}

impl Steps {
    pub fn new(steps: Vec<u32>) -> Self {
        Self { steps }
    }

    pub fn nth_as_mins(&self, position: usize) -> Option<u32> {
        self.at(position)
    }

    fn at(&self, position: usize) -> Option<u32> {
        self.steps.get(position).copied()
    }
}
