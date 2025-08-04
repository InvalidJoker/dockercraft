
use reqwest::Client;
use crate::updaters::manifest::{DownloadLink, ServerVersion, ServerVersions};
use crate::updaters::paper::{BuildResponse, VersionsResponse};

pub async fn update_versions(servers: &mut ServerVersions) -> anyhow::Result<()> {
    let client = Client::builder()
        .user_agent("Minecraft Server Updater/1.0")
        .build()?;

    println!("Updating server versions...");

    for server in servers.iter_mut() {
        match server.name.as_str() {
            "Paper" => update_paper(&client, server, "paper").await?,
            "Folia" => update_paper(&client, server, "folia").await?,
            _ => {}
        }
    }

    Ok(())
}

async fn update_paper(client: &Client, server: &mut ServerVersion, project: &str) -> anyhow::Result<()> {
    let url = format!("https://fill.papermc.io/v3/projects/{}/versions", project);
    let versions_resp = client.get(&url).send().await?.error_for_status()?;
    let parsed = versions_resp.json::<VersionsResponse>().await?;

    let mut links = Vec::new();

    for version in parsed.versions {
        let version_str = &version.version.id;


        if (project == "paper" || project == "folia") && (version.version.java.version.minimum < 21 || version.version.java.version.minimum == -1) {
            continue;
        }

        let download_url = get_latest_build(client, project, version_str).await?;
        links.push(DownloadLink {
            version: version_str.clone(),
            link: download_url,
            java_minimum: Some(version.version.java.version.minimum),
            java_recommended_flags: version.version.java.flags.recommended.clone(),
        });
    }

    server.download_links = Some(links);
    Ok(())
}

async fn get_latest_build(client: &Client, project: &str, version: &str) -> anyhow::Result<String> {
    let url = format!(
        "https://fill.papermc.io/v3/projects/{}/versions/{}/builds/latest",
        project, version
    );
    let resp = client.get(&url).send().await?.error_for_status()?;
    let build: BuildResponse = resp.json().await?;

    for (_, download) in build.downloads {
        if !download.url.is_empty() {
            return Ok(download.url);
        }
    }

    anyhow::bail!("No valid download found for {} {}", project, version)
}
