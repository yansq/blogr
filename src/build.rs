use anyhow::Result;
use pulldown_cmark::{Event, Tag};
use std::fs;
use std::path::{Path, PathBuf};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;

use crate::TEMPLATES;

#[derive(serde::Serialize)]
struct IndexItem {
    permalink: String,
    title: String,
}

/// If you delete a markdown file, it's link will be removed from index, but the generated HTML file in public
/// directory will not be deleted.
pub fn rebuild_site(content_dir: &str, output_dir: &str) -> Result<()> {
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
            let content_modified = fs::metadata(file)?.modified();
            let public_modified = fs::metadata(&public_path)?.modified();
            match (content_modified, public_modified) {
                (Ok(content_modified), Ok(public_modified)) => {
                    if content_modified > public_modified {
                        generate_blog(file, &public_path)?;
                    }
                }
                _ => {
                    generate_blog(file, &public_path)?;
                }
            }
        } else {
            generate_blog(file, &public_path)?;
        }

        html_files.push(public_path);
    }

    generate_index(&html_files, output_dir)?;
    copy_assets(output_dir)?;
    Ok(())
}

fn generate_blog(file: &str, public_path: &str) -> Result<()> {
    // parse markdowns into htmls content
    let markdown = fs::read_to_string(file)?;

    // for codeblock
    let mut is_codeblock = false;
    let mut accumulated_block = String::new();
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["Solarized (light)"];

    let mut events: Vec<Event<'_>> = Vec::new();
    let parser_iter = pulldown_cmark::Parser::new(&markdown).into_offset_iter();
    let mut token = String::from("rust");
    for (event, mut _range) in parser_iter {
        match event {
            Event::Start(Tag::CodeBlock(ref kind)) => {
                if let pulldown_cmark::CodeBlockKind::Fenced(fence) = kind {
                    token = fence.clone().into_string();
                }
                is_codeblock = true;
            }
            Event::Text(text) => {
                if is_codeblock {
                    accumulated_block += &text;
                } else {
                    events.push(Event::Text(text));
                }
            }
            Event::End(Tag::CodeBlock(_)) => {
                let syntax = ps
                    .find_syntax_by_token(&token)
                    .unwrap_or_else(|| ps.find_syntax_plain_text());
                if let Ok(h) = highlighted_html_for_string(&accumulated_block, &ps, syntax, theme) {
                    events.push(Event::Html(h.into()));
                }
                accumulated_block.clear();
                is_codeblock = false;
            }
            _ => events.push(event),
        }
    }

    let mut content = String::new();
    pulldown_cmark::html::push_html(&mut content, events.into_iter());

    // put contents into template
    let mut context = tera::Context::new();
    context.insert("title", &get_title(file));
    context.insert("content", &content);
    context.insert("date", "");
    let rendered = TEMPLATES.render("blog-page.html", &context)?;

    // write to file
    let folder = Path::new(public_path).parent().unwrap();
    let _ = fs::create_dir_all(folder);
    fs::write(public_path, rendered)?;
    Ok(())
}

fn generate_index(files: &[String], output_dir: &str) -> Result<()> {
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
    let rendered = TEMPLATES.render("index.html", &context)?;
    fs::write(format!("{}/index.html", output_dir), rendered)?;
    Ok(())
}

fn get_title(file: &str) -> String {
    let path = PathBuf::from(file);
    String::from(path.file_stem().and_then(|s| s.to_str()).unwrap())
}

fn copy_assets(output_dir: &str) -> Result<()> {
    let dest_path_str = format!("{}/assets", output_dir);
    let dest_path = Path::new(&dest_path_str);
    if dest_path.exists() {
        fs::remove_dir_all(dest_path)?;
    }
    fs::create_dir(dest_path)?;

    let src_path = Path::new("assets");
    if src_path.exists() {
        for entry in fs::read_dir(src_path)? {
            let entry = entry?;
            let src_file = entry.path();
            if src_file.is_file() {
                let dest_file = dest_path.join(entry.file_name());
                fs::copy(&src_file, &dest_file)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use syntect::highlighting::ThemeSet;
    use syntect::html::highlighted_html_for_string;
    use syntect::parsing::SyntaxSet;

    #[test]
    fn syntect_html() {
        // Load these once at the start of your program
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes["Solarized (light)"];

        let syntax = ps
            .find_syntax_by_token("rust")
            .unwrap_or_else(|| ps.find_syntax_plain_text());
        let s = "pub struct Wow { hi: u64 }\nfn blah() -> u64 {}\n";

        if let Ok(h) = highlighted_html_for_string(s, &ps, syntax, theme) {
            println!("{}", h);
        }
    }
}
