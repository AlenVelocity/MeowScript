extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;


pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("new"), Object::Inbuilt(new));
    globals.insert(String::from("random"), Object::Inbuilt(random));
    globals.insert(String::from("uniform"), Object::Inbuilt(uniform));
    globals.insert(String::from("randint"), Object::Inbuilt(randint));
    globals.insert(String::from("choice"), Object::Inbuilt(choice));
    globals.insert(String::from("shuffle"), Object::Inbuilt(shuffle));


    Res {
        globals,
        raw: None,
    }
}
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


