use crate::config::Config;
use postgres_types::{Kind, Type};
use regex::Regex;

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
    let mut deps_toml = String::new();
    let package = config
        .package
        .to_string()
        .expect("unable to serialize package");
    deps_toml.push_str("This file was generated with `clorinde`.Do not modify\n");
    deps_toml.push_str(&package);
    match config.r#async {
        true => {
            match (
                dependency_analysis.has_dependency(),
                dependency_analysis.chrono,
            ) {
                (true, true) => {
                    deps_toml.push_str(
                        "\n\
                    [features]\n\
                    default = [\"deadpool\",\"chrono\"]\n\
                    deadpool = [\"dep:deadpool-postgres\",\"tokio-postgres/default\"]\n\
                    wasm-async = [\"tokio-postgres/js\",\"chrono?/wasmbind\", \"time?/wasm-bindgen\"]    
                    ",
                    );
                }
                _ => deps_toml.push_str(
                    "\n\
                    [features]\n\
                    default = [\"deadpool\"]\n\
                    deadpool = [\"dep:deadpool-postgres\",\"tokio-postgres/default\"]\n\
                    wasm-async = [\"tokio-postgres/js\"]
                    ",
                ),
            }
        }
        _ => {
            deps_toml.push_str(
                "\n\
                    [features]\n\
                    default = []",
            );
            match (
                dependency_analysis.has_dependency(),
                dependency_analysis.chrono,
            ) {
                (true, true) => {
                    deps_toml.push_str(
                        "\n\
                    wasm-sync = [\"chrono?/wasmbind\"]    
                    ",
                    );
                }
                _ => {
                    deps_toml.push_str(
                        "\n\
                    wasm-sync = []    
                    ",
                    );
                }
            }
        }
    };
    match dependency_analysis.chrono {
        true => {
            deps_toml.push_str(
                "\n\
            chrono = [\"dep:chrono\"]\n\
            time = [\"dep:time\"]
            ",
            );
        }
        _ => {
            deps_toml.push_str(
                "\n\
            chrono = []\n\
            time = []
            ",
            );
        }
    };
    deps_toml.push_str(
        "\n\
        [dependencies]\n\
        ## Core dependencies\n\
        # Postgres types\n\
        postgres-types = {\"0.2.8\", features = [\"derive\"] }\n\
        # Postgres interaction\n\
        postgres-protocol = {\"0.6.7\"}\n\
        # Iterator utils required for working with `postgres_protocol::types::ArrayValues`\n\
        fallible-iterator = {\"0.2.0\"}
        ",
    );
    match dependency_analysis.has_dependency() {
        true => deps_toml.push_str(
            "\n\
            ## Types dependencies\n\
            # TIME, DATE, TIMESTAMP or TIMESTAMPZ\n\
            chrono = {\"0.4.39\", optional = true }
            time = {\"0.3.37\", optional = true }
        ",
        ),
        _ => {}
    }
    match dependency_analysis.uuid {
        true => deps_toml.push_str(
            "\n\
            # UUID
            uuid = {\"1.11.0\"}
        ",
        ),
        _ => {}
    };
    match dependency_analysis.mac_addr {
        true => deps_toml.push_str(
            "\n\
           # MAC ADDRESS\n\
           eui48 = {\"1.1.0\", default-features = false }
        ",
        ),
        _ => {}
    };
    match dependency_analysis.decimal {
        true => deps_toml.push_str(
            "\n\
            # DECIMAL\n\
            rust_decimal = {\"1.36.0\", features = [\"db-postgres\"] }
        ",
        ),
        _ => {}
    };
    match dependency_analysis.uuid {
        true => deps_toml.push_str(
            "\n\
            # JSON or JSONB\n\
            serde_json = {\"1.0.134\", features = [\"raw_value\"] }\n\
            serde = {\"1.0.217\", features = [\"derive\"] }\n\
        ",
        ),
        _ => {}
    };
    match (config.serialize, !dependency_analysis.json) {
        (true, true) => deps_toml.push_str(
            "\n\
            ## Serialize\n\
            serde = {\"1.0.217\", features = [\"derive\"] }
        ",
        ),
        _ => {}
    };
    match config.r#async {
        true => deps_toml.push_str(
            "\n\
            ## Async client dependencies\n\
            # Async utils\n\
            futures = {\"0.3.31\"}\n\
            \n\
            ## Async features dependencies\n\
            # Async connection pooling\n\
            deadpool-postgres = {\"0.14.1\", optional = true }
        ",
        ),
        _ => deps_toml.push_str(
            "\n\
            ## Sync client dependencies\n\
            # Postgres sync client\n\
            postgres = {\"0.19.9\", features = [] }
        ",
        ),
    };
    match (config.r#async, dependency_analysis.chrono) {
        (true, true) => deps_toml.push_str(
            "\n\
            # Postgres async client\n\
            tokio-postgres = {\"0.7.12\", default-features = false, features = [\"with-chrono-0_4\",\"with-time-0_3\"]}\n\
        ",
        ),
        _ => deps_toml.push_str(
            "\n\
            # Postgres async client\n\
            tokio-postgres = {\"0.7.12\", default-features = false, features = []}\n\
        ",
        ),
    };

    match config.workspace {
        true => {
            let re = Regex::new(r#"\{"(\d+(\.\d+)*)""#).unwrap();
            let deps_toml = deps_toml
                .lines()
                .skip(1)
                .filter(|line| !line.trim_start().starts_with('#'))
                .collect::<Vec<&str>>()
                .join("\n");

            let deps_toml = re.replace_all(&deps_toml, "{ workspace = true ");
            deps_toml.to_string()
        }
        _ => {
            let re = Regex::new(r#"\{"(.*?)\""#).unwrap();
            let deps_toml = re.replace_all(&deps_toml, |caps: &regex::Captures| {
                let matched_number = &caps[1];
                format!("{{version = \"{}\"", matched_number)
            });
            deps_toml.to_string()
        }
    }
}
