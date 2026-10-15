use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
use wasm_bindgen::{closure::Closure, prelude::*, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Document, Element, Event, HtmlInputElement, HtmlSelectElement, Response};

#[derive(Clone, Deserialize, Serialize)]
struct Project {
    id: String,
    title: String,
    game: String,
    platform: String,
    kind: String,
    status: String,
    summary: String,
    project_url: String,
    sources: Vec<Source>,
    #[serde(default)]
    updated: Option<String>,
    #[serde(default)]
    strategy: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    source_available: String,
    #[serde(default)]
    first_found: Option<String>,
    #[serde(default)]
    project_started: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Source {
    label: String,
    url: String,
}

#[derive(Deserialize)]
struct Catalog {
    updated: String,
    projects: Vec<Project>,
}

#[derive(Default)]
struct AppState {
    catalog: Option<Catalog>,
    query: String,
    kind: String,
    sort: String,
    platform: String,
    strategy: String,
    status: String,
    source_available: String,
    language: String,
    first_found_since: String,
    project_started_by: String,
    visible_limit: usize,
}

const SHELL: &str = r###"
<header class="site-header">
  <a class="brand" href="./" aria-label="Recompiled home"><span class="brand-mark" aria-hidden="true"><i></i><i></i><i></i></span>recompiled<span class="brand-period">.</span></a>
  <nav class="top-nav" aria-label="Main navigation"><a href="#catalog">Explore the index</a><a href="#about">How this is sourced</a><a class="github-link" href="#about">About the index <span aria-hidden="true">↘</span></a></nav>
</header>
<main>
  <section class="hero" aria-labelledby="hero-title">
    <div class="hero-copy"><p class="kicker"><span class="live-indicator"></span><span id="status-line">Preparing catalog</span></p>
      <h1 id="hero-title">Games have<br><span>second lives.</span></h1>
      <p class="hero-summary">A growing index of the people opening old games up: rebuilding source code, bringing binaries to new machines, and translating stories along the way.</p>
      <a class="hero-jump" href="#catalog">Find a project <span aria-hidden="true">↓</span></a>
    </div>
    <div class="hero-figure" aria-label="Catalog at a glance"><div class="orbit orbit-one"></div><div class="orbit orbit-two"></div><div class="orbit-core"><span>GAME</span><b>↗</b><span>AGAIN</span></div><div class="figure-note note-top">COMMUNITY BUILT</div><div class="figure-note note-bottom">SOURCE · PORT · LANGUAGE</div><div class="orbit-pin pin-one"></div><div class="orbit-pin pin-two"></div></div>
  </section>
  <section class="facts" aria-label="Index overview">
    <div class="fact"><strong id="project-total">—</strong><span>projects indexed</span></div><div class="fact"><strong id="game-total">—</strong><span>games represented</span></div><div class="fact"><strong id="source-total">—</strong><span>primary sources linked</span></div><p>Catalog refreshed <span id="catalog-updated">—</span></p>
  </section>
  <section class="catalog" id="catalog" aria-labelledby="catalog-title">
    <div class="catalog-heading"><div><p class="kicker">The index</p><h2 id="catalog-title">Browse the work</h2></div><p>Every listing leads back to its project and the references used to identify it.</p></div>
    <div class="catalog-tools"><label class="search-field"><span class="search-icon" aria-hidden="true"></span><span class="sr-only">Search projects</span><input id="search" type="search" placeholder="Search a game, platform, or project" autocomplete="off"><kbd>/</kbd></label><div class="sort-control"><label for="sort">Order</label><select id="sort"><option value="recent">Recently updated</option><option value="oldest">Oldest update</option><option value="title">Game title</option><option value="first-found">Recently found here</option></select></div></div>
    <div class="catalog-tabs" id="kind-tabs" role="group" aria-label="Filter by project type"><button data-kind="all" aria-pressed="true">Everything</button><button data-kind="Decompilation" aria-pressed="false">Decompilations</button><button data-kind="Recompilation" aria-pressed="false">Recompilations</button><button data-kind="Translation" aria-pressed="false">Translations</button><span id="result-count" aria-live="polite"></span></div>
    <details class="filter-drawer"><summary><span>More ways to explore</span><span class="filter-hint">Platform · method · evidence · dates · language</span></summary><div class="filter-grid">
      <label>Platform<select id="filter-platform" data-filter><option value="">Any platform</option></select></label>
      <label>Approach<select id="filter-strategy" data-filter><option value="">Any approach</option></select></label>
      <label>Project stage<select id="filter-status" data-filter><option value="">Any stage</option></select></label>
      <label>Source available<select id="filter-source" data-filter><option value="">Any source type</option></select></label>
      <label>Translation language<select id="filter-language" data-filter><option value="">Any language</option></select></label>
      <label>First found here, since<input id="filter-found" data-filter type="date" aria-label="First found here since"></label>
      <label>Project started by<input id="filter-started" data-filter type="date" aria-label="Project started by"></label>
      <button id="clear-filters" class="clear-filters" type="button">Clear filters</button>
    </div><p class="filter-footnote">“First found here” records when we added a project to this index. No primary-source project start dates are verified yet; that filter activates as they are added.</p></details>
    <div class="list-head" aria-hidden="true"><span>PROJECT TYPE</span><span>GAME / PROJECT</span><span>STATUS</span></div>
    <div id="projects" class="project-list" aria-live="polite"><div class="loading-state"><span></span>Loading the index…</div></div>
  </section>
  <section class="about" id="about"><div><p class="kicker">Receipts included</p><h2>Follow the work<br>to its source.</h2></div><div class="about-copy"><p>This index is built from project repositories, developer progress pages, and translation release pages. Each entry links to the original project and the references used to identify it, so you can check its status for yourself.</p><p>Listings describe code and patch projects only. They do not host game files, copyrighted assets, or ROMs. Uncertain project status stays labeled as uncertain.</p><a href="#about" class="source-note">How entries are sourced <span aria-hidden="true">↘</span></a></div></section>
</main>
<footer><a class="brand" href="./"><span class="brand-mark" aria-hidden="true"><i></i><i></i><i></i></span>recompiled<span class="brand-period">.</span></a><span>Made for the curious, by the curious.</span><span>Independent community index</span></footer>
"###;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or_else(|| JsValue::from_str("This page needs a browser document"))?;
    let root = document
        .get_element_by_id("app")
        .ok_or_else(|| JsValue::from_str("The page shell is missing its app root"))?;
    root.set_inner_html(SHELL);

    let state = Rc::new(RefCell::new(AppState {
        kind: "all".into(),
        sort: "recent".into(),
        visible_limit: 30,
        ..AppState::default()
    }));
    bind_controls(&document, &state)?;
    spawn_local(load_catalog(document, state));
    Ok(())
}

fn bind_controls(document: &Document, state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let search = document
        .get_element_by_id("search")
        .ok_or_else(|| JsValue::from_str("Search input is missing"))?;
    let state_for_search = Rc::clone(state);
    let document_for_search = document.clone();
    let search_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        if let Some(input) = event
            .target()
            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
        {
            state_for_search.borrow_mut().query = input.value();
            state_for_search.borrow_mut().visible_limit = 30;
            render(&document_for_search, &state_for_search.borrow());
        }
    });
    search.add_event_listener_with_callback("input", search_callback.as_ref().unchecked_ref())?;
    search_callback.forget();

    let tabs = document
        .get_element_by_id("kind-tabs")
        .ok_or_else(|| JsValue::from_str("Project filters are missing"))?;
    let state_for_tabs = Rc::clone(state);
    let document_for_tabs = document.clone();
    let tabs_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event.target().and_then(|target| target.dyn_into::<Element>().ok()) else { return; };
        let Ok(Some(button)) = target.closest("[data-kind]") else { return; };
        let Some(kind) = button.get_attribute("data-kind") else { return; };
        state_for_tabs.borrow_mut().kind = kind;
        state_for_tabs.borrow_mut().visible_limit = 30;
        if let Ok(buttons) = document_for_tabs.query_selector_all("[data-kind]") {
            for index in 0..buttons.length() {
                if let Some(item) = buttons.item(index).and_then(|item| item.dyn_into::<Element>().ok()) {
                    let selected = item.get_attribute("data-kind").as_deref() == Some(state_for_tabs.borrow().kind.as_str());
                    let _ = item.set_attribute("aria-pressed", if selected { "true" } else { "false" });
                }
            }
        }
        render(&document_for_tabs, &state_for_tabs.borrow());
    });
    tabs.add_event_listener_with_callback("click", tabs_callback.as_ref().unchecked_ref())?;
    tabs_callback.forget();

    let sort = document
        .get_element_by_id("sort")
        .ok_or_else(|| JsValue::from_str("Sort control is missing"))?;
    let state_for_sort = Rc::clone(state);
    let document_for_sort = document.clone();
    let sort_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        if let Some(select) = event
            .target()
            .and_then(|target| target.dyn_into::<HtmlSelectElement>().ok())
        {
            state_for_sort.borrow_mut().sort = select.value();
            state_for_sort.borrow_mut().visible_limit = 30;
            render(&document_for_sort, &state_for_sort.borrow());
        }
    });
    sort.add_event_listener_with_callback("change", sort_callback.as_ref().unchecked_ref())?;
    sort_callback.forget();

    let filters = document.query_selector_all("[data-filter]")?;
    for index in 0..filters.length() {
        let Some(control) = filters.item(index).and_then(|item| item.dyn_into::<Element>().ok()) else { continue; };
        let Some(id) = control.get_attribute("id") else { continue; };
        let state_for_filter = Rc::clone(state);
        let document_for_filter = document.clone();
        let filter_id = id.clone();
        let filter_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
            let Some(select) = event.target().and_then(|target| target.dyn_into::<HtmlSelectElement>().ok()) else {
                if let Some(input) = event.target().and_then(|target| target.dyn_into::<HtmlInputElement>().ok()) {
                    set_filter_value(&mut state_for_filter.borrow_mut(), &filter_id, input.value());
                    state_for_filter.borrow_mut().visible_limit = 30;
                    render(&document_for_filter, &state_for_filter.borrow());
                }
                return;
            };
            set_filter_value(&mut state_for_filter.borrow_mut(), &filter_id, select.value());
            state_for_filter.borrow_mut().visible_limit = 30;
            render(&document_for_filter, &state_for_filter.borrow());
        });
        control.add_event_listener_with_callback("change", filter_callback.as_ref().unchecked_ref())?;
        filter_callback.forget();
    }

    let clear = document.get_element_by_id("clear-filters").ok_or_else(|| JsValue::from_str("Clear filters button is missing"))?;
    let state_for_clear = Rc::clone(state);
    let document_for_clear = document.clone();
    let clear_callback = Closure::<dyn FnMut(Event)>::new(move |_| {
        let mut state = state_for_clear.borrow_mut();
        state.platform.clear(); state.strategy.clear(); state.status.clear(); state.source_available.clear();
        state.language.clear(); state.first_found_since.clear(); state.project_started_by.clear(); state.visible_limit = 30;
        drop(state);
        for id in ["filter-platform", "filter-strategy", "filter-status", "filter-source", "filter-language", "filter-found", "filter-started"] {
            if let Some(element) = document_for_clear.get_element_by_id(id) {
                if let Ok(select) = element.clone().dyn_into::<HtmlSelectElement>() { select.set_value(""); }
                if let Ok(input) = element.dyn_into::<HtmlInputElement>() { input.set_value(""); }
            }
        }
        render(&document_for_clear, &state_for_clear.borrow());
    });
    clear.add_event_listener_with_callback("click", clear_callback.as_ref().unchecked_ref())?;
    clear_callback.forget();

    let results = document
        .get_element_by_id("projects")
        .ok_or_else(|| JsValue::from_str("Project list is missing"))?;
    let state_for_more = Rc::clone(state);
    let document_for_more = document.clone();
    let more_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event.target().and_then(|target| target.dyn_into::<Element>().ok()) else { return; };
        if let Ok(Some(_)) = target.closest("[data-more]") {
            state_for_more.borrow_mut().visible_limit += 30;
            render(&document_for_more, &state_for_more.borrow());
        }
    });
    results.add_event_listener_with_callback("click", more_callback.as_ref().unchecked_ref())?;
    more_callback.forget();
    Ok(())
}

async fn load_catalog(document: Document, state: Rc<RefCell<AppState>>) {
    let result = async {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("Browser window is unavailable"))?;
        let response = JsFuture::from(window.fetch_with_str("./catalog.json")).await?;
        let response: Response = response.dyn_into()?;
        if !response.ok() {
            return Err(JsValue::from_str("Could not load the project index"));
        }
        let json = JsFuture::from(response.json()?).await?;
        let raw = js_sys::JSON::stringify(&json)?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Catalog response is not JSON text"))?;
        serde_json::from_str::<Catalog>(&raw)
            .map_err(|error| JsValue::from_str(&format!("Catalog data is invalid: {error}")))
    }.await;

    match result {
        Ok(catalog) => {
            populate_filters(&document, &catalog);
            state.borrow_mut().catalog = Some(catalog);
            set_text(&document, "status-line", "Catalog online");
            if let Some(kicker) = document.get_element_by_id("status-line") {
                if let Some(parent) = kicker.parent_element() { let _ = parent.class_list().add_1("is-live"); }
            }
            render(&document, &state.borrow());
        }
        Err(error) => {
            web_sys::console::error_1(&error);
            set_html(&document, "projects", "<div class=\"empty-state\"><h3>The catalog did not load</h3><p>Reload the page or check your connection.</p></div>");
            set_text(&document, "status-line", "Catalog unavailable");
        }
    }
}

fn render(document: &Document, state: &AppState) {
    let Some(catalog) = state.catalog.as_ref() else { return; };
    let needle = state.query.trim().to_lowercase();
    let mut projects: Vec<&Project> = catalog.projects.iter().filter(|project| {
        (state.kind == "all" || project.kind == state.kind)
            && (state.platform.is_empty() || project.platform == state.platform)
            && (state.strategy.is_empty() || project.strategy == state.strategy)
            && (state.status.is_empty() || status_bucket(&project.status) == state.status)
            && (state.source_available.is_empty() || project.source_available == state.source_available)
            && (state.language.is_empty() || project.language == state.language)
            && (state.first_found_since.is_empty() || project.first_found.as_deref().is_some_and(|value| value >= state.first_found_since.as_str()))
            && (state.project_started_by.is_empty() || project.project_started.as_deref().is_some_and(|value| value <= state.project_started_by.as_str()))
            && (needle.is_empty() || [
                project.title.as_str(), project.game.as_str(), project.platform.as_str(),
                project.summary.as_str(), project.kind.as_str(), project.status.as_str(), project.strategy.as_str(),
                project.language.as_str(), project.source_available.as_str(),
            ].iter().any(|field| field.to_lowercase().contains(&needle)))
    }).collect();

    match state.sort.as_str() {
        "oldest" => projects.sort_by(|a, b| a.updated.cmp(&b.updated)),
        "title" => projects.sort_by(|a, b| a.game.to_lowercase().cmp(&b.game.to_lowercase())),
        "first-found" => projects.sort_by(|a, b| b.first_found.cmp(&a.first_found)),
        _ => projects.sort_by(|a, b| b.updated.cmp(&a.updated)),
    }

    let game_count = catalog.projects.iter().map(|project| project.game.as_str()).collect::<BTreeSet<_>>().len();
    let source_count = catalog.projects.iter().flat_map(|project| project.sources.iter().map(|source| source.url.as_str())).collect::<BTreeSet<_>>().len();
    set_text(document, "project-total", &catalog.projects.len().to_string());
    set_text(document, "game-total", &game_count.to_string());
    set_text(document, "source-total", &source_count.to_string());
    set_text(document, "catalog-updated", &format_date(&catalog.updated));
    set_text(document, "result-count", &format!("{} {}", projects.len(), if projects.len() == 1 { "project" } else { "projects" }));

    let shown = projects.len().min(state.visible_limit);
    let html = if projects.is_empty() {
        "<div class=\"empty-state\"><h3>No projects found</h3><p>Try another title, platform, or project type.</p></div>".to_string()
    } else {
        let mut rows = projects.iter().take(shown).map(|project| project_markup(project)).collect::<String>();
        if shown < projects.len() {
            rows.push_str(&format!("<div class=\"load-more-row\"><button type=\"button\" data-more>Show more projects <span>({} left)</span></button></div>", projects.len() - shown));
        }
        rows
    };
    set_html(document, "projects", &html);
}

fn project_markup(project: &Project) -> String {
    let source_links = project.sources.iter().map(|source| format!(
        "<a href=\"{}\" target=\"_blank\" rel=\"noreferrer\">{}</a>",
        escape_html(&safe_link(&source.url)), escape_html(&source.label)
    )).collect::<String>();
    let updated = project.updated.as_deref().map(|date| format!("<small>Updated {}</small>", escape_html(&format_date(date)))).unwrap_or_default();
    let method = if project.strategy.is_empty() { String::new() } else { format!("<span class=\"metadata-chip\">{}</span>", escape_html(&project.strategy)) };
    let evidence = if project.source_available.is_empty() { String::new() } else { format!("<span class=\"metadata-chip\">{}</span>", escape_html(&project.source_available)) };
    let language = if project.language.is_empty() { String::new() } else { format!("<span class=\"metadata-chip\">{}</span>", escape_html(&project.language)) };
    let found = project.first_found.as_deref().map(|date| format!("<small>First found here {}</small>", escape_html(&format_date(date)))).unwrap_or_default();
    let started = project.project_started.as_deref().map(|date| format!("<small>Project page dated {}</small>", escape_html(&format_date(date)))).unwrap_or_default();
    format!(
        "<article class=\"project-row\"><div class=\"project-kind\"><span class=\"kind-mark kind-{}\"></span>{}</div><div class=\"project-copy\"><h3><a href=\"{}\" target=\"_blank\" rel=\"noreferrer\">{}</a></h3><p class=\"game-name\">{} <span>/</span> {}</p><p class=\"summary\">{}</p><div class=\"project-metadata\">{}{}{}{}{}</div><div class=\"source-links\"><span>Sources</span>{}</div></div><div class=\"project-state\"><span class=\"status-dot\"></span>{}{}</div></article>",
        escape_html(&project.kind.to_lowercase()), escape_html(&project.kind),
        escape_html(&safe_link(&project.project_url)), escape_html(&project.title),
        escape_html(&project.game), escape_html(&project.platform), escape_html(&project.summary),
        method, evidence, language, found, started, source_links, escape_html(&project.status), updated
    )
}

fn set_filter_value(state: &mut AppState, id: &str, value: String) {
    match id {
        "filter-platform" => state.platform = value,
        "filter-strategy" => state.strategy = value,
        "filter-status" => state.status = value,
        "filter-source" => state.source_available = value,
        "filter-language" => state.language = value,
        "filter-found" => state.first_found_since = value,
        "filter-started" => state.project_started_by = value,
        _ => {}
    }
}

fn populate_filters(document: &Document, catalog: &Catalog) {
    for (id, label, values) in [
        ("filter-platform", "Any platform", catalog.projects.iter().map(|p| p.platform.as_str()).collect::<BTreeSet<_>>()),
        ("filter-strategy", "Any approach", catalog.projects.iter().map(|p| p.strategy.as_str()).filter(|v| !v.is_empty()).collect::<BTreeSet<_>>()),
        ("filter-status", "Any stage", catalog.projects.iter().map(|p| status_bucket(&p.status)).collect::<BTreeSet<_>>()),
        ("filter-source", "Any source type", catalog.projects.iter().map(|p| p.source_available.as_str()).filter(|v| !v.is_empty()).collect::<BTreeSet<_>>()),
        ("filter-language", "Any language", catalog.projects.iter().map(|p| p.language.as_str()).filter(|v| !v.is_empty()).collect::<BTreeSet<_>>()),
    ] {
        let options = std::iter::once(format!("<option value=\"\">{}</option>", escape_html(label)))
            .chain(values.into_iter().map(|value| format!("<option value=\"{}\">{}</option>", escape_html(value), escape_html(value))))
            .collect::<String>();
        set_html(document, id, &options);
    }
    let started = catalog.projects.iter().any(|project| project.project_started.is_some());
    if let Some(input) = document.get_element_by_id("filter-started") {
        if !started { let _ = input.set_attribute("disabled", ""); }
    }
}

fn status_bucket(status: &str) -> &'static str {
    let value = status.to_lowercase();
    if value.contains("unclear") || value.contains("uncertain") || value.contains("verify") {
        "Unverified"
    } else if value.contains("released") {
        "Released"
    } else if value.contains("complete") {
        "Complete"
    } else if value.contains("playable") {
        "Playable"
    } else if value.contains("experimental") || value.contains("early") || value.contains("preview") {
        "Early / experimental"
    } else {
        "In progress"
    }
}

fn safe_link(value: &str) -> String {
    if value.starts_with("https://") { value.to_string() } else { "#".into() }
}

fn escape_html(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}

fn format_date(value: &str) -> String {
    let date = value.split('T').next().unwrap_or(value);
    let mut parts = date.split('-');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(year), Some(month), Some(day)) if year.len() == 4 => {
            let month_name = match month { "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr", "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug", "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec", _ => return value.to_string() };
            format!("{month_name} {day} {year}")
        }
        _ => value.to_string(),
    }
}

fn set_text(document: &Document, id: &str, value: &str) {
    if let Some(element) = document.get_element_by_id(id) { element.set_text_content(Some(value)); }
}

fn set_html(document: &Document, id: &str, value: &str) {
    if let Some(element) = document.get_element_by_id(id) { element.set_inner_html(value); }
}
