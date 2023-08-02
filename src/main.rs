use lazy_static::lazy_static;
use log::info;
use std::fs;
use std::path::Path;
use std::time::Instant;
use structopt::StructOpt;
use tera::Tera;
use walkdir::WalkDir;

mod cli;

use cli::{Action::*, CommandLineArgs};

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
        }
        Test => println!("test success"),
    };

    println!("Hello, world!");
}

fn rebuild_site(content_dir: &str, output_dir: &str) -> Result<(), anyhow::Error> {
    // delete old files
    let _ = fs::remove_dir_all(output_dir);

    // get markdowns
    let markdown_files: Vec<String> = WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .collect();

    let mut html_files: Vec<String> = Vec::with_capacity(markdown_files.len());

    for file in &markdown_files {
        // parse markdowns into htmls content
        let markdown = fs::read_to_string(file)?;
        let parser = pulldown_cmark::Parser::new(&markdown);
        let mut content = String::new();
        pulldown_cmark::html::push_html(&mut content, parser);

        // put contents into template
        let mut context = tera::Context::new();
        context.insert("title", file);
        context.insert("content", &content);
        context.insert("date", "");
        let rendered = TEMPLATES.render("blog-page.html", &context);

        // write to file
        let html_file = file
            .replace(content_dir, output_dir)
            .replace(".md", ".html");
        let folder = Path::new(&html_file).parent().unwrap();
        let _ = fs::create_dir_all(folder);
        fs::write(&html_file, rendered.unwrap())?;
        html_files.push(html_file);
    }

    Ok(())
}
