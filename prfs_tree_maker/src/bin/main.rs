use chrono::prelude::*;
use dotenv::dotenv;
use halo2_gadgets::{
    poseidon::{
        primitives::{self as poseidon, ConstantLength, P128Pow5T3 as OrchardNullifier, Spec},
        Hash,
    },
    utilities::UtilitiesInstructions,
};
use halo2_proofs::halo2curves::pasta::Fp;
use hyper::{body::HttpBody as _, Client, Uri};
use hyper::{Body, Method, Request, Response};
use hyper_tls::HttpsConnector;
use prfs_tree_maker::{
    addresses,
    config::{END_BLOCK, GETH_ENDPOINT, START_BLOCK},
    TreeMakerError,
};
use std::fs::{File, OpenOptions};
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{info, instrument::WithSubscriber, metadata::LevelFilter};
use tracing_subscriber::{
    fmt::{format::Writer, time::FormatTime},
    prelude::__tracing_subscriber_SubscriberExt,
    EnvFilter, Layer,
};

pub(crate) struct MockTime;
impl FormatTime for MockTime {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let time = Utc::now().format("%y-%m-%d %H:%M:%S");
        write!(w, "{}", time)
    }
}

#[tokio::main]
async fn main() -> Result<(), TreeMakerError> {
    std::env::set_var("RUST_LOG", "info");

    let now = Utc::now();
    println!("Tree maker starts");
    println!("start time: {}", now);

    {
        let dotenv_path = dotenv()?;
        println!(".env path: {:?}", dotenv_path);
    }

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let log_files_path = project_root.join(format!("log_files"));
    println!("log file path: {:?}", log_files_path);
    println!("geth endpoint: {}", GETH_ENDPOINT);

    if log_files_path.exists() == false {
        File::create(&log_files_path).unwrap();
    }

    let _guard = {
        let mut layers = Vec::new();

        let console_log_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_timer(MockTime)
            .with_filter(EnvFilter::from_default_env())
            .with_filter(LevelFilter::INFO)
            .boxed();

        layers.push(console_log_layer);

        let log_dir = project_root.join("log_files");
        std::fs::create_dir_all(&log_dir)?;

        let file_appender = tracing_appender::rolling::daily(&log_dir, "tree_maker.log");

        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let file_log_layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .with_target(false)
            .with_timer(MockTime)
            .with_ansi(false)
            .with_filter(EnvFilter::from_default_env())
            .boxed();

        layers.push(file_log_layer);

        println!(
            "File logger is attached. Log files will be periodically rotated. log dir: {}",
            log_dir.to_string_lossy(),
        );

        println!("Following log invocation will be handled by global logger");

        let subscriber = tracing_subscriber::registry().with(layers);

        tracing::subscriber::set_global_default(subscriber)
            .expect("Unable to set a global collector");

        tracing::info!("log info");
        tracing::warn!("log warn");
        tracing::error!("log error");

        tracing::info!("logging starts");

        _guard
    };

    addresses::get_addresses().await?;

    // grow::grow_tree().await?;
    // climb::climb_up().await?;

    Ok(())
}
