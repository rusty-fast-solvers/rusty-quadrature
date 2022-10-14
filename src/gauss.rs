//! Definition of various Gauss type rules

use rusty_element::cell::Quadrilateral;
pub use rusty_element::cell::{Interval, ReferenceCell};

pub mod interval;
pub mod quadrilateral;

pub struct GaussRule<C: ReferenceCell> {
    cell: C,
}

impl GaussRule<Interval> {
    pub fn new() -> Self {
        GaussRule { cell: Interval {} }
    }
}

impl GaussRule<Quadrilateral> {
    pub fn new() -> Self {
        GaussRule {
            cell: Quadrilateral,
        }
    }
}
