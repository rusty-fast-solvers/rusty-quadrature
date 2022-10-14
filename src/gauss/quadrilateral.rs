//! Gauss quadrature rule for a quadrilateral
//!

use super::{GaussRule, Interval, Quadrilateral};
use crate::types::NumericalQuadratureContainer;
use crate::types::NumericalQuadratureRule;

impl NumericalQuadratureRule for GaussRule<Quadrilateral> {
    type C = Quadrilateral;

    fn cell(&self) -> &Self::C {
        &self.cell
    }
    fn get_rule(&self, order: usize) -> crate::types::NumericalQuadratureContainer {
        let interval_rule = GaussRule::<Interval>::new().get_rule(order);

        let n = interval_rule.weights.len();

        let mut quad_points = Vec::<f64>::with_capacity(2 * n * n);
        let mut quad_weights = Vec::<f64>::with_capacity(n * n);

        for i in 0..n {
            for j in 0..n {
                quad_points.push(*interval_rule.points.get(i).unwrap());
                quad_points.push(*interval_rule.points.get(j).unwrap());
                quad_weights.push(
                    *interval_rule.weights.get(i).unwrap() * *interval_rule.weights.get(j).unwrap(),
                );
            }
        }

        NumericalQuadratureContainer {
            dim: 2,
            points: quad_points,
            weights: quad_weights,
        }
    }
}
