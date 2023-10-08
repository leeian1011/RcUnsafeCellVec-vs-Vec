use std::env;
use std::{collections::HashMap, time::UNIX_EPOCH};

use timer::register_logging;
use timer::ExampleObject;
const ARGUMENT_COUNT: usize = 3;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != ARGUMENT_COUNT {
        println!("Usage: cargo r --release --bin vec [vector-size] [hashmap-entry-size]");
        panic!();
    }

    let vector_size: usize = arguments[1].clone().parse().expect("no vector size");

    let hashmap_entry_count: usize = arguments[2].clone().parse().expect("no hashmap size");
    if hashmap_entry_count < 1 {
        panic!("minimum entry size of 1");
    }

    let file_appender = tracing_appender::rolling::minutely("./logs/", "normal_hashmap.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let _ = register_logging(non_blocking);

    let mut normal_hashmap: HashMap<usize, Vec<ExampleObject>> = HashMap::new();

    for i in 0..hashmap_entry_count {
        normal_hashmap.insert(i, ExampleObject::generate_vec(vector_size));
    }

    loop {
        let start = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        for i in 0..hashmap_entry_count {
            let _ = normal_hashmap.get(&i).unwrap().clone();
        }
        let end = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        tracing::info!("{}", end - start);
    }
}
