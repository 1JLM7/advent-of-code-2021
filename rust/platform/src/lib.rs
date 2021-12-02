use std::{fmt::Debug, path::PathBuf, str::FromStr};
use structopt::StructOpt;
use tracing::Instrument;

pub use anyhow;
use tokio::runtime::Builder;

pub trait Challenge {
    fn stage1(self, data: String) -> anyhow::Result<()>;
    fn stage2(self, data: String) -> anyhow::Result<()>;
}

#[derive(Debug)]
enum Stage {
    Stage1,
    Stage2,
}

impl FromStr for Stage {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "stage1" => Self::Stage1,
            "stage2" => Self::Stage2,
            _ => anyhow::bail!("Cannot parse stage from {:?}", s),
        })
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::Stage1
    }
}

#[derive(Debug, StructOpt)]
struct Options {
    /// File pointing to the challenge data file
    #[structopt(name = "data file")]
    filename: PathBuf,
    /// Stage to run
    #[structopt(short, long, default_value = "stage1")]
    challenge: Stage,
}

#[tracing::instrument]
pub fn run<C: 'static + Debug + Challenge + Send>(challenge: C) -> anyhow::Result<()> {
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .or_else(|_| tracing_subscriber::EnvFilter::try_new("info"))
                .unwrap(),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false));
    tracing_log::LogTracer::init().unwrap_or_else(|err| {
        tracing::error!("Couldn't set up logger: {}", err);
    });

    let rt = Builder::new_current_thread()
        .thread_name("aoc-platform-thread")
        .max_blocking_threads(1)
        .build()?;
    let args: Options = Options::from_args();
    rt.block_on(async {
        let data = tokio::fs::read_to_string(args.filename)
            .instrument(tracing::trace_span!("read-data"))
            .await?;
        match args.challenge {
            Stage::Stage1 => {
                rt.spawn_blocking(move || challenge.stage1(data))
                    .instrument(tracing::trace_span!("exec-challenge"))
                    .await?
            }
            Stage::Stage2 => {
                rt.spawn_blocking(move || challenge.stage2(data))
                    .instrument(tracing::trace_span!("exec-challenge"))
                    .await?
            }
        }
    })
}

#[macro_export]
macro_rules! challenge {
    ($c: expr) => {
        fn main() -> $crate::anyhow::Result<()> {
            $crate::run($c)
        }
    };
}
