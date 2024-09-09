//! Really dirty, but a low-effort way to get the Readme as html

use convert_case::Case;
use markdown::Options;
use std::cmp::Ordering;
use std::fmt::Write;
use std::path::Path;

pub fn generate_html_consts(markdown_file: &Path) -> String {
    let md_content = std::fs::read_to_string(markdown_file).unwrap();
    let html = markdown::to_html_with_options(&md_content, &Options::gfm()).unwrap();
    let mut headings = Vec::new();
    let mut writing_headings = Vec::new();
    let mut headings_depth = 0;
    for line in html.lines() {
        if line.starts_with("<h") {
            let h = line.chars().nth(2).unwrap().to_digit(10).unwrap();
            match h.cmp(&headings_depth) {
                Ordering::Equal => {
                    let written = writing_headings.pop().unwrap();
                    headings.push(written);
                }
                Ordering::Less => {
                    while headings_depth >= h {
                        let written = writing_headings.pop().unwrap();
                        headings.push(written);
                        headings_depth -= 1;
                    }
                    headings_depth += 1;
                }
                Ordering::Greater => {
                    headings_depth += 1;
                }
            }
            writing_headings.push(String::new());
        }
        for wh in &mut writing_headings {
            wh.write_fmt(format_args!("{line}\n")).unwrap();
        }
    }
    for h in writing_headings {
        headings.push(h);
    }
    let mut name_to_content = Vec::new();
    for h in headings {
        let first = h.lines().next().unwrap();
        let hdr = first.get(4..first.len() - 5).unwrap();
        name_to_content.push((hdr.to_string(), h));
    }
    generate_parts(name_to_content)
}

fn generate_parts(mut parts: Vec<(String, String)>) -> String {
    parts.sort();
    let to_scream = convert_case::Converter::new().to_case(Case::ScreamingSnake);
    let mut out = "//! Generated code, do not edit\n\n".to_string();
    for (name, content) in parts {
        let const_name = to_scream
            .convert(name)
            .replace(['Ä', 'Å'], "A")
            .replace('Ö', "O")
            .replace('/', "_");
        let (hash_start, hash_end) = if content.contains('"') {
            ("r#", "#")
        } else {
            ("", "")
        };
        out.write_fmt(format_args!(
            "pub const {const_name}: &str = {hash_start}\"{content}\"{hash_end};\n\n"
        ))
        .unwrap();
    }
    out
}
