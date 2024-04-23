use chrono::{NaiveDate, TimeZone, Utc};
use octocrab::{models::IssueState, Octocrab};
use roux::{
    util::{FeedOption, RouxError, TimePeriod},
    Subreddit,
};
use std::{env, fs::File, io::Write};

mod ai_summarizer;

/// Kind of repository
enum RepositoryKind {
    Engine,
    Library,
    Game,
    Rust,
}

/// Github repository
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

    fn is_game(&self) -> bool {
        matches!(self.kind, RepositoryKind::Game)
    }

    fn is_rust(&self) -> bool {
        matches!(self.kind, RepositoryKind::Rust)
    }
}

fn repositories() -> Vec<Repository> {
    let mut engine_repositories = vec![
        Repository::engine("Rust-SDL2", "rust-sdl2"),
        Repository::engine("bevyengine", "bevy"),
        Repository::engine("PistonDevelopers", "piston"),
        Repository::engine("not-fl3", "macroquad"),
        Repository::engine("ggez", "ggez"),
        Repository::engine("nannou-org", "nannou"),
        Repository::engine("jeremyletang", "rust-sfml"),
        Repository::engine("amethyst", "bracket-lib"),
        Repository::engine("17cupsofcoffee", "tetra"),
        Repository::engine("godot-rust", "gdnative"),
        Repository::engine("deltaphc", "raylib-rs"),
        Repository::engine("PsichiX", "oxygengine"),
        Repository::engine("VincentFoulon80", "console_engine"),
        Repository::engine("AryanpurTech", "BlueEngine"),
        Repository::engine("Nazariglez", "notan"),
        Repository::engine("CleanCut", "rusty_engine"),
        Repository::engine("geng-engine", "geng"),
        Repository::engine("FyroxEngine", "Fyrox"),
        Repository::engine("redpenguinyt", "gemini-rust"),
        Repository::engine("attackgoat", "screen-13"),
        Repository::engine("MalekiRe", "stereokit-rs"),
        Repository::engine("jice-nospam", "doryen-rs"),
        Repository::engine("polymonster", "hotline"),
        Repository::engine("AmbientRun", "Ambient"),
        Repository::engine("PistonDevelopers", "turbine"),
        Repository::engine("markusmoenig", "Eldiron"),
        Repository::engine("JustAPotota", "defold-rs"),
        Repository::engine("leetvr", "hotham"),
        Repository::engine("PikuseruConsole", "pikuseru"),
        Repository::engine("gamercade-io", "gamercade_console"),
        Repository::engine("jjant", "runty8"),
        Repository::engine("Maix0", "pixel_engine"),
    ];

    // TODO add everything from https://github.com/EmbarkStudios/rust-ecosystem?tab=readme-ov-file#open-source
    let mut library_repositories = vec![
        Repository::library("Jondolf", "bevy_xpbd"),
        Repository::library("LechintanTudor", "sparsey"),
        Repository::library("djeedai", "bevy_hanabi"),
        Repository::library("iced-rs", "iced"),
        Repository::library("loopystudios", "bevy_vello"),
        Repository::library("ManevilleF", "hexx"),
        Repository::library("nicopap", "cuicui_layout"),
        Repository::library("lucaspoffo", "renet"),
        Repository::library("makspll", "bevy_mod_scripting"),
        Repository::library("rust-windowing", "winit"),
        Repository::library("HouraiTeahouse", "backroll-rs"),
        Repository::library("gfx-rs", "wgpu"),
        Repository::library("hadronized", "luminance-rs"),
        Repository::library("mun-lang", "mun"),
    ];

    let mut rust_repositories = vec![
        Repository::rust("rust-gamedev", "arewegameyet"), //
    ];

    let mut game_repositories = vec![
        Repository::game("veloren", "veloren"),
        Repository::game("a-b-street", "abstreet"),
        Repository::game("mkhan45", "SIMple-Mechanics"),
    ];

    let mut repositories = Vec::new();
    repositories.append(&mut engine_repositories);
    repositories.append(&mut library_repositories);
    repositories.append(&mut rust_repositories);
    repositories.append(&mut game_repositories);

    repositories
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
    repositories: &Vec<Repository>,
) -> octocrab::Result<String> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    #[cfg(feature = "llama")]
    let model_path =
        std::env::var("LLAMA_MODEL_PATH").expect("LLAMA_MODEL_PATH env variable is required");
    #[cfg(feature = "llama")]
    let ai = ai_summarizer::AiSummarizer::new(model_path.as_str(), None);

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
                #[cfg(feature = "llama")]
                let text = {
                    ai.summarize(
                        if text.len() > 500 {
                            &text[..500]
                        } else {
                            &text
                        },
                        Some(llama_cpp_rs::llama_cpp::SessionParams {
                            n_batch: 32000,
                            ..Default::default()
                        }),
                    );
                    let text = text.split("<|im_end|>").collect::<Vec<_>>()[0];
                    println!("Summary: {}", text);

                    text
                };

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
    repositories: &Vec<Repository>,
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

async fn rfc_updates(repositories: &Vec<Repository>) -> octocrab::Result<String> {
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
