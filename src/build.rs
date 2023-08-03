use std::fs;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::TEMPLATES;

#[derive(serde::Serialize)]
struct IndexItem {
    permalink: String,
    title: String,
}

/// If you delete a markdown file, it's link will be removed from index, but the generated HTML file in public
/// directory will not be deleted.
pub fn rebuild_site(content_dir: &str, output_dir: &str) -> Result<(), anyhow::Error> {
    if !Path::new(&output_dir).exists() {
        fs::create_dir(output_dir)?;
    }

    // get markdown files
    let markdown_files: Vec<String> = WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .collect();

    let mut html_files: Vec<String> = Vec::with_capacity(markdown_files.len());

    for file in &markdown_files {
        let public_path = file
            .replace(content_dir, output_dir)
            .replace(".md", ".html");
        if Path::new(&public_path).exists() {
            let content_modified = fs::metadata(file).unwrap().modified().unwrap();
            let public_modified = fs::metadata(&public_path).unwrap().modified().unwrap();
            if content_modified > public_modified {
                generate_blog(file, &public_path);
            }
        } else {
            generate_blog(file, &public_path);
        }

        html_files.push(public_path);
    }

    generate_index(&html_files, output_dir)?;
    Ok(())
}

fn generate_blog(file: &str, public_path: &str) {
    // parse markdowns into htmls content
    let markdown = fs::read_to_string(file).unwrap();
    let parser = pulldown_cmark::Parser::new(&markdown);
    let mut content = String::new();
    pulldown_cmark::html::push_html(&mut content, parser);

    // put contents into template
    let mut context = tera::Context::new();
    context.insert("title", &get_title(file));
    context.insert("content", &content);
    context.insert("date", "");
    let rendered = TEMPLATES.render("blog-page.html", &context);

    // write to file
    let folder = Path::new(public_path).parent().unwrap();
    let _ = fs::create_dir_all(folder);
    fs::write(public_path, rendered.unwrap()).unwrap();
}

fn generate_index(files: &[String], output_dir: &str) -> Result<(), anyhow::Error> {
    let index_list: Vec<IndexItem> = files
        .iter()
        .map(|f| {
            let f = f.trim_start_matches(output_dir);
            let title = get_title(f);
            IndexItem {
                permalink: f.to_string(),
                title,
            }
        })
        .collect();
    let mut context = tera::Context::new();
    context.insert("pages", &index_list);
    let rendered = TEMPLATES.render("index.html", &context);
    fs::write(format!("{}/index.html", output_dir), rendered.unwrap())?;
    Ok(())
}

fn get_title(file: &str) -> String {
    let path = PathBuf::from(file);
    String::from(path.file_stem().and_then(|s| s.to_str()).unwrap())
}
