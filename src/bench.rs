use axum_test::TestServer;
use criterion::{Criterion, criterion_group, criterion_main};

fn create_criterion() -> Criterion {
    Criterion::default()
        .measurement_time(std::time::Duration::from_secs(60))
        .sample_size(1000)
}

pub fn bench_stuff_static(c: &mut Criterion) {
    let app = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(static_vs_dynamic::static_traits::router());
    let server = TestServer::new(app).unwrap();
    c.bench_function("stuff_static", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async { 
                let res = server.get("/stuff").await;
                assert!(res.status_code().is_success());
            });
    });
}

pub fn bench_stuff_dyn(c: &mut Criterion) {
    let app = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(static_vs_dynamic::dyn_traits::router());
    let server = TestServer::new(app).unwrap();
    c.bench_function("stuff_dyn", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async { 
                let res = server.get("/stuff").await;
                assert!(res.status_code().is_success());
            });
    });
}

criterion_group! {
    name = benches;
    config = create_criterion();
    targets = bench_stuff_static, bench_stuff_dyn
}
criterion_main!(benches);
