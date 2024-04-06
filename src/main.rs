use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::builder().build()?;

    let releases = octocrab
        .repos("rust-lang", "rust")
        .releases()
        .list()
        .send()
        .await?
        .items;

    for release in releases {
        println!(
            "Release: {body}", //
            body = release.body.unwrap_or_default(),
        );
    }

    Ok(())
}
