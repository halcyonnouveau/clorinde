use std::fmt::Write;

use postgres_types::{Kind, Type};

use crate::config::{Config, CrateDependency, Dependency};

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

fn write_dep(buf: &mut String, name: &str, mut dep: Dependency) {
    if dep.workspace.unwrap_or(false) {
        dep.version = None;
        dep.workspace = Some(true);
    } else {
        dep.workspace = None;
    }

    let dep_str = toml::to_string(&dep)
        .unwrap()
        .replace('\n', ", ")
        .trim_end_matches(", ")
        .to_string();

    writeln!(buf, "{} = {{ {} }}", name, dep_str).unwrap();
}

pub fn gen_cargo_file(dependency_analysis: &DependencyAnalysis, config: &Config) -> String {
    let workspace = config.use_workspace_deps;
    let package = config
        .package
        .to_string()
        .expect("unable to serialize package");

    let mut buf = String::new();
    writeln!(
        buf,
        "# This file was generated with `clorinde`. Do not modify."
    )
    .unwrap();
    writeln!(buf, "{}", package).unwrap();

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

        writeln!(buf, "[features]").unwrap();
        writeln!(buf, "default = [{}]", default_features.join(", ")).unwrap();
        writeln!(
            buf,
            "deadpool = [\"dep:deadpool-postgres\", \"tokio-postgres/default\"]"
        )
        .unwrap();
        writeln!(buf, "wasm-async = [{}]", wasm_features.join(", ")).unwrap();
    } else {
        let mut wasm_features = vec![];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            wasm_features.push("\"chrono?/wasmbind\"");
        }

        writeln!(buf, "[features]").unwrap();
        writeln!(buf, "default = []").unwrap();
        writeln!(buf, "wasm-sync = [{}]", wasm_features.join(", ")).unwrap();
    }

    if dependency_analysis.chrono {
        writeln!(buf, "\nchrono = [\"dep:chrono\"]").unwrap();
        writeln!(buf, "time = [\"dep:time\"]").unwrap();
    } else {
        writeln!(buf, "\nchrono = []").unwrap();
        writeln!(buf, "time = []").unwrap();
    }

    writeln!(buf, "\n[dependencies]").unwrap();
    writeln!(buf, "## Core dependencies").unwrap();
    writeln!(buf, "# Postgres types").unwrap();

    write_dep(
        &mut buf,
        "postgres-types",
        Dependency {
            version: Some("0.2.9".to_string()),
            features: Some(vec!["derive".to_string()]),
            workspace: Some(workspace),
            ..Default::default()
        },
    );

    writeln!(buf, "# Postgres interaction").unwrap();
    write_dep(
        &mut buf,
        "postgres-protocol",
        Dependency {
            version: Some("0.6.8".to_string()),
            workspace: Some(workspace),
            ..Default::default()
        },
    );

    writeln!(
        buf,
        "# Iterator utils required for working with `postgres_protocol::types::ArrayValues`"
    )
    .unwrap();
    write_dep(
        &mut buf,
        "fallible-iterator",
        Dependency {
            version: Some("0.2.0".to_string()),
            workspace: Some(workspace),
            ..Default::default()
        },
    );

    // add custom type crates
    if !config.types.mapping.is_empty() {
        writeln!(buf, "\n## Custom type crates").unwrap();

        let references_default_crate = config.types.mapping.values().any(|t| {
            match t {
                crate::config::TypeMapping::Simple(t) => t,
                crate::config::TypeMapping::Detailed { rust_type, .. } => rust_type,
            }
            .starts_with("ctypes::")
        });

        if !config.types.crate_info.is_empty() {
            for (name, dep) in &config.types.crate_info {
                match dep {
                    CrateDependency::Simple(version) => {
                        writeln!(buf, "{} = \"{}\"", name, version).unwrap();
                    }
                    CrateDependency::Detailed(dependency) => {
                        write_dep(&mut buf, name, dependency.to_owned());
                    }
                }
            }
        } else if references_default_crate {
            writeln!(buf, "ctypes = {{ path = \"../ctypes\" }}").unwrap();
        }
    }

    let mut client_features = Vec::new();

    if dependency_analysis.has_dependency() {
        writeln!(buf, "\n## Types dependencies").unwrap();
        if dependency_analysis.chrono {
            writeln!(buf, "# TIME, DATE, TIMESTAMP or TIMESTAMPZ").unwrap();
            write_dep(
                &mut buf,
                "chrono",
                Dependency {
                    version: Some("0.4.39".to_string()),
                    workspace: Some(workspace),
                    optional: Some(true),
                    ..Default::default()
                },
            );
            write_dep(
                &mut buf,
                "time",
                Dependency {
                    version: Some("0.3.37".to_string()),
                    workspace: Some(workspace),
                    optional: Some(true),
                    ..Default::default()
                },
            );

            client_features.push("with-chrono-0_4".to_string());
            client_features.push("with-time-0_3".to_string());
        }
        if dependency_analysis.uuid {
            writeln!(buf, "# UUID").unwrap();
            write_dep(
                &mut buf,
                "uuid",
                Dependency {
                    version: Some("1.13.1".to_string()),
                    workspace: Some(workspace),
                    ..Default::default()
                },
            );

            client_features.push("with-uuid-1".to_string());
        }
        if dependency_analysis.mac_addr {
            writeln!(buf, "# MAC ADDRESS").unwrap();
            write_dep(
                &mut buf,
                "eui48",
                Dependency {
                    version: Some("1.1.0".to_string()),
                    workspace: Some(workspace),
                    default_features: Some(false),
                    ..Default::default()
                },
            );

            client_features.push("with-eui48-1".to_string());
        }
        if dependency_analysis.decimal {
            writeln!(buf, "# DECIMAL").unwrap();
            write_dep(
                &mut buf,
                "rust_decimal",
                Dependency {
                    version: Some("1.36.0".to_string()),
                    workspace: Some(workspace),
                    features: Some(vec!["db-postgres".to_string()]),
                    ..Default::default()
                },
            );
        }
        if dependency_analysis.json {
            writeln!(buf, "# JSON or JSONB").unwrap();
            write_dep(
                &mut buf,
                "serde_json",
                Dependency {
                    version: Some("1.0.134".to_string()),
                    workspace: Some(workspace),
                    features: Some(vec!["raw_value".to_string()]),
                    ..Default::default()
                },
            );
            write_dep(
                &mut buf,
                "serde",
                Dependency {
                    version: Some("1.0.217".to_string()),
                    workspace: Some(workspace),
                    features: Some(vec!["derive".to_string()]),
                    ..Default::default()
                },
            );
            client_features.push("with-serde_json-1".to_string());
        }
    }

    // add serde if serializing but not using json type
    if config.serialize && !dependency_analysis.json {
        writeln!(buf, "# JSON or JSONB").unwrap();
        write_dep(
            &mut buf,
            "serde",
            Dependency {
                version: Some("1.0.217".to_string()),
                workspace: Some(workspace),
                features: Some(vec!["derive".to_string()]),
                ..Default::default()
            },
        );
        client_features.push("with-serde_json-1".to_string());
    }

    if config.sync {
        writeln!(buf, "\n## Sync client dependencies").unwrap();
        writeln!(buf, "# Postgres sync client").unwrap();
        write_dep(
            &mut buf,
            "postgres",
            Dependency {
                version: Some("0.19.10".to_string()),
                workspace: Some(workspace),
                features: Some(client_features.clone()),
                ..Default::default()
            },
        );
    }

    if config.r#async {
        writeln!(buf, "\n## Async client dependencies").unwrap();
        writeln!(buf, "# Postgres async client").unwrap();
        write_dep(
            &mut buf,
            "tokio-postgres",
            Dependency {
                version: Some("0.7.13".to_string()),
                workspace: Some(workspace),
                features: Some(client_features),
                default_features: Some(false),
                ..Default::default()
            },
        );

        writeln!(buf, "# Async utils").unwrap();
        write_dep(
            &mut buf,
            "futures",
            Dependency {
                version: Some("0.3.31".to_string()),
                workspace: Some(workspace),
                ..Default::default()
            },
        );

        writeln!(buf, "\n## Async features dependencies").unwrap();
        writeln!(buf, "# Async connection pooling").unwrap();
        write_dep(
            &mut buf,
            "deadpool-postgres",
            Dependency {
                version: Some("0.14.1".to_string()),
                workspace: Some(workspace),
                optional: Some(true),
                ..Default::default()
            },
        );
    }

    buf
}
