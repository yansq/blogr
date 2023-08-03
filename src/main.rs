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
            let _ = rebuild_site(CONTENT_DIR, PUBLIC_DIR);
            info!("Building success!");
        }
        Server => {
            let _ = rebuild_site(CONTENT_DIR, PUBLIC_DIR);
            println!("Start server...");
            let _ = start_server();
        }
        Test => println!("test success"),
    };

    println!("Hello, world!");
}
