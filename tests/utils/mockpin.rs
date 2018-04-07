
use std::rc::Rc;
use std::cell::RefCell;

use utils::hal;
use utils::mockdevice::MockDevice;

pub struct MockPin {
    pub set: bool,
    device: Rc<RefCell<MockDevice>>,
}

impl MockPin {
    pub fn new(device: Rc<RefCell<MockDevice>>) -> MockPin {
        MockPin { set: false, device }
    }
}

impl hal::digital::OutputPin for MockPin {
    fn is_low(&self) -> bool { !self.set }
    fn is_high(&self) -> bool { self.set }

    fn set_low(&mut self) {
        self.set = false;
        self.device.borrow_mut().select_chip()
    }

    fn set_high(&mut self) {
        self.set = true;
        self.device.borrow_mut().deselect_chip()
    }
}
