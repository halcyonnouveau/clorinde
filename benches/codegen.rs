use std::{path::PathBuf, str::FromStr};

use clorinde::{config::Config, conn::clorinde_conn};
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    clorinde::container::cleanup(false).ok();
    clorinde::container::setup(false).unwrap();
    let client = &mut clorinde_conn().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    clorinde::load_schema(client, &["../test_codegen/schema.sql"]).unwrap();

    let mut cfg = Config::default();

    cfg.queries = PathBuf::from_str("../test_codegen/queries").unwrap();
    cfg.destination = tmp.into_path();
    cfg.sync = true;
    cfg.r#async = false;
    cfg.serialize = true;

    c.bench_function("codegen_sync", |b| {
        b.iter(|| clorinde::gen_live(client, cfg.clone()).unwrap())
    });

    cfg.sync = false;
    cfg.r#async = true;

    c.bench_function("codegen_async", |b| {
        b.iter(|| clorinde::gen_live(client, cfg.clone()).unwrap())
    });
    clorinde::container::cleanup(false).unwrap();
}
criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
