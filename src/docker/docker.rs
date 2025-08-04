use anyhow::Result;
use bollard::Docker;
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use bollard::auth::DockerCredentials;
use bollard::query_parameters::{BuildImageOptions, PushImageOptions};
use tar::Builder;
use http_body_util::Full;
use log::warn;

pub async fn build_paper_image(version: &str, download_url: &str) -> Result<()> {
    let docker = Docker::connect_with_socket_defaults()?;
    let repo = "ghcr.io/invalidjoker/dockercraft";

    let build_args = HashMap::from([
        ("DOWNLOAD_URL".to_string(), download_url.to_string()),
        ("IN_DEPLOYMENT".to_string(), "true".to_string()),
    ]);

    let tag_base = format!("paper-{}", version);
    let full_tag = format!("{}:{}", repo, tag_base);
    let docker_dir = "./docker";

    if !std::path::Path::new(docker_dir).exists() {
        return Err(anyhow::anyhow!("Docker directory does not exist: {}", docker_dir));
    }

    let tar_data = {
        let mut archive = Builder::new(Vec::new());
        archive.append_dir_all(".", docker_dir)?;
        archive.into_inner()?
    };


    let tar_stream = http_body_util::Either::Left(Full::new(bytes::Bytes::from(
        tar_data,
    )));
    let mut image = docker.build_image(
        BuildImageOptions {
            buildargs: Some(build_args),
            t: Some(full_tag.clone()),
            rm: true,
            ..BuildImageOptions::default()
        },
        None,
        Some(tar_stream),
    );



    while let Some(msg) = image.next().await {
        let msg = msg?;
        if let Some(err) = msg.error {
            eprintln!("Error: {}", err);
        }
        if let Some(stream) = msg.stream {
            print!("{}", stream);
        }
    }

    let username = std::env::var("GITHUB_ACTOR").unwrap_or_else(|_| "unknown".to_string());
    let password = std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| "unknown".to_string());

    if username == "unknown" || password == "unknown" {
        warn!("GITHUB_ACTOR or GITHUB_TOKEN environment variables are not set. Skipping push to registry.");
        return Err(anyhow::anyhow!(
            "GITHUB_ACTOR or GITHUB_TOKEN environment variables are not set. Skipping push to registry."
        ));
    }

    let options = PushImageOptions {
        tag: Some(tag_base),
        platform: None,
    };

    let credentials = DockerCredentials {
        username: Some(username),
        password: Some(password),
        serveraddress: Some("https://ghcr.io".to_string()),
        ..Default::default()
    };

    let mut push = docker.push_image(
        repo,
        Some(options),
        Some(credentials),
    );

    while let Some(msg) = push.next().await {
        match msg {
            Ok(msg) => {
                if let Some(err) = msg.error {
                    eprintln!("Error: {}", err);
                }
            }
            Err(e) => {
                eprintln!("Push error: {}", e);
            }
        }
    }
    println!("Successfully built and pushed image: {}", full_tag);
    Ok(())
}
