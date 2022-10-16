//! Definition of 1d Gauss rules
//!

use super::gauss_legendre::{GAUSS_POINTS, GAUSS_WEIGHTS};
use super::{SimplexRule, Interval};
use crate::types::NumericalQuadratureContainer;
use crate::types::NumericalQuadratureRule;

impl NumericalQuadratureRule for SimplexRule<Interval> {
    fn dim(&self) -> usize {
        1
    }

    fn get_rule(&self, order: usize) -> Result<crate::types::NumericalQuadratureContainer, ()> {
        // Only rules up to 30 points are implemented.
        if (order == 0) || (order > 100) {
            return Err(());
        }

        let npoints = order;
        let address = (npoints * (npoints - 1)) / 2;
        Ok(NumericalQuadratureContainer {
            dim: self.dim(),
            points: (&GAUSS_POINTS[address..(address + npoints)]).to_vec(),
            weights: (&GAUSS_WEIGHTS[address..(address + npoints)]).to_vec(),
        })
    }
}
