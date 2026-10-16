use serde::Deserialize;
use web_sys::Document;

#[derive(Clone, Deserialize)]
pub struct ProcessResource {
    pub id: String,
    pub position: usize,
    pub title: String,
    pub kind: String,
    pub publisher_author: String,
    pub topic_key: String,
    pub description: String,
    pub url: String,
    pub source_url: String,
    pub source_evidence: String,
    pub recommendation_rationale: String,
    pub popularity_platform: String,
    pub popularity_value: String,
    pub popularity_unit: String,
    #[serde(default)]
    pub popularity_secondary_value: Option<String>,
    #[serde(default)]
    pub popularity_secondary_unit: Option<String>,
    pub popularity_checked_on: String,
    pub popularity_evidence: String,
    pub evidence_url: String,
}

pub fn render(document: &Document, items: &[ProcessResource], locale: &str) {
    let html = items
        .iter()
        .filter_map(|item| card_markup(item, locale))
        .collect::<String>();
    if let Some(container) = document.get_element_by_id("process-resource-cards") {
        container.set_inner_html(&html);
    }
}

fn card_markup(item: &ProcessResource, locale: &str) -> Option<String> {
    let kind_key = match item.kind.as_str() {
        "video" => "resource_kind_video",
        "article" => "resource_kind_article",
        "course" => "resource_kind_course",
        _ => return None,
    };
    let topic_key = match item.topic_key.as_str() {
        "decompilation" => "resource_topic_decompilation",
        "static_recompilation" => "resource_topic_static_recompilation",
        "source_port" => "resource_topic_source_port",
        "api_translation" => "resource_topic_api_translation",
        _ => return None,
    };
    let platform_key = match item.popularity_platform.as_str() {
        "youtube" => "resource_platform_youtube",
        "github" => "resource_platform_github",
        _ => return None,
    };
    let unit_key = match item.popularity_unit.as_str() {
        "views" => "resource_unit_views",
        "stars" => "resource_unit_stars",
        _ => return None,
    };
    let metric_value = match (
        item.popularity_platform.as_str(),
        item.popularity_unit.as_str(),
    ) {
        ("youtube", "views") => {
            if item.popularity_secondary_unit.as_deref() != Some("likes")
                || item
                    .popularity_secondary_value
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
            {
                return None;
            }
            format!(
                "{} {} · {} {}",
                super::escape_html(&item.popularity_value),
                super::i18n::tr(locale, unit_key),
                super::escape_html(item.popularity_secondary_value.as_deref()?),
                super::i18n::tr(locale, "resource_unit_likes")
            )
        }
        ("github", "stars") => {
            if item.popularity_secondary_unit.is_some() || item.popularity_secondary_value.is_some()
            {
                return None;
            }
            format!(
                "{} {}",
                super::escape_html(&item.popularity_value),
                super::i18n::tr(locale, unit_key)
            )
        }
        _ => return None,
    };
    let url = super::safe_link(&item.url);
    let source = super::safe_link(&item.source_url);
    let evidence = super::safe_link(&item.evidence_url);
    if url.is_empty() || source.is_empty() || evidence.is_empty() || item.title.trim().is_empty() {
        return None;
    }
    let checked = super::format_date(&item.popularity_checked_on, locale);
    Some(format!(
        "<article class=\"process-resource-card\" data-resource-id=\"{}\" data-resource-position=\"{}\"><div class=\"process-resource-topline\"><span class=\"process-resource-topic\">{}</span><span class=\"process-resource-kind\">{}</span></div><h3 class=\"process-resource-title\"><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a></h3><p class=\"process-resource-byline\">{} {}</p><p class=\"process-resource-description\">{}</p><div class=\"process-resource-popularity\"><span class=\"process-resource-metric\">{} · {}</span><small>{} {}</small></div><p class=\"process-resource-rationale\"><strong>{}</strong> {}</p><details class=\"process-resource-provenance\"><summary>{}</summary><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a><p>{}</p><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a><p>{}</p></details></article>",
        super::escape_html(&item.id),
        item.position,
        super::i18n::tr(locale, topic_key),
        super::i18n::tr(locale, kind_key),
        super::escape_html(&url),
        super::escape_html(&item.title),
        super::i18n::tr(locale, "resource_by"),
        super::escape_html(&item.publisher_author),
        super::escape_html(&item.description),
        super::i18n::tr(locale, platform_key),
        metric_value,
        super::i18n::tr(locale, "resource_checked"),
        super::escape_html(&checked),
        super::i18n::tr(locale, "resource_recommended_because"),
        super::escape_html(&item.recommendation_rationale),
        super::i18n::tr(locale, "resource_evidence"),
        super::escape_html(&source),
        super::i18n::tr(locale, "resource_primary_source"),
        super::icon_markup("external"),
        super::escape_html(&item.source_evidence),
        super::escape_html(&evidence),
        super::i18n::tr(locale, "resource_popularity_evidence"),
        super::icon_markup("external"),
        super::escape_html(&item.popularity_evidence)
    ))
}
