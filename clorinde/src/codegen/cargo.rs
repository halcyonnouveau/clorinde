use std::{collections::HashSet, fs, path::Path};

use cargo_toml::{Dependency, DependencyDetail, InheritedDependencyDetail};
use postgres_types::{Kind, Type};

use crate::config::{Config, DependencyTable, UseWorkspaceDeps};

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

fn to_cargo_dep(dep: &DependencyTable, use_workspace: bool) -> Dependency {
    if use_workspace {
        // for workspace dependencies, use Inherited variant
        let mut inherited = InheritedDependencyDetail::default();
        inherited.workspace = true;

        if let Some(features) = &dep.features {
            inherited.features = features.clone();
        }

        if let Some(optional) = dep.optional {
            inherited.optional = optional;
        }

        Dependency::Inherited(inherited)
    } else {
        let mut detail = DependencyDetail::default();
        detail.version = dep.version.clone();
        detail.path = dep.path.clone();
        detail.features = dep.features.clone().unwrap_or_default();
        detail.optional = dep.optional.unwrap_or(false);
        detail.default_features = dep.default_features.unwrap_or(true);
        Dependency::Detailed(Box::new(detail))
    }
}

pub fn gen_cargo_file(dependency_analysis: &DependencyAnalysis, config: &Config) -> String {
    let mut manifest = config.manifest.clone();

    let (use_workspace_deps, workspace_deps) = match &config.use_workspace_deps {
        UseWorkspaceDeps::Bool(true) => (true, get_workspace_deps(Path::new("./Cargo.toml"))),
        UseWorkspaceDeps::Bool(false) => (false, HashSet::new()),
        UseWorkspaceDeps::Path(path) => (true, get_workspace_deps(path)),
    };

    if config.r#async {
        let mut default_features = vec!["deadpool".to_string()];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            default_features.push("chrono".to_string());
        }

        manifest
            .features
            .insert("default".to_string(), default_features);
        manifest.features.insert(
            "deadpool".to_string(),
            vec![
                "dep:deadpool-postgres".to_string(),
                "tokio-postgres/default".to_string(),
            ],
        );

        let mut wasm_features = vec!["tokio-postgres/js".to_string()];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            wasm_features.push("chrono?/wasmbind".to_string());
            wasm_features.push("time?/wasm-bindgen".to_string());
        }
        manifest
            .features
            .insert("wasm-async".to_string(), wasm_features);
    } else {
        manifest.features.insert("default".to_string(), vec![]);

        let mut wasm_features = vec![];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            wasm_features.push("chrono?/wasmbind".to_string());
        }
        manifest
            .features
            .insert("wasm-sync".to_string(), wasm_features);
    }

    // Add chrono/time features
    if dependency_analysis.chrono {
        manifest
            .features
            .insert("chrono".to_string(), vec!["dep:chrono".to_string()]);
        manifest
            .features
            .insert("time".to_string(), vec!["dep:time".to_string()]);
    } else {
        manifest.features.insert("chrono".to_string(), vec![]);
        manifest.features.insert("time".to_string(), vec![]);
    }

    // Core dependencies
    let postgres_types_dep = DependencyTable::new("0.2.9").features(vec!["derive"]);
    manifest.dependencies.insert(
        "postgres-types".to_string(),
        to_cargo_dep(
            &postgres_types_dep,
            use_workspace_deps && workspace_deps.contains("postgres-types"),
        ),
    );

    let postgres_protocol_dep = DependencyTable::new("0.6.8");
    manifest.dependencies.insert(
        "postgres-protocol".to_string(),
        to_cargo_dep(
            &postgres_protocol_dep,
            use_workspace_deps && workspace_deps.contains("postgres-protocol"),
        ),
    );

    let mut client_features = Vec::new();

    // Type dependencies
    if dependency_analysis.has_dependency() {
        if dependency_analysis.chrono {
            let chrono_features = if config.serialize || dependency_analysis.json {
                vec!["serde"]
            } else {
                vec![]
            };
            let chrono_dep = DependencyTable::new("0.4.40")
                .features(chrono_features)
                .optional();
            manifest.dependencies.insert(
                "chrono".to_string(),
                to_cargo_dep(
                    &chrono_dep,
                    use_workspace_deps && workspace_deps.contains("chrono"),
                ),
            );

            let time_dep = DependencyTable::new("0.3.41").optional();
            manifest.dependencies.insert(
                "time".to_string(),
                to_cargo_dep(
                    &time_dep,
                    use_workspace_deps && workspace_deps.contains("time"),
                ),
            );

            client_features.push("with-chrono-0_4");
            client_features.push("with-time-0_3");
        }

        if dependency_analysis.uuid {
            let uuid_features = if config.serialize || dependency_analysis.json {
                vec!["serde"]
            } else {
                vec![]
            };

            let uuid_dep = DependencyTable::new("1.16.0").features(uuid_features);
            manifest.dependencies.insert(
                "uuid".to_string(),
                to_cargo_dep(
                    &uuid_dep,
                    use_workspace_deps && workspace_deps.contains("uuid"),
                ),
            );
            client_features.push("with-uuid-1");
        }

        if dependency_analysis.mac_addr {
            let eui48_dep = DependencyTable::new("1.1.0").no_default_features();
            manifest.dependencies.insert(
                "eui48".to_string(),
                to_cargo_dep(
                    &eui48_dep,
                    use_workspace_deps && workspace_deps.contains("eui48"),
                ),
            );
            client_features.push("with-eui48-1");
        }

        if dependency_analysis.decimal {
            let rust_decimal_dep = DependencyTable::new("1.37.1").features(vec!["db-postgres"]);
            manifest.dependencies.insert(
                "rust_decimal".to_string(),
                to_cargo_dep(
                    &rust_decimal_dep,
                    use_workspace_deps && workspace_deps.contains("rust_decimal"),
                ),
            );
        }

        if dependency_analysis.json {
            let serde_json_dep = DependencyTable::new("1.0.140").features(vec!["raw_value"]);
            manifest.dependencies.insert(
                "serde_json".to_string(),
                to_cargo_dep(
                    &serde_json_dep,
                    use_workspace_deps && workspace_deps.contains("serde_json"),
                ),
            );

            let serde_dep = DependencyTable::new("1.0.219").features(vec!["derive"]);
            manifest.dependencies.insert(
                "serde".to_string(),
                to_cargo_dep(
                    &serde_dep,
                    use_workspace_deps && workspace_deps.contains("serde"),
                ),
            );
            client_features.push("with-serde_json-1");
        }
    }

    // Add serde if serializing but not using json type
    if config.serialize && !dependency_analysis.json {
        let serde_dep = DependencyTable::new("1.0.219").features(vec!["derive"]);
        manifest.dependencies.insert(
            "serde".to_string(),
            to_cargo_dep(
                &serde_dep,
                use_workspace_deps && workspace_deps.contains("serde"),
            ),
        );
        client_features.push("with-serde_json-1");
    }

    // Postgres client
    let postgres_dep = DependencyTable::new("0.19.10")
        .features(client_features.iter().map(|s| s.to_string()).collect());

    manifest.dependencies.insert(
        "postgres".to_string(),
        to_cargo_dep(
            &postgres_dep,
            use_workspace_deps && workspace_deps.contains("postgres"),
        ),
    );

    // Async dependencies
    if config.r#async {
        let tokio_postgres_dep = DependencyTable::new("0.7.13")
            .features(client_features.iter().map(|s| s.to_string()).collect());

        manifest.dependencies.insert(
            "tokio-postgres".to_string(),
            to_cargo_dep(
                &tokio_postgres_dep,
                use_workspace_deps && workspace_deps.contains("tokio-postgres"),
            ),
        );

        let futures_dep = DependencyTable::new("0.3.31");
        manifest.dependencies.insert(
            "futures".to_string(),
            to_cargo_dep(
                &futures_dep,
                use_workspace_deps && workspace_deps.contains("futures"),
            ),
        );

        let deadpool_dep = DependencyTable::new("0.14.1").optional();
        manifest.dependencies.insert(
            "deadpool-postgres".to_string(),
            to_cargo_dep(
                &deadpool_dep,
                use_workspace_deps && workspace_deps.contains("deadpool-postgres"),
            ),
        );
    }

    let mut output = String::from("# This file was generated with `clorinde`. Do not modify.\n\n");
    output.push_str(&toml::to_string(&manifest).expect("Failed to serialize manifest"));
    output
}
