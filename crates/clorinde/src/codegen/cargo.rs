use std::fmt::Write;

use indoc::{formatdoc, writedoc};
use postgres_types::{Kind, Type};

use crate::config::{Config, CrateDependency};

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

pub fn gen_cargo_file(dependency_analysis: &DependencyAnalysis, config: &Config) -> String {
    let package = config
        .package
        .to_string()
        .expect("unable to serialize package");

    let mut buf = formatdoc! {r#"
        # This file was generated with `clorinde`. Do not modify
        {package}
    "#};

    if config.r#async {
        let mut default_features = vec!["\"deadpool\""];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            default_features.push("\"chrono\"");
        }

        let mut wasm_features = vec!["\"tokio-postgres/js\""];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            wasm_features.push("\"chrono/wasmbind\"");
            wasm_features.push("\"time/wasm-bindgen\"");
        }

        let default_features = default_features.join(", ");
        let wasm_features = wasm_features.join(", ");

        writedoc! { buf, r#"

            [features]
            default = [{default_features}]
            deadpool = ["dep:deadpool-postgres", "tokio-postgres/default"]
            wasm-async = [{wasm_features}]
        "#}
        .unwrap()
    } else {
        let mut wasm_features = vec![];
        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            wasm_features.push("\"chrono/wasmbind\"");
        }

        let wasm_features = wasm_features.join(", ");

        writedoc! { buf, r#"

            [features]
            default = []
            wasm-sync = [{wasm_features}]
        "#}
        .unwrap();
    }

    if dependency_analysis.chrono {
        writedoc! { buf, r#"

            chrono = ["dep:chrono"]
            time = ["dep:time"]
        "#}
        .unwrap();
    } else {
        writedoc! { buf, r#"

            chrono = []
            time = []
        "#}
        .unwrap();
    }

    writedoc! { buf, r#"

        [dependencies]
        ## Core dependencies
        # Postgres types
        postgres-types = {{ version = "0.2.8", features = ["derive"] }}
        # Postgres interaction
        postgres-protocol = "0.6.7"
        # Iterator utils required for working with `postgres_protocol::types::ArrayValues`
        fallible-iterator = "0.2.0"
    "#}
    .unwrap();

    // add custom type crates
    if !config.types.mapping.is_empty() {
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
                        writedoc! { buf, r#"
                        {name} = "{version}"
                    "#}
                        .unwrap();
                    }
                    CrateDependency::Detailed {
                        version,
                        path,
                        features,
                        default_features,
                        optional,
                    } => {
                        writedoc! { buf, r#"
                        {name} = {{"#}
                        .unwrap();

                        let mut first = true;

                        if let Some(version) = version {
                            write!(buf, r#"version = "{version}""#).unwrap();
                            first = false;
                        }
                        if let Some(path) = path {
                            if !first {
                                write!(buf, ", ").unwrap();
                            }
                            write!(buf, r#"path = "{path}""#).unwrap();
                            first = false;
                        }
                        if let Some(features) = features {
                            if !first {
                                write!(buf, ", ").unwrap();
                            }
                            write!(buf, r#"features = ["#).unwrap();
                            for (i, feature) in features.iter().enumerate() {
                                if i > 0 {
                                    write!(buf, ", ").unwrap();
                                }
                                write!(buf, r#""{feature}""#).unwrap();
                            }
                            write!(buf, "]").unwrap();
                            first = false;
                        }
                        if let Some(default_features) = default_features {
                            if !first {
                                write!(buf, ", ").unwrap();
                            }
                            write!(buf, r#"default-features = {default_features}"#).unwrap();
                            first = false;
                        }
                        if let Some(optional) = optional {
                            if !first {
                                write!(buf, ", ").unwrap();
                            }
                            write!(buf, r#"optional = {optional}"#).unwrap();
                        }

                        writedoc! { buf, r#"}}
                        "#}
                        .unwrap();
                    }
                }
            }
        } else if references_default_crate {
            writedoc! { buf, r#"
            ctypes = {{ path = "../ctypes" }}
        "#}
            .unwrap();
        }
    }

    let mut client_features = String::new();

    if dependency_analysis.has_dependency() {
        writeln!(buf, "\n## Types dependencies").unwrap();
        if dependency_analysis.json {
            writedoc! { buf, r#"
                # JSON or JSONB
                serde_json = {{ version = "1.0.134", features = ["raw_value"] }}
                serde = {{ version = "1.0.217", features = ["derive"] }}
            "#}
            .unwrap();
            write!(client_features, r#""with-serde_json-1","#).unwrap();
        }
        if dependency_analysis.chrono {
            writedoc! { buf, r#"
                # TIME, DATE, TIMESTAMP or TIMESTAMPZ
                chrono = {{ version = "0.4.39", optional = true }}
                time = {{ version = "0.3.37", optional = true }}
            "#}
            .unwrap();
            write!(client_features, r#""with-chrono-0_4","#).unwrap();
            write!(client_features, r#""with-time-0_3","#).unwrap();
        }
        if dependency_analysis.uuid {
            writedoc! { buf, r#"
                # UUID
                uuid = "1.11.0"
            "#}
            .unwrap();
            write!(client_features, r#""with-uuid-1","#).unwrap();
        }
        if dependency_analysis.mac_addr {
            writedoc! { buf, r#"
                # MAC ADDRESS
                eui48 = {{ version = "1.1.0", default-features = false }}
            "#}
            .unwrap();
            write!(client_features, r#""with-eui48-1","#).unwrap();
        }
        if dependency_analysis.decimal {
            writedoc! { buf, r#"
                # DECIMAL
                rust_decimal = {{ version = "1.36.0", features = ["db-postgres"] }}
            "#}
            .unwrap();
        }
    }

    if config.sync {
        writedoc! { buf, r#"

            ## Sync client dependencies
            # Postgres sync client
            postgres = {{ version = "0.19.9", features = [{client_features}] }}
        "#}
        .unwrap();
    }

    if config.r#async {
        writedoc! { buf, r#"

            ## Async client dependencies
            # Postgres async client
            tokio-postgres = {{ version = "0.7.12", default-features = false, features = [{client_features}] }}
            # Async utils
            async-trait = "0.1.78"
            futures = "0.3.31"

            ## Async features dependencies
            # Async connection pooling
            deadpool-postgres = {{ version = "0.14.1", optional = true }}
        "#}
        .unwrap();
    }

    buf
}
