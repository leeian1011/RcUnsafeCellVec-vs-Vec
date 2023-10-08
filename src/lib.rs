use rand::{thread_rng, Rng};
use std::cell::UnsafeCell;
use std::rc::Rc;
use std::str::FromStr;
use tracing_appender::non_blocking::NonBlocking;

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

    pub fn generate_rr_vec(count: usize) -> Rc<UnsafeCell<Vec<Self>>> {
        let mut vector: Vec<ExampleObject> = vec![];
        for _ in 0..count {
            vector.push(Self::generate_random());
        }

        Rc::new(UnsafeCell::new(vector))
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
        Err(_) => String::from("local"),
    };

    let log_level = match std::env::var("RUST_LOG") {
        Ok(input) => match tracing::Level::from_str(&input) {
            Ok(level) => level,
            Err(_) => tracing::Level::INFO,
        },
        Err(_) => tracing::Level::INFO,
    };

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
        _ => unreachable!(
            "Invalid LOG_MODE: {} (only 'local' and 'cloud' are valid option)",
            log_mode
        ),
    }

    Ok(())
}
