/// The actual implementation of the `Nop` preprocessor. This would usually go
/// in your main `lib.rs` file.
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use serde_yaml::Value;
use std::borrow::Cow;

/// A no-op preprocessor.
pub struct Nop;

impl Nop {
    pub fn new() -> Nop {
        Nop
    }
}

fn collect_lines(lines: Vec<String>) -> String {
    lines
        .into_iter()
        .map(|mut s| {
            s.push('\n');
            s
        })
        .collect()
}

impl Preprocessor for Nop {
    fn name(&self) -> &str {
        "nop-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        //
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                let mut lines: Vec<String> =
                    chapter.content.lines().map(|s| s.to_owned()).collect();
                if lines.len() > 0 && lines[0] == "---" {
                    let end_yaml = lines.iter().skip(1).position(|s| s == "---").map(|x| x + 1);
                    if let Some(end_yaml) = end_yaml {
                        let value: Value =
                            serde_yaml::from_str(&collect_lines(lines[1..end_yaml].to_vec()))
                                .expect("Deserialization failed!");

                        let title = value
                            .get("title")
                            .expect("Title not found")
                            .as_str()
                            .expect("Title is not String");
                        let mut new_title_line = "# ".to_string();
                        new_title_line.push_str(title);
                        lines.splice(0..end_yaml + 1, vec![new_title_line]);

                        lines.iter_mut().skip(1).for_each(|line| {
                            if line.starts_with("### ") {
                                line.replace_range(0..4, "**");
                                line.push_str("**");
                            } else if line.starts_with("##") {
                                line.replace_range(0..2, "###")
                            } else if line.starts_with("#") {
                                line.replace_range(0..1, "##")
                            }
                        });
                    }
                }

                chapter.content = collect_lines(lines);
            }
        });

        // we *are* a no-op preprocessor after all
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
