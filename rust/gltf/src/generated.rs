/// Buffer 
/// A buffer points to binary geometry, animation, or skins. 
struct Buffer {
    byteLength: i32,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    name: String,
    uri: String,
}

/// Material Normal Texture Info 
/// The normal map texture. 
struct MaterialNormalTextureInfo {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    index: i32,
    scale: f32,
    texCoord: i32,
}

/// Material Occlusion Texture Info 
/// The occlusion map texture. 
struct MaterialOcclusionTextureInfo {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    index: i32,
    strength: f32,
    texCoord: i32,
}

/// Material PBR Metallic Roughness 
/// A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology. When not specified, all the default values of `pbrMetallicRoughness` apply. 
struct MaterialPBRMetallicRoughness {
    baseColorFactor: Vec<f32>,
    baseColorTexture: TextureInfo,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    metallicFactor: f32,
    metallicRoughnessTexture: TextureInfo,
    roughnessFactor: f32,
}

/// Texture Info 
/// The emissive map texture. 
struct TextureInfo {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    index: i32,
    texCoord: i32,
}

/// Material 
/// The material appearance of a primitive. 
struct Material {
    alphaCutoff: f32,
    alphaMode: String,
    doubleSided: bool,
    emissiveFactor: Vec<f32>,
    emissiveTexture: TextureInfo,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    name: String,
    normalTexture: MaterialNormalTextureInfo,
    occlusionTexture: MaterialOcclusionTextureInfo,
    pbrMetallicRoughness: MaterialPBRMetallicRoughness,
}

/// Accessor Sparse 
/// Sparse storage of attributes that deviate from their initialization value. 
struct AccessorSparse {
    count: i32,
    extensions: serd_json::Value,
    extras: serd_json::Value,
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
    extensions: serd_json::Value,
    extras: serd_json::Value,
    max: Vec<f32>,
    min: Vec<f32>,
    name: String,
    normalized: bool,
    sparse: AccessorSparse,
    r#type: String,
}

/// Scene 
/// The root nodes of a scene. 
struct Scene {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    name: String,
    nodes: Vec<i32>,
}

/// Asset 
/// Metadata about the glTF asset. 
struct Asset {
    copyright: String,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    generator: String,
    minVersion: String,
    version: String,
}

/// Sampler 
/// Texture sampler properties for filtering and wrapping modes. 
struct Sampler {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    magFilter: i32,
    minFilter: i32,
    name: String,
    wrapS: i32,
    wrapT: i32,
}

/// Image 
/// Image data used to create a texture. Image can be referenced by URI or `bufferView` index. `mimeType` is required in the latter case. 
struct Image {
    bufferView: i32,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    mimeType: String,
    name: String,
    uri: String,
}

/// Camera Perspective 
/// A perspective camera containing properties to create a perspective projection matrix. 
struct CameraPerspective {
    aspectRatio: f32,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    yfov: f32,
    zfar: f32,
    znear: f32,
}

/// Camera Orthographic 
/// An orthographic camera containing properties to create an orthographic projection matrix. 
struct CameraOrthographic {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    xmag: f32,
    ymag: f32,
    zfar: f32,
    znear: f32,
}

/// Camera 
/// A camera's projection.  A node can reference a camera to apply a transform to place the camera in the scene. 
struct Camera {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    name: String,
    orthographic: CameraOrthographic,
    perspective: CameraPerspective,
    r#type: String,
}

/// Skin 
/// Joints and matrices defining a skin. 
struct Skin {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    inverseBindMatrices: i32,
    joints: Vec<i32>,
    name: String,
    skeleton: i32,
}

/// Texture 
/// A texture and its sampler. 
struct Texture {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    name: String,
    sampler: i32,
    source: i32,
}

/// glTF Property 
///  
struct glTFProperty {
    extensions: serd_json::Value,
    extras: serd_json::Value,
}

/// glTF Property 
///  
struct glTFProperty {
    extensions: serd_json::Value,
    extras: serd_json::Value,
}

/// Animation Sampler 
/// Combines input and output accessors with an interpolation algorithm to define a keyframe graph (but not its target). 
struct AnimationSampler {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    input: i32,
    interpolation: String,
    output: i32,
}

/// glTF Property 
///  
struct glTFProperty {
    extensions: serd_json::Value,
    extras: serd_json::Value,
}

/// Animation Channel Target 
/// The index of the node and TRS property to target. 
struct AnimationChannelTarget {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    node: i32,
    path: String,
}

/// glTF Property 
///  
struct glTFProperty {
    extensions: serd_json::Value,
    extras: serd_json::Value,
}

/// Animation Channel 
/// Targets an animation's sampler at a node's property. 
struct AnimationChannel {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    sampler: i32,
    target: AnimationChannelTarget,
}

/// Animation 
/// A keyframe animation. 
struct Animation {
    channels: Vec<AnimationChannel>,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    name: String,
    samplers: Vec<AnimationSampler>,
}

/// Buffer View 
/// A view into a buffer generally representing a subset of the buffer. 
struct BufferView {
    buffer: i32,
    byteLength: i32,
    byteOffset: i32,
    byteStride: i32,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    name: String,
    target: i32,
}

/// glTF Property 
///  
struct glTFProperty {
    extensions: serd_json::Value,
    extras: serd_json::Value,
}

/// glTF Property 
///  
struct glTFProperty {
    extensions: serd_json::Value,
    extras: serd_json::Value,
}

/// Mesh Primitive 
/// Geometry to be rendered with the given material. 
struct MeshPrimitive {
    attributes: HashMap<String, i32>,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    indices: i32,
    material: i32,
    mode: i32,
    targets: Vec<HashMap<String, i32>>,
}

/// Mesh 
/// A set of primitives to be rendered.  A node can contain one mesh.  A node's transform places the mesh in the scene. 
struct Mesh {
    extensions: serd_json::Value,
    extras: serd_json::Value,
    name: String,
    primitives: Vec<MeshPrimitive>,
    weights: Vec<f32>,
}

/// Node 
/// A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node can have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), only TRS properties may be present; `matrix` will not be present. 
struct Node {
    camera: i32,
    children: Vec<i32>,
    extensions: serd_json::Value,
    extras: serd_json::Value,
    matrix: Vec<f32>,
    mesh: i32,
    name: String,
    rotation: Vec<f32>,
    scale: Vec<f32>,
    skin: i32,
    translation: Vec<f32>,
    weights: Vec<f32>,
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
    extensions: serd_json::Value,
    extensionsRequired: Vec<String>,
    extensionsUsed: Vec<String>,
    extras: serd_json::Value,
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

