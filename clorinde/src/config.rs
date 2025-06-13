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
#[serde(default, deny_unknown_fields)]
pub struct Config {
    // Shared with CLI
    /// Use `podman` instead of `docker`
    pub podman: bool,
    /// Directory containing the queries
    pub queries: PathBuf,
    /// Destination folder for generated modules
    pub destination: PathBuf,
    /// Generate synchronous rust code
    pub sync: bool,
    /// Generate asynchronous rust code
    pub r#async: bool,
    /// Derive serde's `Serialize` trait for generated types
    pub serialize: bool,
    /// Ignore query files prefixed with underscore
    #[serde(rename = "ignore-underscore-files")]
    pub ignore_underscore_files: bool,
    /// Container image to use for `schema` command
    #[serde(rename = "container-image")]
    pub container_image: String,
    /// Container wait time in milliseconds after health check
    #[serde(rename = "container-wait")]
    pub container_wait: u64,
    /// Make bind functions private to force usage of params() method
    #[serde(rename = "params-only")]
    pub params_only: bool,

    // Config file exclusive
    /// Custom type settings
    pub types: Types,
    /// The `package` table of the generated `Cargo.toml`
    pub package: Package,
    /// Options to configure code style of generated code
    pub style: Style,
    /// List of static files to copy into the generated directory
    #[serde(rename = "static")]
    pub static_files: Vec<StaticFile>,
    /// Use workspace dependencies
    #[serde(rename = "use-workspace-deps")]
    pub use_workspace_deps: UseWorkspaceDeps,
}

impl Config {
    /// Create config from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Create a new builder with default values
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
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
            container_image: "docker.io/library/postgres:latest".to_string(),
            container_wait: 250,
            queries: PathBuf::from_str("queries/").unwrap(),
            destination: PathBuf::from_str("clorinde").unwrap(),
            sync: false,
            r#async: true,
            serialize: false,
            ignore_underscore_files: false, // Default to false for backwards compatibility
            params_only: false,
            types: Types {
                crate_info: HashMap::new(),
                mapping: HashMap::new(),
                derive_traits: vec![],
                type_traits_mapping: HashMap::new(),
            },
            package: Package::default(),
            style: Style::default(),
            static_files: vec![],
            use_workspace_deps: UseWorkspaceDeps::Bool(false),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum StaticFile {
    Simple(PathBuf),
    Detailed {
        path: PathBuf,
        #[serde(default, rename = "hard-link")]
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
#[serde(default, deny_unknown_fields)]
pub struct Types {
    /// Crates to add as a dependency for custom types
    #[serde(rename = "crates")]
    pub crate_info: HashMap<String, CrateDependency>,
    /// Mapping for postgres to rust types
    pub mapping: HashMap<String, TypeMapping>,
    /// Derive traits added to all generated row structs and custom types
    #[serde(rename = "derive-traits")]
    pub derive_traits: Vec<String>,
    /// Mapping for custom postgres types (eg. domains, enums, etc) to derive traits
    #[serde(rename = "type-traits-mapping")]
    pub type_traits_mapping: HashMap<String, Vec<String>>,
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
#[serde(deny_unknown_fields)]
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
        #[serde(default = "default_true", rename = "is-copy")]
        is_copy: bool,
        #[serde(default = "default_true", rename = "is-params")]
        is_params: bool,
        #[serde(default, rename = "attributes")]
        attributes: Vec<String>,
    },
}

impl TypeMapping {
    pub fn get_attributes(&self) -> &[String] {
        match self {
            TypeMapping::Simple(_) => &[],
            TypeMapping::Detailed { attributes, .. } => attributes,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default, deny_unknown_fields)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: String,
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
            name: "clorinde".to_string(),
            version: "0.1.0".to_string(),
            edition: "2021".to_string(),
            publish: Publish::default(),
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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Style {
    /// Enforces all enum variants to use CamelCase, leaving postgres value in-tact
    #[serde(rename = "enum-variant-camel-case")]
    pub enum_variant_camel_case: bool,
}

#[derive(Debug, Default, Clone)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// Use `podman` instead of `docker`
    pub fn podman(mut self, podman: bool) -> Self {
        self.config.podman = podman;
        self
    }

    /// Set container image to use for schema command
    pub fn container_image(mut self, container_image: impl Into<String>) -> Self {
        self.config.container_image = container_image.into();
        self
    }

    /// Set container wait time in milliseconds after health check
    pub fn container_wait(mut self, container_wait: u64) -> Self {
        self.config.container_wait = container_wait;
        self
    }

    /// Set directory containing the queries
    pub fn queries(mut self, queries: impl Into<PathBuf>) -> Self {
        self.config.queries = queries.into();
        self
    }

    /// Set just the package name, keeping other package defaults
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.config.package.name = name.into();
        self
    }

    /// Set destination folder for generated modules
    pub fn destination(mut self, destination: impl Into<PathBuf>) -> Self {
        self.config.destination = destination.into();
        self
    }

    /// Generate synchronous rust code
    pub fn sync(mut self, sync: bool) -> Self {
        self.config.sync = sync;
        self
    }

    /// Generate asynchronous rust code
    pub fn r#async(mut self, r#async: bool) -> Self {
        self.config.r#async = r#async;
        self
    }

    /// Derive serde's `Serialize` trait for generated types
    pub fn serialize(mut self, serialize: bool) -> Self {
        self.config.serialize = serialize;
        self
    }

    /// Ignore query files prefixed with underscore
    pub fn ignore_underscore_files(mut self, ignore_underscore_files: bool) -> Self {
        self.config.ignore_underscore_files = ignore_underscore_files;
        self
    }

    /// Make bind functions private to force usage of params() method
    pub fn params_only(mut self, params_only: bool) -> Self {
        self.config.params_only = params_only;
        self
    }

    /// Set custom type settings
    pub fn types(mut self, types: Types) -> Self {
        self.config.types = types;
        self
    }

    /// Set package metadata for the generated `Cargo.toml`
    pub fn package(mut self, package: Package) -> Self {
        self.config.package = package;
        self
    }

    /// Set style options for generated code
    pub fn style(mut self, style: Style) -> Self {
        self.config.style = style;
        self
    }

    /// Add a static file to copy
    pub fn add_static_file(mut self, file: StaticFile) -> Self {
        self.config.static_files.push(file);
        self
    }

    /// Set static files to copy
    pub fn static_files(mut self, files: Vec<StaticFile>) -> Self {
        self.config.static_files = files;
        self
    }

    /// Configure workspace dependencies
    pub fn use_workspace_deps(mut self, use_workspace_deps: UseWorkspaceDeps) -> Self {
        self.config.use_workspace_deps = use_workspace_deps;
        self
    }

    /// Add a type mapping
    pub fn add_type_mapping(mut self, key: impl Into<String>, mapping: TypeMapping) -> Self {
        self.config.types.mapping.insert(key.into(), mapping);
        self
    }

    /// Add a crate dependency
    pub fn add_crate_dependency(
        mut self,
        name: impl Into<String>,
        dependency: CrateDependency,
    ) -> Self {
        self.config.types.crate_info.insert(name.into(), dependency);
        self
    }

    /// Add a simple crate dependency
    pub fn add_simple_crate_dependency(
        mut self,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        self.config
            .types
            .crate_info
            .insert(name.into(), CrateDependency::Simple(version.into()));
        self
    }

    /// Add a derive trait for all generated structs/types
    pub fn add_derive_trait(mut self, trait_name: impl Into<String>) -> Self {
        self.config.types.derive_traits.push(trait_name.into());
        self
    }

    /// Set derive traits for all generated structs/types
    pub fn derive_traits(mut self, traits: Vec<impl Into<String>>) -> Self {
        self.config.types.derive_traits = traits.into_iter().map(Into::into).collect();
        self
    }

    /// Add a type-specific trait mapping
    pub fn add_type_trait_mapping(
        mut self,
        type_name: impl Into<String>,
        traits: Vec<impl Into<String>>,
    ) -> Self {
        self.config.types.type_traits_mapping.insert(
            type_name.into(),
            traits.into_iter().map(Into::into).collect(),
        );
        self
    }

    /// Build the Config
    pub fn build(self) -> Config {
        self.config
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
