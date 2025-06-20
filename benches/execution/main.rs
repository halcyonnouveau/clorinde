use std::fmt::Write;

use clorinde::conn::clorinde_conn;
use criterion::{BenchmarkId, Criterion};
use diesel::{Connection, PgConnection};
use postgres::{Client, fallible_iterator::FallibleIterator};
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Runtime;

mod clorinde_benches;
mod diesel_benches;
mod postgres_benches;
mod sqlx_benches;
mod tokio_postgres_benches;

const QUERY_SIZE: &[usize] = &[1, 100, 10_000];
const INSERT_SIZE: &[usize] = &[1, 100, 1000];

fn clear(client: &mut Client) {
    client
    .batch_execute("TRUNCATE TABLE comments CASCADE;TRUNCATE TABLE posts CASCADE;TRUNCATE TABLE users CASCADE").unwrap();
}

fn prepare_client(
    size: usize,
    client: &mut Client,
    hair_color_init: impl Fn(usize) -> Option<&'static str>,
) {
    clear(client);
    let mut query = String::from("INSERT INTO users (name, hair_color) VALUES");
    let mut params = Vec::with_capacity(2 * size);

    for x in 0..size {
        write!(
            &mut query,
            "{} (${}, ${})",
            if x == 0 { "" } else { "," },
            2 * x + 1,
            2 * x + 2
        )
        .unwrap();
        params.push((format!("User {x}"), hair_color_init(x)));
    }

    let params = params
        .iter()
        .flat_map(|(a, b)| [a as _, b as _])
        .collect::<Vec<_>>();

    client.execute(&query, &params).unwrap();
}

fn prepare_full(client: &mut Client) {
    prepare_client(100, client, |i| {
        Some(if i % 2 == 0 { "black" } else { "brown" })
    });

    let user_ids = client
        .query_raw("SELECT id FROM users", std::iter::empty::<u32>())
        .unwrap()
        .map(|row| Ok(row.get("id")))
        .collect::<Vec<i32>>()
        .unwrap();

    let data = user_ids
        .iter()
        .flat_map(|user_id| {
            (0..10).map(move |i| (format!("Post {i} by user {user_id}"), user_id, None))
        })
        .collect::<Vec<_>>();

    let mut insert_query = String::from("INSERT INTO posts(title, user_id, body) VALUES");

    for x in 0..data.len() {
        write!(
            insert_query,
            "{} (${}, ${}, ${})",
            if x == 0 { "" } else { "," },
            3 * x + 1,
            3 * x + 2,
            3 * x + 3
        )
        .unwrap();
    }

    let data = data
        .iter()
        .flat_map(|(title, user_id, body): &(_, _, Option<String>)| {
            [title as _, user_id as _, body as _]
        })
        .collect::<Vec<_>>();

    client.execute(&insert_query as &str, &data).unwrap();

    let all_posts = client
        .query_raw("SELECT id FROM posts", std::iter::empty::<u32>())
        .unwrap()
        .map(|row| Ok(row.get("id")))
        .collect::<Vec<i32>>()
        .unwrap();

    let data = all_posts
        .iter()
        .flat_map(|post_id| {
            (0..10).map(move |i| (format!("Comment {i} on post {post_id}"), post_id))
        })
        .collect::<Vec<_>>();

    let mut insert_query = String::from("INSERT INTO comments(text, post_id) VALUES");

    for x in 0..data.len() {
        write!(
            insert_query,
            "{} (${}, ${})",
            if x == 0 { "" } else { "," },
            2 * x + 1,
            2 * x + 2,
        )
        .unwrap();
    }

    let data = data
        .iter()
        .flat_map(|(title, post_id)| [title as _, post_id as _])
        .collect::<Vec<_>>();

    client.execute(&insert_query, &data).unwrap();
}

fn bench(c: &mut Criterion) {
    clorinde::container::cleanup(false).ok();
    clorinde::container::setup(false, "docker.io/library/postgres:latest", 250).unwrap();

    let clorinde_client = &mut clorinde_conn().unwrap();
    let rt: &'static Runtime = Box::leak(Box::new(Runtime::new().unwrap()));

    let sync_client = &mut postgres::Client::connect(
        "postgresql://postgres:postgres@127.0.0.1:5435/postgres",
        postgres::NoTls,
    )
    .unwrap();

    let async_client = &mut rt.block_on(async {
        let (client, conn) = tokio_postgres::connect(
            "postgresql://postgres:postgres@127.0.0.1:5435/postgres",
            tokio_postgres::NoTls,
        )
        .await
        .unwrap();
        rt.spawn(conn);
        client
    });

    let diesel_conn =
        &mut PgConnection::establish("postgresql://postgres:postgres@127.0.0.1:5435/postgres")
            .unwrap();

    let sqlx_pool = &mut rt.block_on(async {
        PgPoolOptions::new()
            .max_connections(1)
            .connect("postgresql://postgres:postgres@127.0.0.1:5435/postgres")
            .await
            .unwrap()
    });

    clorinde::load_schema(clorinde_client, &["benches/schema.sql"]).unwrap();

    {
        let mut group = c.benchmark_group("bench_trivial_query");
        for size in QUERY_SIZE {
            prepare_client(*size, sync_client, |_| None);
            group.bench_function(BenchmarkId::new("diesel", size), |b| {
                diesel_benches::bench_trivial_query(b, diesel_conn)
            });
            group.bench_function(BenchmarkId::new("sqlx", size), |b| {
                sqlx_benches::bench_trivial_query(b, sqlx_pool, rt)
            });
            group.bench_function(BenchmarkId::new("postgres", size), |b| {
                postgres_benches::bench_trivial_query(b, sync_client);
            });
            group.bench_function(BenchmarkId::new("tokio_postgres", size), |b| {
                tokio_postgres_benches::bench_trivial_query(b, async_client);
            });
            group.bench_function(BenchmarkId::new("clorinde", size), |b| {
                clorinde_benches::sync::bench_trivial_query(b, sync_client);
            });
            group.bench_function(BenchmarkId::new("clorinde_async", size), |b| {
                clorinde_benches::bench_trivial_query(b, clorinde_client);
            });
        }
        group.finish();
    }
    {
        let mut group = c.benchmark_group("bench_medium_complex_query");
        for size in QUERY_SIZE {
            prepare_client(*size, sync_client, |i| {
                Some(if i % 2 == 0 { "black" } else { "brown" })
            });
            group.bench_function(BenchmarkId::new("diesel", size), |b| {
                diesel_benches::bench_medium_complex_query(b, diesel_conn)
            });
            group.bench_function(BenchmarkId::new("sqlx", size), |b| {
                sqlx_benches::bench_medium_complex_query(b, sqlx_pool, rt)
            });
            group.bench_function(BenchmarkId::new("postgres", size), |b| {
                postgres_benches::bench_medium_complex_query(b, sync_client);
            });
            group.bench_function(BenchmarkId::new("tokio_postgres", size), |b| {
                tokio_postgres_benches::bench_medium_complex_query(b, async_client);
            });
            group.bench_function(BenchmarkId::new("clorinde", size), |b| {
                clorinde_benches::sync::bench_medium_complex_query(b, sync_client);
            });
            group.bench_function(BenchmarkId::new("clorinde_async", size), |b| {
                clorinde_benches::bench_medium_complex_query(b, clorinde_client);
            });
        }
        group.finish();
    }
    {
        let mut group = c.benchmark_group("bench_loading_associations_sequentially");
        prepare_full(sync_client);
        group.bench_function("diesel", |b| {
            diesel_benches::loading_associations_sequentially(b, diesel_conn)
        });
        group.bench_function("sqlx", |b| {
            sqlx_benches::loading_associations_sequentially(b, sqlx_pool, rt)
        });
        group.bench_function("postgres", |b| {
            postgres_benches::loading_associations_sequentially(b, sync_client)
        });
        group.bench_function("tokio_postgres", |b| {
            tokio_postgres_benches::loading_associations_sequentially(b, async_client);
        });
        group.bench_function("clorinde", |b| {
            clorinde_benches::sync::loading_associations_sequentially(b, sync_client)
        });
        group.bench_function("clorinde_async", |b| {
            clorinde_benches::loading_associations_sequentially(b, clorinde_client)
        });
        group.finish();
    }
    {
        let mut group = c.benchmark_group("bench_insert");
        for size in INSERT_SIZE {
            group.bench_with_input(BenchmarkId::new("diesel", size), size, |b, i| {
                clear(sync_client);
                diesel_benches::bench_insert(b, diesel_conn, *i)
            });
            group.bench_with_input(BenchmarkId::new("sqlx", size), size, |b, i| {
                clear(sync_client);
                sqlx_benches::bench_insert(b, sqlx_pool, rt, *i)
            });
            group.bench_with_input(BenchmarkId::new("postgres", size), size, |b, i| {
                clear(sync_client);
                postgres_benches::bench_insert(b, sync_client, *i);
            });
            group.bench_with_input(BenchmarkId::new("tokio_postgres", size), size, |b, i| {
                tokio_postgres_benches::bench_insert(b, async_client, *i);
            });
            group.bench_with_input(BenchmarkId::new("clorinde", size), size, |b, i| {
                clear(sync_client);
                clorinde_benches::sync::bench_insert(b, sync_client, *i);
            });
            group.bench_with_input(BenchmarkId::new("clorinde_async", size), size, |b, i| {
                clear(sync_client);
                clorinde_benches::bench_insert(b, clorinde_client, *i);
            });
        }
        group.finish();
    }

    clorinde::container::cleanup(false).unwrap();
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
