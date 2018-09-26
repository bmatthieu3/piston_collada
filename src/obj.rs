// Note: this is mostly copied and adapted from https://github.com/PistonDevelopers/wavefront_obj,
// could potentially be moved to a shared dependency

/// A set of objects from the Collada document
#[derive(Clone, Debug)]
pub struct ObjSet {
  /// Which material library to use.
  pub material_library: Option<String>,
  /// The set of objects.
  pub objects: Vec<Object>,
}

/// A mesh object.
#[derive(Clone, Debug)]
pub struct Object {
  pub id: String,
  /// A human-readable name for this object. This can be set in blender.
  pub name: String,
  /// The set of vertices this object is composed of. These are referenced
  /// by index in `faces`.
  pub vertices: Vec<Vertex>,
  /// The set of attached joints for each vertex. Should match
  /// length of 'vertices' if present
  pub joint_weights: Vec<JointWeights>,
  /// The set of texture vertices referenced by this object. The actual
  /// vertices are indexed by the second element in a `VTNIndex`.
  pub tex_vertices: Vec<TVertex>,
  /// The set of normals referenced by this object. This are are referenced
  /// by the third element in a `VTNIndex`.
  pub normals: Vec<Normal>,
  /// A set of shapes (with materials applied to them) of which this object is
  /// composed.
  pub geometry: Vec<Geometry>,
}

/// A set of shapes, all using the given material.
#[derive(Clone, Debug)]
pub struct Geometry {
  /// A reference to the material to apply to this geometry.
  pub material_name: Option<String>,
  /// Should we use smooth shading when rendering this?
  pub smooth_shading_group: usize,
  /// The shapes of which this geometry is composed.
  pub shapes: Vec<Shape>,
}

/// The various shapes supported by this library.
///
/// Convex polygons more complicated than a triangle are automatically
/// converted into triangles.
#[derive(Clone, Copy, Debug, Hash)]
pub enum Shape {
  /// A point specified by its position.
  Point(VTNIndex),
  /// A line specified by its endpoints.
  Line(VTNIndex, VTNIndex),
  /// A triangle specified by its three vertices.
  Triangle(VTNIndex, VTNIndex, VTNIndex),
}

/// A single 3-dimensional point on the corner of an object.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

/// Represents the weights of any joints that should
/// control the vertex with skinned animation
#[derive(Clone, Copy, Debug)]
pub struct JointWeights {
  /// Indices of joints attached to this vertex.
  /// Maximum of 4 joints
  pub joints: [usize; 4],
  /// Weights for each joint attached to this vertex.
  /// Maximum of 4 joints
  pub weights: [f32; 4],
}
/// A single 3-dimensional normal
pub type Normal = Vertex;

/// A single 2-dimensional point on a texture. "Texure Vertex".
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct TVertex {
  pub x: f64,
  pub y: f64,
}

/// An index into the `vertices` array of an object, representing a vertex in
/// the mesh. After parsing, this is guaranteed to be a valid index into the
/// array, so unchecked indexing may be used.
pub type VertexIndex = usize;

/// An index into the `texture vertex` array of an object.
///
/// Unchecked indexing may be used, because the values are guaranteed to be in
/// range by the parser.
pub type TextureIndex = usize;

/// An index into the `normals` array of an object.
///
/// Unchecked indexing may be used, because the values are guaranteed to be in
/// range by the parser.
pub type NormalIndex = usize;

/// An index into the vertex array, with an optional index into the texture
/// array. This is used to define the corners of shapes which may or may not
/// be textured.
pub type VTNIndex = (VertexIndex, Option<TextureIndex>, Option<NormalIndex>);
