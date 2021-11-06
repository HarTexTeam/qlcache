use criterion::Criterion;
use qlcache::{
    ql::QueryBuilder,
    QlCache
};

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("create_schema_bench", |b| {
        let cache = QlCache::new();

        b.iter(|| {
            cache
                .execute(
                    QueryBuilder::create()
                        .schema()
                        .if_not_exist()
                        .name(String::from("SchemaName"))
                        .build()
                        .unwrap()
                )
                .unwrap()
        });
    });
}

criterion::criterion_group!(benches, benchmark);
criterion::criterion_main!(benches);
