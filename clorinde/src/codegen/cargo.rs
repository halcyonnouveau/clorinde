use std::{collections::HashSet, fmt::Write, fs, path::Path};

use postgres_types::{Kind, Type};

use crate::config::{Config, CrateDependency, DependencyTable, UseWorkspaceDeps};

/// Register use of typed requiring specific dependencies
#[derive(Debug, Clone, Default)]
pub struct DependencyAnalysis {
    pub chrono: bool,
    pub json: bool,
    pub uuid: bool,
    pub mac_addr: bool,
    pub decimal: bool,
}

impl DependencyAnalysis {
    pub fn analyse(&mut self, ty: &Type) {
        match ty.kind() {
            Kind::Simple => match *ty {
                Type::TIME | Type::DATE | Type::TIMESTAMP | Type::TIMESTAMPTZ => self.chrono = true,
                Type::JSON | Type::JSONB => self.json = true,
                Type::UUID => self.uuid = true,
                Type::MACADDR => self.mac_addr = true,
                Type::NUMERIC => self.decimal = true,
                _ => {}
            },
            Kind::Array(ty) => self.analyse(ty),
            Kind::Domain(ty) => self.analyse(ty),
            Kind::Composite(fields) => {
                for field in fields {
                    self.analyse(field.type_())
                }
            }
            _ => {}
        }
    }

    pub fn has_dependency(&self) -> bool {
        self.chrono | self.json | self.uuid | self.mac_addr | self.decimal
    }
}

struct CargoWriter {
    buf: String,
    use_workspace_deps: bool,
    workspace_deps: HashSet<String>,
}

impl CargoWriter {
    fn get_workspace_deps(manifest_path: &Path) -> HashSet<String> {
        let mut deps = HashSet::new();
        if let Ok(contents) = fs::read_to_string(manifest_path) {
            if let Ok(manifest) = contents.parse::<toml::Value>() {
                if let Some(workspace) = manifest
                    .get("workspace")
                    .and_then(|w| w.get("dependencies"))
                {
                    deps.extend(
                        workspace
                            .as_table()
                            .into_iter()
                            .flat_map(|t| t.keys())
                            .map(|s| s.to_string()),
                    );
                }
            }
        }
        deps
    }

    fn new(config: &Config) -> Self {
        let use_workspace_deps = match &config.use_workspace_deps {
            UseWorkspaceDeps::Bool(enabled) => *enabled,
            UseWorkspaceDeps::Path(_) => true,
        };

        let workspace_deps = match &config.use_workspace_deps {
            UseWorkspaceDeps::Bool(true) => {
                CargoWriter::get_workspace_deps(Path::new("./Cargo.toml"))
            }
            UseWorkspaceDeps::Bool(false) => HashSet::new(),
            UseWorkspaceDeps::Path(path) => CargoWriter::get_workspace_deps(path),
        };

        Self {
            buf: String::new(),
            use_workspace_deps,
            workspace_deps,
        }
    }

    fn line(&mut self, line: &str) {
        writeln!(self.buf, "{}", line).unwrap();
    }

    fn dep(&mut self, name: &str, mut dep: DependencyTable) {
        // add `workspace = true` when `use-workspace-deps` option is enabled
        // and dependency appears in user's Cargo.toml `[workspace.dependencies]`
        if self.use_workspace_deps && self.workspace_deps.contains(name) {
            dep.version = None;
            dep.workspace = Some(true);
        } else {
            dep.workspace = None;
        }

        if dep.is_simple_version() {
            writeln!(self.buf, "{} = \"{}\"", name, dep.version.unwrap()).unwrap();
        } else {
            let dep_str = toml::to_string(&dep)
                .unwrap()
                .replace('\n', ", ")
                .trim_end_matches(", ")
                .to_string();

            writeln!(self.buf, "{} = {{ {} }}", name, dep_str).unwrap();
        }
    }

    fn into_string(self) -> String {
        self.buf
    }
}

pub fn gen_cargo_file(dependency_analysis: &DependencyAnalysis, config: &Config) -> String {
    let mut cargo = CargoWriter::new(config);

    cargo.line("# This file was generated with `clorinde`. Do not modify.");
    cargo.line(
        &config
            .package
            .to_string()
            .expect("unable to serialize package"),
    );

    if config.r#async {
        let mut default_features = vec!["\"deadpool\""];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            default_features.push("\"chrono\"");
        }

        let mut wasm_features = vec!["\"tokio-postgres/js\""];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            wasm_features.push("\"chrono?/wasmbind\"");
            wasm_features.push("\"time?/wasm-bindgen\"");
        }

        cargo.line("[features]");
        cargo.line(&format!("default = [{}]", default_features.join(", ")));
        cargo.line("deadpool = [\"dep:deadpool-postgres\", \"tokio-postgres/default\"]");
        cargo.line(&format!("wasm-async = [{}]", wasm_features.join(", ")));
    } else {
        let mut wasm_features = vec![];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            wasm_features.push("\"chrono?/wasmbind\"");
        }

        cargo.line("[features]");
        cargo.line("default = []");
        cargo.line(&format!("wasm-sync = [{}]", wasm_features.join(", ")));
    }

    if dependency_analysis.chrono {
        cargo.line("\nchrono = [\"dep:chrono\"]");
        cargo.line("time = [\"dep:time\"]");
    } else {
        cargo.line("\nchrono = []");
        cargo.line("time = []");
    }

    cargo.line("\n[dependencies]");
    cargo.line("## Core dependencies");
    cargo.line("# Postgres types");

    cargo.dep(
        "postgres-types",
        DependencyTable::new("0.2.9").features(vec!["derive"]),
    );

    cargo.line("# Postgres interaction");
    cargo.dep("postgres-protocol", DependencyTable::new("0.6.8"));

    cargo
        .line("# Iterator utils required for working with `postgres_protocol::types::ArrayValues`");
    cargo.dep("fallible-iterator", DependencyTable::new("0.2.0"));

    let mut client_features = Vec::new();

    if dependency_analysis.has_dependency() {
        cargo.line("\n## Types dependencies");
        if dependency_analysis.chrono {
            cargo.line("# TIME, DATE, TIMESTAMP or TIMESTAMPZ");
            cargo.dep("chrono", DependencyTable::new("0.4.39").optional());
            cargo.dep("time", DependencyTable::new("0.3.37").optional());

            client_features.push("with-chrono-0_4".to_string());
            client_features.push("with-time-0_3".to_string());
        }
        if dependency_analysis.uuid {
            cargo.line("# UUID");
            cargo.dep("uuid", DependencyTable::new("1.13.1"));
            client_features.push("with-uuid-1".to_string());
        }
        if dependency_analysis.mac_addr {
            cargo.line("# MAC ADDRESS");
            cargo.dep("eui48", DependencyTable::new("1.1.0").no_default_features());
            client_features.push("with-eui48-1".to_string());
        }
        if dependency_analysis.decimal {
            cargo.line("# DECIMAL");
            cargo.dep(
                "rust_decimal",
                DependencyTable::new("1.36.0").features(vec!["db-postgres"]),
            );
        }
        if dependency_analysis.json {
            cargo.line("# JSON or JSONB");
            cargo.dep(
                "serde_json",
                DependencyTable::new("1.0.134").features(vec!["raw_value"]),
            );
            cargo.dep(
                "serde",
                DependencyTable::new("1.0.217").features(vec!["derive"]),
            );
            client_features.push("with-serde_json-1".to_string());
        }
    }

    // add serde if serializing but not using json type
    if config.serialize && !dependency_analysis.json {
        cargo.line("# JSON or JSONB");
        cargo.dep(
            "serde",
            DependencyTable::new("1.0.217").features(vec!["derive"]),
        );
        client_features.push("with-serde_json-1".to_string());
    }

    if config.sync {
        cargo.line("\n## Sync client dependencies");
        cargo.line("# Postgres sync client");
        cargo.dep(
            "postgres",
            DependencyTable::new("0.19.10").features(client_features.clone()),
        );
    }

    if config.r#async {
        cargo.line("\n## Async client dependencies");
        cargo.line("# Postgres async client");
        cargo.dep(
            "tokio-postgres",
            DependencyTable::new("0.7.13").features(client_features),
        );

        cargo.line("# Async utils");
        cargo.dep("futures", DependencyTable::new("0.3.31"));

        cargo.line("\n## Async features dependencies");
        cargo.line("# Async connection pooling");
        cargo.dep(
            "deadpool-postgres",
            DependencyTable::new("0.14.1").optional(),
        );
    }

    if !config.types.crate_info.is_empty() {
        cargo.line("\n## Custom type dependencies");
        let mut crates: Vec<_> = config.types.crate_info.iter().collect();
        crates.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

        for (name, dep) in crates {
            match dep {
                CrateDependency::Simple(version) => {
                    cargo.line(&format!("{} = \"{}\"", name, version));
                }
                CrateDependency::Detailed(dependency) => {
                    cargo.dep(name, dependency.to_owned());
                }
            }
        }
    }

    cargo.into_string()
}
