mod docker;
mod updaters;

use crate::updaters::updater::update_versions;
use updaters::manifest::{fetch_server_versions, write_server_versions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server_versions = fetch_server_versions().await?;
    let _original_versions = server_versions.clone();

    if let Err(e) = update_versions(&mut server_versions).await {
        println!("ERROR during update_versions: {:#}", e);
        return Err(e); // optional: return or handle gracefully
    }

    println!("Updated server versions successfully.");

    write_server_versions("updated_server_versions.json", &server_versions)
        .expect("Failed to write updated server_versions.json");

    for version in server_versions.iter().filter(|s| s.name == "Paper") {
        if let Some(links) = &version.download_links {
            for link in links {
                docker::docker::build_paper_image(&link.version, &link.link).await?;
            }
        }
    }

    Ok(())
}
