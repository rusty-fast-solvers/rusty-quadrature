//! Definition of various Gauss type rules

use std::marker::PhantomData;

use rusty_element::cell::Quadrilateral;
use rusty_element::cell::Triangle;
pub use rusty_element::cell::{Interval, ReferenceCell};

pub mod interval;
pub mod quadrilateral;
pub mod triangle;

pub struct GaussRule<C: ReferenceCell> (
    PhantomData<C>,
);

impl GaussRule<Interval> {
    pub fn new() -> Self {
        GaussRule(PhantomData)
    }
}

impl GaussRule<Quadrilateral> {
    pub fn new() -> Self {
        GaussRule(PhantomData)
    }
}

impl GaussRule<Triangle> {
    pub fn new() -> Self {
        GaussRule(PhantomData)
    }
}
