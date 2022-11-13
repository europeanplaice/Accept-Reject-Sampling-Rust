
use rand::prelude::*;

pub fn acceptance_rejection_sampling(target_fn: fn(x: f64) -> f64, n_sample: u32) -> Vec<f64>{
    let mut rng = thread_rng();

    let mut accepted = vec![];
    for _ in 0..n_sample {
        let x = rng.gen();
        let u = rng.gen();
        let r = target_fn(x);
        if r < u {
            accepted.push(r);
        }
    }
    return accepted;
    
}
