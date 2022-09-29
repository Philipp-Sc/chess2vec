use chess2vec::core::get_keys;

use env_logger::Env;
/*
RUST_LOG=error,debug,info cargo run --example test
*/
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(
        Env::default().default_filter_or(format!("{}=info", env!("CARGO_PKG_NAME"))),
    ).init();
    log::info!("Running `main()`");
    let keys = get_keys();
    log::info!("{:?}",keys);
    Ok(())
}