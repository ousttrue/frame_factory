use std::collections::HashMap;

/// Accessor Sparse Indices 
/// Index array of size `count` that points to those accessor attributes that deviate from their initialization value. Indices must strictly increase. 
struct AccessorSparseIndices {
    bufferView: i32,
    byteOffset: i32,
    componentType: i32,
    extensions: serde_json::Value,
    extras: serde_json::Value,
}

/// Accessor Sparse Values 
/// Array of size `count` times number of components, storing the displaced accessor attributes pointed by `indices`. Substituted values must have the same `componentType` and number of components as the base accessor. 
struct AccessorSparseValues {
    bufferView: i32,
    byteOffset: i32,
    extensions: serde_json::Value,
    extras: serde_json::Value,
}

/// Accessor Sparse 
/// Sparse storage of attributes that deviate from their initialization value. 
struct AccessorSparse {
    count: i32,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    indices: AccessorSparseIndices,
    values: AccessorSparseValues,
}

/// Accessor 
/// A typed view into a bufferView.  A bufferView contains raw binary data.  An accessor provides a typed view into a bufferView or a subset of a bufferView similar to how WebGL's `vertexAttribPointer()` defines an attribute in a buffer. 
struct Accessor {
    bufferView: i32,
    byteOffset: i32,
    componentType: i32,
    count: i32,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    max: Vec<f32>,
    min: Vec<f32>,
    name: String,
    normalized: bool,
    sparse: AccessorSparse,
    r#type: String,
}

/// Animation Channel Target 
/// The index of the node and TRS property to target. 
struct AnimationChannelTarget {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    node: i32,
    path: String,
}

/// Animation Channel 
/// Targets an animation's sampler at a node's property. 
struct AnimationChannel {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    sampler: i32,
    target: AnimationChannelTarget,
}

/// Animation Sampler 
/// Combines input and output accessors with an interpolation algorithm to define a keyframe graph (but not its target). 
struct AnimationSampler {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    input: i32,
    interpolation: String,
    output: i32,
}

/// Animation 
/// A keyframe animation. 
struct Animation {
    channels: Vec<AnimationChannel>,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    name: String,
    samplers: Vec<AnimationSampler>,
}

/// Asset 
/// Metadata about the glTF asset. 
struct Asset {
    copyright: String,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    generator: String,
    minVersion: String,
    version: String,
}

/// Buffer View 
/// A view into a buffer generally representing a subset of the buffer. 
struct BufferView {
    buffer: i32,
    byteLength: i32,
    byteOffset: i32,
    byteStride: i32,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    name: String,
    target: i32,
}

/// Buffer 
/// A buffer points to binary geometry, animation, or skins. 
struct Buffer {
    byteLength: i32,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    name: String,
    uri: String,
}

/// Camera Orthographic 
/// An orthographic camera containing properties to create an orthographic projection matrix. 
struct CameraOrthographic {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    xmag: f32,
    ymag: f32,
    zfar: f32,
    znear: f32,
}

/// Camera Perspective 
/// A perspective camera containing properties to create a perspective projection matrix. 
struct CameraPerspective {
    aspectRatio: f32,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    yfov: f32,
    zfar: f32,
    znear: f32,
}

/// Camera 
/// A camera's projection.  A node can reference a camera to apply a transform to place the camera in the scene. 
struct Camera {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    name: String,
    orthographic: CameraOrthographic,
    perspective: CameraPerspective,
    r#type: String,
}

/// Image 
/// Image data used to create a texture. Image can be referenced by URI or `bufferView` index. `mimeType` is required in the latter case. 
struct Image {
    bufferView: i32,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    mimeType: String,
    name: String,
    uri: String,
}

/// Texture Info 
/// The emissive map texture. 
struct TextureInfo {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    index: i32,
    texCoord: i32,
}

/// Material Normal Texture Info 
/// The normal map texture. 
struct MaterialNormalTextureInfo {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    index: TextureInfo,
    scale: f32,
    texCoord: i32,
}

/// Material Occlusion Texture Info 
/// The occlusion map texture. 
struct MaterialOcclusionTextureInfo {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    index: TextureInfo,
    strength: f32,
    texCoord: i32,
}

/// Material PBR Metallic Roughness 
/// A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology. When not specified, all the default values of `pbrMetallicRoughness` apply. 
struct MaterialPBRMetallicRoughness {
    baseColorFactor: Vec<f32>,
    baseColorTexture: TextureInfo,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    metallicFactor: f32,
    metallicRoughnessTexture: TextureInfo,
    roughnessFactor: f32,
}

/// Material 
/// The material appearance of a primitive. 
struct Material {
    alphaCutoff: f32,
    alphaMode: String,
    doubleSided: bool,
    emissiveFactor: Vec<f32>,
    emissiveTexture: TextureInfo,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    name: String,
    normalTexture: MaterialNormalTextureInfo,
    occlusionTexture: MaterialOcclusionTextureInfo,
    pbrMetallicRoughness: MaterialPBRMetallicRoughness,
}

/// Mesh Primitive 
/// Geometry to be rendered with the given material. 
struct MeshPrimitive {
    attributes: HashMap<String, i32>,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    indices: i32,
    material: i32,
    mode: i32,
    targets: Vec<HashMap<String, i32>>,
}

/// Mesh 
/// A set of primitives to be rendered.  A node can contain one mesh.  A node's transform places the mesh in the scene. 
struct Mesh {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    name: String,
    primitives: Vec<MeshPrimitive>,
    weights: Vec<f32>,
}

/// Node 
/// A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node can have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), only TRS properties may be present; `matrix` will not be present. 
struct Node {
    camera: i32,
    children: Vec<i32>,
    extensions: serde_json::Value,
    extras: serde_json::Value,
    matrix: Vec<f32>,
    mesh: i32,
    name: String,
    rotation: Vec<f32>,
    scale: Vec<f32>,
    skin: i32,
    translation: Vec<f32>,
    weights: Vec<f32>,
}

/// Sampler 
/// Texture sampler properties for filtering and wrapping modes. 
struct Sampler {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    magFilter: i32,
    minFilter: i32,
    name: String,
    wrapS: i32,
    wrapT: i32,
}

/// Scene 
/// The root nodes of a scene. 
struct Scene {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    name: String,
    nodes: Vec<i32>,
}

/// Skin 
/// Joints and matrices defining a skin. 
struct Skin {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    inverseBindMatrices: i32,
    joints: Vec<i32>,
    name: String,
    skeleton: i32,
}

/// Texture 
/// A texture and its sampler. 
struct Texture {
    extensions: serde_json::Value,
    extras: serde_json::Value,
    name: String,
    sampler: i32,
    source: i32,
}

/// glTF 
/// The root object for a glTF asset. 
struct glTF {
    accessors: Vec<Accessor>,
    animations: Vec<Animation>,
    asset: Asset,
    bufferViews: Vec<BufferView>,
    buffers: Vec<Buffer>,
    cameras: Vec<Camera>,
    extensions: serde_json::Value,
    extensionsRequired: Vec<String>,
    extensionsUsed: Vec<String>,
    extras: serde_json::Value,
    images: Vec<Image>,
    materials: Vec<Material>,
    meshes: Vec<Mesh>,
    nodes: Vec<Node>,
    samplers: Vec<Sampler>,
    scene: i32,
    scenes: Vec<Scene>,
    skins: Vec<Skin>,
    textures: Vec<Texture>,
}

