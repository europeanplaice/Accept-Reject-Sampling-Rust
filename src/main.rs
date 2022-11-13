use acceptance_rejection::acceptance_rejection_sampling;
use csv::Writer;

fn target_fn(x: f64) -> f64 {
    let r: f64 = x * (1. - x);
    return r;
}

fn main() {
    let sample = acceptance_rejection_sampling(target_fn, 1000);
}

