use std::{
    cell::UnsafeCell,
    collections::HashMap,
    str::FromStr,
    sync::{Arc, OnceLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use models::ExampleCell;
use tracing_appender::non_blocking::NonBlocking;

mod models;
static LOG_FILENAME: OnceLock<String> = OnceLock::new();

fn main() {
    LOG_FILENAME.get_or_init(|| String::from("normalhashmap.log"));
    let file_appender = tracing_appender::rolling::minutely("./logs/", LOG_FILENAME.get().unwrap());
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let _ = register_logging(non_blocking);

    let mut normal_hashmap: HashMap<usize, Vec<models::ExampleObject>> = HashMap::new();

    for i in 0..100 {
        normal_hashmap.insert(i, models::ExampleObject::generate_vec(100));
    }

    std::thread::spawn(move || loop {
        let start = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        for i in 0..100 {
            let _ = normal_hashmap.get(&i).unwrap().clone();
        }
        let end = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        tracing::info!("{}", end - start);
    });

    let mut arc_uc_hashmap: HashMap<usize, Arc<ExampleCell<Vec<models::ExampleObject>>>> =
        HashMap::new();

    for i in 0..100 {
        arc_uc_hashmap.insert(i, models::ExampleObject::generate_rr_vec(100));
    }

    std::thread::spawn(move || loop {
        let start = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        for i in 0..100 {
            let _ = arc_uc_hashmap.get(&i).unwrap().clone();
        }
        let end = UNIX_EPOCH.elapsed().unwrap().as_nanos();
        tracing::info!("RC/UNSAFECELL HashMap => {}", end - start);
    });

    std::thread::sleep(Duration::from_secs(5));
}

fn register_logging(writer: NonBlocking) -> anyhow::Result<()> {
    use tracing_subscriber::{
        fmt::Layer, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
    };

    let log_mode = match std::env::var("LOG_MODE") {
        Ok(mode) => mode,
        Err(_) => {
            println!("environment variable `LOG_MODE` not found, use default value 'local'.");
            String::from("local")
        }
    };

    let log_level = match std::env::var("RUST_LOG") {
        Ok(input) => match tracing::Level::from_str(&input) {
            Ok(level) => level,
            Err(_) => {
                eprintln!("invalid environment variable `RUST_LOG` ({input}), fallback to use default value 'INFO'.");
                tracing::Level::INFO
            }
        },
        Err(_) => {
            println!("environment variable `RUST_LOG` not found, use default value 'INFO'.");
            tracing::Level::INFO
        }
    };

    println!("using `RUST_LOG` {}", log_level);

    match log_mode.to_lowercase().as_str() {
        "local" => {
            tracing_subscriber::registry()
                .with(Layer::new().with_writer(std::io::stdout))
                .with(
                    Layer::new()
                        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
                        .with_writer(writer)
                        .json(),
                )
                .with(
                    EnvFilter::builder()
                        .with_default_directive(log_level.into())
                        .with_env_var("LOG_LEVEL")
                        .from_env_lossy()
                        .add_directive("rdkafka=off".parse()?)
                        .add_directive("librdkafka=off".parse()?),
                )
                .init();
        }
        "cloud" => {
            // construct a subscriber that prints formatted traces to stdout
            let subscriber = tracing_subscriber::FmtSubscriber::builder()
                // .with_max_level(TRACING_LEVEL.get().into())
                .with_thread_ids(true)
                .with_thread_names(true)
                .json()
                .finish();

            // use that subscriber to process traces emitted after this point
            match tracing::subscriber::set_global_default(subscriber) {
                Ok(_) => {}
                Err(_) => panic!("Failed to subscribe to tracing logs"),
            };
        }
        _ => unreachable!(
            "Invalid LOG_MODE: {} (only 'local' and 'cloud' are valid option)",
            log_mode
        ),
    }

    Ok(())
}
