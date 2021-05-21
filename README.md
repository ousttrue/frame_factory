# frame_factory 🏭
rust 練習

```
+------------------------+
|rust renderer dll       |
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
* [ ] separate crate dll, scene and resource

## Scene
* [x] MVP Matrix, ConstantBuffer
* [x] Camera, Mouse
* [ ] Teapot
* [x] SubMesh Material
* [ ] ColorTexture
* [ ] generate tangent, normal map
* [ ] image loader(STL)

## glTF loader
* [x] parse JsonSchema
* [x] shared reference
* [x] generate rust struct from JsonSchema
* [x] glTF2
* [ ] unlit material
* [ ] pbr material

## Gizmo
* [ ] grid
* [ ] Translate
* [ ] Rotate
