use serde::Deserialize;
use std::collections::BTreeMap;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlSelectElement};

#[derive(Clone, Deserialize)]
pub struct ConsoleHardware {
    pub id: String,
    pub name: String,
    pub maker_id: String,
    pub maker: String,
    pub system_type: String,
    #[serde(default)]
    pub release_year: Option<i32>,
    pub asset_path: String,
    pub source_page_url: String,
    pub source_image_url: String,
    pub license: String,
    #[serde(default)]
    pub license_url: Option<String>,
    pub creator: String,
    pub attribution: String,
    pub alt: String,
    pub verified_at: String,
    #[serde(default)]
    pub maker_logo_path: Option<String>,
    #[serde(default)]
    pub maker_logo_page_url: Option<String>,
    #[serde(default)]
    pub maker_logo_license: Option<String>,
    #[serde(default)]
    pub maker_logo_license_url: Option<String>,
    #[serde(default)]
    pub maker_logo_creator: Option<String>,
    #[serde(default)]
    pub maker_logo_attribution: Option<String>,
    pub source_logo_path: String,
    pub source_logo_label: String,
    #[serde(default)]
    pub source_logo_page_url: Option<String>,
    #[serde(default)]
    pub source_logo_license: Option<String>,
    #[serde(default)]
    pub source_logo_license_url: Option<String>,
    #[serde(default)]
    pub source_logo_creator: Option<String>,
    #[serde(default)]
    pub source_logo_attribution: Option<String>,
    #[serde(default)]
    pub asset_changes: Option<String>,
}

pub fn populate_controls(
    document: &Document,
    items: &[ConsoleHardware],
    locale: &str,
    selected_maker: &str,
) {
    let Some(select) = document
        .get_element_by_id("filter-console-maker")
        .and_then(|element| element.dyn_into::<HtmlSelectElement>().ok())
    else {
        return;
    };
    let makers = items
        .iter()
        .map(|item| (item.maker_id.clone(), item.maker.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut options = format!(
        "<option value=\"\">{}</option>",
        super::escape_html(super::i18n::tr(locale, "console_any_maker"))
    );
    for (id, name) in makers {
        options.push_str(&format!(
            "<option value=\"{}\">{}</option>",
            super::escape_html(&id),
            super::escape_html(&name)
        ));
    }
    select.set_inner_html(&options);
    if !selected_maker.is_empty() {
        select.set_value(selected_maker);
    }
}

pub fn render(
    document: &Document,
    items: &[ConsoleHardware],
    locale: &str,
    query: &str,
    selected_maker: &str,
    selected_type: &str,
) {
    let has_gallery = !items.is_empty();
    if let Some(controls) = document.get_element_by_id("console-gallery-controls") {
        if has_gallery {
            let _ = controls.remove_attribute("hidden");
        } else {
            let _ = controls.set_attribute("hidden", "");
        }
    }
    let normalized_query = query.trim().to_lowercase();
    let filtered = items
        .iter()
        .filter(|item| selected_maker.is_empty() || item.maker_id == selected_maker)
        .filter(|item| selected_type.is_empty() || item.system_type == selected_type)
        .filter(|item| {
            normalized_query.is_empty()
                || format!(
                    "{} {} {}",
                    item.name,
                    item.maker,
                    item.release_year.unwrap_or_default()
                )
                .to_lowercase()
                .contains(&normalized_query)
        })
        .collect::<Vec<_>>();

    if let Some(count) = document.get_element_by_id("console-gallery-count") {
        if has_gallery {
            let _ = count.remove_attribute("hidden");
            count.set_text_content(Some(&format!(
                "{} {}",
                filtered.len(),
                super::i18n::tr(locale, "console_gallery_count")
            )));
        } else {
            let _ = count.set_attribute("hidden", "");
            count.set_text_content(None);
        }
    }
    let html = if filtered.is_empty() {
        if has_gallery {
            format!(
                "<p class=\"console-gallery-empty\">{}</p>",
                super::i18n::tr(locale, "console_gallery_empty")
            )
        } else {
            format!(
                "<div class=\"console-gallery-empty\" role=\"status\">{}<p>{}</p></div>",
                super::icon_markup("image"),
                super::i18n::tr(locale, "console_gallery_preparing")
            )
        }
    } else {
        filtered
            .iter()
            .map(|item| card_markup(item, locale))
            .collect::<String>()
    };
    if let Some(container) = document.get_element_by_id("console-gallery-items") {
        container.set_inner_html(&html);
    }
}

fn card_markup(item: &ConsoleHardware, locale: &str) -> String {
    let image_path = safe_asset(&item.asset_path, "assets/consoles/");
    let image_src = if image_path.is_empty() {
        String::new()
    } else {
        format!("./{image_path}")
    };
    let preview_src = console_preview_asset(&image_path)
        .map(|path| format!("./{path}"))
        .unwrap_or_else(|| image_src.clone());
    let source_page = super::safe_link(&item.source_page_url);
    let original = super::safe_link(&item.source_image_url);
    let source_logo_path = safe_asset(&item.source_logo_path, "assets/source-logos/");
    let maker_logo_path = item
        .maker_logo_path
        .as_deref()
        .map(|path| safe_asset(path, "assets/maker-logos/"))
        .unwrap_or_default();
    let maker_logo = if maker_logo_path.is_empty() {
        format!("<span>{}</span>", super::escape_html(&item.maker))
    } else {
        format!(
            "<img src=\"./{}\" alt=\"{}\" loading=\"lazy\">",
            super::escape_html(&maker_logo_path),
            super::escape_html(&item.maker)
        )
    };
    let source_logo = if source_logo_path.is_empty() || source_page.is_empty() {
        String::new()
    } else {
        format!(
            "<a class=\"console-source-mark\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" aria-label=\"{}: {}\"><img src=\"./{}\" alt=\"\" loading=\"lazy\"></a>",
            super::escape_html(&source_page),
            super::escape_html(super::i18n::tr(locale, "console_photo_source")),
            super::escape_html(&item.source_logo_label),
            super::escape_html(&source_logo_path)
        )
    };
    let maker_credit = match (
        item.maker_logo_page_url.as_deref(),
        item.maker_logo_license.as_deref(),
    ) {
        (Some(page), Some(license)) if !maker_logo_path.is_empty() => {
            let page = super::safe_link(page);
            let maker_creator = item.maker_logo_creator.as_deref().unwrap_or(&item.maker);
            if page.is_empty() {
                String::new()
            } else {
                format!(
                    "<p><strong>{}</strong> {} · {}</p>",
                    super::i18n::tr(locale, "console_maker_mark"),
                    super::escape_html(maker_creator),
                    linked_license(license, item.maker_logo_license_url.as_deref())
                ) + &item
                    .maker_logo_attribution
                    .as_deref()
                    .map(|text| format!("<p>{}</p>", super::escape_html(text)))
                    .unwrap_or_default()
            }
        }
        _ => String::new(),
    };
    let source_logo_credit = match (
        item.source_logo_page_url.as_deref(),
        item.source_logo_license.as_deref(),
    ) {
        (Some(page), Some(license)) => {
            let page = super::safe_link(page);
            let creator = item
                .source_logo_creator
                .as_deref()
                .unwrap_or(&item.source_logo_label);
            if page.is_empty() {
                String::new()
            } else {
                format!(
                    "<p><strong>{}</strong> {} · {}</p><p><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a></p><p>{}</p>",
                    super::escape_html(&item.source_logo_label),
                    super::escape_html(creator),
                    linked_license(license, item.source_logo_license_url.as_deref()),
                    super::escape_html(&page),
                    super::escape_html(&item.source_logo_label),
                    super::escape_html(item.source_logo_attribution.as_deref().unwrap_or(""))
                )
            }
        }
        _ => String::new(),
    };
    let source_credit = if source_page.is_empty() || image_src.is_empty() || original.is_empty() {
        String::new()
    } else {
        let photo_license = linked_license(&item.license, item.license_url.as_deref());
        format!(
            "<details class=\"console-gallery-credit\"><summary>{} · {}</summary><p><strong>{}</strong> {}</p><p><strong>{}</strong> {}</p><p><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a></p><p>{}</p><p><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a></p><p>{}</p><p><strong>{}</strong> {}</p>{}{}</details>",
            super::i18n::tr(locale, "console_photo_credit"),
            super::i18n::tr(locale, "console_photo_license"),
            super::i18n::tr(locale, "console_photo_creator"),
            super::escape_html(&item.creator),
            super::i18n::tr(locale, "console_photo_license"),
            photo_license,
            super::escape_html(&source_page),
            super::i18n::tr(locale, "console_photo_source"),
            super::escape_html(&item.source_logo_label),
            super::escape_html(&item.attribution),
            super::escape_html(&original),
            super::i18n::tr(locale, "console_original_image"),
            super::icon_markup("external"),
            item.asset_changes.as_deref().map(super::escape_html).unwrap_or_default(),
            super::i18n::tr(locale, "console_verified"),
            super::escape_html(&item.verified_at),
            maker_credit,
            source_logo_credit
        )
    };
    let kind_key = match item.system_type.as_str() {
        "home" => "console_type_home",
        "handheld" => "console_type_handheld",
        "hybrid" => "console_type_hybrid",
        "add_on" => "console_type_addon",
        _ => "console_type_other",
    };
    let year = item
        .release_year
        .map(|year| year.to_string())
        .unwrap_or_else(|| super::i18n::tr(locale, "console_year_unknown").to_string());
    format!(
        "<article class=\"console-gallery-card\" data-console-id=\"{}\"><div class=\"console-gallery-photo\"><img src=\"{}\" alt=\"{}\" loading=\"lazy\" decoding=\"async\" fetchpriority=\"low\"> <span class=\"console-maker-mark\">{}</span>{}</div><div class=\"console-gallery-caption\"><h3>{}</h3><div class=\"console-gallery-meta\"><span>{}</span><span>{}</span></div><p class=\"console-gallery-maker-name\">{}</p>{}</div></article>",
        super::escape_html(&item.id),
        super::escape_html(&preview_src),
        super::escape_html(&item.alt),
        maker_logo,
        source_logo,
        super::escape_html(&item.name),
        year,
        super::i18n::tr(locale, kind_key),
        super::escape_html(&item.maker),
        source_credit
    )
}

fn console_preview_asset(original: &str) -> Option<String> {
    let file_name = original.strip_prefix("assets/consoles/")?;
    let (stem, extension) = file_name.rsplit_once('.')?;
    if !matches!(
        extension.to_ascii_lowercase().as_str(),
        "jpg" | "jpeg" | "png"
    ) {
        return None;
    }
    let path = format!("assets/console-previews/{stem}.jpg");
    if safe_asset(&path, "assets/console-previews/").is_empty() {
        None
    } else {
        Some(path)
    }
}

fn linked_license(label: &str, url: Option<&str>) -> String {
    let Some(url) = url.map(super::safe_link).filter(|url| !url.is_empty()) else {
        return super::escape_html(label);
    };
    format!(
        "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
        super::escape_html(&url),
        super::escape_html(label)
    )
}

fn safe_asset(value: &str, prefix: &str) -> String {
    if !value.starts_with(prefix)
        || value.contains("..")
        || !value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '/' | '-' | '_' | '.')
        })
    {
        return String::new();
    }
    value.to_string()
}
