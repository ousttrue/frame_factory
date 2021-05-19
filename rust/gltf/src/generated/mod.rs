use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// Accessor Sparse Indices
/// Index array of size `count` that points to those accessor attributes that deviate from their initialization value. Indices must strictly increase.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AccessorSparseIndices {
    bufferView: Option<i32>,
    byteOffset: Option<i32>,
    componentType: Option<i32>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
}

/// Accessor Sparse Values
/// Array of size `count` times number of components, storing the displaced accessor attributes pointed by `indices`. Substituted values must have the same `componentType` and number of components as the base accessor.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AccessorSparseValues {
    bufferView: Option<i32>,
    byteOffset: Option<i32>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
}

/// Accessor Sparse
/// Sparse storage of attributes that deviate from their initialization value.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AccessorSparse {
    count: Option<i32>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    indices: Option<AccessorSparseIndices>,
    values: Option<AccessorSparseValues>,
}

/// Accessor
/// A typed view into a bufferView.  A bufferView contains raw binary data.  An accessor provides a typed view into a bufferView or a subset of a bufferView similar to how WebGL's `vertexAttribPointer()` defines an attribute in a buffer.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Accessor {
    bufferView: Option<i32>,
    byteOffset: Option<i32>,
    componentType: Option<i32>,
    count: Option<i32>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    #[serde(default)]
    max: Vec<f32>,
    #[serde(default)]
    min: Vec<f32>,
    name: Option<String>,
    normalized: Option<bool>,
    sparse: Option<AccessorSparse>,
    r#type: Option<String>,
}

/// Animation Channel Target
/// The index of the node and TRS property to target.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AnimationChannelTarget {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    node: Option<i32>,
    path: Option<String>,
}

/// Animation Channel
/// Targets an animation's sampler at a node's property.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AnimationChannel {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    sampler: Option<i32>,
    target: Option<AnimationChannelTarget>,
}

/// Animation Sampler
/// Combines input and output accessors with an interpolation algorithm to define a keyframe graph (but not its target).
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AnimationSampler {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    input: Option<i32>,
    interpolation: Option<String>,
    output: Option<i32>,
}

/// Animation
/// A keyframe animation.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Animation {
    #[serde(default)]
    channels: Vec<AnimationChannel>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    name: Option<String>,
    #[serde(default)]
    samplers: Vec<AnimationSampler>,
}

/// Asset
/// Metadata about the glTF asset.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Asset {
    copyright: Option<String>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    generator: Option<String>,
    minVersion: Option<String>,
    version: Option<String>,
}

/// Buffer View
/// A view into a buffer generally representing a subset of the buffer.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct BufferView {
    buffer: Option<i32>,
    byteLength: Option<i32>,
    byteOffset: Option<i32>,
    byteStride: Option<i32>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    name: Option<String>,
    target: Option<i32>,
}

/// Buffer
/// A buffer points to binary geometry, animation, or skins.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Buffer {
    byteLength: Option<i32>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    name: Option<String>,
    uri: Option<String>,
}

/// Camera Orthographic
/// An orthographic camera containing properties to create an orthographic projection matrix.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CameraOrthographic {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    xmag: Option<f32>,
    ymag: Option<f32>,
    zfar: Option<f32>,
    znear: Option<f32>,
}

/// Camera Perspective
/// A perspective camera containing properties to create a perspective projection matrix.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CameraPerspective {
    aspectRatio: Option<f32>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    yfov: Option<f32>,
    zfar: Option<f32>,
    znear: Option<f32>,
}

/// Camera
/// A camera's projection.  A node can reference a camera to apply a transform to place the camera in the scene.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Camera {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    name: Option<String>,
    orthographic: Option<CameraOrthographic>,
    perspective: Option<CameraPerspective>,
    r#type: Option<String>,
}

/// Image
/// Image data used to create a texture. Image can be referenced by URI or `bufferView` index. `mimeType` is required in the latter case.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Image {
    bufferView: Option<i32>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    mimeType: Option<String>,
    name: Option<String>,
    uri: Option<String>,
}

/// Texture Info
/// The emissive map texture.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct TextureInfo {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    index: Option<i32>,
    texCoord: Option<i32>,
}

/// Material Normal Texture Info
/// The normal map texture.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct MaterialNormalTextureInfo {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    index: Option<i32>,
    scale: Option<f32>,
    texCoord: Option<i32>,
}

/// Material Occlusion Texture Info
/// The occlusion map texture.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct MaterialOcclusionTextureInfo {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    index: Option<i32>,
    strength: Option<f32>,
    texCoord: Option<i32>,
}

/// Material PBR Metallic Roughness
/// A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology. When not specified, all the default values of `pbrMetallicRoughness` apply.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct MaterialPBRMetallicRoughness {
    #[serde(default)]
    baseColorFactor: Vec<f32>,
    baseColorTexture: Option<TextureInfo>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    metallicFactor: Option<f32>,
    metallicRoughnessTexture: Option<TextureInfo>,
    roughnessFactor: Option<f32>,
}

/// Material
/// The material appearance of a primitive.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Material {
    alphaCutoff: Option<f32>,
    alphaMode: Option<String>,
    doubleSided: Option<bool>,
    #[serde(default)]
    emissiveFactor: Vec<f32>,
    emissiveTexture: Option<TextureInfo>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    name: Option<String>,
    normalTexture: Option<MaterialNormalTextureInfo>,
    occlusionTexture: Option<MaterialOcclusionTextureInfo>,
    pbrMetallicRoughness: Option<MaterialPBRMetallicRoughness>,
}

/// Mesh Primitive
/// Geometry to be rendered with the given material.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct MeshPrimitive {
    attributes: Option<HashMap<String, i32>>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    indices: Option<i32>,
    material: Option<i32>,
    mode: Option<i32>,
    #[serde(default)]
    targets: Vec<HashMap<String, i32>>,
}

/// Mesh
/// A set of primitives to be rendered.  A node can contain one mesh.  A node's transform places the mesh in the scene.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Mesh {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    name: Option<String>,
    #[serde(default)]
    primitives: Vec<MeshPrimitive>,
    #[serde(default)]
    weights: Vec<f32>,
}

/// Node
/// A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node can have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), only TRS properties may be present; `matrix` will not be present.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Node {
    camera: Option<i32>,
    #[serde(default)]
    children: Vec<i32>,
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    #[serde(default)]
    matrix: Vec<f32>,
    mesh: Option<i32>,
    name: Option<String>,
    #[serde(default)]
    rotation: Vec<f32>,
    #[serde(default)]
    scale: Vec<f32>,
    skin: Option<i32>,
    #[serde(default)]
    translation: Vec<f32>,
    #[serde(default)]
    weights: Vec<f32>,
}

/// Sampler
/// Texture sampler properties for filtering and wrapping modes.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Sampler {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    magFilter: Option<i32>,
    minFilter: Option<i32>,
    name: Option<String>,
    wrapS: Option<i32>,
    wrapT: Option<i32>,
}

/// Scene
/// The root nodes of a scene.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Scene {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    name: Option<String>,
    #[serde(default)]
    nodes: Vec<i32>,
}

/// Skin
/// Joints and matrices defining a skin.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Skin {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    inverseBindMatrices: Option<i32>,
    #[serde(default)]
    joints: Vec<i32>,
    name: Option<String>,
    skeleton: Option<i32>,
}

/// Texture
/// A texture and its sampler.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Texture {
    extensions: Option<serde_json::Value>,
    extras: Option<serde_json::Value>,
    name: Option<String>,
    sampler: Option<i32>,
    source: Option<i32>,
}

/// glTF
/// The root object for a glTF asset.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct glTF {
    #[serde(default)]
    accessors: Vec<Accessor>,
    #[serde(default)]
    animations: Vec<Animation>,
    asset: Option<Asset>,
    #[serde(default)]
    bufferViews: Vec<BufferView>,
    #[serde(default)]
    buffers: Vec<Buffer>,
    #[serde(default)]
    cameras: Vec<Camera>,
    extensions: Option<serde_json::Value>,
    #[serde(default)]
    extensionsRequired: Vec<String>,
    #[serde(default)]
    extensionsUsed: Vec<String>,
    extras: Option<serde_json::Value>,
    #[serde(default)]
    images: Vec<Image>,
    #[serde(default)]
    materials: Vec<Material>,
    #[serde(default)]
    meshes: Vec<Mesh>,
    #[serde(default)]
    nodes: Vec<Node>,
    #[serde(default)]
    samplers: Vec<Sampler>,
    scene: Option<i32>,
    #[serde(default)]
    scenes: Vec<Scene>,
    #[serde(default)]
    skins: Vec<Skin>,
    #[serde(default)]
    textures: Vec<Texture>,
}

