use anyhow::Result;
use bollard::Docker;
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use bollard::query_parameters::BuildImageOptions;
use tar::Builder;
use http_body_util::Full;

pub async fn build_paper_image(version: &str, download_url: &str) -> Result<()> {
    let docker = Docker::connect_with_socket_defaults()?;

    let build_args = HashMap::from([
        ("DOWNLOAD_URL".to_string(), download_url.to_string()),
        ("IN_DEPLOYMENT".to_string(), "true".to_string()),
    ]);

    let tag = format!("paper:{}", version);
    let docker_dir = "./docker";

    // Ensure the docker directory exists
    if !std::path::Path::new(docker_dir).exists() {
        return Err(anyhow::anyhow!("Docker directory does not exist: {}", docker_dir));
    }

    // Create a tarball of the Docker build context (everything inside docker_dir)
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
            t: Some(tag),
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

    println!("✅ Built image: paper:{}", version);
    Ok(())
}
