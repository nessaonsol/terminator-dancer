use crate::{Result};
use crate::types::*;

pub struct ConformanceHarness {
    pub passed: usize,
    pub failed: usize,
}

impl ConformanceHarness {
    pub fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
        }
    }

    pub fn run_test<F>(&mut self, name: &str, test_fn: F)
    where
        F: FnOnce() -> Result<()>,
    {
        match test_fn() {
            Ok(()) => {
                println!("✅ {}", name);
                self.passed += 1;
            }
            Err(e) => {
                println!("❌ {}: {}", name, e);
                self.failed += 1;
            }
        }
    }

    pub fn report(&self) {
        println!("Conformance test results: {} passed, {} failed", self.passed, self.failed);
    }
}
