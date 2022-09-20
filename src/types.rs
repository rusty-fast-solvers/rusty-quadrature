//! Type definitions.

/// This enum defines multi-dimensional entities.
///
/// We have
/// - Point as 0 dimensional entity.
/// - Edge as 1 dimensional entity.
/// - Surface as 2 dimensional entity.
/// - Volume as 3 dimensional entity.
pub enum EntityType {
    Point,
    Edge,
    Surface,
    Volume,
}

/// A cell type is a marker for the topological type of a cell.
///
/// Examples are triangles, quadrilaterals, hexahedrals, etc.
pub trait CellType {}

/// A topology that maps to a unit triangle.
struct Triangle;

/// A topology that maps to a unit square.
struct Quadrilateral;

impl CellType for Triangle {}
impl CellType for Quadrilateral {}

/// A container for numerical quadrature rules.
pub struct NumericalQuadratureContainer {
    /// The dimension d of a single point.
    pub dim: usize,

    /// The weights of the quadrature rule.
    pub weights: Vec<f64>,
    /// The point coordinates of the quadrature rule.
    ///
    /// A single point has the coordinates p_1, p_2, ..., p_d,
    /// with d being the dimension of the point (typically, 1, 2, or 3).
    /// The vector points stores all points in consecutive order.
    /// Hence, the first point starts at position zero, the second point at
    /// position d, and the third point at position 2d.
    pub points: Vec<f64>,
}

/// Storage for connectivity information.
///
/// Connectivity is important for many singular
/// quadrature rules. We need to know how cells are
/// connected to each other.
pub struct Connectivity {
    /// Describe the type of the entity that is shared
    /// by two cells. Cells can be connected via a shared
    /// point, shared edge, shared surface, etc.
    pub entity_type: EntityType,

    /// The local indices for the shared entity,
    /// for example the first edge of one triangle
    /// could be shared with the second edge of the neighboring
    /// triangle.
    pub entity_indices: (usize, usize),
}

/// A trait for a general numerical quadrature rule. Depending
/// on a specified order a `NumericalQuadratureContainer` is
/// returned with the weights and points of the rule.
pub trait NumericalQuadratureRule<C: CellType> {
    /// Return the quadrature rule for a given order.
    fn get_rule(&self, order: usize) -> NumericalQuadratureContainer;

    /// Return the dimension of the rule (e.g. 2 if defined over unit triangle)
    fn get_dim(&self) -> usize;
}

/// A trait for singular quadrature rules. These are rules that
/// depend on the connectivity information between two cells.
/// So we need to supply the `Connectivity` structure. The result
/// is two separate quadrature containers for the two cells that
/// are integrated against each other.
pub trait SingularQuadratureRule<C: CellType> {
    /// Return the dimension of the rule (e.g. 2 if defined over unit triangle)    
    fn get_dim(&self) -> usize;

    /// Return the quadrature rule for two cells.
    ///
    /// The method takes an `order` parameter and `connectivity` information
    /// that specifies how the two cells are linked to each other.
    fn get_rule(
        &self,
        order: usize,
        connectivity: Connectivity,
    ) -> (NumericalQuadratureContainer, NumericalQuadratureContainer);
}
