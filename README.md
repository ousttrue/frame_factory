# frame_factory 🏭
rust 練習

```
+------------------------+
|rust frame_factory.dll  |
|+-----+  +-------------+|
||Scene|->|D3D Resources||
|+-----+  +-------------+|
+------------------------+
  A     |Render
  |     |
  |     v
Texture2DRenderTarget
+---------+
| Window  |
|c++ imgui|
+---------+
```
c++ frontend, rust d3d11 renderer

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
* [ ] logger
* [ ] SDL2
* [x] separate crate dll, scene and resource

## Scene
* [x] MVP Matrix, ConstantBuffer
* [x] Camera, Mouse
* [ ] Teapot
* [x] SubMesh Material
* [x] Node
* [ ] generate tangent
* [ ] TextureLoader image-rs
* [ ] Skinning
* [ ] MorphTarget
* [ ] Animation

## Renderer
* [ ] ColorTexture
* [ ] NormalTexture

## glTF loader
* [x] parse JsonSchema
* [x] shared reference
* [x] generate rust struct from JsonSchema
* [x] glTF2
* [ ] textures
* [ ] materials
    * [ ] unlit material
    * [ ] pbr material
* [x] meshes
* [ ] nodes

## Gizmo
* [ ] grid
* [ ] Translate
* [ ] Rotate
