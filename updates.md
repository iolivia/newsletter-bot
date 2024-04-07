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




# Library Updates

## bevy_xpbd v0.4.0

 **Bevy XPBD 0.4** features several new features, bug fixes, and quality of life improvements. Here are some highlights:

- **Generic colliders**: Bevy XPBD no longer relies on just `Collider` for collision detection. You can implement custom collision backends!
- **Parry and Nalgebra are optional**: The Parry and Nalgebra dependencies are now behind feature flags (enabled by default). If you don't need collision detection or have a custom collision backend, you can disable them!
- **Access contact impulses**: It is often useful to know how strong collisions are. This information is now available in `Collision` events and the `Collisions` resource.
- **Debug render contacts**: Contact normals and impulses can now be debug rendered.
- **Layer rework**: Collision layers have been reworked to be more versatile and explicit with less footguns.
- **[Bevy 0.13](https://bevyengine.org/news/bevy-0-13/) support**: Bevy XPBD has been updated to the latest version of Bevy.
- **Colliders from primitives**: Colliders can be created from the new [geometric primitives](https://bevyengine.org/news/bevy-0-13/#primitive-shapes) introduced in Bevy 0.13.
- **`PhysicsGizmos` gizmo config group**: Debug rendering has its own gizmo configuration instead of using the global configuration.

Check out the [announcement blog post](http://joonaa.dev/blog/05/bevy-xpbd-0-4-0) for a more in-depth overview of what's changed and why. A more complete changelog can also be found after the migration guide below.

#### Migration Guide

##### Default Features (#327)

The default `Collider` now requires either the `parry-f32` or `parry-f64` feature depending on the precision you are using for Bevy XPBD. However, if you don't need colliders or have a custom collision backend, you can leave the feature disabled.

##### Layer Rework (#313)

Collision layers have been reworked, see #313.

- Groups are now called **memberships** and masks are called **filters**. This also matches Rapier's naming.
- Memberships and filters use a type called `LayerMask`, which is a bitmask for layers and a newtype for `u32`.
- All methods like `add_group`, `remove_mask`, and so on have been removed. Instead, modify the properties directly.

```rust
let layers1 = CollisionLayers::new(0b00010, 0b0111);
let layers2 = CollisionLayers::new(GameLayer::Player, [GameLayer::Enemy, GameLayer::Ground]);
let layers3 = CollisionLayers::new(LayerMask(0b0001), LayerMask::ALL);
```

Modifying layers is now done by modifying the memberships or filters directly:

```rust
layers.memberships.remove(GameLayer::Environment);
layers.filters.add([GameLayer::Environment, GameLayer::Tree]);

// Bitwise ops also work since we're accessing the bitmasks/layermasks directly.
layers.memberships |= GameLayer::Player; // You could also use a bitmask like 0b0010.
```

##### Debug rendering

The `PhysicsDebugConfig` resource and `PhysicsDebugRenderer` system parameter have been removed in favor of the new `PhysicsGizmos` [gizmo configuration group](https://bevyengine.org/news/bevy-0-13/#multiple-gizmo-configurations).

Before:

```rust
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        // Configure physics debug rendering
        .insert_resource(PhysicsDebugConfig {
            aabb_color: Some(Color::WHITE),
            ..default()
        })
        .run();
}
```

After:

```rust
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        // Configure physics debug rendering
        .insert_gizmo_group(
            PhysicsGizmos {
                aabb_color: Some(Color::WHITE),
                ..default()
            },
            GizmoConfig::default(),
        )
        .run();
}
```

This also allows you to configure e.g. line width for just physics gizmos by configuring their `GizmoConfig`.

##### Renamed `Collider` constructors (#326)

- Replace `Collider::ball` with `Collider::circle` in 2D and `Collider::sphere` in 3D
- Replace `Collider::cuboid` with `Collider::rectangle` in 2D

##### Ray and shape casting (#329)

For spatial queries, replace `Vec2`/`Vec3` directions with [`Direction2d`](https://docs.rs/bevy/0.13.0/bevy/math/primitives/struct.Direction2d.html)/[`Direction3d`](https://docs.rs/bevy/0.13.0/bevy/math/primitives/struct.Direction3d.html).

```rust
// Before
let caster = RayCaster::new(Vec3::ZERO, Vec3::X);

// After
let caster = RayCaster::new(Vec3::ZERO, Direction3d::X);
```

This applies to `RayCaster`, `ShapeCaster`, `SpatialQuery` methods like `cast_ray`, and many other methods that use directions.

#### What's Changed

* docs: Fix incorrect docs for mass component auto-initialization by @johanhelsing in https://github.com/Jondolf/bevy_xpbd/pull/234
* Don't overwrite schedules when adding plugin by @johanhelsing in https://github.com/Jondolf/bevy_xpbd/pull/236
* Take child collider rotation into account for contact normals by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/238
* Fix mesh visibility not being reset when physics debug is disabled by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/242
* Filter collisions between children of the same rigidbody in broad phase by @mbrea-c in https://github.com/Jondolf/bevy_xpbd/pull/241
* Added variant TriMeshWithFlags to ComputedCollider, fix #248 by @Adamkob12 in https://github.com/Jondolf/bevy_xpbd/pull/251
* Fix rotations when center of mass is offset by @mbrea-c in https://github.com/Jondolf/bevy_xpbd/pull/250
* Use any_orthogonal_vector to get orthogonal vector by @ollef in https://github.com/Jondolf/bevy_xpbd/pull/255
* Fix tests and doc examples, make `cargo test` compile by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/267
* fix: make `clear_forces_and_impulses` public by @ActuallyHappening in https://github.com/Jondolf/bevy_xpbd/pull/257
* Scale debug rendering of center of mass dot by axis lengths by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/268
* docs: Added Character Controller recommendation for Bevy Tnua what supports Bevy XPBD  by @dror-g in https://github.com/Jondolf/bevy_xpbd/pull/270
* Fix `Rotation` change detection triggering every frame by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/272
* Don't overwrite `Time<Physics>` when `PhysicsPlugins` are added by @johanhelsing in https://github.com/Jondolf/bevy_xpbd/pull/276
* Implement `MapEntities` for `AabbIntervals` by @johanhelsing in https://github.com/Jondolf/bevy_xpbd/pull/275
* Implement `MapEntities` for collider components by @johanhelsing in https://github.com/Jondolf/bevy_xpbd/pull/277
* Apply scale in `Collider::set_shape` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/278
* Fix dead custom constraints link in docs by @PerryPeak in https://github.com/Jondolf/bevy_xpbd/pull/280
* Ignore static-static collisions in broad phase by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/283
* Fix rotation change detection in integrator by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/284
* Fix static body handling in `update_aabb_intervals` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/285
* Fix `DistanceJoint` distance limits by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/286
* Preserve collisions between inactive entities, add sensor example by @TeamDman in https://github.com/Jondolf/bevy_xpbd/pull/266
* docs: use the read function for iterating over events by @tremorris1999 in https://github.com/Jondolf/bevy_xpbd/pull/290
* docs: corrects other outdated calls to .iter by @tremorris1999 in https://github.com/Jondolf/bevy_xpbd/pull/291
* Fix `Time` inconsistency after substepping loop by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/294
* Make PreparePlugin configurable by @Rigidity in https://github.com/Jondolf/bevy_xpbd/pull/292
* Adding Collider::round_cuboid by @kav in https://github.com/Jondolf/bevy_xpbd/pull/300
* Add section about camera following jitter to FAQ by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/305
* Add intersection and point queries to `Collider` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/307
* Debug render contact normals by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/308
* Implement cast_ray_predicate to allow filtering the colliders with a function by @Affinator in https://github.com/Jondolf/bevy_xpbd/pull/297
* Fix colliders without `RigidBody` not working by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/323
* fix raycast does not follow entity transform without rigidbody by @zwazel in https://github.com/Jondolf/bevy_xpbd/pull/310
* Store impulses in contacts and refactor contact data by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/324
* Add `ColliderBackendPlugin` and support generic colliders by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/311
* Rework layers by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/313
* Make `Collider` optional, allowing usage without Parry or Nalgebra by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/327
* Fix doc examples by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/330
* Update to Bevy 0.13 by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/315

#### New Contributors

* @mattdm made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/227
* @mbrea-c made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/241
* @Adamkob12 made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/251
* @ollef made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/255
* @ActuallyHappening made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/257
* @dror-g made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/270
* @PerryPeak made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/280
* @TeamDman made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/266
* @tremorris1999 made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/290
* @Rigidity made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/292
* @kav made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/300
* @Affinator made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/297
* @zwazel made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/310

**Full Changelog**: https://github.com/Jondolf/bevy_xpbd/compare/v0.3.0...v0.4.0

## bevy_xpbd v0.3.0

 0.3 is a huge release with tons of new features, bug fixes, and quality of life improvements, especially for collision detection. Here are some highlights:

- **Modular narrow phase**: Narrow phase collision detection has been refactored into modular plugins instead of being tightly coupled with the solver
- **Improved contact stability**: Collisions are significantly more stable, and dynamic friction has been fixed to handle different masses correctly
- **Collider scale**: Colliders can be scaled using `Transform`
- **Child colliders**: Colliders can be freely nested using entity hierarchies
- **Async colliders**: Colliders can be automatically generated from meshes and glTF scenes
- **Accessing, modifying and filtering collision**: The new `Collisions` resource can be used for accessing and modifying contacts in custom systems
- **`Transform` for positions**: `Transform` can be used instead of the internal `Position` and `Rotation` components
- **Debug rendering**: Physics objects and interactions can be rendered for debugging purposes
- **Dominance**: Dynamic bodies can be configured to have infinite mass relative to bodies with a lesser dominance
- **Bevy 0.12 support**: Bevy XPBD has been updated to Bevy 0.12
- **Time unification**: Timing and scheduling resources have been replaced by the unified `Time<Physics>` and `Time<Substeps>` resources

Read the more complete changelog after the migration guide for more details.

#### Migration guide

Here is a (non-exhaustive) migration guide for migrating from 0.2 to 0.3.

##### Collision iteration

Before:

```rust
fn my_system(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contact) in collision_event_reader.iter() {
        println!("Penetration depth: {}", contact.penetration);
    }
}
```

After:

```rust
fn my_system(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        for manifold in contacts.manifolds.iter() {
            for contact in manifold.contacts.iter() {
                println!("Penetration depth: {}", contact.penetration);
            }
        }
    }
}
```

This is more verbose, but it provides multiple contact points instead just one. In the future, this will hopefully be made more ergonomic with helper methods.

A new and more powerful `Collisions` resource was also added. It can be used to achieve a similar result.

##### Collider scale (#189)

Before:

```rust
let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
commands.spawn((
    PbrBundle {
        mesh,
        transform: Transform::from_scale(Vec3::new(10.0, 1.0, 10.0)),
        ..default()
    },
    // Collider isn't scaled by transform
    Collider::cuboid(10.0, 1.0, 10.0),
    RigidBody::Static,
));
```

After:

```rust
let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
commands.spawn((
    PbrBundle {
        mesh,
        transform: Transform::from_scale(Vec3::new(10.0, 1.0, 10.0)),
        ..default()
    },
    // Collider size takes transform scale into account
    Collider::cuboid(1.0, 1.0, 1.0),
    RigidBody::Static,
));
```

##### Collider creation from meshes

- `trimesh_from_bevy_mesh` → `trimesh_from_mesh`
- `convex_decomposition_from_bevy_mesh` → `convex_decomposition_from_mesh`

##### Unified time (#214)

- Replace every `DeltaTime` in `PhysicsSchedule` and every `SubDeltaTime` in `SubstepSchedule` with `Time`, elsewhere explicitly use `Time<Physics>` and `Time<Substep>`
- When advancing physics manually, instead of setting `DeltaTime`, advance the `Time<Physics>` clock using `time.advance_by(...)`
- Replace `PhysicsTimestep` with `Time::new_with(Physics::fixed_hz(...))` and so on
- Replace `PhysicsLoop::pause/resume` with `Time::<Physics>::pause/unpause`
- Replace `PhysicsLoop::step` with advancing the physics clock using `Time::<Physics>::advance_by`
- Replace `PhysicsTimescale` usage with `Time::<Physics>::with/set_relative_speed`

#### What's Changed

* Debug render colliders by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/73
* Add more global and entity-level debug render options by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/74
* Add debug rendering axes and joints and improve configuration API by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/76
* Move `SpatialQuery` methods to `SpatialQueryPipeline` by @LeshaInc in https://github.com/Jondolf/bevy_xpbd/pull/77
* Add `enabled` flag to `PhysicsDebugConfig` to globally disable debug rendering at runtime by @LeshaInc in https://github.com/Jondolf/bevy_xpbd/pull/78
* Add `trimesh_with_flags` and `trimesh_from_bevy_mesh_with_flags` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/79
* Add PhysicsTimescale resource for slow-motion or fast-forward simulation by @dasisdormax in https://github.com/Jondolf/bevy_xpbd/pull/80
* Use world-space center of mass in penetration constraint by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/81
* Add `clear` methods for ray and shape hits and clear the hits on disable by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/82
* Expand AABBs only in the movement direction by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/83
* Improve simulation stability when object are far from world origin  by @LeshaInc in https://github.com/Jondolf/bevy_xpbd/pull/84
* Add back global contact positions by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/86
* Run transform propagation before `init_rigid_bodies`. Fixes #88 by @LeshaInc in https://github.com/Jondolf/bevy_xpbd/pull/89
* Add criterion benchmarks by @LeshaInc in https://github.com/Jondolf/bevy_xpbd/pull/91
* Setup `PhysicsSchedule` and `SubstepSchedule` to use single-threaded executor by @LeshaInc in https://github.com/Jondolf/bevy_xpbd/pull/92
* Use contact manifolds instead of single contacts for collisions by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/90
* Add `ExternalImpulse` and `ExternalAngularImpulse` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/95
* Filter operation reodered for a little performance boost by @TrustNoOneElse in https://github.com/Jondolf/bevy_xpbd/pull/98
* Store local contact normals and transform them into world-space at each solve by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/97
* Support `Transform` for moving and positioning bodies by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/96
* Fix typo in docs by @Zentropivity in https://github.com/Jondolf/bevy_xpbd/pull/103
* Automatically add `Position` and `Rotation` for colliders by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/101
* Separate narrow phase from solver into `NarrowPhasePlugin` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/100
* feature: Add DistanceJoint and 2D and 3D examples. by @shanecelis in https://github.com/Jondolf/bevy_xpbd/pull/105
* Debug render rigid body axes at center of mass by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/109
* Added text that explained how to use the example 'chain_3d' by @Aztro-dev in https://github.com/Jondolf/bevy_xpbd/pull/107
* Add "Bounciness" and "Elasticity" aliases for `Restitution` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/117
* Clamp coefficient of restitution between 0 and 1 and improve restitution docs by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/118
* Add `current_position` getter for `RigidBodyQueryItem` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/120
* Remove solver iteration loop by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/121
* Clarify that external forces and impulses are local (edit: not true, fixed by #144) by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/123
* Register `CoefficientCombine` by @grace125 in https://github.com/Jondolf/bevy_xpbd/pull/126
* Correct time dilation for FixedUpdate schedule. by @starwolfy in https://github.com/Jondolf/bevy_xpbd/pull/128
* Fix center of mass and inertia computations and add tests by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/127
* add Rotation::from_sin_cos constructor by @RJ in https://github.com/Jondolf/bevy_xpbd/pull/130
* Avoid allocate on spatial queries. ray_hits_callback, shape_hits_callback, intersections_callback by @bnyu in https://github.com/Jondolf/bevy_xpbd/pull/133
* Dynamic friction fix by @felixbjorkeson in https://github.com/Jondolf/bevy_xpbd/pull/52
* Fix 2d `position_to_transform` changing child `Transform` z-component by @ShaddowSpy in https://github.com/Jondolf/bevy_xpbd/pull/134
* Make Rotation & PreviousRotation derive PartialEq by @RJ in https://github.com/Jondolf/bevy_xpbd/pull/136
* Fix tests on mac m1 by @RJ in https://github.com/Jondolf/bevy_xpbd/pull/138
* Register/Reflect Component of Sensor by @grace125 in https://github.com/Jondolf/bevy_xpbd/pull/141
* Fix `Transform` initialization for children and refactor `prepare.rs` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/140
* Clarify that forces are in world space and improve force docs by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/144
* Fix math cross-platform determinism by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/145
* Combine `insert` calls into bundles to reduce archetype count by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/151
* Avoid incremental updates to Qbvh by @NiseVoid in https://github.com/Jondolf/bevy_xpbd/pull/152
* Fix contact stability for non-convex colliders by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/156
* Collision post processing by @datael in https://github.com/Jondolf/bevy_xpbd/pull/155
* Add `Trigger` alias for `Sensor` and improve `Sensor` docs by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/160
* Make `PreviousGlobalTransform` public and derive traits by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/166
* Add contact queries by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/169
* Use cache in CI and run tests on multiple operating systems by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/153
* Ensure newly created bodies get simulated the frame they are added by @RJ in https://github.com/Jondolf/bevy_xpbd/pull/174
* Skip re-writing the same values to components, to satisfy bevy's change detection. by @RJ in https://github.com/Jondolf/bevy_xpbd/pull/173
* Add PR template by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/176
* Improve `Collisions` API and docs by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/175
* Fix sleeping and collision event issues by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/112
* Add `Dominance` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/178
* Move narrow phase collision validity checks to broad phase by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/179
* Add `ContactReportingPlugin` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/182
* Move all collision logic and plugins into new `collision` module by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/184
* Add debug rendering for sleeping bodies by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/185
* Use swept AABBs for collider AABBs by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/188
* Child colliders by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/154
* Collider scale by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/189
* Map entities by @johanhelsing in https://github.com/Jondolf/bevy_xpbd/pull/192
* Add `ColliderDensity` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/194
* Debug render spatial queries by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/186
* Expose `SyncConfig` fields by @johanhelsing in https://github.com/Jondolf/bevy_xpbd/pull/195
* Async colliders by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/190
* Warn about explosive behavior caused by overlapping at spawn by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/202
* Add `ignore_self` property for `RayCaster` and `ShapeCaster` by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/205
* Use `Transform` instead of `Position` and `Rotation` in examples by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/201
* Fix sleeping debug rendering for child colliders by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/206
* Improve documentation by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/208
* Make `debug-plugin` a default feature and improve docs by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/209
* Enable bevy's png feature in the 3d examples by @Friz64 in https://github.com/Jondolf/bevy_xpbd/pull/203
* Improve movement in examples by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/210
* Refactor character controller examples and support gamepad input by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/212
* Update to `actions/checkout@v4`. by @waywardmonkeys in https://github.com/Jondolf/bevy_xpbd/pull/215
* docs: Use more backticks. by @waywardmonkeys in https://github.com/Jondolf/bevy_xpbd/pull/216
* Use intradoc links rather than #method anchors. by @waywardmonkeys in https://github.com/Jondolf/bevy_xpbd/pull/220
* Move x comparison in broad phase up to early-out as soon as possible by @datael in https://github.com/Jondolf/bevy_xpbd/pull/219
* Improve character controller examples by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/222
* Add "Acknowledgements" section to README.md by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/143
* Upgrade to Bevy 0.12 by @Jondolf in https://github.com/Jondolf/bevy_xpbd/pull/187

#### New Contributors
* @LeshaInc made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/77
* @dasisdormax made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/80
* @TrustNoOneElse made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/98
* @Zentropivity made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/103
* @shanecelis made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/105
* @Aztro-dev made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/107
* @grace125 made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/126
* @starwolfy made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/128
* @RJ made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/130
* @bnyu made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/133
* @felixbjorkeson made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/52
* @ShaddowSpy made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/134
* @datael made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/155
* @Friz64 made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/203
* @waywardmonkeys made their first contribution in https://github.com/Jondolf/bevy_xpbd/pull/215

**Full Changelog**: https://github.com/Jondolf/bevy_xpbd/compare/v0.2.0...v0.3.0

## sparsey v0.12.0

 Complete rewrite improving performance in all benchmarks.

### Changed

- `World` renamed to `EntityStorage`.
- `Resources` renamed to `ResourceStorage`.
- `World` now includes an `EntityStorage` and a `ResourceStorage`.

### Removed

- Removed `Schedule` and `ScheduleBuilder`.
- Resources can no longer be `!Send` or `!Sync`.

## sparsey v0.11.1

 #### Added
- Added `get_mut` and `try_get_mut` functions for getting mutable references to resources from a `&mut Resources`.

## iced 0.12.1

 ##### Added
- `extend` and `from_vec` methods for `Column` and `Row`. [#2264](https://github.com/iced-rs/iced/pull/2264)
- `PartialOrd`, `Ord`, and `Hash` implementations for `keyboard::Modifiers`. [#2270](https://github.com/iced-rs/iced/pull/2270)
- `clipboard` module in `advanced` module. [#2272](https://github.com/iced-rs/iced/pull/2272)
- Default `disabled` style for `checkbox` and `hovered` style for `Svg`. [#2273](https://github.com/iced-rs/iced/pull/2273)
- `From<u16>` and `From<i32>` implementations for `border::Radius`. [#2274](https://github.com/iced-rs/iced/pull/2274)
- `size_hint` method for `Component` trait. [#2275](https://github.com/iced-rs/iced/pull/2275)

##### Fixed
- Black images when using OpenGL backend in `iced_wgpu`. [#2259](https://github.com/iced-rs/iced/pull/2259)
- Documentation for `horizontal_space` and `vertical_space` helpers. [#2265](https://github.com/iced-rs/iced/pull/2265)
- WebAssembly platform. [#2271](https://github.com/iced-rs/iced/pull/2271)
- Decouple `Key` from `keyboard::Modifiers` and apply them to `text` in `KeyboardInput`. [#2238](https://github.com/iced-rs/iced/pull/2238)
- Text insertion not being prioritized in `TextInput` and `TextEditor`. [#2278](https://github.com/iced-rs/iced/pull/2278)
- `iced_tiny_skia` clipping line strokes. [#2282](https://github.com/iced-rs/iced/pull/2282)

Many thanks to...

- @bungoboingo
- @PolyMeilex
- @rizzen-yazston
- @wash2

## iced 0.12.0

 ##### Added
- Multi-window support. [#1964](https://github.com/iced-rs/iced/pull/1964)
- `TextEditor` widget (or multi-line text input). [#2123](https://github.com/iced-rs/iced/pull/2123)
- `Shader` widget. [#2085](https://github.com/iced-rs/iced/pull/2085)
- Shadows. [#1882](https://github.com/iced-rs/iced/pull/1882)
- Vectorial text for `Canvas`. [#2204](https://github.com/iced-rs/iced/pull/2204)
- Layout consistency. [#2192](https://github.com/iced-rs/iced/pull/2192)
- Explicit text caching. [#2058](https://github.com/iced-rs/iced/pull/2058)
- Gradients in Oklab color space. [#2055](https://github.com/iced-rs/iced/pull/2055)
- `Themer` widget. [#2209](https://github.com/iced-rs/iced/pull/2209)
- `Transform` primitive. [#2120](https://github.com/iced-rs/iced/pull/2120)
- Cut functionality for `TextEditor`. [#2215](https://github.com/iced-rs/iced/pull/2215)
- Primary clipboard support. [#2240](https://github.com/iced-rs/iced/pull/2240)
- Disabled state for `Checkbox`. [#2109](https://github.com/iced-rs/iced/pull/2109)
- `skip_taskbar` window setting for Windows. [#2211](https://github.com/iced-rs/iced/pull/2211)
- `fetch_maximized` and `fetch_minimized` commands in `window`. [#2189](https://github.com/iced-rs/iced/pull/2189)
- `run_with_handle` command in `window`. [#2200](https://github.com/iced-rs/iced/pull/2200)
- `show_system_menu` command in `window`. [#2243](https://github.com/iced-rs/iced/pull/2243)
- `text_shaping` method for `Tooltip`. [#2172](https://github.com/iced-rs/iced/pull/2172)
- `interaction` method for `MouseArea`. [#2207](https://github.com/iced-rs/iced/pull/2207)
- `hovered` styling for `Svg` widget. [#2163](https://github.com/iced-rs/iced/pull/2163)
- `height` method for `TextEditor`. [#2221](https://github.com/iced-rs/iced/pull/2221)
- Customizable style for `TextEditor`. [#2159](https://github.com/iced-rs/iced/pull/2159)
- Customizable style for `QRCode`. [#2229](https://github.com/iced-rs/iced/pull/2229)
- Border width styling for `Toggler`. [#2219](https://github.com/iced-rs/iced/pull/2219)
- `RawText` variant for `Primitive` in `iced_graphics`. [#2158](https://github.com/iced-rs/iced/pull/2158)
- `Stream` support for `Command`. [#2150](https://github.com/iced-rs/iced/pull/2150)
- Access to bounds/content bounds from a `Scrollable` viewport. [#2072](https://github.com/iced-rs/iced/pull/2072)
- `Frame::scale_nonuniform` method. [#2070](https://github.com/iced-rs/iced/pull/2070)
- `theme::Custom::with_fn` to generate completely custom themes. [#2067](https://github.com/iced-rs/iced/pull/2067)
- `style` attribute for `Font`. [#2041](https://github.com/iced-rs/iced/pull/2041)
- Texture filtering options for `Image`. [#1894](https://github.com/iced-rs/iced/pull/1894)
- `default` and `shift_step` methods for `slider` widgets. [#2100](https://github.com/iced-rs/iced/pull/2100)
- `Custom` variant to `command::Action`. [#2146](https://github.com/iced-rs/iced/pull/2146)
- Mouse movement events for `MouseArea`. [#2147](https://github.com/iced-rs/iced/pull/2147)
- Dracula, Nord, Solarized, and Gruvbox variants for `Theme`. [#2170](https://github.com/iced-rs/iced/pull/2170)
- Catppuccin, Tokyo Night, Kanagawa, Moonfly, Nightfly and Oxocarbon variants for `Theme`. [#2233](https://github.com/iced-rs/iced/pull/2233)
- `From<T> where T: Into<PathBuf>` for `svg::Handle`. [#2235](https://github.com/iced-rs/iced/pull/2235)
- `on_open` and `on_close` handlers for `PickList`. [#2174](https://github.com/iced-rs/iced/pull/2174)
- Support for generic `Element` in `Tooltip`. [#2228](https://github.com/iced-rs/iced/pull/2228)
- Container and `gap` styling for `Scrollable`. [#2239](https://github.com/iced-rs/iced/pull/2239)
- Use `Borrow` for both `options` and `selected` in PickList. [#2251](https://github.com/iced-rs/iced/pull/2251)
- `clip` property for `Container`, `Column`, `Row`, and `Button`. #[2252](https://github.com/iced-rs/iced/pull/2252)

##### Changed
- Enable WebGPU backend in `wgpu` by default instead of WebGL. [#2068](https://github.com/iced-rs/iced/pull/2068)
- Update `glyphon` to `0.4`. [#2203](https://github.com/iced-rs/iced/pull/2203)
- Require `Send` on stored pipelines. [#2197](https://github.com/iced-rs/iced/pull/2197)
- Update `wgpu` to `0.19`, `glyphon` to `0.5`, `softbuffer` to `0.4`, `window-clipboard` to `0.4`, and `raw-window-handle` to `0.6`. [#2191](https://github.com/iced-rs/iced/pull/2191)
- Update `winit` to `0.29`. [#2169](https://github.com/iced-rs/iced/pull/2169)
- Provide actual bounds to `Shader` primitives. [#2149](https://github.com/iced-rs/iced/pull/2149)
- Deny warnings in `test` workflow. [#2135](https://github.com/iced-rs/iced/pull/2135)
- Update `wgpu` to `0.18` and `cosmic-text` to `0.10`. [#2122](https://github.com/iced-rs/iced/pull/2122)
- Compute vertex positions in the shader. [#2099](https://github.com/iced-rs/iced/pull/2099)
- Migrate twox-hash -> xxhash-rust and switch to Xxh3 for better performance. [#2080](https://github.com/iced-rs/iced/pull/2080)
- Add `keyboard` subscriptions and rename `subscription::events` to `event::listen`. [#2073](https://github.com/iced-rs/iced/pull/2073)
- Use workspace dependencies and package inheritance. [#2069](https://github.com/iced-rs/iced/pull/2069)
- Update `wgpu` to `0.17`. [#2065](https://github.com/iced-rs/iced/pull/2065)
- Support automatic style type casting for `Button`. [#2046](https://github.com/iced-rs/iced/pull/2046)
- Make `with_clip` and `with_save` in `Frame` able to return the data of the provided closure. [#1994](https://github.com/iced-rs/iced/pull/1994)
- Use `Radians` for angle fields in `Arc` and `arc::Elliptical`. [#2027](https://github.com/iced-rs/iced/pull/2027)
- Assert dimensions of quads are normal in `iced_tiny_skia`. [#2082](https://github.com/iced-rs/iced/pull/2082)
- Remove `position` from `overlay::Element`. [#2226](https://github.com/iced-rs/iced/pull/2226)
- Add a capacity limit to the `GlyphCache` in `iced_tiny_skia`. [#2210](https://github.com/iced-rs/iced/pull/2210)
- Use pointer equality to speed up `PartialEq` implementation of `image::Bytes`. [#2220](https://github.com/iced-rs/iced/pull/2220)
- Update `bitflags`, `glam`, `kurbo`, `ouroboros`, `qrcode`, and `sysinfo` dependencies. [#2227](https://github.com/iced-rs/iced/pull/2227)
- Improve some widget ergonomics. [#2253](https://github.com/iced-rs/iced/pull/2253)

##### Fixed
- Clipping of `TextInput` selection. [#2199](https://github.com/iced-rs/iced/pull/2199)
- `Paragraph::grapheme_position` when ligatures are present. [#2196](https://github.com/iced-rs/iced/pull/2196)
- Docs to include missing feature tags. [#2184](https://github.com/iced-rs/iced/pull/2184)
- `PaneGrid` click interaction on the top edge. [#2168](https://github.com/iced-rs/iced/pull/2168)
- `iced_wgpu` not rendering text in SVGs. [#2161](https://github.com/iced-rs/iced/pull/2161)
- Text clipping. [#2154](https://github.com/iced-rs/iced/pull/2154)
- Text transparency in `iced_tiny_skia`. [#2250](https://github.com/iced-rs/iced/pull/2250)
- Layout invalidation when `Tooltip` changes `overlay`. [#2143](https://github.com/iced-rs/iced/pull/2143)
- `Overlay` composition. [#2142](https://github.com/iced-rs/iced/pull/2142)
- Incorrect GIF for the `progress_bar` example. [#2141](https://github.com/iced-rs/iced/pull/2141)
- Standalone compilation of `iced_renderer` crate. [#2134](https://github.com/iced-rs/iced/pull/2134)
- Maximize window button enabled when `Settings::resizable` is `false`. [#2124](https://github.com/iced-rs/iced/pull/2124)
- Width of horizontal scrollbar in `Scrollable`. [#2084](https://github.com/iced-rs/iced/pull/2084)
- `ComboBox` widget panic on wasm. [#2078](https://github.com/iced-rs/iced/pull/2078)
- Majority of unresolved documentation links. [#2077](https://github.com/iced-rs/iced/pull/2077)
- Web examples not running. [#2076](https://github.com/iced-rs/iced/pull/2076)
- GIFs and video examples broken. [#2074](https://github.com/iced-rs/iced/pull/2074)
- `@interpolate(flat)` not used as attribute. [#2071](https://github.com/iced-rs/iced/pull/2071)
- `Checkbox` and `Toggler` hidden behind scrollbar in `styling` example. [#2062](https://github.com/iced-rs/iced/pull/2062)
- Absolute `LineHeight` sometimes being `0`. [#2059](https://github.com/iced-rs/iced/pull/2059)
- Paste while holding ALT. [#2006](https://github.com/iced-rs/iced/pull/2006)
- `Command<T>::perform` to return a `Command<T>`. [#2000](https://github.com/iced-rs/iced/pull/2000)
- `convert_text` not called on `Svg` trees. [#1908](https://github.com/iced-rs/iced/pull/1908)
- Unused `backend.rs` file in renderer crate. [#2182](https://github.com/iced-rs/iced/pull/2182)
- Some `clippy::pedantic` lints. [#2096](https://github.com/iced-rs/iced/pull/2096)
- Some minor clippy fixes. [#2092](https://github.com/iced-rs/iced/pull/2092)
- Clippy docs keyword quoting. [#2091](https://github.com/iced-rs/iced/pull/2091)
- Clippy map transformations. [#2090](https://github.com/iced-rs/iced/pull/2090)
- Inline format args for ease of reading. [#2089](https://github.com/iced-rs/iced/pull/2089)
- Stuck scrolling in `Scrollable` with touch events. [#1940](https://github.com/iced-rs/iced/pull/1940)
- Incorrect unit in `system::Information`. [#2223](https://github.com/iced-rs/iced/pull/2223)
- `size_hint` not being called from `element::Map`. [#2224](https://github.com/iced-rs/iced/pull/2224)
- `size_hint` not being called from `element::Explain`. [#2225](https://github.com/iced-rs/iced/pull/2225)
- Slow touch scrolling for `TextEditor` widget. [#2140](https://github.com/iced-rs/iced/pull/2140)
- `Subscription::map` using unreliable function pointer hash to identify mappers. [#2237](https://github.com/iced-rs/iced/pull/2237)
- Missing feature flag docs for `time::every`. [#2188](https://github.com/iced-rs/iced/pull/2188)
- Event loop not being resumed on Windows while resizing. [#2214](https://github.com/iced-rs/iced/pull/2214)
- Alpha mode misconfiguration in `iced_wgpu`. [#2231](https://github.com/iced-rs/iced/pull/2231)
- Outdated documentation leading to a dead link. [#2232](https://github.com/iced-rs/iced/pull/2232)

Many thanks to...

- @akshayr-mecha
- @alec-deason
- @arslee07
- @AustinMReppert
- @avsaase
- @blazra
- @brianch
- @bungoboingo
- @Calastrophe
- @casperstorm
- @cfrenette
- @clarkmoody
- @Davidster
- @Decodetalkers
- @derezzedex
- @DoomDuck
- @dtzxporter
- @Dworv
- @fogarecious
- @GyulyVGC
- @hicaru
- @ids1024
- @Imberflur
- @jhannyj
- @jhff
- @jim-ec
- @joshuamegnauth54
- @jpttrssn
- @julianbraha
- @Koranir
- @lufte
- @matze
- @MichalLebeda
- @MoSal
- @MrAntix
- @nicksenger
- @Nisatru
- @nyurik
- @Remmirad
- @ripytide
- @snaggen
- @Tahinli
- @tarkah
- @tzemanovic
- @varbhat
- @VAWVAW
- @william-shere
- @wyatt-herkamp

## bevy_vello v0.1.0

 #### 0.1.0 (2024-03-26)

- Initial release

## hexx 0.16.1

 #### What's Changed

* impl PartialEq and Eq on HexBounds by @ManevilleF in https://github.com/ManevilleF/hexx/pull/160
* Fix direction const values by @ManevilleF in https://github.com/ManevilleF/hexx/pull/162

**Full Changelog**: https://github.com/ManevilleF/hexx/compare/0.16.0...0.16.1

## hexx 0.16.0

 Quite a big release, a lot of fixes, new features, and unfortunately a lot of breaking changes ! 

If you have **any** trouble migrating from previous version, please let me know and I'll do whatever I can to clarify the changes and help with the migration.

#### Changelog

* Removed methods deprecated in previous versions
* Added `z` field in the `Debug` impl of `Hex` (#156)
* Added `xyz` fields in the `Debug` impl of directions (#156)
* (**BREAKING**) Hex neighbors are now following a clockwise order (#157)
* (**BREAKING**) Hex diagonal neighbors are now following a clockwise order (#157)
* Added new `hex_area` example (#157)
* Removed deprecated `ser_de` feature, use `serde` instead

##### New grid utilities (#154)

* Added new `grid` feature gate
* Added `GridVertex` and `GridEgde` types, representing oriented grid vertices
and edges

##### New directions (#156, #157)

* (**BREAKING**) Hex edge and diagonal neighbors are now following a clockwise order
* (**BREAKING**) Direction types are now following a clockwise order
* (**BREAKING**) Renamed `Direction` to `EdgeDirection`, and is no longer an enum.
Instead of the oriented variants use associated const values:
  * `Direction::TopRight` -> `EdgeDirection::FLAT_TOP_RIGHT` or `EdgeDirection::POINTY_RIGHT`
  * `Direction::Top` -> `EdgeDirection::FLAT_TOP` or `EdgeDirection::POINTY_TOP_RIGHT`
  * `Direction::TopLeft` -> `EdgeDirection::FLAT_TOP_LEFT` or `EdgeDirection::POINTY_TOP_LEFT`
  * `Direction::BottomLeft` -> `EdgeDirection::FLAT_BOTTOM_LEFT` or `EdgeDirection::POINTY_LEFT`
  * `Direction::Bottom` -> `EdgeDirection::FLAT_BOTTOM` or `EdgeDirection::POINTY_BOTTOM_LEFT`
  * `Direction::BottomRight` -> `EdgeDirection::FLAT_BOTTOM_RIGHT` or `EdgeDirection::POINTY_BOTTOM_RIGHT`
* (**BREAKING**) Renamed `DiagonalDirection` to `VertexDirection`, and is no
longer an enum. Instead of the oriented variants use associated const values:
  * `DiagonalDirection::Right` -> `VertexDirection::FLAT_RIGHT` or `VertexDirection::POINTY_BOTTOM_RIGHT`
  * `DiagonalDirection::TopRight` -> `VertexDirection::FLAT_TOP_RIGHT` or `VertexDirection::POINTY_TOP_RIGHT`
  * `DiagonalDirection::TopLeft` -> `VertexDirection::FLAT_TOP_LEFT` or `VertexDirection::POINTY_TOP`
  * `DiagonalDirection::Left` -> `VertexDirection::FLAT_LEFT` or `VertexDirection::POINTY_TOP_LEFT`
  * `DiagonalDirection::BottomLeft` -> `VertexDirection::FLAT_BOTTOM_LEFT` or `VertexDirection::POINTY_BOTTOM_LEFT`
  * `DiagonalDirection::BottomRight` -> `VertexDirection::FLAT_BOTTOM_RIGHT` or `VertexDirection::POINTY_BOTTOM`
* Fixed angle inconsistencies in both direction types
* (**BREAKING**) Removed `HexOrientation::direction_angle` method

##### Mesh generation overhaul (#152)

* Added new `MeshInfo` methods:
  * `with_scale`
  * `with_uv_scale`
  * `centroid`
  * `uv_centroid`
* (**BREAKING**) Changed the way `ColumnMeshBuilder` generates quad to be consistent
with hexagonal faces
* (**BREAKING**) Changed inner `ColumnMeshBuilder` fields, but the builder API was
kept consistent
* Fixed the way `ColumnMeshBuilder` generate the hexagonal caps, which could behave
strangely with non center aligned layout
* Added a `mesh::utils` modules for primitive shape management
* Added `ColumnMeshBuilder::with_sides_uv_options_fn` for block based options setting
* Added mesh insetting options:
  * `ColumnMeshBuilder::with_caps_inset_options` to inset the column hexagonal faces
  * `ColumnMeshBuilder::with_sides_inset_options` to inset the column side quads
  * `PlaneMeshBuilder::with_inset_options` to inset the hexagonal face

#### Pull requests

* Feat/mesh insetting by @ManevilleF in https://github.com/ManevilleF/hexx/pull/152
* Feat/new directions by @ManevilleF in https://github.com/ManevilleF/hexx/pull/156
* Grid edges and vertices by @ManevilleF in https://github.com/ManevilleF/hexx/pull/154
* Follow up fixes and improvements to new directions by @ManevilleF in https://github.com/ManevilleF/hexx/pull/157

**Full Changelog**: https://github.com/ManevilleF/hexx/compare/0.15.0...0.16.0

## hexx 0.15.0

 #### What's Changed
* Bevy 0.13 by @ManevilleF in https://github.com/ManevilleF/hexx/pull/149

**Full Changelog**: https://github.com/ManevilleF/hexx/compare/0.14.0...0.15.0

## hexx 0.14.0

 #### Release notes

- MSRV set to `1.72.1`
- Added gizmos to mesh_builder example

##### Breaking changes

- **Fixed** UV generation for hexagonal planes, as a consequence:
  -  Deprecated `UVOptions::quad_default` in favor of `UVOptions::new` 
  -  Deprecated `UVOptions::cap_default` in favor of `UVOptions::new 
- `MeshInfo::cheap_hexagonal_column` now has 12 vertices instead of 13

#### What's Changed
* 0.14 chores by @ManevilleF in https://github.com/ManevilleF/hexx/pull/145

**Full Changelog**: https://github.com/ManevilleF/hexx/compare/0.13.0...0.14.0

## hexx 0.13.0

 #### Release notes

##### algorithms

* (**BREAKING**) `a_star` `cost` function parameter now takes two adjacent `Hex`
nodes instead of one, allowing for more use cases (#130, #128)
* Fixed `field_of_movement` algorithm (#142, #127)

##### Dependencies

* Bumped `bevy_inspector_egui` dev dependency (#129)
* Added `bevy_egui` dev dependency (#143)

##### Examples

* Added a `sprite_sheet` bevy example (#135)
* Improved `mesh_builder` example (#143)

##### Additions

* Added `HexLayout::rect_size` method (#135)
* Added `ColumnMeshBuilder::center_aligned` option (#139)
* Added `PlaneMeshBuilder::center_aligned` option (#139)
* Added `Hex::to_array_f32` utility method (#141)
* Added `Hex::to_cubic_array_f32` utility method (#141)
* Added `HexLayout::fract_hex_to_world_pos` method (#141, #138, #140)
* Added `HexLayout::world_pos_to_fract_hex` method (#141, #138, #140)
* Added `HexOrientationData::forward` method (#141)
* Added `HexOrientationData::inverse` method (#141)
* Added coordinate expressive const values for `Direction` (#144)
* Added coordinate expressive const values for `DiagonalDirection` (#144)

##### Mesh generation

* `ColumnMeshBuilder` now accepts custom `UvOptions` for each 6 sides (#143)
  * Added `ColumnMeshBuilder::with_multi_sides_uv_options` method (#143)
* `UVOptions` changes:
  * (**BREAKING**) changed `flip_x` and `flip_y` fields to `flip` BVec2 (#143)
  * Added `rect` field, to remap the coordinates in specific sections (#143)
  * Added `with_rect` builder method (#143)
  * Changed the order of operations in `alter_uvs` (#143)
* (**BREAKING**) Fixed quad generation which had upside down uvs (#143)

##### Deprecation

* Deprecated `MeshInfo::hexagonal_plane` in favor of `PlaneMeshBuilder` (#139)


#### What's Changed

* Updated bevy_inspector_egui by @ManevilleF in https://github.com/ManevilleF/hexx/pull/129
* Feat/algorithm improvements by @ManevilleF in https://github.com/ManevilleF/hexx/pull/130
* Sprite Sheet example by @ManevilleF in https://github.com/ManevilleF/hexx/pull/135
* Center aligned meshes by @ManevilleF in https://github.com/ManevilleF/hexx/pull/139
* Fractional hex on HexLayout by @ManevilleF in https://github.com/ManevilleF/hexx/pull/141
* Better UV options by @ManevilleF in https://github.com/ManevilleF/hexx/pull/143
* QOL improvements by @ManevilleF in https://github.com/ManevilleF/hexx/pull/144
* Fix for field_of_movement algorithm by @ManevilleF in https://github.com/ManevilleF/hexx/pull/142

**Full Changelog**: https://github.com/ManevilleF/hexx/compare/0.12.0...0.13.0

## hexx 0.12.0

 #### What's Changed
* Mesh can be scaled by @ManevilleF in https://github.com/ManevilleF/hexx/pull/121
* typo fixes by @asibahi in https://github.com/ManevilleF/hexx/pull/122
* Bevy 0.12 by @ManevilleF in https://github.com/ManevilleF/hexx/pull/123

#### New Contributors
* @asibahi made their first contribution in https://github.com/ManevilleF/hexx/pull/122

**Full Changelog**: https://github.com/ManevilleF/hexx/compare/0.11.0...0.12.0

## hexx 0.11.0

 #### What's Changed

* Hexagonal plane simplification by @ManevilleF in https://github.com/ManevilleF/hexx/pull/119
* Use of rust built in const f32 by @ManevilleF in https://github.com/ManevilleF/hexx/pull/113
* Rust 1.72.0 by @ManevilleF in https://github.com/ManevilleF/hexx/pull/114

**Full Changelog**: https://github.com/ManevilleF/hexx/compare/0.10.0...0.11.0

## hexx 0.10.1 [YANKED]

 Yanked to fix #116 

#### What's Changed

* Use of rust built in const f32 by @ManevilleF in https://github.com/ManevilleF/hexx/pull/113
* Rust 1.72.0 by @ManevilleF in https://github.com/ManevilleF/hexx/pull/114

**Full Changelog**: https://github.com/ManevilleF/hexx/compare/0.10.0...0.10.1

## renet 0.0.15 - RenetSteam and Bevy 0.13

 This release has no major changes for renet itself, but comes with a new version for bevy 0.13 and now the [renet_steam](https://github.com/lucaspoffo/renet/tree/master/renet_steam) crate is released (transport layer using steam API).

#### Changelog

##### RenetSteam

* Released version 0.0.1. Checkout the [README](https://github.com/lucaspoffo/renet/tree/master/renet_steam) or the [Bevy Demo](https://github.com/lucaspoffo/renet/tree/master/demo_bevy) to see how to use it. 

##### Renetcode

*  Use timeout given from ConnectToken instead of fixed 15 secs. [(commit)](https://github.com/lucaspoffo/renet/commit/9add0a326098f69a6a7489554c05f85dbec8c530)

##### BevyRenet

* Implement Component for ClientId. [(PR)](https://github.com/lucaspoffo/renet/pull/140) by [SylvKT](https://github.com/SylvKT)
* Add emit_server_events_system, now, transports can update and disconnect clients before events are emitted. [(commit)](https://github.com/lucaspoffo/renet/commit/c8485040e32de483a6c7f398a22cdedf72088c0d)
* Removed steam feature, now to use steam, you add `renet_steam` with the bevy feature. [(commit)](https://github.com/lucaspoffo/renet/commit/9584c6972496f985eef853079552a83e913f0641)
* Update to bevy 0.13. [(PR)](https://github.com/lucaspoffo/renet/pull/147) by [dgsantana](https://github.com/dgsantana)
* Make all systems public, this makes possible to manually add the systems and use them for ordering. [(commit)](https://github.com/lucaspoffo/renet/commit/b9d88f94fc7a84f8febdf12f8788d286f73d6157)

##### RenetVisualizer

* Update egui to 0.26

## renet 0.0.14 - Some goodies

 This release has some small quality changes: 

- Use `ClientId` instead of a u64 for client ids, more type safety.
- Add `is_connecting` and `is_connected` to `RenetClient`, this should make it a bit easier to write code transport agnostic.

In the main branch we have the unreleased transport layer for steam if you want to test. The steam transport is also available in the bevy_renet crate under the feature `steam` (only available in the main branch). 

Full changelog is available below. Thanks for all the contributors, and for all my sponsors :pray:

#### FULL CHANGELOG

##### Added ⭐

* Added `is_connecting`, `is_connected` to `RenetClient`, this should make it easier to write code more transport agnostic. Also added `set_connected`, `set_connecting` so the transport layer can keep the connection status updated for the `RenetClient` (if you are using the default transport layer you will not need call it, but if you have a custom transport layer you will need to call them). [(PR)](https://github.com/lucaspoffo/renet/pull/119) by [OleStrohm](https://github.com/OleStrohm)
* Added methods `can_send_message` and `channel_available_memory` for `RenetClient` and `RenetServer`. [(commit)](https://github.com/lucaspoffo/renet/commit/edec20e4a2a9bc003375de5a7ffad4f4081c198b)
* Renetcode: make fields for ConnectToken public. [(PR)](https://github.com/lucaspoffo/renet/pull/116) by [UkoeHB](https://github.com/UkoeHB)

##### Fixed 🐛

* Fixed `RenetServer.connected_clients` not returning the correct numbers of connected clients. [(commit)](https://github.com/lucaspoffo/renet/commit/a7bced4cfb7fbe60f1447f9eb6423d2a8bc1cc6e)

##### Changed 🛠️

* Use ClientId struct instead of u64 for clients ids, better type safety for users. [(PR)](https://github.com/lucaspoffo/renet/pull/103) by [roboteng](https://github.com/roboteng)
* NetcodeServer: now accepts multiple server addresses when being created. [(PR)](https://github.com/lucaspoffo/renet/pull/102)
* NetcodeServer: change arguments when creating it, now accepts only a configuration struct `ServerConfig`, that has all previous arguments. [(PR)](https://github.com/lucaspoffo/renet/pull/102)
* RenetVisualizer: updated to egui 0.23. [(PR)](https://github.com/lucaspoffo/renet/pull/103) by [Zajozor](https://github.com/Zajozor)
* BevyRenet: updated to bevy 0.12 [(commit)](https://github.com/lucaspoffo/renet/commit/fb71a405deaf2f90c8d5f609c0e40f5a594beacf)
* BevyRenet: `client_disconnected`, `client_connecting`, `client_just_connected`, `client_just_disconnected` has been moved out of the transport module. Now they use the methods from `RenetClient` instead of the transport layer. [(PR)](https://github.com/lucaspoffo/renet/pull/119) by [OleStrohm](https://github.com/OleStrohm)

##### Removed 🔥

* Removed `is_connecting`, `is_connected`, `is_disconnected` from `NetcodeClientTransport`, use the methods from `RenetClient` instead. [(PR)](https://github.com/lucaspoffo/renet/pull/119) by [OleStrohm](https://github.com/OleStrohm)

#### Contributors :pray:

- [OleStrohm](https://github.com/OleStrohm)
- [UkoeHB](https://github.com/UkoeHB)
- [Zajozor](https://github.com/Zajozor)
- [roboteng](https://github.com/roboteng)
- [Shatur](https://github.com/Shatur)



# Requests for Contribution

## rust-sdl2 - 1 Beginner Open Issues

* Support "bundled" feature with "gfx", "image", "mixer" and "ttf" features
## bevy - 100 Beginner Open Issues

* Improve documentation and example around enabling audio formats in Cargo.toml
* Revert "Support calculating normals for indexed meshes"
* Make an example for bevy_ui gizmos layout overlay
* Split apart and reorganize gizmo examples
* Add examples demonstrating how to follow the player with a camera
## macroquad - 1 Beginner Open Issues

* is_key_down(KeyCode::LeftShift) reports as false for any modifier keys on macos
## rust-sfml - 1 Beginner Open Issues

* Better documentation and support for finding native libs on windows
## hotham - 2 Beginner Open Issues

* [Maintenance] Fix broken skinning test
* [Maintenance] Standardise naming of binaries in examples


# Discussions

* [Just finished rendering my first ever sphere WGPU and I’m so proud 🥲](https://i.redd.it/p06vdjozxy1c1.jpg)

* [Making a Top-down game inspired by RimWorld, Valheim, and The Majesty](https://v.redd.it/6dxey4dr9azb1)

* [Realtime Ray Marching implemented with Rust and wgpu](https://v.redd.it/y7g9rrlyoo4c1)

* [Simulating 100,000 pedestrians with a multi-threaded Rust-core simulation library (TerraCrowds) and Unity WebGL in a web browser](https://v.redd.it/j8271fsisu9c1)

* [A kids game with no ads made with Rust + Bevy](https://www.reddit.com/r/rust_gamedev/comments/1ap02w0/a_kids_game_with_no_ads_made_with_rust_bevy/)

* [Running 30 000 animated characters with Unity WebGL + WebAssembly with our Rust-based TerraCrowds engine running at 60Hz and 4K resolution in the browser.](https://v.redd.it/xidmk6jsmc1c1)

* [Implemented experimental online multiplayer (devlog in comments)](https://v.redd.it/reox4l1qdimc1)

* [Two months later, and I've created a devlog for my game](https://i.redd.it/tx8v2whvkzwb1.png)

* [Ray tracing and Ray marching implemented entirely in Rust.](https://i.redd.it/gc8nwxdgd32c1.jpeg)

* [Is the borrow checker poison for game dev?](https://www.reddit.com/r/rust_gamedev/comments/192ny85/is_the_borrow_checker_poison_for_game_dev/)

* [Rust Gamedev Meetup 33](https://gamedev.rs/blog/rust-gamedev-meetup-33/)

* [Veloren 0.16 release](https://veloren.net/blog/release-0-16/)

* [A Sokoban powered by bevy](https://www.reddit.com/r/rust_gamedev/comments/1bov9bh/a_sokoban_powered_by_bevy/)

* [Voltum - Merge game made with Rust + Bevy](https://www.reddit.com/r/rust_gamedev/comments/1bozo5s/voltum_merge_game_made_with_rust_bevy/)

* [Geometry Batching, a Better Approach?](https://www.reddit.com/r/rust_gamedev/comments/1boewf2/geometry_batching_a_better_approach/)

* [Combine And Conquer 0.8.0 is now available [multi-planetary automation game]](https://buckmartin.de/combine-and-conquer/2024-03-23-v0.8.0.html)

* [Just started on a particle system(in Rust), I think it looks cool.](https://v.redd.it/w4ljsboszypc1)

* [GPU Particle Research — Bevy Hanabi, Part 2](https://medium.com/@Sou1gh0st/gpu-particle-research-bevy-hanabi-part-2-1354fc021f38)

* [[Media] Fyrox now supports hot reloading - you can write your game while it is running and almost immediately see the results. This is super useful for rapid prototyping and now Rust is as fast for game development as scripting languages.](https://www.youtube.com/watch?v=vq6P3Npydmw)

* [Bowfishing Blitz: A Game of Refractive Aberration (my first jam game made using Rust and wgpu)](https://itch.io/jam/acerola-jam-0/rate/2579027)

* [Added a slight effect to the edges of player vision. Good or Bad?](https://www.youtube.com/watch?v=csKK7U-tcsg)

* [Attention Game Devs! Unleash Your Creativity in the EPIC 72 Hour Game Jam by .fun Domains! (This Weekend) 🎮🏆🚀](https://www.reddit.com/r/rust_gamedev/comments/1bifhlk/attention_game_devs_unleash_your_creativity_in/)

* [Strategy to implement units in a RTS game in Rust?](https://www.reddit.com/r/rust_gamedev/comments/1bhkwub/strategy_to_implement_units_in_a_rts_game_in_rust/)

* [Publishing a Game](https://www.reddit.com/r/rust_gamedev/comments/1bgazh2/publishing_a_game/)

* [GPU Particle Research — Bevy Hanabi, Part 1](https://twitter.com/Sou1gh0st/status/1769027238610325518)

