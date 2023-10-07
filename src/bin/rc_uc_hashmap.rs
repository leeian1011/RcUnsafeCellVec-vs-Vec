use timer::register_logging;
use timer::ExampleCell;
use timer::ExampleObject;

use std::{collections::HashMap, rc::Rc, time::UNIX_EPOCH};
fn main() {
    let file_appender = tracing_appender::rolling::minutely("./logs/", "rc-uc-hashmap.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let _ = register_logging(non_blocking);

    let mut arc_uc_hashmap: HashMap<usize, Rc<ExampleCell<Vec<ExampleObject>>>> = HashMap::new();

    for i in 0..100 {
        arc_uc_hashmap.insert(i, ExampleObject::generate_rr_vec(100));
    }

    loop {
        let start = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        for i in 0..100 {
            let _ = arc_uc_hashmap.get(&i).unwrap().clone();
        }
        let end = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        tracing::info!("{}", end - start);
    }
}
