use crate::highlighter::highlighter::{HighlightElement, HighlightKind};
use crate::theme::theme::Theme;

fn process_highlight(highlight: &HighlightElement, text: &str, theme: &Theme) -> String {
    let highlight_text = &text[highlight.start..highlight.end];
    match highlight.kind {
        HighlightKind::Keyword => {
            format!(
                "<span style=\"color:{}\">{}</span>",
                theme.keyword.to_css(),
                highlight_text
            )
        }
        HighlightKind::Function => {
            format!(
                "<span style=\"color:{}\">{}</span>",
                theme.function.to_css(),
                highlight_text
            )
        }
        HighlightKind::String_ => {
            format!(
                "<span style=\"color:{}\">{}</span>",
                theme.string.to_css(),
                highlight_text
            )
        }
        HighlightKind::Comment => {
            format!(
                "<span style=\"color:{}\">{}</span>",
                theme.comment.to_css(),
                highlight_text
            )
        }
    }
}

pub fn render_html(highlights: &[HighlightElement], text: &str, theme: &Theme) -> String {
    let mut parts: Vec<String> = Vec::new();

    let html_start = format!(
        "<pre style=\"background:{}; display: inline-block; padding:20px; min-width:500px;\"><code style=\"color:{}; font-family:consolas; font-size:15px\">",
        theme.background.to_css(),
        theme.main_text.to_css()
    );
    parts.push(html_start.replace("\t", "    "));

    let mut prev_end = 0;

    for highlight in highlights {
        if highlight.start > 0 {
            parts.push(
                text[prev_end..highlight.start]
                    .replace("\t", "    ")
                    .to_owned(),
            );
        }
        prev_end = highlight.end;
        let highlight_html = process_highlight(highlight, text, theme);
        parts.push(highlight_html);
    }
    if highlights.len() > 0 {
        let last_highlight_end = highlights[highlights.len() - 1].end;
        if last_highlight_end < text.len() {
            parts.push(text[last_highlight_end..].replace("\t", "    ").to_owned());
        }
    }
    let html_end = String::from("</code></pre>");
    parts.push(html_end);
    parts.join("")
}
