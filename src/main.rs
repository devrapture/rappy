mod cli;
mod utils;

mod constant;

use std::process;

use cli::CLiConfig;
use utils::logger::Logger;

fn main() {
    let config = match CLiConfig::run() {
        Ok(config) => config,
        Err(e) => {
            Logger::error(&format!("Error {}",e));
            process::exit(1)
        }
    };
}
