# Engine Updates

## bevy v0.13.2

 A full diff of what's been fixed can be seen here:  https://github.com/bevyengine/bevy/compare/v0.13.1...v0.13.2



# Library Updates

## bevy_vello v0.1.2

 #### 0.1.2 (2024-04-08)

##### fixed

- Fixes a window hang issue in bevy on native platforms

**Full Changelog**: https://github.com/loopystudios/bevy_vello/compare/v0.1.1...v0.1.2

## bevy_vello v0.1.1

 #### 0.1.1 (2024-04-04)

##### fixed

- fixed panic on Windows when window is minimized


## bevy_mod_scripting v0.6.0

 #### What's Changed
* Add lua documentation link to readme.md by @makspll in https://github.com/makspll/bevy_mod_scripting/pull/107
* Proxy derive macros, rustc plugin codegen, safety improvements by @makspll in https://github.com/makspll/bevy_mod_scripting/pull/67


**Full Changelog**: https://github.com/makspll/bevy_mod_scripting/compare/v0.5.0...v0.6.0

## ash-molten Release 0.19.0+1.2.8

 ##### Changed
- [PR#83](https://github.com/EmbarkStudios/ash-molten/pull/83) Upgrade `ash` to `0.38`


## cargo-deny 0.14.21

 ##### Fixed
- [PR#643](https://github.com/EmbarkStudios/cargo-deny/pull/643) resolved [#629](https://github.com/EmbarkStudios/cargo-deny/issues/629) by making the hosted git (github, gitlab, bitbucket) org/user name comparison case-insensitive. Thanks [@pmnlla](https://github.com/pmnlla)!
- [PR#649](https://github.com/EmbarkStudios/cargo-deny/pull/649) fixed an issue where depending on the same crate multiple times by using different `cfg()/triple` targets could cause features to be resolved incorrectly and thus crates to be not pulled into the graph used for checking.

## cfg-expr 

 ##### Changed
- [PR#69](https://github.com/EmbarkStudios/cfg-expr/pull/69) updated the builtin target list to 1.77.2.

## krates 

 ##### Fixed
- [PR#81](https://github.com/EmbarkStudios/krates/pull/81) re-resolved [#79](https://github.com/EmbarkStudios/krates/issues/79) because the PR#80 completely broke in the presence of cargo patches.

## krates 

 ##### Fixed
- [PR#80](https://github.com/EmbarkStudios/krates/pull/80) resolved [#79](https://github.com/EmbarkStudios/krates/issues/79) by fixing an extreme edge case with dependency renaming.



# Requests for Contribution

## rust-sdl2 - 1 Beginner Open Issues

* Support "bundled" feature with "gfx", "image", "mixer" and "ttf" features
## bevy - 100 Beginner Open Issues

* `check-missing-*-in-docs` job does not install Rust
* Clarify camera coordinate system further
* Add `ExitCode` to `AppExit`
* Remove unecessary lint
* Create an example showcasing disjoint mutable access to the world via `unsafe_world_cell`
## macroquad - 1 Beginner Open Issues

* is_key_down(KeyCode::LeftShift) reports as false for any modifier keys on macos
## rust-sfml - 1 Beginner Open Issues

* Better documentation and support for finding native libs on windows
## hotham - 2 Beginner Open Issues

* [Maintenance] Fix broken skinning test
* [Maintenance] Standardise naming of binaries in examples
## iced - 2 Beginner Open Issues

* Toast widget
* Superfluous alignment functions
## winit - 2 Beginner Open Issues

* Windows: inner_size() reports 0x0 size when window is minimized
* Minimum and Maximum size become incorrect when the HiDPI factor changes
## wgpu - 6 Beginner Open Issues

* Fix synchronization validation error in `water` example on Vulkan 
* buffer_get_mapped_range different way of passing range than buffer_map_async
* Test Buffer to Buffer and Buffer to/from Image Copies with Problematic Usages
* Use WGSL instead of GLSL in library documentation 
* WGPU_TRACE file from Firefox Nightly is missing final close-bracket
## abstreet - 7 Beginner Open Issues

* 15 minute neighborhoods
* Populate lakes with boats
* Represent people inside buildings better using procedural generation
* Feature request: equity evaluation using census data
* Render crowds of pedestrians better


# Discussions

* [Spell Casting system short devlog (written in Rust)](https://youtu.be/PGMY7xQ4Qpo)

* [This Month in Rust GameDev: Call for Submissions!](https://www.reddit.com/r/rust_gamedev/comments/1c1qjd5/this_month_in_rust_gamedev_call_for_submissions/)

* [We're still not game, but progress continues.](https://www.reddit.com/r/rust_gamedev/comments/1c15y7j/were_still_not_game_but_progress_continues/)

* [banging my head against the wall (someone help me think about data structures)](https://www.reddit.com/r/rust_gamedev/comments/1c12cum/banging_my_head_against_the_wall_someone_help_me/)

* [Working on a casting system with the first spell (in Rust)](https://v.redd.it/m9mmqv24jotc1)

* [Macroquad Accumulative Frame](https://www.reddit.com/r/rust_gamedev/comments/1c12h75/macroquad_accumulative_frame/)

* [How-to: Rendering a concave polygon with borders?](https://www.reddit.com/r/rust_gamedev/comments/1bzzldw/howto_rendering_a_concave_polygon_with_borders/)

* [Bevy vs Macroquad anti-aliasing ](https://www.reddit.com/r/rust_gamedev/comments/1bzcz2k/bevy_vs_macroquad_antialiasing/)

* [glTF Support Merged into Fyrox](https://github.com/FyroxEngine/Fyrox/commit/deeebe575af2b69364e4c36fd41bf91c1c1d946e)

* [A Blog on my experience learning Rust](https://open.substack.com/pub/miteshcodes/p/the-rusty-experience?r=3nfjry&amp;utm_campaign=post&amp;utm_medium=web&amp;showWelcomeOnShare=true)

* [Close Quarters Combat simulator written in Rust](https://v.redd.it/va0d4of07nsc1)

* [Cross-engine tool for lossless compression of sprites with identical areas](https://github.com/elringus/sprite-dicing)

* [Rust gamedev roadmap](https://www.reddit.com/r/rust_gamedev/comments/1bvo1mn/rust_gamedev_roadmap/)

* [[Bevy] Janky 3D tile-based level creator that I made over the last week or so](https://v.redd.it/q4fic5xr9bsc1)

* [Hey is it possible to render planet quads using wgpu?](https://www.reddit.com/r/rust_gamedev/comments/1bugr1y/hey_is_it_possible_to_render_planet_quads_using/)

* [Invitation to author a IMGUI book!](https://www.reddit.com/r/rust_gamedev/comments/1bsywd4/invitation_to_author_a_imgui_book/)

