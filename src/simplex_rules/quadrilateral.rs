//! Gauss quadrature rule for a quadrilateral
//!

use super::{SimplexRule, Interval, Quadrilateral};
use crate::types::NumericalQuadratureContainer;
use crate::types::NumericalQuadratureRule;

impl NumericalQuadratureRule for SimplexRule<Quadrilateral> {

    fn dim(&self) -> usize {
        2
    }

    fn get_rule(&self, order: usize) -> Result<crate::types::NumericalQuadratureContainer, ()> {

        let interval_rule = SimplexRule::<Interval>::new().get_rule(order)?;

        let n = interval_rule.weights.len();
        assert_eq!(n, interval_rule.points.len());

        let mut quad_points = Vec::<f64>::with_capacity(2 * n * n);
        let mut quad_weights = Vec::<f64>::with_capacity(n * n);


        for i in 0..n {
            for j in 0..n {
                quad_points.push(interval_rule.points[i]);
                quad_points.push(interval_rule.points[j]);
                quad_weights.push(
                    interval_rule.weights[i] * interval_rule.weights[j],
                );
            }
        }

        Ok(NumericalQuadratureContainer {
            dim: self.dim(),
            points: quad_points,
            weights: quad_weights,
        })
    }
}
