use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::Write;

pub struct Robot {
    log_file: RefCell<File>,
    hw_errors: Cell<u32>,
}

impl Robot {
    pub fn new(log_file: File) -> Self {
        Robot {
            log_file: RefCell::new(log_file),
            hw_errors: Cell::new(0),
        }
    }
    pub fn add_hw_error(&self) {
        let n = self.hw_errors.get();
        self.hw_errors.set(n + 1);
    }

    pub fn has_hw_errors(&self) -> bool {
        self.hw_errors.get() > 0
    }

    pub fn log(&self, message: &str) {
        let mut file = self.log_file.borrow_mut();
        writeln!(file, "{}", message).unwrap();
    }
}
