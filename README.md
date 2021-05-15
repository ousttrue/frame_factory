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

## ToDo

## Scene
* [x] MVP Matrix, ConstantBuffer
* [x] Camera, Mouse
* [ ] Teapot
* [ ] glTF2
* [ ] SubMesh Material
* [ ] Texture
* [ ] grid

## Gizmo
* [ ] Translate
* [ ] Rotate

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

## Gizmo
## Animation, Timeline

## Windows API

* https://crates.io/crates/winapi
* [winapi-rsのあれこれ](https://qiita.com/LNSEAB/items/88056dfd74a50676dec0)
