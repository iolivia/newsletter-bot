use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::builder().build()?;

    let commit = octocrab
        .commits("XAMPPRocky", "octocrab")
        .get("15c0e31")
        .await?;

    for file in commit.files.unwrap() {
        println!(
            "File: {file}, Additions: {additions}, Deletions: {deletions}",
            file = file.filename,
            additions = file.additions,
            deletions = file.deletions,
        );
    }

    Ok(())
}
