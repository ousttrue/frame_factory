# frame_factory 🏭
rust 練習

```
+------------------------------+
|rust renderer dll             |
|Scene Resource Renderer(D3D11)|
+------------------------------+
  A            A
  |            |
+--------+     |
| Window |     |
|c++ main|-> imgui
+--------+
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
* [ ] SDL2

## Scene
* [x] MVP Matrix, ConstantBuffer
* [x] Camera, Mouse
* [ ] Teapot
* [ ] SubMesh Material
* [ ] Texture

## glTF loader
* [ ] glTF2

## Gizmo
* [ ] grid
* [ ] Translate
* [ ] Rotate
