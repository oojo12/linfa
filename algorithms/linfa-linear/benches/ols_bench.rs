use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use linfa::traits::Fit;
use linfa_datasets::generate::make_dataset;
use linfa_linear::{LinearRegression, TweedieRegressor};
use statrs::distribution::{DiscreteUniform, Laplace};
use linfa::Dataset;
use ndarray::Ix1;

#[allow(unused_must_use)]
fn perform_ols(dataset: &Dataset<f64, f64, Ix1>) {
    let model = LinearRegression::new();
    model.fit(dataset);
}

#[allow(unused_must_use)]
fn perform_glm(dataset: &Dataset<f64, f64, Ix1>) {
    let model = TweedieRegressor::params().power(0.).alpha(0.);
    model.fit(dataset);
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Linfa_linear");
    let sizes: [usize; 3] = [1_000, 10_000, 100_000];
    let num_feats: [usize; 2] = [5, 10];

    let feat_distr = Laplace::new(0.5, 5.).unwrap();
    let target_distr = DiscreteUniform::new(0, 5).unwrap();

    let ols_id = "OLS-".to_string();
    let glm_id = "GLM-".to_string();

    for size in sizes {
        for num_feat in num_feats {
            let suffix = format!("{}Feats", num_feat);
            let mut func_name = ols_id.clone();
            func_name.push_str(&suffix);
            
            let dataset = make_dataset(size, num_feat, 1, feat_distr, target_distr);
            let dataset = dataset.into_single_target();

            group.bench_with_input(
                BenchmarkId::new(&func_name, size),
                &dataset,
                |b, dataset| {
                    b.iter(|| perform_ols(&dataset));
                },
            );

            let mut func_name = glm_id.clone();
            func_name.push_str(&suffix);
            group.bench_with_input(
                BenchmarkId::new(&func_name, size),
                &dataset,
                |b, dataset| {
                    b.iter(|| perform_glm(&dataset));
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
