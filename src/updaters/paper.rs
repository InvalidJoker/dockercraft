use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct VersionsResponse {
    pub versions: Vec<VersionResponse>,
}

#[derive(Debug, Deserialize)]
pub struct VersionResponse {
    pub version: PaperVersion,
}

#[derive(Debug, Deserialize)]
pub struct PaperVersion {
    pub id: String,
    pub java: Java,
}

#[derive(Debug, Deserialize)]
pub struct Java {
    pub flags: JavaFlags,
    pub version: JavaVersion,
}

#[derive(Debug, Deserialize)]
pub struct JavaFlags {
    pub recommended: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct JavaVersion {
    pub minimum: i32,
}

#[derive(Debug, Deserialize)]
pub struct BuildResponse {
    pub downloads: HashMap<String, Download>,
}

#[derive(Debug, Deserialize)]
pub struct Download {
    pub url: String,
}
