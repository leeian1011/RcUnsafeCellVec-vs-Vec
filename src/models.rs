use rand::{thread_rng, Rng};
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub enum ExampleEnum {
    One,
    Two,
    Three,
}

impl ExampleEnum {
    fn gen_random() -> Self {
        let rand = thread_rng().gen_range(0..2);
        match rand {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            _ => unreachable!("???"),
        }
    }
}

pub enum ExampleEnumTwo {
    Four,
    Five,
}

impl ExampleEnumTwo {
    fn gen_random() -> Self {
        let rand = thread_rng().gen_bool(0.5);
        match rand {
            true => Self::Four,
            false => Self::Five,
        }
    }
}

pub struct ExampleCell<T> {
    celled: UnsafeCell<T>,
}

unsafe impl<ExampleObject> Sync for ExampleCell<Vec<ExampleObject>> {}

impl<T> ExampleCell<T> {
    pub fn new(data: T) -> Self {
        Self {
            celled: UnsafeCell::new(data),
        }
    }
}

pub struct ExampleObject {
    x: f64,
    y: ExampleEnum,
    z: ExampleEnumTwo,
}

impl ExampleObject {
    pub fn generate_vec(count: usize) -> Vec<Self> {
        let mut vector: Vec<ExampleObject> = vec![];
        for _ in 0..count {
            vector.push(Self::generate_random());
        }

        vector
    }

    pub fn generate_rr_vec(count: usize) -> Arc<ExampleCell<Vec<Self>>> {
        let mut vector: Vec<ExampleObject> = vec![];
        for _ in 0..count {
            vector.push(Self::generate_random());
        }

        Arc::new(ExampleCell::new(vector))
    }

    pub fn generate_random() -> Self {
        let rand: f64 = thread_rng().gen();
        Self {
            x: rand,
            y: ExampleEnum::gen_random(),
            z: ExampleEnumTwo::gen_random(),
        }
    }
}
