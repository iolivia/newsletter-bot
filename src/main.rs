use chrono::{NaiveDate, TimeZone, Utc};
use octocrab::{models::IssueState, Octocrab};
use roux::{
    util::{FeedOption, RouxError, TimePeriod},
    Subreddit,
};
use std::{env, fs::File, io::Write};

/// Kind of repository
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum RepositoryKind {
    Engine,
    Library,
    Game,
    Rust,
}

/// Github repository
#[derive(Debug, Clone, Eq, PartialEq)]
struct Repository {
    owner: String,
    name: String,
    kind: RepositoryKind,
}

impl Repository {
    fn new(owner: &str, name: &str, kind: RepositoryKind) -> Self {
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
            kind,
        }
    }

    fn engine(owner: &str, name: &str) -> Self {
        Self::new(owner, name, RepositoryKind::Engine)
    }

    fn library(owner: &str, name: &str) -> Self {
        Self::new(owner, name, RepositoryKind::Library)
    }

    fn game(owner: &str, name: &str) -> Self {
        Self::new(owner, name, RepositoryKind::Game)
    }

    fn rust(owner: &str, name: &str) -> Self {
        Self::new(owner, name, RepositoryKind::Rust)
    }

    fn is_engine(&self) -> bool {
        matches!(self.kind, RepositoryKind::Engine)
    }

    fn is_library(&self) -> bool {
        matches!(self.kind, RepositoryKind::Library)
    }

    #[allow(dead_code)]
    fn is_game(&self) -> bool {
        matches!(self.kind, RepositoryKind::Game)
    }

    #[allow(dead_code)]
    fn is_rust(&self) -> bool {
        matches!(self.kind, RepositoryKind::Rust)
    }
}

fn repositories() -> Vec<Repository> {
    let engine_repositories: Vec<_> = [
        ("Rust-SDL2", "rust-sdl2"),
        ("bevyengine", "bevy"),
        ("PistonDevelopers", "piston"),
        ("not-fl3", "macroquad"),
        ("ggez", "ggez"),
        ("nannou-org", "nannou"),
        ("jeremyletang", "rust-sfml"),
        ("amethyst", "bracket-lib"),
        ("17cupsofcoffee", "tetra"),
        ("godot-rust", "gdnative"),
        ("deltaphc", "raylib-rs"),
        ("PsichiX", "oxygengine"),
        ("VincentFoulon80", "console_engine"),
        ("AryanpurTech", "BlueEngine"),
        ("Nazariglez", "notan"),
        ("CleanCut", "rusty_engine"),
        ("geng-engine", "geng"),
        ("FyroxEngine", "Fyrox"),
        ("redpenguinyt", "gemini-rust"),
        ("attackgoat", "screen-13"),
        ("MalekiRe", "stereokit-rs"),
        ("jice-nospam", "doryen-rs"),
        ("polymonster", "hotline"),
        ("AmbientRun", "Ambient"),
        ("PistonDevelopers", "turbine"),
        ("markusmoenig", "Eldiron"),
        ("JustAPotota", "defold-rs"),
        ("leetvr", "hotham"),
        ("PikuseruConsole", "pikuseru"),
        ("gamercade-io", "gamercade_console"),
        ("jjant", "runty8"),
        ("Maix0", "pixel_engine"),
    ]
    .map(|(owner, name)| Repository::engine(owner, name))
    .into_iter()
    .collect();

    let library_repositories: Vec<_> = [
        ("Jondolf", "bevy_xpbd"),
        ("LechintanTudor", "sparsey"),
        ("djeedai", "bevy_hanabi"),
        ("iced-rs", "iced"),
        ("loopystudios", "bevy_vello"),
        ("ManevilleF", "hexx"),
        ("nicopap", "cuicui_layout"),
        ("lucaspoffo", "renet"),
        ("makspll", "bevy_mod_scripting"),
        ("rust-windowing", "winit"),
        ("HouraiTeahouse", "backroll-rs"),
        ("gfx-rs", "wgpu"),
        ("hadronized", "luminance-rs"),
        ("mun-lang", "mun"),
        ("EmbarkStudios", "ash-molten"),
        ("EmbarkStudios", "buildkite-jobify"),
        ("EmbarkStudios", "cargo-about"),
        ("EmbarkStudios", "cargo-deny"),
        ("EmbarkStudios", "cargo-fetcher"),
        ("EmbarkStudios", "cervo"),
        ("EmbarkStudios", "cfg-expr"),
        ("EmbarkStudios", "cloud-dns"),
        ("EmbarkStudios", "crash-handling"),
        ("EmbarkStudios", "discord-sdk"),
        ("EmbarkStudios", "fsr-rs"),
        ("EmbarkStudios", "gsutil"),
        ("EmbarkStudios", "kajiya"),
        ("EmbarkStudios", "krates"),
        ("EmbarkStudios", "mirror-mirror"),
        ("EmbarkStudios", "octobors"),
        ("EmbarkStudios", "physx"),
        ("EmbarkStudios", "poll-promise"),
        ("EmbarkStudios", "presser"),
        ("EmbarkStudios", "puffin"),
        ("EmbarkStudios", "relnotes"),
        ("EmbarkStudios", "rpmalloc-rs"),
        ("EmbarkStudios", "rust-gpu"),
        ("EmbarkStudios", "rymder"),
        ("EmbarkStudios", "spdx"),
        ("EmbarkStudios", "spirv-tools-rs"),
        ("EmbarkStudios", "superluminal-perf"),
        ("EmbarkStudios", "tame-gcs"),
        ("EmbarkStudios", "tame-oauth"),
        ("EmbarkStudios", "tame-oidc"),
        ("EmbarkStudios", "tame-webpurify"),
        ("EmbarkStudios", "texture-synthesis"),
        ("EmbarkStudios", "tiny-bench"),
        ("EmbarkStudios", "tracing-ext-ffi-subscriber"),
        ("EmbarkStudios", "tracing-logfmt"),
        ("EmbarkStudios", "tryhard"),
    ]
    .map(|(owner, name)| Repository::library(owner, name))
    .into_iter()
    .collect();

    let rust_repositories: Vec<_> = [("rust-gamedev", "arewegameyet")]
        .map(|(owner, name)| Repository::rust(owner, name))
        .into_iter()
        .collect();

    let game_repositories: Vec<_> = [
        ("veloren", "veloren"),
        ("a-b-street", "abstreet"),
        ("mkhan45", "SIMple-Mechanics"),
    ]
    .map(|(owner, name)| Repository::game(owner, name))
    .into_iter()
    .collect();

    [
        engine_repositories,
        library_repositories,
        rust_repositories,
        game_repositories,
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[tokio::main]
async fn main() {
    // Args
    let args: Vec<String> = env::args().collect();
    let start = NaiveDate::parse_from_str(&args[1], "%Y-%m-%d").expect("Failed to parse date");
    let end = NaiveDate::parse_from_str(&args[2], "%Y-%m-%d").expect("Failed to parse date");
    println!("Args {} - {}", start, end);

    let repositories = repositories();

    let engine_updates = engine_updates(start, end, &repositories)
        .await
        .expect("engine_updates");

    let library_updates = library_updates(start, end, &repositories)
        .await
        .expect("library_updates");

    let rfc_updates = rfc_updates(&repositories).await.expect("rfc_updates");
    let discussions = discussions(start, end).await.expect("start");

    let updates = format!(
        "{}\n\n{}\n\n{}\n\n{}",
        engine_updates, library_updates, rfc_updates, discussions
    );

    let mut file = File::create("updates.md").expect("Failed to create file");
    file.write_all(updates.as_bytes())
        .expect("Failed to write to file");
}

async fn engine_updates(
    start: NaiveDate,
    end: NaiveDate,
    repositories: &[Repository],
) -> octocrab::Result<String> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let mut markdown = String::from("# Engine Updates\n\n");

    for repository in repositories.iter().filter(|repo| repo.is_engine()) {
        println!("{}/{}", repository.owner, repository.name);

        let releases = octocrab
            .repos(&repository.owner, &repository.name)
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
                    repo = repository.name,
                    name = release.name.unwrap_or_default(),
                    text = text,
                ));
            }
        }
    }

    Ok(markdown)
}

async fn library_updates(
    start: NaiveDate,
    end: NaiveDate,
    repositories: &[Repository],
) -> octocrab::Result<String> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let mut markdown = String::from("# Library Updates\n\n");

    for repository in repositories.iter().filter(|repo| repo.is_library()) {
        println!("{}/{}", repository.owner, repository.name);

        let releases = octocrab
            .repos(&repository.owner, &repository.name)
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
                    repo = repository.name,
                    name = release.name.unwrap_or_default(),
                    text = text,
                ));
            }
        }
    }

    Ok(markdown)
}

async fn rfc_updates(repositories: &[Repository]) -> octocrab::Result<String> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let mut markdown = String::from("# Requests for Contribution\n\n");

    for repository in repositories {
        let labels = octocrab
            .issues(&repository.owner, &repository.name)
            .list_labels_for_repo()
            .per_page(100)
            .send()
            .await?
            .items;

        let keywords = ["first", "good", "easy", "beginner", "help", "wanted"];
        let negative_keywords = [
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
            .issues(&repository.owner, &repository.name)
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
            repository.owner,
            repository.name,
            open_issues.len(),
            if open_issues.is_empty() { "❌" } else { "✅" }
        );

        if !open_issues.is_empty() {
            markdown.push_str(&format!(
                "## {} - {} Beginner Open Issues\n\n",
                repository.name,
                open_issues.len()
            ));

            for open_issue in open_issues.iter().take(5) {
                markdown.push_str(&format!("* {name}\n", name = open_issue.title));
            }
        }
    }

    Ok(markdown)
}

async fn discussions(start: NaiveDate, end: NaiveDate) -> Result<String, RouxError> {
    let subreddit = Subreddit::new("rust_gamedev");
    let options = FeedOption::new().period(TimePeriod::ThisYear);

    let mut top_posts = subreddit
        .top(100, Some(options.clone()))
        .await?
        .data
        .children;

    let mut hot_posts = subreddit
        .hot(100, Some(options)) //
        .await?
        .data
        .children;

    let mut posts = Vec::new();
    posts.append(&mut top_posts);
    posts.append(&mut hot_posts);

    let mut markdown = String::from("# Discussions\n\n");

    for post in posts {
        let post_date = Utc
            .timestamp_opt(post.data.created_utc as i64, 0)
            .unwrap()
            .date_naive();

        if post_date >= start && post_date <= end {
            println!("Found top post: {}", post.data.title);

            markdown.push_str(&format!(
                "* [{title}]({url})\n\n",
                title = post.data.title,
                url = post.data.url.unwrap_or_default()
            ));
        }
    }

    Ok(markdown)
}
