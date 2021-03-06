# frame_factory ๐ญ
rust ็ทด็ฟ

```
+------------------------+
|rust frame_factory.dll  |
|+-----+  +-------------+|
||Scene|->|D3D Resources||
|+-----+  +-------------+|---+
+------------------------+   |
  A     |Render              |
  |     |                    |
  |     v                    |
Texture2DRenderTarget        v
+---------+              +--------+
| Window  |------------->|imgui   |
|c++      |              |dll c++ |
|         |              |mangling|
+---------+              +--------+
```
c++ frontend, rust d3d11 renderer

# Tools
## rust/from_jsonschema

Read glTF's JsonSchema and generate rust code for serde_json.

## rust/clanggen ๐ง

Read imgui.h and generate rust code for c++ ffi. Shared the imgui.dll between c++ and rust.

# Build

```
$ cd rust/frame_factory
rust/frame_factory$ cargo install cargo-vcpkg
rust/frame_factory$ cargo vcpkg build
rust/frame_factory$ cargo build
```

# ToDo
## GUI
* [x] Window ใจ Renderer ใ dll ๅ้ข
* [x] c++ frontend + imgui
* [x] WinMain
* [x] RenderTarget
* [x] WindowResize
* [x] FPS
* [x] save window state
* [x] docking
* [x] font
* [ ] logger ๐ง
* [x] SDL2
* [x] separate crate dll, scene and resource
* [x] menu file open
* [x] multi view
* [ ] multi view focus
* [ ] view close button
* [ ] share imgui between c++ and rust

## Scene
* [x] MVP Matrix, ConstantBuffer
* [x] Camera, Mouse
* [ ] Teapot
* [x] SubMesh Material
* [x] Node
* [ ] generate tangent
* [ ] Skinning
* [ ] MorphTarget
* [ ] Animation

## Renderer
* [x] TextureLoader image-rs
* [x] DepthBuffer
* [x] ColorTexture
* [x] ccw
* [ ] NormalTexture
* [ ] ErrorHandling ๐ง

## Shader
* [x] Texture

## glTF loader
* [x] parse JsonSchema
* [x] shared reference
* [x] generate rust struct from JsonSchema
* [x] glTF2
* [x] textures
* [ ] vertex: normal
* [ ] vertex: uv
* [x] materials
    * [ ] unlit material
    * [ ] pbr material
* [x] meshes
* [x] nodes

## Gizmo
* [ ] grid
* [ ] Translate
* [ ] Rotate
