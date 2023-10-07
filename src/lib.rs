use rand::{thread_rng, Rng};
use std::cell::UnsafeCell;
use std::rc::Rc;
use std::str::FromStr;
use tracing_appender::non_blocking::NonBlocking;
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

    pub fn generate_rr_vec(count: usize) -> Rc<ExampleCell<Vec<Self>>> {
        let mut vector: Vec<ExampleObject> = vec![];
        for _ in 0..count {
            vector.push(Self::generate_random());
        }

        Rc::new(ExampleCell::new(vector))
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

pub fn register_logging(writer: NonBlocking) -> anyhow::Result<()> {
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
