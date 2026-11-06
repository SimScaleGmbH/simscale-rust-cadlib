// Code generated for the SimScale cadlib Rust bindings. DO NOT EDIT.
//! Model types generated from the cadlib JSON schemas.

use serde::{Deserialize, Serialize};

/// Level of faults allowed in tessellation (Supported only for Parasolid models)
/// Used in quality checks to classify the tessellation faults in three levels: mild, moderate and serious.
/// The default is `Mild`, which allows only minor faults that do not significantly affect the quality of the tessellation.
/// `Moderate` allows more significant faults that may affect the quality but are still acceptable for most applications.
/// `Serious` allows all faults, including those that may result in a poor quality tessellation, and should be used with caution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllowedFacetFaultsLevel {
    #[serde(rename = "mild")]
    Mild,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "serious")]
    Serious,
}

/// Parameters for bounding box info query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBoxParameters {
    /// Ids of faces to measure in addition to the model and its bodies. When
    /// given, the response also holds the box enclosing all of them and one box
    /// per face
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<String>>,
}

/// Response for bounding box query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBoxResponse {
    /// Bounding box of the entire model
    #[serde(rename = "modelBoundingBox")]
    pub model_bounding_box: Box,
    /// Bounding box of each solid and sheet in the model, keyed by its entity id
    #[serde(rename = "boundingBoxPerBody")]
    pub bounding_box_per_body: std::collections::HashMap<String, Box>,
    /// Single box enclosing all the requested faces. Set only when faces were
    /// requested
    #[serde(
        rename = "facesBoundingBox",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub faces_bounding_box: Option<Box>,
    /// Bounding box of each requested face, keyed by its face id. Set only when
    /// faces were requested
    #[serde(
        rename = "boundingBoxPerFace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bounding_box_per_face: Option<std::collections::HashMap<String, Box>>,
}

/// 3D bounding box representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Box {
    /// Minimum X coordinate
    #[serde(rename = "minX")]
    pub min_x: f64,
    /// Minimum Y coordinate
    #[serde(rename = "minY")]
    pub min_y: f64,
    /// Minimum Z coordinate
    #[serde(rename = "minZ")]
    pub min_z: f64,
    /// Maximum X coordinate
    #[serde(rename = "maxX")]
    pub max_x: f64,
    /// Maximum Y coordinate
    #[serde(rename = "maxY")]
    pub max_y: f64,
    /// Maximum Z coordinate
    #[serde(rename = "maxZ")]
    pub max_z: f64,
}

/// Parameters for compute center of gravity query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterOfGravityParameters {
    /// Ids of the faces and edges to combine into a single center of gravity, as
    /// `entities` returns them. Any other entity type fails the query.
    /// Must not be empty
    pub entities: Vec<String>,
}

/// Response for compute center of gravity query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterOfGravityResponse {
    /// Center of gravity as a 3D vector
    pub cog: Vector3,
}

/// Circle approximating a planar patch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CirclePrimitive {
    /// Center of the circle
    pub center: Vector3,
    /// Radius of the circle
    pub radius: f64,
    /// Outward normal of the patch
    pub normal: Vector3,
}

/// Color information for an entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    /// Id of the solid or sheet the color is defined on
    pub entity_id: String,
    /// Color in hex format (e.g., "#ff0000")
    pub color: String,
}

/// Parameters for get model colors query (empty)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorsParameters;

/// Response for get model colors query
/// List of colors defined on solids and sheets
pub type ColorsResponse = Vec<Color>;

/// Parameters for detect rotation query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectCommonRotationParameters {
    /// Ids of the faces to check for common rotation, as `entities` returns them. The query will check if these faces are rotationally aligned and share the same rotation center and axis. Must not be empty
    pub faces: Vec<String>,
}

/// Response for detect common rotation query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectCommonRotationResponse {
    /// Center of common rotation
    pub center: Vector3,
    /// Axis of common rotation
    pub axis: Vector3,
}

/// Parameters for detect enclosure query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectEnclosureParameters {
    /// Point to check for enclosure
    pub point: Vector3,
}

/// Response for detect enclosure query
/// List of solids enclosing the input point
pub type DetectEnclosureResponse = Vec<String>;

/// Length metric for a single edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeMetric {
    /// Id of the edge
    #[serde(rename = "entityId")]
    pub entity_id: String,
    /// Length of the edge
    pub length: f64,
}

/// Response for entities query — a flat list of all entities.
///
/// Each entity references its parents and children by their 0-based index in the list.
/// Solids and sheets are the top-level entities.
pub type EntitiesResponse = Vec<Entity>;

/// An entity in the CAD model topology (solids, sheets, faces, edges, vertices).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Unique entity id, e.g. `A1_I2_B3_TE4` or `B1_TE2`
    #[serde(rename = "entityId")]
    pub entity_id: String,
    /// Human-readable entity name, e.g. `face_name@body_name`
    pub name: String,
    /// Type of the entity
    pub r#type: EntityType,
    /// Indices of the children of this entity in this list
    pub children: Vec<i32>,
    /// Indices in this list of all entities this entity is a child of.
    /// Empty for top-level entities, i.e. solids and sheets.
    /// An entity can have several parents: an edge is usually shared by two faces,
    /// a vertex by several edges.
    pub parents: Vec<i32>,
}

/// Type of entity. A body is never part of the topology, since it is
/// represented by its top-level solids or sheets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "SOLID")]
    Solid,
    #[serde(rename = "SHEET")]
    Sheet,
    #[serde(rename = "FACE")]
    Face,
    #[serde(rename = "EDGE")]
    Edge,
    #[serde(rename = "VERTEX")]
    Vertex,
}

/// Surface area metric for a single face
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceMetric {
    /// Id of the face
    #[serde(rename = "entityId")]
    pub entity_id: String,
    /// Surface area of the face
    pub area: f64,
}

/// Parameters for comfort maps query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateComfortMapDiscParameters {
    /// Center point of the region of interest on the ground plane. The disc will be generated in a circular region around this center point.
    pub center: Vector2,
    /// Height above ground of the region of interest (distance between the center point and the plane where the disc will be generated). Must not be negative
    pub height: f64,
    /// Radius of the region of interest on the ground plane (disc will be generated in a circular region with this radius around the center point). Must be greater than zero
    pub floor_radius: f64,
    /// Tessellation fineness parameter to control the size of the disc's tessellation,. Must be greater than zero, and large enough that the disc stays under the limit of 100 million triangles
    pub fineness: f64,
    /// Ids of the faces that make up the ground (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<String>>,
    /// Height of the ground plane. Defaults to 0 when omitted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ground_level: Option<f64>,
    /// Defines the encoding of the file which contains the disc (optional). With `ascii` or `binary`, or when omitted, the disc is written as a single file; with `zipped-ascii` or `zipped-binary` the result is unpacked into the directory
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_encoding: Option<ResultEncoding>,
}

/// Response for comfort maps query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateComfortMapDiscResponse {
    /// Number of triangles in the tessellated disc. Always greater than zero: a
    /// disc with no geometry fails the query instead
    pub number_of_triangles: i32,
    /// Bounding box of the tessellated disc (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<Box>,
}

/// Parameters for material points query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialPointsParameters {
    /// Defines whether one point is computed for the whole model or one per solid
    /// and sheet. The response sets the matching field and leaves the other unset
    #[serde(rename = "queryType")]
    pub query_type: MaterialPointsQueryType,
}

/// Query type for material points
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialPointsQueryType {
    #[serde(rename = "entireModel")]
    EntireModel,
    #[serde(rename = "perBody")]
    PerBody,
}

/// Response for material points query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialPointsResponse {
    /// A point inside the model's material. Set when `queryType` is `entireModel`
    #[serde(
        rename = "modelMaterialPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub model_material_point: Option<Vector3>,
    /// A point inside each solid and sheet, keyed by its entity id. Set when
    /// `queryType` is `perBody`
    #[serde(
        rename = "materialPointPerBody",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub material_point_per_body: Option<std::collections::HashMap<String, Vector3>>,
}

/// Parameters for model metrics query
// An empty object on the wire, not a null.
#[allow(clippy::empty_structs_with_brackets)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetricsParameters {}

/// Response for model metrics query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetricsResponse {
    /// Sum of the volumes in `solids`
    #[serde(rename = "modelVolume")]
    pub model_volume: f64,
    /// Aggregated surface area of all solids and sheets
    #[serde(rename = "modelSurfaceArea")]
    pub model_surface_area: f64,
    /// Volume and surface area metrics per solid
    pub solids: Vec<VolumeMetric>,
    /// Volume and surface area metrics per sheet
    pub sheets: Vec<VolumeMetric>,
    /// Surface area metrics per face
    pub faces: Vec<FaceMetric>,
    /// Length metrics per edge
    pub edges: Vec<EdgeMetric>,
}

/// The kind of primitive that approximates a planar patch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchType {
    #[serde(rename = "rectangle")]
    Rectangle,
    #[serde(rename = "circle")]
    Circle,
}

/// One patch of adjacent, coplanar faces, together with the primitive approximating it.
///
/// `patch_type` names which primitive describes the patch: exactly one of `rectangle` and
/// `circle` is set, the one `patch_type` names.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanarPatch {
    /// The faces that make up this patch
    pub faces: Vec<String>,
    /// The kind of primitive that approximates this patch
    #[serde(rename = "patchType")]
    pub patch_type: PatchType,
    /// The rectangle approximating this patch, set when `patch_type` is `rectangle`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rectangle: Option<RectanglePrimitive>,
    /// The circle approximating this patch, set when `patch_type` is `circle`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circle: Option<CirclePrimitive>,
}

/// Parameters for planar patch primitives query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanarPatchPrimitivesParameters {
    /// Ids of the planar faces to group into patches, as `entities` returns them. The query fails if any of the given faces is not planar. Must not be empty
    pub faces: Vec<String>,
}

/// Response for planar patch primitives query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanarPatchPrimitivesResponse {
    /// The patches found among the requested faces
    pub patches: Vec<PlanarPatch>,
}

/// Rectangle approximating a planar patch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectanglePrimitive {
    /// The four corners of the rectangle, counterclockwise around `normal`
    pub corners: Vec<Vector3>,
    /// Outward normal of the patch
    pub normal: Vector3,
}

/// Encoding format for STL result data, used in some queries to control how the result is encoded (e.g. for comfort map disc)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResultEncoding {
    #[serde(rename = "ascii")]
    Ascii,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "zipped-ascii")]
    ZippedAscii,
    #[serde(rename = "zipped-binary")]
    ZippedBinary,
}

/// Parameters for the tessellate faces query.
///
/// Tessellates a chosen set of faces: each face becomes its own STL file, named
/// after the face id. A face listed more than once is tessellated only once.
///
/// Supported only for Parasolid models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TessellateFacesParameters {
    /// Ids of the faces to tessellate, as `entities` returns them. Each face is
    /// tessellated into its own STL file, named after the face id. A face
    /// requested more than once is tessellated only once. Must not be empty
    pub faces: Vec<String>,
    /// Defines the tessellation strategy. Defaults to `chordTol` when omitted
    #[serde(
        rename = "tessellationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tessellation_type: Option<TessellationType>,
    /// Maximum facet width for tessellationType=maxFacetWidth. If omitted, it is
    /// derived from the size of the face
    #[serde(
        rename = "maxFacetWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_facet_width: Option<f64>,
    /// Level of faults allowed in the tessellation. Used in quality checks.
    /// Defaults to `mild` when omitted
    #[serde(
        rename = "allowedFacetFaultsLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_facet_faults_level: Option<AllowedFacetFaultsLevel>,
}

/// Response for the tessellate faces query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TessellateFacesResponse {
    /// Unit normal of each requested face, evaluated at the mid point of the
    /// face, keyed by face id
    #[serde(rename = "faceNormals")]
    pub face_normals: std::collections::HashMap<String, Vector3>,
}

/// Parameters for tessellate query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TessellateParameters {
    /// If true, one STL file is written per face; if false, one per region
    #[serde(rename = "writeOneFilePerFace")]
    pub write_one_file_per_face: bool,
    /// If true, computes the surface area of the tessellation and returns it as
    /// `tess_surface_area` in the response
    #[serde(rename = "calculateArea")]
    pub calculate_area: bool,
    /// Supported only for Parasolid models. Defines the tessellation strategy.
    /// Has no default: the query fails with `invalid-parameter` if it is not set
    #[serde(
        rename = "tessellationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tessellation_type: Option<TessellationType>,
    /// Supported only for Parasolid models. Maximum facet width, used when `tessellation_type`
    /// is `maxFacetWidth`. If omitted, it is computed from the bounding box of the body
    #[serde(
        rename = "maxFacetWidth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_facet_width: Option<f64>,
    /// Supported only for Parasolid models. Level of faults allowed in the tessellation.
    /// Used in quality checks. Defaults to `mild` when omitted
    #[serde(
        rename = "allowedFacetFaultsLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_facet_faults_level: Option<AllowedFacetFaultsLevel>,
    /// Supported only for nTop models. Granularity at which the solid's field is sampled.
    /// Make this smaller to get more triangles and lower max edge length.
    /// Must be greater than zero. Defaults to 0.0015 when omitted
    #[serde(
        rename = "featureSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub feature_size: Option<f64>,
    /// Supported only for nTop models. Controls how much the mesh is decimated.
    /// Make this higher to get less triangles and higher max edge length.
    /// Must be greater than zero. Defaults to 10 when omitted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adaptivity: Option<f64>,
    /// Supported only for nTop models. Specifies the maximum allowable length for any edge in the tessellated
    /// mesh. During refinement, edges longer than this value are subdivided to ensure the
    /// mesh respects this limit. Must be greater than zero. Defaults to 10 when omitted
    #[serde(
        rename = "maxEdgeLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_edge_length: Option<f64>,
}

/// Response for tessellation query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TessellateResponse {
    /// Number of bodies in the tessellated geometry
    #[serde(rename = "numBodies")]
    pub num_bodies: i32,
    /// Surface area of the tessellated geometry. Set only when `calculate_area`
    /// was requested
    #[serde(
        rename = "tessSurfaceArea",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tess_surface_area: Option<f64>,
}

/// Tessellation strategy (Supported only for Parasolid models)
/// `ChordTol` controls curve tolerance by specifying the maximum allowable distance between a true curved edge
/// and the straight-line chords (facet edges) used to approximate it.
/// The `MaxFacetWidth` strategy uses a maximum facet width to control the maximum size of the facets, which can be more intuitive in some cases.
/// When using `MaxFacetWidth`, the `maxFacetWidth` parameter controls the maximum width of the facets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TessellationType {
    #[serde(rename = "chordTol")]
    ChordTol,
    #[serde(rename = "maxFacetWidth")]
    MaxFacetWidth,
}

/// 2D vector representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

/// 3D vector representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

/// Volume and surface area metrics for a single solid or sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMetric {
    /// Id of the solid or sheet
    #[serde(rename = "entityId")]
    pub entity_id: String,
    /// Volume of the entity, when applicable
    pub volume: f64,
    /// Surface area of the entity
    #[serde(rename = "surfaceArea")]
    pub surface_area: f64,
}

/// Parameters for compute volume query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeParameters {
    /// Ids of the solids and sheets to measure, as `entities` returns them.
    /// Must not be empty
    #[serde(rename = "entityIds")]
    pub entity_ids: Vec<String>,
}

/// Response for volume query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeResponse {
    /// Sum of the volumes in `volumes`
    pub total: f64,
    /// Volume of each measured entity, keyed by its entity id
    pub volumes: std::collections::HashMap<String, f64>,
}
