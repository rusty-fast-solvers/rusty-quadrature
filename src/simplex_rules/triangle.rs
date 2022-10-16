use super::gauss_legendre::{GAUSS_POINTS, GAUSS_WEIGHTS};
use super::{SimplexRule, Triangle};
use crate::types::NumericalQuadratureContainer;
use crate::types::NumericalQuadratureRule;
use std::collections::HashMap;
use rusty_element::cell::ReferenceCellType;

type QuadraturePair = (Vec<f64>, Vec<f64>);

lazy_static! {
    static ref MY_MAP: HashMap<ReferenceCellType, HashMap<u32, QuadraturePair>> = {
        let mut m = HashMap::<ReferenceCellType, HashMap<u32, QuadraturePair>>::new();
        m.insert(ReferenceCellType::Triangle, {
            let mut inner = HashMap::<u32, QuadraturePair>::new();
            inner.insert(0, (vec![0.0, 1.0], vec![2.0, 2.0]));
            inner
        });
        m

    };
}

impl NumericalQuadratureRule for SimplexRule<Triangle> {
    fn dim(&self) -> usize {
        2
    }

    fn get_rule(&self, order: usize) -> Result<crate::types::NumericalQuadratureContainer, ()> {
        // Only rules up to 30 points are implemented.
        if (order == 0) || (order > 30) {
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
