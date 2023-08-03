use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::TEMPLATES;

#[derive(serde::Serialize)]
struct IndexItem {
    permalink: String,
    title: String,
}

pub fn rebuild_site(content_dir: &str, output_dir: &str) -> Result<(), anyhow::Error> {
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

        generate_index(&html_files, output_dir)?;
    }

    Ok(())
}

fn generate_index(files: &[String], output_dir: &str) -> Result<(), anyhow::Error> {
    let index_list: Vec<IndexItem> = files
        .iter()
        .map(|f| {
            let f = f.trim_start_matches(output_dir);
            let title = f.trim_start_matches('/').trim_end_matches(".html");
            IndexItem {
                permalink: f.to_string(),
                title: title.to_string(),
            }
        })
        .collect();
    let mut context = tera::Context::new();
    context.insert("pages", &index_list);
    let rendered = TEMPLATES.render("index.html", &context);
    fs::write(format!("{}/index.html", output_dir), rendered.unwrap())?;
    Ok(())
}
