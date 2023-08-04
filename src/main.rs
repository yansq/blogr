use lazy_static::lazy_static;
use std::{env, time::Instant};
use structopt::StructOpt;
use tera::Tera;

#[macro_use]
extern crate log;

mod build;
mod cli;
mod server;

use build::rebuild_site;
use cli::{Action::*, CommandLineArgs};
use server::start_server;

const CONTENT_DIR: &str = "content";
const PUBLIC_DIR: &str = "public";

// tera template
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                error!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

fn main() {
    init_log();

    let CommandLineArgs { action } = CommandLineArgs::from_args();

    match action {
        Build => {
            info!("Start building...");
            let _start = Instant::now();
            if let Err(e) = rebuild_site(CONTENT_DIR, PUBLIC_DIR) {
                error!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
            info!("Building success!");
        }
        Server => {
            if let Err(e) = rebuild_site(CONTENT_DIR, PUBLIC_DIR) {
                error!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
            info!("Start server...");
            if let Err(e) = start_server() {
                error!("Server error(s): {}", e);
                std::process::exit(1);
            }
        }
        Test => info!("test success"),
    };
}

fn init_log() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
}
