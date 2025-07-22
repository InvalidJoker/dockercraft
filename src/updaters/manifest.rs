use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DownloadLink {
    pub version: String,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_minimum: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub java_recommended_flags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerVersion {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub is_paperclip: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_snapshot: Option<String>,
    pub download_links: Option<Vec<DownloadLink>>,
}

pub type ServerVersions = Vec<ServerVersion>;

pub async fn fetch_server_versions() -> anyhow::Result<ServerVersions> {
    let file = File::open("server_versions.json")?;
    let server_versions: ServerVersions = serde_json::from_reader(file)?;
    Ok(server_versions)
}

pub fn write_server_versions<P: AsRef<Path>>(
    path: P,
    data: &ServerVersions,
) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    let json = serde_json::to_string_pretty(data)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

impl PartialEq for DownloadLink {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.link == other.link
    }
}