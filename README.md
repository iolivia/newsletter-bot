# Rust GameDev Newsletter Bot 🦀🔥

This repo is an experiment aimed to auto-generate parts of the Rust gamedev newsletter to reduce the maintainer burden. 

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
