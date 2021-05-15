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

## Scene
* [x] MVP Matrix, ConstantBuffer
* [x] Camera, Mouse
* [ ] Teapot
* [ ] SubMesh Material
* [ ] Texture

## glTF loader
* [ ] generate rust struct from JsonSchema
* [ ] glTF2

## Gizmo
* [ ] grid
* [ ] Translate
* [ ] Rotate
