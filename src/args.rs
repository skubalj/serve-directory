//! Command line argument parsing

use std::path::PathBuf;
use std::process;

const HELP: &str = "\
serve-directory

Serve files in the specified directory and subdirectories

USAGE:
  serve-directory [OPTIONS] [FOLDER]

ARGS:
  [FOLDER]            The root directory that should be served (default: '.')

OPTIONS:
  -p, --port <PORT>   The port that the server should bind to (default: 8080)
  -l, --localhost     Deploy on localhost, rather than binding to the external interface
  -v, --verbose       Log additional data to the console
  -h, --help          Prints this help information
  -V, --version       Prints the version
";

pub struct Arguments {
    pub port: u16,
    pub verbose: bool,
    pub localhost: bool,
    pub folder: PathBuf,
}

impl Arguments {
    /// Parses the command line; prints the help or version and exits when asked, and prints the error and the help and exits on invalid input.
    pub fn from_env() -> Self {
        let mut args = pico_args::Arguments::from_env();

        if args.contains(["-h", "--help"]) {
            print!("{HELP}");
            process::exit(0);
        }

        if args.contains(["-V", "--version"]) {
            println!("serve-directory {}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }

        Self::parse_from(args).unwrap_or_else(|error| {
            eprintln!("{error}\n\n{HELP}");
            process::exit(1);
        })
    }

    fn parse_from(mut args: pico_args::Arguments) -> Result<Self, pico_args::Error> {
        let port = args.opt_value_from_str(["-p", "--port"])?.unwrap_or(8080);
        let verbose = args.contains(["-v", "--verbose"]);
        let localhost = args.contains(["-l", "--localhost"]);
        let folder: PathBuf = args.opt_free_from_str()?.unwrap_or(PathBuf::from("."));

        Ok(Self {
            port,
            verbose,
            localhost,
            folder,
        })
    }
}
