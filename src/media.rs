use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Media {
    pub kind: String,
    pub label: String,
    pub url: String,
    #[serde(default)]
    pub preview_url: Option<String>,
    pub source_url: String,
}

pub fn gallery(items: &[Media], locale: &str) -> String {
    if items.is_empty() {
        return String::new();
    }
    let cards = items.iter().filter(|item| !super::safe_link(&item.url).is_empty()).map(|item| {
        let label = super::escape_html(&item.label);
        let video = item.kind == "video";
        let action = if video && item.url.starts_with("https://www.youtube.com/") { super::i18n::tr(locale, "watch_youtube") } else if video { super::i18n::tr(locale, "open_video") } else { super::i18n::tr(locale, "open_image") };
        let preview = item.preview_url.as_deref().filter(|url| !super::safe_link(url).is_empty())
            .map(|url| format!("<img src=\"{}\" alt=\"{}\" loading=\"lazy\" decoding=\"async\" referrerpolicy=\"no-referrer\" width=\"640\" height=\"360\">", super::escape_html(url), label))
            .unwrap_or_default();
        let play = if video { "<span class=\"media-play\" aria-hidden=\"true\"><svg class=\"ui-icon\"><use href=\"#icon-play\" xlink:href=\"#icon-play\"></use></svg></span>" } else { "" };
        let source = super::safe_link(&item.source_url);
        let provenance = if source.starts_with("https://github.com/") { super::i18n::tr(locale, "project_readme") } else { super::i18n::tr(locale, "project_website") };
        format!("<figure class=\"media-card\"><a class=\"media-preview\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" aria-label=\"{}: {}\">{}{}</a><figcaption><a class=\"media-title\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}<span>{} <svg class=\"ui-icon ui-icon-small\" aria-hidden=\"true\"><use href=\"#icon-external\" xlink:href=\"#icon-external\"></use></svg></span></a><a class=\"media-provenance\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a></figcaption></figure>",
            super::escape_html(&item.url), action, label, preview, play, super::escape_html(&item.url), label, action,
            super::escape_html(&source), provenance)
    }).collect::<String>();
    let title = super::i18n::tr(locale, "media_section");
    format!("<section class=\"media-section\" aria-label=\"{}\"><h4>{}</h4><div class=\"media-gallery\">{}</div></section>", title, title, cards)
}
