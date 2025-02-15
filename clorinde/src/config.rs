use miette::{Diagnostic, IntoDiagnostic, Result};
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Use `podman` instead of `docker`
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
    /// Derive serde's `Serialize` trait for generated types
    #[serde(default = "default_false")]
    pub serialize: bool,

    /// Custom type settings
    #[serde(default)]
    pub types: Types,
    /// The `package` table of the generated `Cargo.toml`
    #[serde(default)]
    pub package: Package,
    /// List of static files to copy into the generated directory
    #[serde(default, rename = "static")]
    pub static_files: Vec<StaticFile>,
    /// Use workspace dependencies
    #[serde(default, rename = "use-workspace-deps")]
    pub use_workspace_deps: UseWorkspaceDeps,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum StaticFile {
    Simple(PathBuf),
    Detailed {
        path: PathBuf,
        #[serde(default = "default_false", rename = "hard-link")]
        hard_link: bool,
    },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum UseWorkspaceDeps {
    Bool(bool),
    Path(PathBuf),
}

impl Default for UseWorkspaceDeps {
    fn default() -> Self {
        UseWorkspaceDeps::Bool(false)
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Types {
    /// Crates to add as a dependency for custom types
    #[serde(default, rename = "crates")]
    pub crate_info: HashMap<String, CrateDependency>,
    /// Mapping for custom types
    #[serde(default)]
    pub mapping: HashMap<String, TypeMapping>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CrateDependency {
    /// Simple version string
    Simple(String),
    /// Detailed table information
    Detailed(DependencyTable),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DependencyTable {
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<bool>,
    pub path: Option<String>,
    pub optional: Option<bool>,
    pub features: Option<Vec<String>>,
    #[serde(rename = "default-features")]
    pub default_features: Option<bool>,
}

impl DependencyTable {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: Some(version.into()),
            ..Default::default()
        }
    }

    pub fn features(mut self, features: Vec<impl Into<String>>) -> Self {
        self.features = Some(features.into_iter().map(Into::into).collect());
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = Some(true);
        self
    }

    pub fn no_default_features(mut self) -> Self {
        self.default_features = Some(false);
        self
    }

    pub fn is_simple_version(&self) -> bool {
        matches!(
            self,
            DependencyTable {
                version: Some(_),
                path: None,
                workspace: None,
                optional: None,
                features: None,
                default_features: None,
            }
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum TypeMapping {
    Simple(String),
    Detailed {
        #[serde(rename = "rust-type")]
        rust_type: String,
        #[serde(rename = "is-copy", default = "default_true")]
        is_copy: bool,
        #[serde(rename = "is-params", default = "default_true")]
        is_params: bool,
    },
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

impl Default for Config {
    fn default() -> Self {
        Self {
            podman: false,
            queries: default_queries(),
            destination: default_destination(),
            sync: false,
            r#async: true,
            serialize: false,
            types: Types {
                crate_info: HashMap::new(),
                mapping: HashMap::new(),
            },
            package: Package::default(),
            static_files: vec![],
            use_workspace_deps: UseWorkspaceDeps::Bool(false),
        }
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
