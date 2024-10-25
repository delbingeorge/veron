use criterion::{black_box, criterion_group, criterion_main, Criterion};
use veron::Veron;

fn execute_simple_script(c: &mut Criterion) {
    let mut runtime = Veron::new();
    
    c.bench_function("simple_addition", |b| {
        b.iter(|| {
            runtime.execute(black_box("1 + 1"))
        })
    });
}

fn execute_complex_script(c: &mut Criterion) {
    let mut runtime = Veron::new();
    
    c.bench_function("object_operations", |b| {
        b.iter(|| {
            runtime.execute(black_box(r#"
                let obj = { x: 1, y: 2 };
                obj.x + obj.y;
            "#))
        })
    });
}

criterion_group!(benches, execute_simple_script, execute_complex_script);
criterion_main!(benches);