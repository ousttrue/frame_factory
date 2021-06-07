# frame_factory 🏭
rust 練習

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

## rust/from_cpp_header 🔧

Read imgui.h and generate rust code for c++ ffi. Shared the imgui.dll between c++ and rust.

# ToDo
## GUI
* [x] Window と Renderer を dll 分離
* [x] c++ frontend + imgui
* [x] WinMain
* [x] RenderTarget
* [x] WindowResize
* [x] FPS
* [x] save window state
* [x] docking
* [x] font
* [ ] logger 🔧
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
* [ ] ErrorHandling 🔧

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
