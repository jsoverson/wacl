use std::path::PathBuf;
use structopt::StructOpt;
use wasmcloud_host::{HostBuilder, Result};

use log::info;

#[derive(StructOpt, Debug, Clone)]
struct Cli {
    /// Verbose logging
    #[structopt(long = "verbose", short = "v")]
    verbose: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Actor command name
    #[structopt()]
    command: String,

    /// JSON data
    #[structopt(default_value = "\"\"")]
    data: String,
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let cli: Cli = Cli::from_args();
    let verbose_options = if cli.verbose {
        "wasmcloud=info,wasmcloud_host=info,wacl=info"
    } else {
        ""
    };
    let _ = env_logger::Builder::from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, verbose_options),
    )
    .format_module_path(true)
    .try_init();

    start_host(cli).await
}

async fn start_host(args: Cli) -> Result<()> {
    info!("Starting wasmcloud host");
    let host_builder = HostBuilder::new();
    let host = host_builder.build();

    info!("Reading wasm file {}", args.input.to_string_lossy());
    let actor = wasmcloud_host::Actor::from_file(args.input)?;
    let name = actor.name();
    let key = actor.public_key();
    info!("Loaded actor {}", name);

    host.start().await?;
    host.start_actor(actor).await?;

    let json_string = args.data;
    let json: serde_json::value::Value = serde_json::from_str(&json_string)?;
    info!("Calling actor {}::{} ({})", name, args.command, key);
    let messagebytes = serdeconv::to_msgpack_vec(&json)?;
    let result = host.call_actor(&key, &args.command, &messagebytes).await?;
    let parser_result = serdeconv::from_msgpack_slice::<serde_json::value::Value>(&result)?;
    println!("{}", serde_json::to_string(&parser_result)?);
    Ok(())
}
