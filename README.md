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

* [x] Window と Renderer を dll 分離
* [x] c++ frontend + imgui
* [x] MVP Matrix, ConstantBuffer
* [x] WinMain
* [x] WindowResize
* [ ] docking, RenderTarget
* [ ] save window state
* [ ] Camera, Mouse
* [ ] Cube, RightHanded Y-UP
* [ ] SubMesh Material
* [ ] Texture
* [ ] glTF2

## Gizmo
## Animation, Timeline

## Windows API

* https://crates.io/crates/winapi
* [winapi-rsのあれこれ](https://qiita.com/LNSEAB/items/88056dfd74a50676dec0)
