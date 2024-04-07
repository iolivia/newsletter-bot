use chrono::NaiveDate;
use octocrab::{models::IssueState, Octocrab};
use std::{env, fs::File, io::Write};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    // Args
    let args: Vec<String> = env::args().collect();
    let start = NaiveDate::parse_from_str(&args[1], "%Y-%m-%d").expect("Failed to parse date");
    let end = NaiveDate::parse_from_str(&args[2], "%Y-%m-%d").expect("Failed to parse date");
    println!("Args {} - {}", start, end);

    let engine_updates = engine_updates(start, end).await?;
    let rfc_updates = rfc_updates().await?;

    let updates = format!("{}\n\n{}", engine_updates, rfc_updates);

    let mut file = File::create("updates.md").expect("Failed to create file");
    file.write_all(updates.as_bytes())
        .expect("Failed to write to file");

    Ok(())
}

async fn engine_updates(start: NaiveDate, end: NaiveDate) -> octocrab::Result<String> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let engines = vec![
        "Rust-SDL2/rust-sdl2",
        "bevyengine/bevy",
        "PistonDevelopers/piston",
        "not-fl3/macroquad",
        "ggez/ggez",
        "nannou-org/nannou",
        "jeremyletang/rust-sfml",
        "amethyst/bracket-lib",
        "17cupsofcoffee/tetra",
        "godot-rust/gdnative",
        "deltaphc/raylib-rs",
        "PsichiX/oxygengine",
        "VincentFoulon80/console_engine",
        "AryanpurTech/BlueEngine",
        "Nazariglez/notan",
        "CleanCut/rusty_engine",
        "geng-engine/geng",
        "FyroxEngine/Fyrox",
        "redpenguinyt/gemini-rust",
        "attackgoat/screen-13",
        "MalekiRe/stereokit-rs",
        "jice-nospam/doryen-rs",
        "polymonster/hotline",
        "AmbientRun/Ambient",
        "PistonDevelopers/turbine",
        "markusmoenig/Eldiron",
        "JustAPotota/defold-rs",
        "leetvr/hotham",
        "PikuseruConsole/pikuseru",
        "gamercade-io/gamercade_console",
        "jjant/runty8",
        "Maix0/pixel_engine",
    ];

    let mut markdown = String::from("# Engine Updates\n\n");

    for engine in engines {
        let (owner, repo) = engine.split_once('/').unwrap();

        println!("{}/{}", owner, repo);

        let releases = octocrab
            .repos(owner, repo)
            .releases()
            .list()
            .send()
            .await?
            .items;

        for release in releases {
            let is_new = {
                let is_after_start = release.published_at.unwrap().naive_utc().date() >= start;
                let is_before_end = release.published_at.unwrap().naive_utc().date() <= end;

                is_after_start && is_before_end
            };

            println!(
                "Found release: {status} {tag_name} {published_at}",
                published_at = release.published_at.unwrap(),
                tag_name = release.tag_name,
                status = if is_new { "✅" } else { "❌" }
            );

            let text = release.body.unwrap_or_default().replace("# ", "### ");

            if is_new && !text.is_empty() {
                markdown.push_str(&format!(
                    "## {repo} {name}\n\n {text}\n\n",
                    repo = repo,
                    name = release.name.unwrap_or_default(),
                    text = text,
                ));
            }
        }
    }

    Ok(markdown)
}

async fn rfc_updates() -> octocrab::Result<String> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let projects = [
        "Rust-SDL2/rust-sdl2",
        "bevyengine/bevy",
        "PistonDevelopers/piston",
        "not-fl3/macroquad",
        "ggez/ggez",
        "nannou-org/nannou",
        "jeremyletang/rust-sfml",
        "amethyst/bracket-lib",
        "17cupsofcoffee/tetra",
        "godot-rust/gdnative",
        "deltaphc/raylib-rs",
        "PsichiX/oxygengine",
        "VincentFoulon80/console_engine",
        "AryanpurTech/BlueEngine",
        "Nazariglez/notan",
        "CleanCut/rusty_engine",
        "geng-engine/geng",
        "FyroxEngine/Fyrox",
        "redpenguinyt/gemini-rust",
        "attackgoat/screen-13",
        "MalekiRe/stereokit-rs",
        "jice-nospam/doryen-rs",
        "polymonster/hotline",
        "AmbientRun/Ambient",
        "PistonDevelopers/turbine",
        "markusmoenig/Eldiron",
        "JustAPotota/defold-rs",
        "leetvr/hotham",
        "PikuseruConsole/pikuseru",
        "gamercade-io/gamercade_console",
        "jjant/runty8",
        "Maix0/pixel_engine",
        // "https://github.com/makspll/bevy_mod_scripting/labels/help%20wanted",
        // "https://github.com/bevyengine/bevy/labels/D-Good-First-Issue",
        // "https://github.com/ggez/ggez/labels/%2AGOOD%20FIRST%20ISSUE%2A",
        // "https://github.com/FyroxEngine/Fyrox/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22",
        // "https://github.com/rust-gamedev/arewegameyet#contribute",
        // "https://graphite.rs/volunteer/guide/",
        // "https://github.com/rust-windowing/winit/issues?q=is%3Aopen+is%3Aissue+label%3A%22difficulty%3A+easy%22",
        // "https://github.com/HouraiTeahouse/backroll-rs/issues",
        // "https://github.com/search?q=user%3AEmbarkStudios+state%3Aopen&type=issues",
        // "https://github.com/gfx-rs/wgpu/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22",
        // "https://github.com/hadronized/luminance-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22low+hanging+fruit%22",
        // "https://gitlab.com/veloren/veloren/-/issues?label_name=beginner",
        // "https://github.com/a-b-street/abstreet/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22",
        // "https://github.com/mun-lang/mun/labels/good%20first%20issue",
        // "https://github.com/mkhan45/SIMple-Mechanics/labels/good%20first%20issue",
        // "https://github.com/bevyengine/bevy/labels/D-Good-First-Issue",
        // "https://github.com/AmbientRun/Ambient/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22",
    ];

    let mut markdown = String::from("# Requests for Contribution\n\n");

    for project in projects {
        let (owner, repo) = project.split_once('/').unwrap();

        let labels = octocrab
            .issues(owner, repo)
            .list_labels_for_repo()
            .per_page(100)
            .send()
            .await?
            .items;

        let keywords = vec!["first", "good", "easy", "beginner", "help", "wanted"];
        let negative_keywords = vec![
            "challenging",
            "complex",
            "hard",
            "difficult",
            "performance",
            "crash",
        ];

        let relevant_labels = labels
            .iter()
            .filter(|label| {
                let has_keyword_in_name = keywords
                    .iter()
                    .any(|keyword| label.name.to_lowercase().contains(keyword));

                let has_keyword_in_description = keywords.iter().any(|keyword| {
                    label
                        .description
                        .clone()
                        .map(|x| x.contains(keyword))
                        .unwrap_or_default()
                });

                let has_negative_keyword_in_name = negative_keywords
                    .iter()
                    .any(|keyword| label.name.to_lowercase().contains(keyword));

                let has_negative_keyword_in_description = negative_keywords.iter().any(|keyword| {
                    label
                        .description
                        .clone()
                        .map(|x| x.contains(keyword))
                        .unwrap_or_default()
                });

                (has_keyword_in_name || has_keyword_in_description)
                    && !has_negative_keyword_in_name
                    && !has_negative_keyword_in_description
            })
            .map(|label| label.name.clone())
            .collect::<Vec<_>>();

        let issues = octocrab
            .issues(owner, repo)
            .list()
            .labels(&relevant_labels)
            .per_page(100)
            .send()
            .await?
            .items;

        let open_issues = issues
            .iter()
            .filter(|issue| issue.state == IssueState::Open)
            .collect::<Vec<_>>();

        println!(
            "{}/{} - {:?} Beginner Open Issues - {}",
            owner,
            repo,
            open_issues.len(),
            if open_issues.is_empty() { "❌" } else { "✅" }
        );

        if !open_issues.is_empty() {
            markdown.push_str(&format!(
                "## {} - {} Beginner Open Issues\n\n",
                repo,
                open_issues.len()
            ));

            for open_issue in open_issues.iter().take(5) {
                markdown.push_str(&format!("* {name}\n", name = open_issue.title));
            }
        }
    }

    Ok(markdown)
}
