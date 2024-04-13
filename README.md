# Rust GameDev Newsletter Bot 🦀🔥

This repo is an experiment aimed to auto-generate parts of the Rust gamedev newsletter to reduce the maintainer burden. Some inspiration was taken from the amazing maintainers of [TWIR](https://this-week-in-rust.org/) and their [bot](https://github.com/extrawurst/twir-bot).

❗ Note: it's still very much a WIP experiment, but the initial results are promising, contributions are very much encouraged, or evolutions of this project that lead to a more sustainable newsletter creation.

## 🔥 Overview
Features:
- [X] Filter the updates by a given time range
- [X] Fetch github releases for engine and library updates)
- [X] Fetch github issues for generating contributions
- [X] Fetch reddit threads for open discussions
- [X] Generate basic markdown
- [ ] Dynamic list of repos to scan for releases?
- [ ] Use [tfidf-summarizer](https://github.com/shubham0204/tfidf-summarizer.rs) to summarise release notes
- [ ] For releases with blog posts, crawl the blog post and summarize it
- [ ] Download and insert a few images for each update

## 🏃 How to run 
1. Generate a Github personal access token ([docs](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens))
2. Run the bot locally with a date range
 
```bash
GITHUB_TOKEN=github_pat_<token> cargo run -- 2023-08-31 2024-03-30
```

## 💬 How to contribute
Create a PR with changes, ideally an issue first to align on the work being done and avoid duplication of efforts.   

## 🦀 Demo output
```
GITHUB_TOKEN=github_pat_<token> cargo run -- 2024-04-01 2024-04-13
Args 2024-04-01 - 2024-04-13
Rust-SDL2/rust-sdl2
bevyengine/bevy
Found release: ✅ v0.13.2 2024-04-04 21:01:55 UTC
Found release: ❌ v0.13.1 2024-03-18 22:38:27 UTC
Found release: ❌ v0.13.0 2024-02-17 19:32:58 UTC
Found release: ❌ v0.12.1 2023-11-30 01:23:10 UTC
Found release: ❌ v0.12.0 2023-11-04 17:46:40 UTC
Found release: ❌ v0.11.3 2023-09-27 22:43:41 UTC
Found release: ❌ v0.11.2 2023-08-18 19:03:44 UTC
Found release: ❌ v0.11.1 2023-08-14 23:35:18 UTC
Found release: ❌ v0.11.0 2023-07-09 17:12:21 UTC
Found release: ❌ v0.10.1 2023-03-31 21:21:32 UTC
Found release: ❌ v0.10.0 2023-03-06 17:39:58 UTC
Found release: ❌ v0.9.1 2022-12-01 01:59:38 UTC
Found release: ❌ v0.9.0 2022-11-12 20:40:31 UTC
Found release: ❌ v0.8.1 2022-08-19 00:40:44 UTC
Found release: ❌ v0.8.0 2022-07-30 15:26:06 UTC
Found release: ❌ v0.7.0 2022-04-15 20:16:59 UTC
Found release: ❌ v0.6.1 2022-02-14 21:08:22 UTC
Found release: ❌ v0.6.0 2022-01-08 17:29:05 UTC
Found release: ❌ v0.5.0 2021-04-06 19:15:45 UTC
Found release: ❌ v0.4.0 2020-12-19 19:28:45 UTC
Found release: ❌ v0.3.0 2020-11-03 21:35:02 UTC
Found release: ❌ v0.2.1 2020-09-20 22:59:08 UTC
Found release: ❌ v0.2.0 2020-09-19 22:29:56 UTC

loopystudios/bevy_vello
Found release: ✅ v0.1.2 2024-04-09 03:14:04 UTC
Found release: ✅ v0.1.1 2024-04-05 01:39:31 UTC
Found release: ❌ v0.1.0 2024-03-26 14:24:58 UTC

lucaspoffo/renet
Found release: ❌ 0.0.15 2024-02-22 01:32:52 UTC
Found release: ❌ 0.0.14 2023-11-12 21:28:31 UTC
Found release: ❌ 0.0.13 2023-07-20 03:39:53 UTC
Found release: ❌ 0.0.12 2023-05-19 22:40:44 UTC
Found release: ❌ 0.0.11 2023-03-12 05:29:32 UTC
Found release: ❌ 0.0.10 2022-11-18 23:51:46 UTC
Found release: ❌ 0.0.9 2022-07-25 05:29:52 UTC

Rust-SDL2/rust-sdl2 - 1 Beginner Open Issues - ✅
bevyengine/bevy - 99 Beginner Open Issues - ✅
PistonDevelopers/piston - 0 Beginner Open Issues - ❌
not-fl3/macroquad - 1 Beginner Open Issues - ✅
ggez/ggez - 0 Beginner Open Issues - ❌
nannou-org/nannou - 0 Beginner Open Issues - ❌
jeremyletang/rust-sfml - 1 Beginner Open Issues - ✅
amethyst/bracket-lib - 0 Beginner Open Issues - ❌
17cupsofcoffee/tetra - 0 Beginner Open Issues - ❌
markusmoenig/Eldiron - 0 Beginner Open Issues - ❌
JustAPotota/defold-rs - 0 Beginner Open Issues - ❌
leetvr/hotham - 2 Beginner Open Issues - ✅


Found top post: Spell Casting system short devlog (written in Rust)
Found top post: This Month in Rust GameDev: Call for Submissions!
Found top post: We're still not game, but progress continues.
Found top post: banging my head against the wall (someone help me think about data structures)
Found top post: Working on a casting system with the first spell (in Rust)
Found top post: Macroquad Accumulative Frame
Found top post: How-to: Rendering a concave polygon with borders?
Found top post: Bevy vs Macroquad anti-aliasing
Found top post: glTF Support Merged into Fyrox
Found top post: A Blog on my experience learning Rust
Found top post: Close Quarters Combat simulator written in Rust
Found top post: Cross-engine tool for lossless compression of sprites with identical areas
Found top post: Rust gamedev roadmap
Found top post: [Bevy] Janky 3D tile-based level creator that I made over the last week or so
Found top post: Hey is it possible to render planet quads using wgpu?
Found top post: Invitation to author a IMGUI book!

Saved ./updates.md
```
Example [updates.md](https://github.com/iolivia/newsletter-bot/blob/main/updates.md).
