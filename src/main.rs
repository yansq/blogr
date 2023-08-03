use lazy_static::lazy_static;
use log::info;
use std::time::Instant;
use structopt::StructOpt;
use tera::Tera;

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
                eprintln!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

fn main() {
    env_logger::init();

    let CommandLineArgs { action } = CommandLineArgs::from_args();

    match action {
        Build => {
            info!("Start building...");
            let _start = Instant::now();
            match rebuild_site(CONTENT_DIR, PUBLIC_DIR) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Parsing error(s): {}", e);
                    std::process::exit(1);
                }
            };
            info!("Building success!");
        }
        Server => {
            match rebuild_site(CONTENT_DIR, PUBLIC_DIR) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Parsing error(s): {}", e);
                    std::process::exit(1);
                }
            };
            println!("Start server...");
            match start_server() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Server error(s): {}", e);
                    std::process::exit(1);
                }
            };
        }
        Test => info!("test success"),
    };
}
