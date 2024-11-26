use criterion::{criterion_group, criterion_main, Criterion, black_box};

#[derive(Debug, Clone, PartialEq)]
pub struct WrongStruct {
    pub data: f64,
    pub patch: u8
}

impl std::ops::Mul for WrongStruct {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: self.data * other.data,
            patch: self.patch
        }
    }
}

fn single_float_arithmetics(c: &mut Criterion) {
    let f_f64: f64 = 1.0;
    let f_f32: f32 = 1.0;

    let mut group = c.benchmark_group("Float arithmetics");

    group.bench_function("f32 multiplication 1_000_000 times", |b| b.iter(|| 
        for _ in 0..1_000_000 {
            let _ = black_box(f_f32) * black_box(2.0);
        }
    ));

    group.bench_function("f64 multiplication 1_000_000 times ", |b| b.iter(|| 
        for _ in 0..1_000_000 {
            let _ = black_box(f_f64) * black_box(2.0);
        }
    ));

    group.finish();
}


criterion_group!(benches, single_float_arithmetics);
criterion_main!(benches);