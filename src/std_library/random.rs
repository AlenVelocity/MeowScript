extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct SimpleRandom {
    rng: rand::rngs::ThreadRng,
}

impl SimpleRandom {
    pub fn new() -> Self {
        SimpleRandom { rng: rand::thread_rng() }
    }

    pub fn random(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }

    pub fn uniform(&mut self, a: f64, b: f64) -> f64 {
        self.rng.gen_range(a..b)
    }

    pub fn randint(&mut self, a: i64, b: i64) -> i64 {
        self.rng.gen_range(a..=b)
    }

    pub fn choice<T: Clone>(&mut self, seq: &[T]) -> T {
        seq.choose(&mut self.rng).unwrap().clone()
    }

    pub fn shuffle<T>(&mut self, x: &mut Vec<T>) {
        x.shuffle(&mut self.rng);
    }
}


