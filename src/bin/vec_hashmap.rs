use std::{collections::HashMap, time::UNIX_EPOCH};

use timer::register_logging;
use timer::ExampleObject;

fn main() {
    let file_appender = tracing_appender::rolling::minutely("./logs/", "normal_hashmap.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let _ = register_logging(non_blocking);

    let mut normal_hashmap: HashMap<usize, Vec<ExampleObject>> = HashMap::new();

    for i in 0..100 {
        normal_hashmap.insert(i, ExampleObject::generate_vec(100));
    }

    loop {
        let start = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        for i in 0..100 {
            let _ = normal_hashmap.get(&i).unwrap().clone();
        }
        let end = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        tracing::info!("{}", end - start);
    }
}
