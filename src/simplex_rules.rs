//! Definition of various Gauss type rules

use std::marker::PhantomData;

use rusty_element::cell::Quadrilateral;
use rusty_element::cell::Triangle;
pub use rusty_element::cell::{Interval, ReferenceCell};

pub mod simplex_rule_definitions;
pub mod gauss_legendre;
pub mod interval;
pub mod quadrilateral;
pub mod triangle;

pub struct SimplexRule<C: ReferenceCell> (
    PhantomData<C>,
);

impl SimplexRule<Interval> {
    pub fn new() -> Self {
        SimplexRule(PhantomData)
    }
}

impl SimplexRule<Quadrilateral> {
    pub fn new() -> Self {
        SimplexRule(PhantomData)
    }
}

impl SimplexRule<Triangle> {
    pub fn new() -> Self {
        SimplexRule(PhantomData)
    }
}
