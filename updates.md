# Engine Updates

## bevy v0.13.1

 A full diff of what's been fixed can be seen here:  https://github.com/bevyengine/bevy/compare/v0.13.0...v0.13.1

## bevy v0.13.0

 * [Release Announcement](https://bevyengine.org/news/bevy-0-13/)
* [Migration Guide](https://bevyengine.org/learn/migration-guides/0-12-to-0-13/)

## console_engine v2.6.1

 ### 🐞 Bugfixes

- #26 Replace panic by returning an Error on `init_fill` when the terminal size cannot be returned by crossterm (Thanks to @r59q)

## BlueEngine new update! 0.5.7

 #### [0.5.7] - 2024-02-17

##### Bug Fixes

- Surface error for non zero size on windows ([8426db3](https://github.com/AryanpurTech/BlueEngine/commit/8426db3e46bd709f0df98cf890ffdd73c87ecaef))

##### Features

- Added `control_flow`, `present_mode`, `alpha_mode`, and `desired_maximum_frame_latency` options ([60513a5](https://github.com/AryanpurTech/BlueEngine/commit/60513a547b30284cc2bf0e977d462c69f9a8fb36))
- Fixed scissor bounds bug, added examples ([9a89185](https://github.com/AryanpurTech/BlueEngine/commit/9a89185451f55be11d2821c8c33d8eb1650aee88))
- Added scissor and clear color finally ([ee77156](https://github.com/AryanpurTech/BlueEngine/commit/ee771568340f74374023212e20c6845c5c14b253))

##### Miscellaneous Tasks

- Clear color example and updates ([6e2f434](https://github.com/AryanpurTech/BlueEngine/commit/6e2f4343e501ce860f4154ddca38f2ad01e11076))


## BlueEngine new update!

 ### New version 0.5.1!

Updated wgpu and winit to latest version, and updated texture settings requiring `&'static str` to `StringBuffer`

## BlueEngine new update!

 ##### Documentation

- Added documentation to the entire engine ([5a86e7e](https://github.com/AryanpurTech/BlueEngine/commit/5a86e7ea71a4465e1c5d1e9dcdb10c2d0937d020))

> This is a new milestone for the engine!

## BlueEngine new update!

 ##### Bug Fixes

- Defined wgpu transform matrix [see](https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera) ([5204406](https://github.com/AryanpurTech/BlueEngine/commit/520440645985ff0dd313d108d411634d0aeed3fe))
- Transparency, and fix #43 ([0d4037d](https://github.com/AryanpurTech/BlueEngine/commit/0d4037dec55495c1eed55c6fb36fd470fb47bd98))

##### Features

- Added `render_order` to the objects to control when they are sent to the gpu ([e00910a](https://github.com/AryanpurTech/BlueEngine/commit/e00910a2b91149895b00acb79d5d9fe909b67efb))
- Implemented switching between `perspective` and `orthographic` projection ([2cd24c7](https://github.com/AryanpurTech/BlueEngine/commit/2cd24c7f7a45d6064494b8621d2150a1a2f8091e))
- Option to set perspective or orthographic projection ([297f67e](https://github.com/AryanpurTech/BlueEngine/commit/297f67e87f7cfabb8be1f88ee87d8af9c17d4602))
- Added projection enum ([5177e4a](https://github.com/AryanpurTech/BlueEngine/commit/5177e4ac16f3a9b38068dffc5aef21813f11cdc9))

##### Miscellaneous Tasks

- Added some fixes to the PR ([e1b9217](https://github.com/AryanpurTech/BlueEngine/commit/e1b9217791797f609c6deb794632f08d6b468a8c))
- Added some fixes to the PR ([2559d6b](https://github.com/AryanpurTech/BlueEngine/commit/2559d6b301f6268304bebaa7e5d55d9019b6edfd))

> huge thanks to @akowi-sknobloch for all these contributions!

## BlueEngine new update!

 ##### Features

- Instancing now works, with example. fix #40 ([8e5e2db](https://github.com/AryanpurTech/BlueEngine/commit/8e5e2db84775e91e8ccf919c82e8f5f40312885b))
- Transparency in textures now working ([1dafadf](https://github.com/AryanpurTech/BlueEngine/commit/1dafadfcaea8ae0acf2a8d8ac80e54c3f4c6dfed))

## notan v0.12.0

 #### Changelog

- Updated EGUI to `0.26`.
- Removed `egui::plugin::Output.needs_repaint()`, now is only used internally and not exposed to users.
- Exposed `notan::draw::DrawBuilder` allowing custom builders.
- Exposed `notan::app::AppTimer`.
- Added `draw.point` allowing to draw points. Check `examples/draw_point.rs`.
- Allow to compile the crate without a backend selected.
- Changed `WindowConfig::set_canvas_id` to `WindowConfig::set_app_id` and is not available for wayland too.
- Fixed `app.request_frame()` when using lazy lopps on Window OS.

--

Thanks everybody!

#### Contributors
* @z33ky 
* @joseluis 
* @RichardMarks 

## notan v0.11.0

 #### Changelog
* Update EGUI to 0.23 #291 
* Fix an error acquiring OpenGL Context #292 
* Update inner dependencies

#### Breaking 
If you're using egui to render notan textures you may want to check the [example](https://github.com/Nazariglez/notan/blob/develop/examples/egui_texture.rs) because the API did change. 

--

Thank you so much for the help!

#### Contributors
* @apekros 
* @mantasarm 

## notan v0.10.0

 #### Changelog 

- Added `WindowConfig::set_position` to set x/y position before creating the window.
- Changed `Renderer.begin` uses `Option<ClearOption>` instead of `Option<&ClearOption>`.
- Changed sizes and positions for Window and Textures from `i32` to `u32`.
- Added `AppTimer::elapsed` to return time since init as `Duration`.
- Changed `AppTimer::time_since_init` to `AppTimer::elapsed_f32`.
- Changed `WindowConfig` setter method to use the prefix `set_`. 
- Removed deprecated `Mouse::local_position`.
- Removed deprecated `mat3_screen_to_local`, `mat3_local_to_screen`, `mat3_local_to_local`.
- Updated dependencies to latest versions. 
- Enabled compilation with `--no-default-features` excluding shader compilation macros.
- Deserializing `AtlasFrame` uses a default `pivot` if is empty.
- Added `WindowConfig::set_window_icon_data`.
- Added `WindowConfig::set_taskbar_icon_data`.
- Added example `window_icon_from_raw.rs`.
- Changed `glsl_layout` dependency for `crevice`.
- Updated EGUI to `0.22`.
- Fixed `egui` panic when custom font are set.  
- Fixed slow scroll speed. 
- Fixed `egui needs_repaint` not working right in some situations.
- Fixed the order of the matrix multiplication for `Draw` methods.
- Improved error messages when `WebGL` and `WebGL2` contexts cannot be adquired.
- Fixed `Buffer` to allow reuse `Uniform Buffers` between pipelines.
- Changed some noisy logs from `debug` to `trace`.
- Added `Clone` to `Random`.
- Reset values of `Mouse::wheel_delta` when the user stops scrolling.
- Added `Mouse::is_scrolling`.
- App's state can use now lifetimes, ie: `State<'n>`.
- Added `Clone` to `AssetsList`.
- The `image` crate on `notan_graphics` is only used when `texture_to_file` is enabled.
- Added `WindowBackend::set_cursor_position`, `Event::MouseMotion` and `Mouse::is_moving`.
- Added new example `window_initial_position.rs`.
- Added mipmap and texture wrapping settings to `RenderTextureBuilder`.
- Added new example `texture_params`.
- Added new example `renderer_stencil`.
- Fixed mouse wheel scroll being ignored when moving the mouse at same time
- Added alt mouse wheel scrolling code to example
- Fixed `set_multisamples`. It is no longer being ignored for winit backend
- Fixed blurry text on egui when using on desktop
- Fixed mono channel audio playing in half of time set for the audio length. 
- Added `is_focused()` for winit backend
- Added `window_focus` example

Thanks everybody!

#### Contributors
* @Allstreamer
* @jonatino
* @devmeat
* @Satellile
* @fand

## rusty_engine v6.0.0

 #### What's Changed
* fix broken link from keyboard state to keyboard events page by @rozkerim in https://github.com/CleanCut/rusty_engine/pull/63
* Upgrade to Bevy 0.12 by @CleanCut in https://github.com/CleanCut/rusty_engine/pull/68

#### New Contributors
* @rozkerim made their first contribution in https://github.com/CleanCut/rusty_engine/pull/63

**Full Changelog**: https://github.com/CleanCut/rusty_engine/compare/v5.2.1...v6.0.0

## Fyrox v0.33

 Release notes - https://fyrox.rs/blog/post/fyrox-game-engine-0-33/

## Fyrox v0.32

 Release notes - https://fyrox.rs/blog/post/fyrox-game-engine-0-32/

## screen-13 v0.11.2

 Internal performance fixes and `parking_lot` is now optional.

**Full Changelog**: https://github.com/attackgoat/screen-13/compare/v0.11.1...v0.11.2

## screen-13 v0.11.1

 Log spam and validation error fixes.

**Full Changelog**: https://github.com/attackgoat/screen-13/compare/v0.11...v0.11.1

## screen-13 v0.11

 Image sampler reduction mode, information structure improvements, and performance improvements.

**Full Changelog**: https://github.com/attackgoat/screen-13/compare/v0.10.0...v0.11

## screen-13 v0.10

 Additional ray tracing functions, resource aliasing, increased performance, no more `input` module and a host of other quality of life improvements.

**Full Changelog**: https://github.com/attackgoat/screen-13/compare/v0.9.4...v0.10.0

## screen-13 v0.9.4

 Significantly improved performance of render graph resolution

## screen-13 v0.9.3

 New `FifoPool` resource pool and fix for Mac OS compilation bug

## screen-13 v0.9.2

 Improved Window surface selection, updated dependencies (including `winit` v0.29)

## screen-13 v0.9

 Image samplers, queue improvements, VR support and performance profiling.

## Eldiron Eldiron Creator v0.8.8

 An early test for releases on the new codebase. Load the starter project to get going.

This release is only meant for functional testing and cannot be used for production as too many features are missing.


## Eldiron Eldiron Creator v0.8.7

 An early test for releases on the new codebase. Load the starter project to get going.

This release is only meant for functional testing and cannot be used for production as too many features are missing.

## Eldiron Eldiron Creator v0.8.6

 An early test for releases on the new codebase.


