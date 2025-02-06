use miette::{Diagnostic, IntoDiagnostic, Result};
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(default = "default_false")]
    pub podman: bool,
    /// Directory containing the queries
    #[serde(default = "default_queries")]
    pub queries: PathBuf,
    /// Destination folder for generated modules
    #[serde(default = "default_destination")]
    pub destination: PathBuf,
    /// Generate synchronous rust code
    #[serde(default = "default_false")]
    pub sync: bool,
    /// Generate asynchronous rust code
    #[serde(default = "default_true")]
    pub r#async: bool,
    /// Derive serde's `Serialize` trait for generated types.
    #[serde(default = "default_false")]
    pub serialize: bool,

    #[serde(default)]
    pub types: Types,
    #[serde(default)]
    pub package: Package,
    #[serde(default, rename = "static")]
    pub static_files: Vec<StaticFile>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum StaticFile {
    Simple(String),
    Detailed {
        path: String,
        #[serde(default = "default_false")]
        hard_link: bool,
    },
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Types {
    #[serde(default, rename = "crates")]
    pub crate_info: HashMap<String, CrateDependency>,
    #[serde(default)]
    pub mapping: HashMap<String, TypeMapping>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CrateDependency {
    Simple(String),
    Detailed {
        version: Option<String>,
        path: Option<String>,
        features: Option<Vec<String>>,
        default_features: Option<bool>,
        optional: Option<bool>,
    },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum TypeMapping {
    Simple(String),
    Detailed {
        rust_type: String,
        #[serde(default = "default_true")]
        is_copy: bool,
        #[serde(default = "default_true")]
        is_params: bool,
        #[serde(default = "default_kind")]
        kind: String,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Publish {
    Bool(bool),
    Repositories(Vec<String>),
}

impl Default for Publish {
    fn default() -> Self {
        Self::Bool(false)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Package {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default = "default_edition")]
    pub edition: String,
    #[serde(default = "default_publish")]
    pub publish: Publish,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(rename = "license-file", skip_serializing_if = "Option::is_none")]
    pub license_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<toml::Value>,
    #[serde(rename = "default-run", skip_serializing_if = "Option::is_none")]
    pub default_run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autobins: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoexamples: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autotests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autobenches: Option<bool>,
    #[serde(rename = "rust-version", skip_serializing_if = "Option::is_none")]
    pub rust_version: Option<String>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub(crate) fn get_type_mapping(&self, ty: &Type) -> Option<&TypeMapping> {
        let key = format!("{}.{}", ty.schema(), ty.name());
        self.types.mapping.get(&key)
    }
}

impl Package {
    pub fn to_string(&self) -> Result<String> {
        let mut output = String::from("[package]\n");
        output.push_str(&toml::to_string_pretty(self).into_diagnostic()?);
        Ok(output)
    }
}

impl Default for Package {
    fn default() -> Self {
        Self {
            name: default_name(),
            version: default_version(),
            edition: default_edition(),
            publish: default_publish(),
            authors: None,
            description: None,
            documentation: None,
            readme: None,
            homepage: None,
            repository: None,
            license: None,
            license_file: None,
            keywords: None,
            categories: None,
            workspace: None,
            build: None,
            links: None,
            exclude: None,
            include: None,
            metadata: None,
            default_run: None,
            autobins: None,
            autoexamples: None,
            autotests: None,
            autobenches: None,
            rust_version: None,
        }
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    Toml(#[from] toml::de::Error),
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_queries() -> PathBuf {
    PathBuf::from_str("queries/").unwrap()
}

fn default_destination() -> PathBuf {
    PathBuf::from_str("clorinde").unwrap()
}

fn default_kind() -> String {
    "simple".to_string()
}

fn default_name() -> String {
    "clorinde".to_string()
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_edition() -> String {
    "2021".to_string()
}

fn default_publish() -> Publish {
    Publish::default()
}
