use miette::{Diagnostic, Result};
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
#[non_exhaustive]
pub struct Config {
    /// Generate field metadata for queries
    #[serde(rename = "generate-field-metadata")]
    pub generate_field_metadata: bool,  
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
    #[deprecated(
        since = "1.0.0",
        note = "please use the `types.derive-traits` configuration instead"
    )]
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
    /// List of static files to copy into the generated directory
    #[serde(rename = "static")]
    pub static_files: Vec<StaticFile>,
    /// Use workspace dependencies
    #[serde(rename = "use-workspace-deps")]
    pub use_workspace_deps: UseWorkspaceDeps,
    /// Options to configure code style of generated code
    pub style: Style,
    /// Custom type settings
    pub types: Types,
    /// The Cargo.toml manifest configuration
    pub manifest: cargo_toml::Manifest<toml::Value>,
}

impl Config {
    /// Create config from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&contents)?;

        if let Some(manifest) = &mut config.manifest.package {
            if manifest.edition == cargo_toml::Inheritable::Set(cargo_toml::Edition::E2015) {
                manifest.edition = cargo_toml::Inheritable::Set(cargo_toml::Edition::E2021);
            }
        }

        Ok(config)
    }

    pub fn builder_from_file<P: AsRef<Path>>(path: P) -> Result<ConfigBuilder, ConfigError> {
        Ok(ConfigBuilder {
            config: Config::from_file(path)?,
        })
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
            generate_field_metadata: false,
            container_wait: 250,
            queries: PathBuf::from_str("queries/").unwrap(),
            destination: PathBuf::from_str("clorinde").unwrap(),
            sync: false,
            r#async: true,
            #[allow(deprecated)]
            serialize: false,
            ignore_underscore_files: false,
            params_only: false,
            types: Types {
                mapping: HashMap::new(),
                derive_traits: vec![],
                type_traits_mapping: HashMap::new(),
            },
            manifest: default_manifest(),
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
#[non_exhaustive]
pub struct Types {
    /// Mapping for postgres to rust types
    pub mapping: HashMap<String, TypeMapping>,
    /// Derive traits added to all generated row structs and custom types
    #[serde(rename = "derive-traits")]
    pub derive_traits: Vec<String>,
    /// Mapping for custom postgres types (eg. domains, enums, etc) to derive traits
    #[serde(rename = "type-traits-mapping")]
    pub type_traits_mapping: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
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

#[allow(deprecated)]
fn default_manifest() -> cargo_toml::Manifest<toml::Value> {
    let mut package = cargo_toml::Package::new("clorinde", "0.0.0");
    package.edition = cargo_toml::Inheritable::Set(cargo_toml::Edition::E2021);
    package.publish = cargo_toml::Inheritable::Set(cargo_toml::Publish::Flag(false));

    cargo_toml::Manifest {
        package: Some(package),
        workspace: None,
        dependencies: Default::default(),
        dev_dependencies: Default::default(),
        build_dependencies: Default::default(),
        target: Default::default(),
        features: Default::default(),
        replace: Default::default(),
        patch: Default::default(),
        lib: None,
        profile: cargo_toml::Profiles::default(),
        badges: Default::default(),
        bin: vec![],
        bench: vec![],
        test: vec![],
        example: vec![],
        lints: cargo_toml::Inheritable::default(),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
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
        if let Some(package) = &mut self.config.manifest.package {
            package.name = name.into();
        } else {
            let mut package = cargo_toml::Package::new(name.into(), "0.1.0");
            package.edition = cargo_toml::Inheritable::Set(cargo_toml::Edition::E2021);
            package.publish = cargo_toml::Inheritable::Set(cargo_toml::Publish::Flag(false));
            self.config.manifest.package = Some(package);
        }
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
    #[deprecated(
        since = "1.0.0",
        note = "please use the `types.derive-traits` configuration instead"
    )]
    #[allow(deprecated)]
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

    /// Set the entire Cargo.toml manifest
    pub fn manifest(mut self, manifest: cargo_toml::Manifest<toml::Value>) -> Self {
        self.config.manifest = manifest;
        self
    }

    /// Set package metadata for the generated `Cargo.toml`
    pub fn package(mut self, package: cargo_toml::Package) -> Self {
        self.config.manifest.package = Some(package);
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

    /// Add a dependency to the generated Cargo.toml
    pub fn add_dependency(
        mut self,
        name: impl Into<String>,
        dependency: cargo_toml::Dependency,
    ) -> Self {
        self.config
            .manifest
            .dependencies
            .insert(name.into(), dependency);
        self
    }

    /// Build the Config
    pub fn build(self) -> Config {
        self.config
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
#[non_exhaustive]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    Toml(#[from] toml::de::Error),
}

fn default_true() -> bool {
    true
}
