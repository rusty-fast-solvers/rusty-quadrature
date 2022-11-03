//! Duffy rules for triangles.
use crate::types::{NumericalQuadratureDefinition, TestTrialNumericalQuadratureDefinition};
use itertools::Itertools;

fn transform_coords(points: &mut Vec<f64>, fun: &impl Fn((f64, f64)) -> (f64, f64)) {
    for (first, second) in points.iter_mut().tuples() {
        (*first, *second) = fun((*first, *second));
    }
}

/// Return the associated vertex indices from an edge index.
fn edge_to_vertices(edge: usize) -> Result<(usize, usize), ()> {
    match edge {
        0 => Ok((1, 2)),
        1 => Ok((2, 0)),
        2 => Ok((0, 1)),
        _ => Err(()),
    }
}

fn create_triangle_mapper(v0: usize, v1: usize) -> impl Fn((f64, f64)) -> (f64, f64) {
    // The vertices in our reference element are
    // 0: (0, 0), 1: (1, 0), 2: (0, 1)
    //
    // The Duffy rule has vertices with a reference element
    // 0: (0, 0), 1: (1, 0), 2: (1, 1)
    // We need to map (0, 0) -> v0, (0, 1) -> v1, and (0, 1) -> v2,
    // where v2 is implicitly defined as the index 3 - v0 - v1 (
    // since all triangle indices together sum up to 3)

    let get_reference_vertex = |index| -> Result<(f64, f64), ()> {
        match index {
            0 => Ok((0.0, 0.0)),
            1 => Ok((1.0, 0.0)),
            2 => Ok((0.0, 1.0)),
            _ => Err(()),
        }
    };

    let p0 = get_reference_vertex(v0).unwrap();
    let p1 = get_reference_vertex(v1).unwrap();
    let p2 = get_reference_vertex(3 - v0 - v1).unwrap();

    //  The tranformation is offset + A * point,
    //  The offset is just identical to p0.

    // The matrix A has two columns. The first column is p1 - p0.
    // The second column is p2 - p1

    let col0 = (p1.0 - p0.0, p1.1 - p0.1);
    let col1 = (p2.0 - p1.0, p2.1 - p1.1);

    // We return a closure that performs the actual transformation.
    // We need to capture its values via move as they stop existing
    // once the closure is moved out of the function.

    move |point: (f64, f64)| -> (f64, f64) {
        (
            p0.0 + col0.0 * point.0 + col1.0 * point.1,
            p0.1 + col0.1 * point.0 + col1.1 * point.1,
        )
    }
}

fn identical_triangles(
    interval_rule: &NumericalQuadratureDefinition,
) -> TestTrialNumericalQuadratureDefinition {
    let NumericalQuadratureDefinition {
        dim,
        order,
        npoints,
        weights,
        points,
    } = interval_rule;

    let n_output_points = 6 * npoints * npoints * npoints * npoints;

    let mut test_output_points = Vec::<f64>::with_capacity(2 * n_output_points);
    let mut trial_output_points = Vec::<f64>::with_capacity(2 * n_output_points);
    let mut output_weights = Vec::<f64>::with_capacity(n_output_points);

    for index1 in 0..*npoints {
        for index2 in 0..*npoints {
            for index3 in 0..*npoints {
                for index4 in 0..*npoints {
                    let eta1 = points[index1];
                    let eta2 = points[index2];
                    let eta3 = points[index3];
                    let xi = points[index4];

                    let eta12 = eta1 * eta2;
                    let eta123 = eta12 * eta3;

                    // First part

                    let weight = weights[index1]
                        * weights[index2]
                        * weights[index3]
                        * weights[index4]
                        * xi
                        * xi
                        * xi
                        * eta1
                        * eta1
                        * eta2;

                    test_output_points.push(xi);
                    test_output_points.push(xi * (1.0 - eta1 + eta12));
                    trial_output_points.push(xi * (1.0 - eta123));
                    trial_output_points.push(xi * (1.0 - eta1));
                    output_weights.push(weight);

                    // Second part

                    test_output_points.push(xi * (1.0 - eta123));
                    test_output_points.push(xi * (1.0 - eta1));
                    trial_output_points.push(xi);
                    trial_output_points.push(xi * (1.0 - eta1 + eta12));
                    output_weights.push(weight);

                    // Third part

                    test_output_points.push(xi);
                    test_output_points.push(xi * (eta1 - eta12 + eta123));
                    trial_output_points.push(xi * (1.0 - eta12));
                    trial_output_points.push(xi * (eta1 - eta12));
                    output_weights.push(weight);

                    // Fourth part

                    test_output_points.push(xi * (1.0 - eta12));
                    test_output_points.push(xi * (eta1 - eta12));
                    trial_output_points.push(xi);
                    trial_output_points.push(xi * (eta1 - eta12 + eta123));
                    output_weights.push(weight);

                    // Fifth part

                    test_output_points.push(xi * (1.0 - eta123));
                    test_output_points.push(xi * (eta1 - eta123));
                    trial_output_points.push(xi);
                    trial_output_points.push(xi * (eta1 - eta12));
                    output_weights.push(weight);

                    // Sixth part

                    test_output_points.push(xi);
                    test_output_points.push(xi * (eta1 - eta12));
                    trial_output_points.push(xi * (1.0 - eta123));
                    trial_output_points.push(xi * (eta1 - eta123));
                    output_weights.push(weight);
                }
            }
        }
    }

    let mapper = create_triangle_mapper(0, 1);

    transform_coords(&mut test_output_points, &mapper);
    transform_coords(&mut trial_output_points, &mapper);

    TestTrialNumericalQuadratureDefinition {
        dim: *dim,
        order: *order,
        npoints: n_output_points,
        weights: output_weights,
        test_points: test_output_points,
        trial_points: trial_output_points,
    }
}

fn edge_adjacent_triangles(
    interval_rule: &NumericalQuadratureDefinition,
    test_singular_edge: usize,
    trial_singular_edge: usize,
) -> TestTrialNumericalQuadratureDefinition {
    let NumericalQuadratureDefinition {
        dim,
        order,
        npoints,
        weights,
        points,
    } = interval_rule;

    let n_output_points = 5 * npoints * npoints * npoints * npoints;

    let mut test_output_points = Vec::<f64>::with_capacity(2 * n_output_points);
    let mut trial_output_points = Vec::<f64>::with_capacity(2 * n_output_points);
    let mut output_weights = Vec::<f64>::with_capacity(n_output_points);

    for index1 in 0..*npoints {
        for index2 in 0..*npoints {
            for index3 in 0..*npoints {
                for index4 in 0..*npoints {
                    let eta1 = points[index1];
                    let eta2 = points[index2];
                    let eta3 = points[index3];
                    let xi = points[index4];

                    let eta12 = eta1 * eta2;
                    let eta123 = eta12 * eta3;

                    // First part

                    let weight = weights[index1]
                        * weights[index2]
                        * weights[index3]
                        * weights[index4]
                        * xi
                        * xi
                        * xi
                        * eta1
                        * eta1;

                    test_output_points.push(xi);
                    test_output_points.push(xi * eta1 * eta3);
                    trial_output_points.push(xi * (1.0 - eta12));
                    trial_output_points.push(xi * (eta1 - eta12));
                    output_weights.push(weight);

                    // Second part

                    test_output_points.push(xi);
                    test_output_points.push(xi * eta1);
                    trial_output_points.push(xi * (1.0 - eta123));
                    trial_output_points.push(xi * (eta12 - eta123));
                    output_weights.push(eta2 * weight);

                    // Third part

                    test_output_points.push(xi * (1.0 - eta12));
                    test_output_points.push(xi * (eta1 - eta12));
                    trial_output_points.push(xi);
                    trial_output_points.push(xi * eta123);
                    output_weights.push(eta2 * weight);

                    // Fourth part

                    test_output_points.push(xi * (1.0 - eta123));
                    test_output_points.push(xi * (eta12 - eta123));
                    trial_output_points.push(xi);
                    trial_output_points.push(xi * eta1);
                    output_weights.push(eta2 * weight);

                    // Fifth part

                    test_output_points.push(xi * (1.0 - eta123));
                    test_output_points.push(xi * (eta1 - eta123));
                    trial_output_points.push(xi);
                    trial_output_points.push(xi * eta12);
                    output_weights.push(eta2 * weight);
                }
            }
        }
    }

    let (v0, v1) = edge_to_vertices(test_singular_edge).unwrap();
    transform_coords(&mut test_output_points, &create_triangle_mapper(1, 2));

    let (v0, v1) = edge_to_vertices(trial_singular_edge).unwrap();
    transform_coords(&mut trial_output_points, &create_triangle_mapper(0, 2));

    TestTrialNumericalQuadratureDefinition {
        dim: *dim,
        order: *order,
        npoints: n_output_points,
        weights: output_weights,
        test_points: test_output_points,
        trial_points: trial_output_points,
    }
}

#[cfg(test)]

mod test {
    use approx::assert_ulps_eq;

    use super::*;

    fn laplace_green(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
        let inv_dist = 1.0 / f64::sqrt((x1 - y1) * (x1 - y1) + (x2 - y2) * (x2 - y2));

        0.25 * std::f64::consts::FRAC_1_PI * inv_dist
    }

    #[test]
    fn test_edge_transformer() {
        let tuple_compare = |tuple1: (f64, f64), tuple2: (f64, f64)| {
            assert_ulps_eq!(tuple1.0, tuple2.0);
            assert_ulps_eq!(tuple1.1, tuple2.1);
        };

        let transformer = create_triangle_mapper(1, 2);

        tuple_compare(transformer((0.0, 0.0)), (1.0, 0.0));
        tuple_compare(transformer((1.0, 0.0)), (0.0, 1.0));
        tuple_compare(transformer((1.0, 1.0)), (0.0, 0.0));

        let transformer = create_triangle_mapper(2, 0);
        tuple_compare(transformer((0.0, 0.0)), (0.0, 1.0));
        tuple_compare(transformer((1.0, 0.0)), (0.0, 0.0));
        tuple_compare(transformer((1.0, 1.0)), (1.0, 0.0));
    }

    #[test]
    fn test_identical_triangles() {
        use crate::simplex_rules::simplex_rule;
        use crate::types::ReferenceCellType;

        for npoints in 1..30 {
            let rule = simplex_rule(ReferenceCellType::Interval, npoints).unwrap();

            let singular_rule = identical_triangles(&rule);

            let mut sum = 0.0;

            for index in 0..singular_rule.npoints {
                let (x1, x2) = (
                    singular_rule.test_points[2 * index],
                    singular_rule.test_points[2 * index + 1],
                );

                let (y1, y2) = (
                    singular_rule.trial_points[2 * index],
                    singular_rule.trial_points[2 * index + 1],
                );

                let weight = singular_rule.weights[index];

                sum += laplace_green(x1, x2, y1, y2) * weight;
            }

            println!("Identical triangle value {}", sum);
        }
    }

    #[test]
    fn test_edge_adjacent_triangles() {
        use crate::simplex_rules::simplex_rule;
        use crate::types::ReferenceCellType;

        // We create two triangles, the reference triangle
        // (0, 0), (1,0). (0, 1)
        // and the second triangle with coordinates
        // (1,0), (1, 1), (0, 1)
        // We integrate the Green's function against those two triangles.

        // First we need to create the reference map to the second triangle.

        let reference_map =
            |point: (f64, f64)| -> (f64, f64) { (1.0 - point.1, point.0 + point.1) };

        for npoints in 1..30 {
            let rule = simplex_rule(ReferenceCellType::Interval, npoints).unwrap();

            let singular_rule = edge_adjacent_triangles(&rule, 0, 1);

            let mut sum = 0.0;

            for index in 0..singular_rule.npoints {
                let (x1, x2) = (
                    singular_rule.test_points[2 * index],
                    singular_rule.test_points[2 * index + 1],
                );

                let (y1, y2) = reference_map((
                    singular_rule.trial_points[2 * index],
                    singular_rule.trial_points[2 * index + 1],
                ));

                let weight = singular_rule.weights[index];
                sum += laplace_green(x1, x2, y1, y2) * weight;
            }

            println!("Edge triangle value {}", sum);
        }
    }
}
