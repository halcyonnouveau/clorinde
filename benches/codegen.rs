use std::{path::PathBuf, str::FromStr};

use clorinde::{config::Config, conn::clorinde_conn};
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    clorinde::container::cleanup(false).ok();
    clorinde::container::setup(false, "docker.io/library/postgres:latest", 250).unwrap();
    let client = &mut clorinde_conn().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    clorinde::load_schema(client, &["tests/codegen/schema.sql"]).unwrap();

    let cfg = Config::builder()
        .queries(PathBuf::from_str("tests/codegen/queries").unwrap())
        .destination(tmp.keep())
        .sync(true)
        .r#async(true)
        .derive_traits(vec!["serde::Serialize".to_string()]);

    c.bench_function("codegen_sync", |b| {
        b.iter(|| clorinde::gen_live(client, cfg.clone().build()).unwrap())
    });

    let cfg = cfg.sync(false).r#async(false);

    c.bench_function("codegen_async", |b| {
        b.iter(|| clorinde::gen_live(client, cfg.clone().build()).unwrap())
    });

    clorinde::container::cleanup(false).unwrap();
}
criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
