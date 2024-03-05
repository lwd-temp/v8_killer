use clap::*;
use tracing::*;
use tracing_subscriber::fmt::time::uptime;

use std::env::current_exe;

use v8_killer_launcher::{default_lib_filename, launch};

/// A simple launcher/injector for V8 Killer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Custom dynamic library to inject
    #[arg(long)]
    lib_name: Option<String>,
    /// Custom configuration file, will pass to the executable by environment variable `V8_KILLER_CONFIG_FILE_PATH`
    #[arg(long)]
    config: Option<String>,
    /// The executable to launch and inject dynamic library
    executable: String,
    /// The arguments for the executable
    #[arg(last = true)]
    arguments: Vec<String>,
}

fn main() {
    tracing_subscriber::fmt().with_timer(uptime()).init();
    let args = Arguments::parse();
    if let Some(config) = &args.config {
        std::env::set_var("V8_KILLER_CONFIG_FILE_PATH", config);
    }
    let lib_filename = if let Some(lib_name) = args.lib_name {
        lib_name
    } else if let Ok(lib_name) = default_lib_filename() {
        lib_name.to_owned()
    } else {
        error!("Can't get default dynamic library name, your platform may not be supported.");
        error!("You can try to specify the dynamic library manually by setting the `--lib-name` argument.");
        std::process::exit(1)
    };
    let lib_path = current_exe().unwrap().parent().unwrap().join(lib_filename);
    let lib_path_str = lib_path.to_str().unwrap();

    info!("Executable: {}", args.executable);
    info!("Args: {:?}", args.arguments);
    info!("Core lib path: {}", lib_path_str);
    launch(lib_path_str, &args.executable, args.arguments.as_slice());
}
