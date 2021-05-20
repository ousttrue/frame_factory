use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// Accessor Sparse Indices
/// Index array of size `count` that points to those accessor attributes that deviate from their initialization value. Indices must strictly increase.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct AccessorSparseIndices {
    pub bufferView: Option<i32>,
    pub byteOffset: Option<i32>,
    pub componentType: Option<i32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
}

/// Accessor Sparse Values
/// Array of size `count` times number of components, storing the displaced accessor attributes pointed by `indices`. Substituted values must have the same `componentType` and number of components as the base accessor.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct AccessorSparseValues {
    pub bufferView: Option<i32>,
    pub byteOffset: Option<i32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
}

/// Accessor Sparse
/// Sparse storage of attributes that deviate from their initialization value.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct AccessorSparse {
    pub count: Option<i32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub indices: Option<AccessorSparseIndices>,
    pub values: Option<AccessorSparseValues>,
}

/// Accessor
/// A typed view into a bufferView.  A bufferView contains raw binary data.  An accessor provides a typed view into a bufferView or a subset of a bufferView similar to how WebGL's `vertexAttribPointer()` defines an attribute in a buffer.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Accessor {
    pub bufferView: Option<i32>,
    pub byteOffset: Option<i32>,
    pub componentType: Option<i32>,
    pub count: Option<i32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub max: Vec<f32>,
    #[serde(default)]
    pub min: Vec<f32>,
    #[serde(default)]
    pub name: String,
    pub normalized: Option<bool>,
    pub sparse: Option<AccessorSparse>,
    #[serde(default)]
    pub r#type: String,
}

/// Animation Channel Target
/// The index of the node and TRS property to target.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct AnimationChannelTarget {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub node: Option<i32>,
    #[serde(default)]
    pub path: String,
}

/// Animation Channel
/// Targets an animation's sampler at a node's property.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct AnimationChannel {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub sampler: Option<i32>,
    pub target: Option<AnimationChannelTarget>,
}

/// Animation Sampler
/// Combines input and output accessors with an interpolation algorithm to define a keyframe graph (but not its target).
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct AnimationSampler {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub input: Option<i32>,
    #[serde(default)]
    pub interpolation: String,
    pub output: Option<i32>,
}

/// Animation
/// A keyframe animation.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Animation {
    #[serde(default)]
    pub channels: Vec<AnimationChannel>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub samplers: Vec<AnimationSampler>,
}

/// Asset
/// Metadata about the glTF asset.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Asset {
    #[serde(default)]
    pub copyright: String,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub generator: String,
    #[serde(default)]
    pub minVersion: String,
    #[serde(default)]
    pub version: String,
}

/// Buffer View
/// A view into a buffer generally representing a subset of the buffer.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct BufferView {
    pub buffer: Option<i32>,
    pub byteLength: Option<i32>,
    pub byteOffset: Option<i32>,
    pub byteStride: Option<i32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub name: String,
    pub target: Option<i32>,
}

/// Buffer
/// A buffer points to binary geometry, animation, or skins.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Buffer {
    pub byteLength: Option<i32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub uri: String,
}

/// Camera Orthographic
/// An orthographic camera containing properties to create an orthographic projection matrix.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct CameraOrthographic {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub xmag: Option<f32>,
    pub ymag: Option<f32>,
    pub zfar: Option<f32>,
    pub znear: Option<f32>,
}

/// Camera Perspective
/// A perspective camera containing properties to create a perspective projection matrix.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct CameraPerspective {
    pub aspectRatio: Option<f32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub yfov: Option<f32>,
    pub zfar: Option<f32>,
    pub znear: Option<f32>,
}

/// Camera
/// A camera's projection.  A node can reference a camera to apply a transform to place the camera in the scene.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Camera {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub name: String,
    pub orthographic: Option<CameraOrthographic>,
    pub perspective: Option<CameraPerspective>,
    #[serde(default)]
    pub r#type: String,
}

/// Image
/// Image data used to create a texture. Image can be referenced by URI or `bufferView` index. `mimeType` is required in the latter case.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Image {
    pub bufferView: Option<i32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub mimeType: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub uri: String,
}

/// Texture Info
/// The emissive map texture.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct TextureInfo {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub index: Option<i32>,
    pub texCoord: Option<i32>,
}

/// Material Normal Texture Info
/// The normal map texture.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct MaterialNormalTextureInfo {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub index: Option<i32>,
    pub scale: Option<f32>,
    pub texCoord: Option<i32>,
}

/// Material Occlusion Texture Info
/// The occlusion map texture.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct MaterialOcclusionTextureInfo {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub index: Option<i32>,
    pub strength: Option<f32>,
    pub texCoord: Option<i32>,
}

/// Material PBR Metallic Roughness
/// A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology. When not specified, all the default values of `pbrMetallicRoughness` apply.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct MaterialPBRMetallicRoughness {
    #[serde(default)]
    pub baseColorFactor: Vec<f32>,
    pub baseColorTexture: Option<TextureInfo>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub metallicFactor: Option<f32>,
    pub metallicRoughnessTexture: Option<TextureInfo>,
    pub roughnessFactor: Option<f32>,
}

/// Material
/// The material appearance of a primitive.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Material {
    pub alphaCutoff: Option<f32>,
    #[serde(default)]
    pub alphaMode: String,
    pub doubleSided: Option<bool>,
    #[serde(default)]
    pub emissiveFactor: Vec<f32>,
    pub emissiveTexture: Option<TextureInfo>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub name: String,
    pub normalTexture: Option<MaterialNormalTextureInfo>,
    pub occlusionTexture: Option<MaterialOcclusionTextureInfo>,
    pub pbrMetallicRoughness: Option<MaterialPBRMetallicRoughness>,
}

/// Mesh Primitive
/// Geometry to be rendered with the given material.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct MeshPrimitive {
    #[serde(default)]
    pub attributes: HashMap<String, i32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub indices: Option<i32>,
    pub material: Option<i32>,
    pub mode: Option<i32>,
    #[serde(default)]
    pub targets: Vec<HashMap<String, i32>>,
}

/// Mesh
/// A set of primitives to be rendered.  A node can contain one mesh.  A node's transform places the mesh in the scene.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Mesh {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub primitives: Vec<MeshPrimitive>,
    #[serde(default)]
    pub weights: Vec<f32>,
}

/// Node
/// A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node can have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), only TRS properties may be present; `matrix` will not be present.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Node {
    pub camera: Option<i32>,
    #[serde(default)]
    pub children: Vec<i32>,
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub matrix: Vec<f32>,
    pub mesh: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub rotation: Vec<f32>,
    #[serde(default)]
    pub scale: Vec<f32>,
    pub skin: Option<i32>,
    #[serde(default)]
    pub translation: Vec<f32>,
    #[serde(default)]
    pub weights: Vec<f32>,
}

/// Sampler
/// Texture sampler properties for filtering and wrapping modes.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Sampler {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub magFilter: Option<i32>,
    pub minFilter: Option<i32>,
    #[serde(default)]
    pub name: String,
    pub wrapS: Option<i32>,
    pub wrapT: Option<i32>,
}

/// Scene
/// The root nodes of a scene.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Scene {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nodes: Vec<i32>,
}

/// Skin
/// Joints and matrices defining a skin.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Skin {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    pub inverseBindMatrices: Option<i32>,
    #[serde(default)]
    pub joints: Vec<i32>,
    #[serde(default)]
    pub name: String,
    pub skeleton: Option<i32>,
}

/// Texture
/// A texture and its sampler.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct Texture {
    pub extensions: Option<serde_json::Value>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub name: String,
    pub sampler: Option<i32>,
    pub source: Option<i32>,
}

/// glTF
/// The root object for a glTF asset.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct glTF {
    #[serde(default)]
    pub accessors: Vec<Accessor>,
    #[serde(default)]
    pub animations: Vec<Animation>,
    pub asset: Option<Asset>,
    #[serde(default)]
    pub bufferViews: Vec<BufferView>,
    #[serde(default)]
    pub buffers: Vec<Buffer>,
    #[serde(default)]
    pub cameras: Vec<Camera>,
    pub extensions: Option<serde_json::Value>,
    #[serde(default)]
    pub extensionsRequired: Vec<String>,
    #[serde(default)]
    pub extensionsUsed: Vec<String>,
    pub extras: Option<serde_json::Value>,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub materials: Vec<Material>,
    #[serde(default)]
    pub meshes: Vec<Mesh>,
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub samplers: Vec<Sampler>,
    pub scene: Option<i32>,
    #[serde(default)]
    pub scenes: Vec<Scene>,
    #[serde(default)]
    pub skins: Vec<Skin>,
    #[serde(default)]
    pub textures: Vec<Texture>,
}

