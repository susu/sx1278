
use utils::hal;

pub struct MockPin {
    pub set: bool
}

impl hal::digital::OutputPin for MockPin {
    fn is_low(&self) -> bool { !self.set }
    fn is_high(&self) -> bool { self.set }
    fn set_low(&mut self) { self.set = false; }
    fn set_high(&mut self) { self.set = true; }
}
