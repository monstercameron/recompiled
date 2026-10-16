use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    rc::Rc,
};
use wasm_bindgen::{closure::Closure, prelude::*, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    Document, Element, Event, HtmlDialogElement, HtmlElement, HtmlInputElement, HtmlSelectElement,
    HtmlVideoElement, KeyboardEvent, Response, UrlSearchParams,
};
mod console_gallery;
mod i18n;
mod media;
mod process_resources;

#[derive(Clone, Deserialize, Serialize)]
struct Project {
    id: String,
    title: String,
    game: String,
    #[serde(default = "default_target_type")]
    target_type: String,
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
    programming_language: String,
    #[serde(default)]
    subtype: String,
    #[serde(default)]
    source_available: String,
    #[serde(default)]
    first_found: Option<String>,
    #[serde(default)]
    project_started: Option<String>,
    #[serde(default)]
    media: Vec<media::Media>,
    #[serde(default)]
    links: Vec<ProjectLink>,
    #[serde(default)]
    programming_language_evidence: Option<ProgrammingLanguageEvidence>,
    #[serde(skip)]
    search_text: String,
    #[serde(skip)]
    search_terms: Vec<String>,
    #[serde(skip)]
    title_sort_key: String,
}

fn default_target_type() -> String {
    "Game".into()
}

#[derive(Clone, Deserialize, Serialize)]
struct Source {
    label: String,
    url: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct ProjectLink {
    kind: String,
    label: String,
    url: String,
    source_url: String,
    #[serde(default)]
    notes: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct ProgrammingLanguageEvidence {
    predominant_language: Option<String>,
    breakdown: Option<BTreeMap<String, f64>>,
    source_url: String,
    evidence: String,
    verified_at: String,
    reason_unknown: Option<String>,
}

#[derive(Clone, Deserialize)]
struct DirectoryItem {
    item_type: String,
    category: String,
    name: String,
    description: String,
    url: String,
    source_url: String,
    source_evidence: String,
    verified_at: String,
}

#[derive(Clone, Deserialize)]
struct FeaturedProject {
    project_id: String,
    position: usize,
    github_stars: u64,
    verified_at: String,
    source_url: String,
    ranking_url: String,
    evidence: String,
}

#[derive(Clone, Deserialize)]
struct GameProfile {
    game_key: String,
    #[serde(default)]
    facts: Vec<GameFact>,
    #[serde(default)]
    wikipedia_url: Option<String>,
    #[serde(default)]
    wikipedia_title: Option<String>,
}

#[derive(Clone, Deserialize)]
struct GameFact {
    field: String,
    value: Value,
    source_url: String,
    evidence: String,
    #[serde(default)]
    date_granularity: Option<String>,
}

#[derive(Deserialize)]
struct Catalog {
    updated: String,
    projects: Vec<Project>,
    #[serde(default)]
    profiles: Vec<GameProfile>,
    #[serde(default)]
    directory_items: Vec<DirectoryItem>,
    #[serde(default)]
    featured_projects: Vec<FeaturedProject>,
    #[serde(default)]
    process_resources: Vec<process_resources::ProcessResource>,
    #[serde(default)]
    console_gallery: Vec<console_gallery::ConsoleHardware>,
    #[serde(skip)]
    profile_indices: HashMap<String, usize>,
    #[serde(skip)]
    profile_search: HashMap<String, Vec<String>>,
    #[serde(skip)]
    profile_search_terms: HashMap<String, Vec<String>>,
    #[serde(skip)]
    target_count: usize,
    #[serde(skip)]
    source_count: usize,
}

impl Catalog {
    fn prepare_indexes(&mut self) {
        self.profile_indices = self
            .profiles
            .iter()
            .enumerate()
            .map(|(index, profile)| (profile.game_key.clone(), index))
            .collect();
        self.profile_search = self
            .profiles
            .iter()
            .map(|profile| {
                let searchable_facts = profile
                    .facts
                    .iter()
                    .map(|fact| game_fact_value(&fact.value).to_lowercase())
                    .collect();
                (profile.game_key.clone(), searchable_facts)
            })
            .collect();
        self.profile_search_terms = self
            .profiles
            .iter()
            .map(|profile| {
                let terms = profile
                    .facts
                    .iter()
                    .flat_map(|fact| search_tokens(&game_fact_value(&fact.value).to_lowercase()))
                    .collect();
                (profile.game_key.clone(), terms)
            })
            .collect();

        for project in &mut self.projects {
            project.search_text = [
                project.title.as_str(),
                project.game.as_str(),
                project.platform.as_str(),
                project.summary.as_str(),
                project.kind.as_str(),
                project.status.as_str(),
                project.strategy.as_str(),
                project.language.as_str(),
                project.programming_language.as_str(),
                project.source_available.as_str(),
            ]
            .into_iter()
            .map(str::to_lowercase)
            .collect::<Vec<_>>()
            .join("\u{1f}");
            project.search_terms = search_tokens(&project.search_text);
            project.title_sort_key = project.game.to_lowercase();
        }

        self.target_count = self
            .projects
            .iter()
            .map(|project| project.game.as_str())
            .collect::<HashSet<_>>()
            .len();
        self.source_count = self
            .projects
            .iter()
            .flat_map(|project| project.sources.iter().map(|source| source.url.as_str()))
            .collect::<HashSet<_>>()
            .len();
    }

    fn profile_for(&self, game: &str) -> Option<&GameProfile> {
        self.profile_indices
            .get(game)
            .and_then(|index| self.profiles.get(*index))
    }
}

#[derive(Default)]
struct AppState {
    catalog: Option<Catalog>,
    locale: String,
    query: String,
    search_mode: String,
    kinds: BTreeSet<String>,
    sort: String,
    platform: String,
    strategy: String,
    status: String,
    source_available: String,
    target_type: String,
    language: String,
    programming_language: String,
    genre: String,
    original_platform: String,
    first_found_since: String,
    project_started_by: String,
    media_kind: String,
    console_query: String,
    console_maker: String,
    console_type: String,
    page: usize,
    page_size: usize,
    media_projects: Vec<usize>,
    media_position: usize,
    media_asset_position: usize,
    media_shuffle: bool,
    media_playing: bool,
}

const SHELL: &str = r###"
<a class="skip-link" href="#catalog" data-ui="skip_link">Skip to project catalog</a>
<svg class="icon-library" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false">
  <symbol id="icon-mark" viewBox="0 0 28 28"><path d="M4 20V9h5v11M12 20V4h5v16M20 20v-8h5v8" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/><path d="M4 24h21" fill="none" stroke="currentColor" stroke-width="1.6" opacity=".55"/></symbol>
  <symbol id="icon-moon" viewBox="0 0 24 24"><path d="M20.2 15.1A8.5 8.5 0 0 1 8.9 3.8 8.6 8.6 0 1 0 20.2 15.1Z" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-sun" viewBox="0 0 24 24"><circle cx="12" cy="12" r="4" fill="none" stroke="currentColor" stroke-width="1.8"/><path d="M12 2v2m0 16v2M4.93 4.93l1.42 1.42m11.3 11.3 1.42 1.42M2 12h2m16 0h2M4.93 19.07l1.42-1.42m11.3-11.3 1.42-1.42" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></symbol>
  <symbol id="icon-external" viewBox="0 0 24 24"><path d="M13 5h6v6m0-6-9 9" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/><path d="M18 13v5a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-down" viewBox="0 0 24 24"><path d="M12 4v15m-6-6 6 6 6-6" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-search" viewBox="0 0 24 24"><circle cx="10.8" cy="10.8" r="6.8" fill="none" stroke="currentColor" stroke-width="1.8"/><path d="m16 16 5 5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></symbol>
  <symbol id="icon-filter" viewBox="0 0 24 24"><path d="M4 6h16M4 12h16M4 18h16" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round"/><circle cx="9" cy="6" r="2" fill="var(--paper)" stroke="currentColor" stroke-width="1.7"/><circle cx="15" cy="12" r="2" fill="var(--paper)" stroke="currentColor" stroke-width="1.7"/><circle cx="8" cy="18" r="2" fill="var(--paper)" stroke="currentColor" stroke-width="1.7"/></symbol>
  <symbol id="icon-chevron" viewBox="0 0 24 24"><path d="m6 9 6 6 6-6" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-code" viewBox="0 0 24 24"><path d="m8 5-6 7 6 7m8-14 6 7-6 7m-3-16-2 18" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-cube" viewBox="0 0 24 24"><path d="m12 3 9 5v8l-9 5-9-5V8l9-5Z" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linejoin="round"/><path d="m3.5 8.5 8.5 5 8.5-5M12 13.5V21m0-18v5" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linejoin="round"/></symbol>
  <symbol id="icon-arrows" viewBox="0 0 24 24"><path d="M4 8h14m0 0-4-4m4 4-4 4M20 16H6m0 0 4-4m-4 4 4 4" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-translate" viewBox="0 0 24 24"><path d="M3 5h12M9 3v2m5 0c-.5 5-4 9-9 11m1-9c1 3 4 6 7 7m-1 6 4-10 4 10m-6.5-3h5" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-terminal" viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="16" rx="2" fill="none" stroke="currentColor" stroke-width="1.7"/><path d="m7 9 3 3-3 3m6 0h4" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-gamepad" viewBox="0 0 24 24"><path d="M7.5 8h9a4.5 4.5 0 0 1 4.3 3.2l1.1 3.9a2.4 2.4 0 0 1-3.8 2.5l-2.4-1.8H8.3l-2.4 1.8a2.4 2.4 0 0 1-3.8-2.5l1.1-3.9A4.5 4.5 0 0 1 7.5 8Z" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linejoin="round"/><path d="M7 10.5v4m-2-2h4m7-1h.01M18 14h.01" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></symbol>
  <symbol id="icon-spark" viewBox="0 0 24 24"><path d="m12 2 1.7 6.3L20 10l-6.3 1.7L12 18l-1.7-6.3L4 10l6.3-1.7L12 2Zm7 13 .8 2.2L22 18l-2.2.8L19 21l-.8-2.2L16 18l2.2-.8L19 15Z" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/></symbol>
  <symbol id="icon-play" viewBox="0 0 24 24"><path d="m9 6 10 6-10 6V6Z" fill="currentColor" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/></symbol>
  <symbol id="icon-pause" viewBox="0 0 24 24"><path d="M7 5h3v14H7zm7 0h3v14h-3z" fill="currentColor"/></symbol>
  <symbol id="icon-shuffle" viewBox="0 0 24 24"><path d="M16 3h5v5m0-5-6.5 6.5M4 7h3c4 0 6 10 10 10h4m-5-5 5 5-5 5M4 17h3c1.2 0 2.2-.9 3.2-2.2" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-image" viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="16" rx="2" fill="none" stroke="currentColor" stroke-width="1.7"/><circle cx="8.5" cy="9" r="1.5" fill="none" stroke="currentColor" stroke-width="1.6"/><path d="m4 17 5-5 3.5 3 2.5-2 5 5" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-link" viewBox="0 0 24 24"><path d="M9.5 14.5 14.5 9.5M7.2 16.8l-1.4 1.4a4 4 0 0 1-5.7-5.7l4.2-4.2a4 4 0 0 1 5.7 0m4-1.1 1.4-1.4a4 4 0 0 1 5.7 5.7l-4.2 4.2a4 4 0 0 1-5.7 0" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" transform="translate(2 0)"/></symbol>
  <symbol id="icon-folder" viewBox="0 0 24 24"><path d="M3 6.5h7l2 2h9v9.8a1.7 1.7 0 0 1-1.7 1.7H4.7A1.7 1.7 0 0 1 3 18.3V6.5Z" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linejoin="round"/><path d="M3 9h18" fill="none" stroke="currentColor" stroke-width="1.7"/></symbol>
  <symbol id="icon-grid" viewBox="0 0 24 24"><rect x="3" y="3" width="7" height="7" rx="1" fill="none" stroke="currentColor" stroke-width="1.7"/><rect x="14" y="3" width="7" height="7" rx="1" fill="none" stroke="currentColor" stroke-width="1.7"/><rect x="3" y="14" width="7" height="7" rx="1" fill="none" stroke="currentColor" stroke-width="1.7"/><rect x="14" y="14" width="7" height="7" rx="1" fill="none" stroke="currentColor" stroke-width="1.7"/></symbol>
  <symbol id="icon-left" viewBox="0 0 24 24"><path d="M19 12H5m0 0 6-6m-6 6 6 6" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-right" viewBox="0 0 24 24"><path d="M5 12h14m0 0-6-6m6 6-6 6" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-up" viewBox="0 0 24 24"><path d="M12 20V5m-6 6 6-6 6 6" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-sort" viewBox="0 0 24 24"><path d="m8 4-4 4 4 4M4 8h10m2 12 4-4-4-4m4 4H10" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-check" viewBox="0 0 24 24"><path d="m5 12 4 4L19 6" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></symbol>
  <symbol id="icon-warning" viewBox="0 0 24 24"><path d="M12 3 2.8 20h18.4L12 3Z" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linejoin="round"/><path d="M12 9v5m0 3h.01" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></symbol>
</svg>
<header class="site-header" id="top">
  <a class="brand" href="./" data-ui-aria-label="home_aria" aria-label="Recompiled home"><svg class="brand-mark ui-icon" aria-hidden="true" focusable="false"><use href="#icon-mark" xlink:href="#icon-mark"></use></svg><span class="brand-copy"><span class="brand-wordmark">recompiled<span class="brand-period">.</span></span><span class="brand-tagline" data-ui="header_tagline">Software ownership index</span></span></a>
  <nav class="top-nav" data-ui-aria-label="nav_aria" aria-label="Main navigation">
    <div class="primary-nav"><a href="#shift" data-ui="nav_why">Why this year</a><a href="#showcase" data-ui="nav_showcase">Popular projects</a><a href="#catalog" data-ui="nav_explore">Explore the index</a><a href="#toolkit" data-ui="nav_tools">Tools &amp; sites</a><a href="#resources" data-ui="nav_resources">Resource guide</a><a class="github-link" href="#about"><span data-ui="nav_sources">How this is sourced</span> <svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-external" xlink:href="#icon-external"></use></svg></a></div>
    <div class="header-actions"><label class="site-language"><span class="sr-only" data-ui="language_label">Language</span><select id="site-language" aria-label="Language" title="Language"><option value="en">English</option><option value="es">Español</option><option value="fr">Français</option><option value="pt">Português</option><option value="ja">日本語</option><option value="zh">中文</option></select></label><button id="theme-toggle" class="theme-toggle" type="button" aria-label="Switch to dark mode" aria-pressed="false"><span id="theme-icon" aria-hidden="true"><svg class="ui-icon"><use href="#icon-moon" xlink:href="#icon-moon"></use></svg></span><span id="theme-label">Dark mode</span></button></div>
  </nav>
</header>
<main>
  <section class="hero" aria-labelledby="hero-title">
    <div class="hero-copy"><p class="kicker"><span class="live-indicator"></span><span id="status-line" data-ui="status_loading">Preparing catalog</span></p>
      <h1 id="hero-title"><span data-ui="hero_first">Software should be</span><br><span data-ui="hero_second">yours to shape.</span></h1>
      <p class="hero-summary" data-ui="hero_summary">A source-linked field guide to taking software into your own hands: recovering code, recompiling binaries, carrying APIs across platforms, adapting tools, and preserving the work.</p>
      <a class="hero-jump" href="#catalog"><span data-ui="hero_cta">Find a project</span> <svg class="ui-icon" aria-hidden="true"><use href="#icon-down" xlink:href="#icon-down"></use></svg></a>
    </div>
    <div class="hero-figure" data-ui-aria-label="hero_figure" aria-label="Catalog at a glance">
      <svg class="hero-signal-map" viewBox="0 0 440 366" aria-hidden="true" focusable="false">
        <defs>
          <pattern id="hero-dot-grid" width="12" height="12" patternUnits="userSpaceOnUse"><circle cx="1.5" cy="1.5" r=".8" class="signal-grid-dot"/></pattern>
          <linearGradient id="hero-trace-gradient" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="var(--blue)"/><stop offset=".52" stop-color="var(--coral)"/><stop offset="1" stop-color="var(--mint)"/></linearGradient>
        </defs>
        <rect class="signal-board" x="12" y="14" width="416" height="338" rx="3"/>
        <rect class="signal-grid" x="13" y="15" width="414" height="336"/>
        <path class="signal-rail" d="M29 183H411M220 31v30m0 244v30"/>
        <path class="signal-trace" d="M108 183h44l25-25 24 25h38l25 25 25-25h43"/>
        <g class="signal-source">
          <rect class="signal-module" x="40" y="146" width="70" height="74" rx="2"/>
          <path class="signal-module-cap" d="M40 154h70"/>
          <path class="signal-code-line" d="M50 166h18m-18 8h36m-36 8h24m-24 8h32"/>
          <rect class="signal-bit" x="50" y="201" width="5" height="5"/><rect class="signal-bit" x="59" y="201" width="5" height="5"/><rect class="signal-bit signal-bit-hot" x="68" y="201" width="5" height="5"/>
        </g>
        <g class="signal-target">
          <rect class="signal-module" x="330" y="146" width="70" height="74" rx="2"/>
          <path class="signal-module-cap" d="M330 154h70"/>
          <circle class="signal-window-dot" cx="339" cy="150" r="1.5"/><circle class="signal-window-dot" cx="345" cy="150" r="1.5"/><circle class="signal-window-dot" cx="351" cy="150" r="1.5"/>
          <rect class="signal-screen" x="340" y="164" width="50" height="42" rx="1"/>
          <path class="signal-screen-line" d="M347 193v-8l7-7 7 12 7-9 7 5 8-10"/>
        </g>
        <circle class="signal-terminal signal-terminal-source" cx="109" cy="183" r="3.5"/><circle class="signal-terminal signal-terminal-target" cx="331" cy="183" r="3.5"/>
        <text class="signal-readout signal-readout-source" x="40" y="239">0x0040 · SOURCE</text><text class="signal-readout signal-readout-target" x="330" y="239">NATIVE · 0x01</text>
      </svg>
      <div class="orbit orbit-one"></div><div class="orbit orbit-two"></div>
      <button class="orbit-core" id="hero-secret-button" type="button" data-ui-aria-label="egg_hero_aria" aria-label="Archive bonus" aria-describedby="hero-secret-hint"><span data-ui="figure_game">CODE</span><b><svg class="ui-icon" aria-hidden="true"><use href="#icon-arrows" xlink:href="#icon-arrows"></use></svg></b><span data-ui="figure_again">YOURS</span></button>
      <span class="sr-only" id="hero-secret-hint" data-ui="egg_hero_hint">Tap the center seven times to find the debug room.</span>
      <div class="figure-note note-top" data-ui="figure_community">COMMUNITY BUILT</div><div class="figure-note note-bottom" data-ui="figure_note">SOURCE · PORT · LANGUAGE</div>
      <div class="orbit-pin pin-one"></div><div class="orbit-pin pin-two"></div>
    </div>
  </section>
  <section class="shift-story" id="shift" aria-labelledby="shift-title">
    <div class="shift-heading"><div><p class="kicker" data-ui="shift_kicker">Why this year feels different · 2026</p><h2 id="shift-title" data-ui="shift_title">The first useful attempt is easier to reach.</h2></div><p data-ui="shift_intro">Reverse engineering still takes skill. What changed is how cheaply one person can explore a codebase, turn a hunch into a tool, and repeat the test-and-fix loop.</p></div>
    <div class="shift-cards" id="shift-cards" data-ui-aria-label="shift_cards_label" aria-label="Changes in the workflow">
      <article class="shift-card"><span class="shift-topic"><svg class="ui-icon" aria-hidden="true"><use href="#icon-terminal" xlink:href="#icon-terminal"></use></svg><span data-ui="shift_one_label">Capable coding models</span></span><h3 data-ui="shift_one_title">From autocomplete to a working session</h3><p data-ui="shift_one_body">Frontier models can stay with multi-step coding and terminal tasks: inspect unfamiliar code, explain a function, draft a utility, then help work through the test output.</p><a href="https://www.anthropic.com/claude-opus-5-5" target="_blank" rel="noopener noreferrer"><span data-ui="shift_one_link">Anthropic’s Opus 5.5 announcement</span> <svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-external" xlink:href="#icon-external"></use></svg></a></article>
      <article class="shift-card"><span class="shift-topic"><svg class="ui-icon" aria-hidden="true"><use href="#icon-spark" xlink:href="#icon-spark"></use></svg><span data-ui="shift_two_label">Lower-cost iteration</span></span><h3 data-ui="shift_two_title">More experiments fit in the day</h3><p data-ui="shift_two_body">Anthropic reports Opus 5.5 costs 40% less on typical workloads than Opus 5. That can lower the cost of trying a symbol pass, a porting script, a localization check, or another test loop.</p><a href="https://www.anthropic.com/claude-opus-5-5#performance-and-cost-effectiveness" target="_blank" rel="noopener noreferrer"><span data-ui="shift_two_link">Read the vendor’s cost and performance notes</span> <svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-external" xlink:href="#icon-external"></use></svg></a></article>
      <article class="shift-card"><span class="shift-topic"><svg class="ui-icon" aria-hidden="true"><use href="#icon-check" xlink:href="#icon-check"></use></svg><span data-ui="shift_three_label">More people can start</span></span><h3 data-ui="shift_three_title">“No-brainer” means try it</h3><p data-ui="shift_three_body">Models can help make tools and explanations on demand, so a small experiment may now be worth attempting. They do not establish a matching decompilation, a working recompilation, or a correct translation for you.</p><a href="#toolkit"><span data-ui="shift_three_link">Start with the tools and communities below</span> <svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-down" xlink:href="#icon-down"></use></svg></a></article>
    </div>
    <p class="shift-caveat"><svg class="ui-icon" aria-hidden="true"><use href="#icon-warning" xlink:href="#icon-warning"></use></svg><span><span data-ui="shift_caveat">Models make the first pass cheaper; correctness still comes from builds, tests, matching behavior, and careful review. See</span> <a href="https://github.com/zeldaret/oot/blob/main/README.md" target="_blank" rel="noopener noreferrer" data-ui="shift_checksum">Ocarina of Time’s checksum build guide</a> and <a href="https://github.com/N64Recomp/N64Recomp/blob/main/CONTRIBUTING.md" target="_blank" rel="noopener noreferrer" data-ui="shift_policy">N64Recomp’s testing policy</a>. <span data-ui="shift_caveat_end">API translation (such as DirectX-to-Vulkan) is different from a fan translation of a game’s text.</span></span></p>
  </section>
  <section class="project-showcase" id="showcase" aria-labelledby="showcase-title">
    <div class="showcase-heading"><div><p class="kicker" data-ui="showcase_kicker">THE PROJECTS PEOPLE KEEP STARING AT</p><h2 id="showcase-title" data-ui="showcase_title">The three most-starred projects in the index</h2></div><p data-ui="showcase_intro">A dated snapshot of GitHub stars for projects tracked here. Counts can change; each card links to its repository and ranking source.</p></div>
    <div class="showcase-grid" id="featured-projects" data-ui-aria-label="showcase_aria" aria-label="Three popular projects"></div>
  </section>
  <section class="curated-directory" id="toolkit" aria-labelledby="toolkit-title">
    <div class="directory-heading"><div><p class="kicker" data-ui="toolkit_kicker">A practical starter shelf</p><h2 id="toolkit-title" data-ui="toolkit_title">Tools &amp; places to follow the work</h2></div><p data-ui="toolkit_intro">A curated set of established tools, project hubs, and reference sites. It is a wayfinding list, not a popularity ranking.</p></div>
    <h3 class="directory-subhead"><svg class="ui-icon" aria-hidden="true"><use href="#icon-code" xlink:href="#icon-code"></use></svg><span data-ui="tools">Tools</span></h3><div id="directory-tools" class="directory-grid" data-ui-aria-label="tools_aria" aria-label="Reverse engineering and modding tools"></div>
    <h3 class="directory-subhead"><svg class="ui-icon" aria-hidden="true"><use href="#icon-cube" xlink:href="#icon-cube"></use></svg><span data-ui="communities">Project hubs &amp; communities</span></h3><div id="directory-sites" class="directory-grid" data-ui-aria-label="sites_aria" aria-label="Software preservation and modding websites"></div>
  </section>
  <section class="process-resource-guide" id="resources" aria-labelledby="resources-title">
    <div class="process-resource-heading"><div><p class="kicker" data-ui="resources_kicker">EXPLANATIONS AND PROJECT REFERENCES</p><h2 id="resources-title" data-ui="resources_title">Five resources to understand the work</h2></div><div><p data-ui="resources_intro">Popular videos and primary project pages introduce software decompilation, static recompilation, native ports, and API translation.</p><p class="process-resource-metric-note" data-ui="resources_metric_note">Views, likes, and stars are separate platform-specific signals, checked on the date shown; they do not form a shared ranking.</p></div></div>
    <div id="process-resource-cards" class="process-resource-grid" data-ui-aria-label="resources_aria" aria-label="Software reconstruction resource guide"></div>
  </section>
  <section class="console-gallery" id="consoles" aria-labelledby="console-gallery-title">
    <div class="console-gallery-heading"><div><p class="kicker" data-ui="console_gallery_kicker">THE HARDWARE BEHIND THE GAMES</p><h2 id="console-gallery-title" data-ui="console_gallery_title">A shelf of original systems</h2></div><p data-ui="console_gallery_intro">Browse console and handheld families represented in the index. Locally cached photos keep their creator, reuse license, and original file page attached; maker marks identify each hardware family.</p></div>
    <div class="console-gallery-controls" id="console-gallery-controls">
      <label><span data-ui="console_search_label">Find a system</span><input id="console-search" type="search" data-ui-placeholder="console_search_placeholder" placeholder="Search console or maker" autocomplete="off"></label>
      <label><span data-ui="console_maker_label">Maker</span><select id="filter-console-maker"><option value="" data-ui="console_any_maker">Any maker</option></select></label>
      <label><span data-ui="console_type_label">Hardware type</span><select id="filter-console-type"><option value="" data-ui="console_type_all">All systems</option><option value="home" data-ui="console_type_home">Home console</option><option value="handheld" data-ui="console_type_handheld">Handheld</option><option value="hybrid" data-ui="console_type_hybrid">Hybrid console</option><option value="add_on" data-ui="console_type_addon">Add-on</option></select></label>
    </div>
    <p class="console-gallery-count" id="console-gallery-count" role="status" aria-live="polite"></p>
    <div id="console-gallery-items" class="console-gallery-grid" data-ui-aria-label="console_gallery_aria" aria-label="Console and handheld photo gallery"></div>
  </section>
  <section class="facts" data-ui-aria-label="facts_aria" aria-label="Index overview">
    <div class="fact"><svg class="ui-icon" aria-hidden="true"><use href="#icon-code" xlink:href="#icon-code"></use></svg><strong id="project-total">—</strong><span data-ui="projects_indexed">projects indexed</span></div><div class="fact"><svg class="ui-icon" aria-hidden="true"><use href="#icon-cube" xlink:href="#icon-cube"></use></svg><strong id="target-total">—</strong><span data-ui="targets_represented">software targets represented</span></div><div class="fact"><svg class="ui-icon" aria-hidden="true"><use href="#icon-external" xlink:href="#icon-external"></use></svg><strong id="source-total">—</strong><span data-ui="primary_sources">primary sources linked</span></div><p><span data-ui="catalog_refreshed">Catalog refreshed</span> <span id="catalog-updated">—</span></p>
  </section>
  <section class="catalog" id="catalog" tabindex="-1" aria-labelledby="catalog-title">
    <div class="catalog-heading"><div><p class="kicker" data-ui="index_kicker">The index</p><h2 id="catalog-title" data-ui="catalog_title">Browse the work</h2></div><p data-ui="catalog_intro">Every listing leads back to its project and the references used to identify it.</p></div>
    <div class="catalog-tools"><div class="search-field"><svg class="ui-icon search-icon" aria-hidden="true"><use href="#icon-search" xlink:href="#icon-search"></use></svg><label for="search" class="sr-only" data-ui="search_label">Search projects</label><input id="search" type="search" data-ui-placeholder="search_placeholder" placeholder="Search a title, platform, or project" autocomplete="off"><span class="search-mode-caption" aria-hidden="true" data-ui="search_mode_caption">Match</span><label for="search-mode" class="sr-only" data-ui="search_mode_label">Search mode</label><select id="search-mode" class="search-mode" data-ui-aria-label="search_mode_label" aria-label="Search mode" aria-describedby="search-mode-help"><option value="fuzzy" data-ui="search_mode_fuzzy">Fuzzy</option><option value="verbatim" data-ui="search_mode_verbatim">Verbatim</option></select><span id="search-mode-help" class="sr-only" data-ui="search_mode_help">Fuzzy allows small typos; verbatim finds the exact phrase.</span></div><div class="sort-control"><svg class="ui-icon" aria-hidden="true"><use href="#icon-sort" xlink:href="#icon-sort"></use></svg><label for="sort" data-ui="order">Order</label><select id="sort"><option value="updated-desc" data-ui="sort_updated_desc">Recently updated</option><option value="updated-asc" data-ui="sort_updated_asc">Oldest update</option><option value="title-asc" data-ui="sort_title_asc">Target A–Z</option><option value="title-desc" data-ui="sort_title_desc">Target Z–A</option><option value="found-desc" data-ui="sort_found_desc">Recently found here</option><option value="found-asc" data-ui="sort_found_asc">First found here</option><option value="kind-asc" data-ui="sort_kind_asc">Project type A–Z</option><option value="kind-desc" data-ui="sort_kind_desc">Project type Z–A</option><option value="target-type-asc" data-ui="sort_target_type_asc">Target type A–Z</option><option value="target-type-desc" data-ui="sort_target_type_desc">Target type Z–A</option><option value="platform-asc" data-ui="sort_platform_asc">Platform A–Z</option><option value="platform-desc" data-ui="sort_platform_desc">Platform Z–A</option><option value="strategy-asc" data-ui="sort_strategy_asc">Approach A–Z</option><option value="strategy-desc" data-ui="sort_strategy_desc">Approach Z–A</option><option value="status-asc" data-ui="sort_status_asc">Stage A–Z</option><option value="status-desc" data-ui="sort_status_desc">Stage Z–A</option><option value="language-asc" data-ui="sort_language_asc">Language A–Z</option><option value="language-desc" data-ui="sort_language_desc">Language Z–A</option></select></div></div>
    <div class="catalog-tabs" id="kind-tabs" role="group" data-ui-aria-label="filter_kind_aria" aria-label="Filter by project type"><button data-kind="all" aria-pressed="true"><svg class="ui-icon" aria-hidden="true"><use href="#icon-cube" xlink:href="#icon-cube"></use></svg><span data-ui="kind_all">Everything</span></button><button data-kind="Decompilation" aria-pressed="false"><svg class="ui-icon" aria-hidden="true"><use href="#icon-code" xlink:href="#icon-code"></use></svg><span data-ui="kind_decomp">Decompilations</span></button><button data-kind="Recompilation" aria-pressed="false"><svg class="ui-icon" aria-hidden="true"><use href="#icon-cube" xlink:href="#icon-cube"></use></svg><span data-ui="kind_recomp">Recompilations</span></button><button data-kind="API Translation" aria-pressed="false"><svg class="ui-icon" aria-hidden="true"><use href="#icon-arrows" xlink:href="#icon-arrows"></use></svg><span data-ui="kind_api">API translations</span></button><button data-kind="Fan Localization" aria-pressed="false"><svg class="ui-icon" aria-hidden="true"><use href="#icon-translate" xlink:href="#icon-translate"></use></svg><span data-ui="kind_fan">Fan localizations</span></button><button data-kind="Reimplementation" aria-pressed="false"><svg class="ui-icon" aria-hidden="true"><use href="#icon-terminal" xlink:href="#icon-terminal"></use></svg><span data-ui="kind_reimplementation">Reimplementations</span></button><button data-kind="Emulator" aria-pressed="false"><svg class="ui-icon" aria-hidden="true"><use href="#icon-gamepad" xlink:href="#icon-gamepad"></use></svg><span data-ui="kind_emulator">Emulators</span></button><span id="result-count" aria-live="polite"></span></div>
    <details class="filter-drawer"><summary><svg class="ui-icon disclosure-icon" aria-hidden="true"><use href="#icon-filter" xlink:href="#icon-filter"></use></svg><span data-ui="filter_more">More ways to explore</span><svg class="ui-icon disclosure-chevron" aria-hidden="true"><use href="#icon-chevron" xlink:href="#icon-chevron"></use></svg><span class="filter-hint" data-ui="filter_hint">Game genre · platform · method · evidence · language</span></summary><div class="filter-grid">
      <label><span data-ui="filter_target_type">Target type</span><select id="filter-target-type" data-filter><option value="" data-ui="any_target">Any target</option><option value="Game" data-ui="target_game">Game</option><option value="Software" data-ui="target_software">Software application</option></select></label>
      <label><span data-ui="filter_platform">Platform</span><select id="filter-platform" data-filter><option value="">Any platform</option></select></label>
      <label><span data-ui="filter_strategy">Approach</span><select id="filter-strategy" data-filter><option value="">Any approach</option></select></label>
      <label><span data-ui="filter_stage">Project stage</span><select id="filter-status" data-filter><option value="">Any stage</option></select></label>
      <label><span data-ui="filter_source">Source available</span><select id="filter-source" data-filter><option value="">Any source type</option></select></label>
      <label><span data-ui="filter_fan_language">Fan localization language</span><select id="filter-language" data-filter><option value="">Any language</option></select></label>
      <label><span data-ui="filter_programming_language">Predominant programming language</span><select id="filter-programming-language" data-filter><option value="">Any language</option></select></label>
      <label><span data-ui="filter_genre">Game genre</span><select id="filter-genre" data-filter><option value="">Any genre</option></select></label>
      <label><span data-ui="filter_original_platform">Original target platform</span><select id="filter-original-platform" data-filter><option value="">Any original platform</option></select></label>
      <label><span data-ui="filter_media">Images &amp; video</span><select id="filter-media" data-filter><option value="">Any media</option><option value="image" data-ui="has_images">Has images</option><option value="video" data-ui="has_video">Has video</option><option value="none" data-ui="no_media">No media linked yet</option></select></label>
      <label><span data-ui="filter_found">First found here, since</span><input id="filter-found" data-filter type="date" aria-label="First found here since" data-ui-aria-label="filter_found"></label>
      <label><span data-ui="filter_started">Project started by</span><input id="filter-started" data-filter type="date" aria-label="Project started by" data-ui-aria-label="filter_started"></label>
      <button id="clear-filters" class="clear-filters" type="button"><svg class="ui-icon" aria-hidden="true"><use href="#icon-sort" xlink:href="#icon-sort"></use></svg><span data-ui="clear_filters">Clear filters</span></button>
    </div><p class="filter-footnote" data-ui="filter_note">“First found here” records when we added a project to this index. No primary-source project start dates are verified yet; that filter activates as they are added.</p></details>
    <div id="table-top" class="table-toolbar" tabindex="-1"><p id="page-info" role="status">Loading the index…</p><label class="page-size"><span data-ui="rows_per_page">Rows per page</span><input id="page-size" type="number" min="1" max="1000" step="1" value="25" list="page-size-presets" aria-describedby="page-size-hint"><datalist id="page-size-presets"><option value="10"><option value="25"><option value="50"><option value="100"><option value="250"><option value="500"></datalist><span id="page-size-hint" class="sr-only" data-ui="rows_per_page_hint">Enter a whole number from 1 to 1,000.</span></label></div>
    <div id="pagination-top" class="pagination-slot"></div>
    <p class="table-scroll-note" id="table-scroll-help" data-ui="scroll_note">Scroll the table horizontally to see every column.</p>
    <div class="table-scroll" role="region" data-ui-aria-label="table_aria" aria-label="Project catalog table" aria-describedby="table-scroll-help" tabindex="0"><table class="catalog-table"><caption class="sr-only" data-ui="table_caption">Projects with target type, platform, approach, stage, translation language, and discovery date. Select a column heading to sort. Open project details for descriptions and primary sources.</caption><thead id="table-head"></thead><tbody id="projects"><tr><td colspan="8"><div class="loading-state"><span aria-hidden="true"></span><div data-ui="page_loading">Loading the index…</div></div></td></tr></tbody></table></div>
    <div id="pagination-bottom" class="pagination-slot"></div>
  </section>
  <section class="about" id="about"><div><p class="kicker" data-ui="about_kicker">Receipts included</p><h2><span data-ui="about_title_first">Understand the work</span><br><span data-ui="about_title_second">and own what you can.</span></h2></div><div class="about-copy"><p data-ui="about_body_one">This index follows projects that recover, rebuild, port, translate, and adapt software. It links claims to repositories, developer progress notes, compatibility documentation, and releases so readers can inspect the work and decide what they can run, change, and continue.</p><p data-ui="about_body_two">Listings describe code and patch projects; they do not host commercial applications, copyrighted assets, or original binaries. Uncertain project status stays labeled as uncertain.</p><a href="#about" class="source-note"><span data-ui="about_link">How entries are sourced</span> <span aria-hidden="true">↘</span></a></div></section>
</main>
<dialog id="project-details-dialog" class="project-details-dialog" aria-modal="true" aria-labelledby="project-details-title">
  <div class="dialog-shell">
    <div class="dialog-heading"><h2 id="project-details-title" tabindex="-1"></h2><button class="dialog-close" type="button" data-dialog-close data-ui-aria-label="close_details_aria" aria-label="Close project details">×</button></div>
    <div id="project-details-content" class="detail-body"></div>
  </div>
</dialog>
<dialog id="media-mode-dialog" class="media-mode-dialog" aria-modal="true" aria-labelledby="media-mode-title">
  <div class="media-mode-shell">
    <header class="media-mode-header"><a class="media-mode-brand" href="./"><svg class="ui-icon" aria-hidden="true"><use href="#icon-mark" xlink:href="#icon-mark"></use></svg>recompiled<span>.</span></a><p data-ui="media_mode_eyebrow">PROJECT REEL · A FIELD GUIDE IN MOTION</p><button class="media-mode-close" type="button" data-reel-action="close" data-ui-aria-label="media_mode_close_aria" aria-label="Close project reel"><span data-ui="media_mode_close">Close</span> <b aria-hidden="true">×</b></button></header>
    <div class="media-mode-main"><section class="media-mode-visual" data-ui-aria-label="media_mode_project_media" aria-label="Project media"><div id="media-mode-stage" class="media-mode-stage"></div><p id="media-mode-caption" class="media-mode-caption"></p></section>
      <section class="media-mode-copy"><p class="media-mode-overline"><span class="media-mode-dot"></span><span data-ui="media_mode_spotlight">COMMUNITY SOFTWARE, RECLAIMED</span></p><div class="media-mode-tags"><span id="media-mode-kind"></span><span id="media-mode-status"></span></div><h1 id="media-mode-title" tabindex="-1"></h1><p id="media-mode-target" class="media-mode-target"></p><p id="media-mode-summary" class="media-mode-summary"></p><div class="media-mode-links"><a id="media-mode-project-link" class="media-mode-primary" href="#" target="_blank" rel="noopener noreferrer"><span data-ui="media_mode_visit">Explore the project</span><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-external" xlink:href="#icon-external"></use></svg></a><a id="media-mode-source-link" class="media-mode-source" href="#" target="_blank" rel="noopener noreferrer"><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-link" xlink:href="#icon-link"></use></svg><span data-ui="media_mode_source">Open media source</span></a></div><div class="media-mode-gallery-wrap"><p class="media-mode-gallery-label" data-ui="media_mode_more_media">MORE FROM THIS PROJECT</p><div id="media-mode-gallery" class="media-mode-gallery"></div></div></section>
    </div>
    <div class="media-mode-footer"><div class="media-mode-progress"><span id="media-mode-count" role="status" aria-live="polite"></span><span class="media-mode-progress-track"><i id="media-mode-progress-fill"></i></span></div><div class="media-mode-controls"><button type="button" class="media-control" data-reel-action="previous" data-ui-aria-label="media_mode_previous_aria" aria-label="Previous project"><svg class="ui-icon" aria-hidden="true"><use href="#icon-left" xlink:href="#icon-left"></use></svg><span data-ui="media_mode_previous">Previous</span></button><button type="button" class="media-control media-control-play" data-reel-action="toggle" aria-label="Pause slideshow"><svg class="ui-icon" aria-hidden="true"><use id="media-mode-play-icon" href="#icon-pause" xlink:href="#icon-pause"></use></svg><span id="media-mode-play-label">Pause</span></button><button type="button" class="media-control" data-reel-action="next" data-ui-aria-label="media_mode_next_aria" aria-label="Next project"><span data-ui="media_mode_next">Next</span><svg class="ui-icon" aria-hidden="true"><use href="#icon-right" xlink:href="#icon-right"></use></svg></button><button type="button" class="media-control media-control-shuffle" data-reel-action="shuffle" data-ui-aria-label="media_mode_shuffle_aria" aria-label="Shuffle projects" aria-pressed="false"><svg class="ui-icon" aria-hidden="true"><use href="#icon-shuffle" xlink:href="#icon-shuffle"></use></svg><span data-ui="media_mode_shuffle">Shuffle</span></button></div></div>
  </div>
</dialog>
<dialog id="archive-terminal" class="archive-terminal" aria-modal="true" aria-labelledby="archive-terminal-title">
  <div class="terminal-shell">
    <div class="terminal-heading"><div><p class="terminal-overline" data-ui="egg_terminal_badge">ARCHIVE DEBUG CONSOLE · READ ONLY</p><h2 id="archive-terminal-title" data-ui="egg_terminal_title">Secret level</h2></div><button class="dialog-close terminal-close" type="button" data-terminal-close data-ui-aria-label="egg_terminal_close_aria" aria-label="Close terminal">×</button></div>
    <p class="terminal-welcome" data-ui="egg_terminal_welcome">A tiny toy terminal for curious players. It never runs system commands.</p>
    <div id="terminal-output" class="terminal-output" role="log" aria-live="polite" aria-relevant="additions text"></div>
    <form id="terminal-form" class="terminal-form" autocomplete="off"><label for="terminal-input" data-ui="egg_terminal_prompt">player@archive:~$</label><input id="terminal-input" type="text" spellcheck="false" autocapitalize="off" autocomplete="off" data-ui-placeholder="egg_terminal_input" data-ui-aria-label="egg_terminal_input_aria" aria-label="Enter a toy terminal command" placeholder="Try help" enterkeyhint="send"><button type="submit" class="terminal-send" data-ui="egg_terminal_send">Run</button></form>
    <p class="terminal-hint" data-ui="egg_terminal_hint">Try help · close with Esc</p>
  </div>
</dialog>
<details class="floating-nav" id="page-quick-nav">
  <summary data-ui-aria-label="quick_nav_aria" aria-label="Quick page navigation" title="Quick page navigation"><svg class="ui-icon" aria-hidden="true"><use href="#icon-grid" xlink:href="#icon-grid"></use></svg><span data-ui="quick_nav_label">Jump to</span></summary>
  <nav data-ui-aria-label="quick_nav_aria" aria-label="Quick page navigation"><button type="button" class="quick-nav-media" data-open-media-mode><svg class="ui-icon" aria-hidden="true"><use href="#icon-play" xlink:href="#icon-play"></use></svg><span data-ui="media_mode_open">Play project reel</span></button><a href="#top"><span data-ui="nav_start">Introduction</span><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-right" xlink:href="#icon-right"></use></svg></a><a href="#shift"><span data-ui="nav_why">Why this year</span><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-right" xlink:href="#icon-right"></use></svg></a><a href="#showcase"><span data-ui="nav_showcase">Popular projects</span><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-right" xlink:href="#icon-right"></use></svg></a><a href="#toolkit"><span data-ui="nav_tools">Tools &amp; sites</span><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-right" xlink:href="#icon-right"></use></svg></a><a href="#resources"><span data-ui="nav_resources">Resource guide</span><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-right" xlink:href="#icon-right"></use></svg></a><a href="#consoles"><span data-ui="nav_gallery">Console gallery</span><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-right" xlink:href="#icon-right"></use></svg></a><a href="#catalog"><span data-ui="nav_explore">Explore the index</span><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-right" xlink:href="#icon-right"></use></svg></a><a href="#about"><span data-ui="quick_nav_sources">Sources &amp; method</span><svg class="ui-icon ui-icon-small" aria-hidden="true"><use href="#icon-right" xlink:href="#icon-right"></use></svg></a></nav>
</details>
<footer><a class="brand" href="./" data-ui-aria-label="home_aria" aria-label="Recompiled home"><svg class="brand-mark ui-icon" aria-hidden="true" focusable="false"><use href="#icon-mark" xlink:href="#icon-mark"></use></svg>recompiled<span class="brand-period">.</span></a><span data-ui="footer_tagline">Made for the curious, by the curious.</span><span data-ui="footer_index">Independent community index</span></footer>
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
    randomize_ambient_motion(&document);
    let locale = preferred_locale();
    apply_language(&document, &locale);
    apply_theme(&document, preferred_theme());

    let state = Rc::new(RefCell::new(AppState {
        locale,
        sort: "updated-desc".into(),
        search_mode: "fuzzy".into(),
        page: 1,
        page_size: 25,
        ..AppState::default()
    }));
    read_url(&mut state.borrow_mut());
    sync_controls(&document, &state.borrow());
    bind_controls(&document, &state)?;
    bind_language_selector(&document, &state)?;
    bind_theme_toggle(&document)?;
    bind_quick_nav(&document)?;
    bind_media_mode(&document, &state)?;
    bind_console_gallery_controls(&document, &state)?;
    bind_easter_eggs(&document, &state)?;
    spawn_local(load_catalog(document, state));
    Ok(())
}

fn preferred_locale() -> String {
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("recompiled-language").ok().flatten())
        .map(|locale| i18n::supported(&locale).to_string())
        .unwrap_or_else(|| "en".into())
}

fn randomize_ambient_motion(document: &Document) {
    let Some(root) = document
        .document_element()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };
    let style = root.style();
    for (duration_property, phase_property, minimum_seconds, spread_seconds) in [
        (
            "--wave-current-duration",
            "--wave-current-phase",
            28.0,
            16.0,
        ),
        (
            "--wave-secondary-duration",
            "--wave-secondary-phase",
            40.0,
            22.0,
        ),
    ] {
        let duration = minimum_seconds + js_sys::Math::random() * spread_seconds;
        let phase = js_sys::Math::random() * duration;
        let _ = style.set_property(duration_property, &format!("{duration:.2}s"));
        let _ = style.set_property(phase_property, &format!("-{phase:.2}s"));
    }
}

fn apply_language(document: &Document, locale: &str) {
    let locale = i18n::supported(locale);
    if let Some(root) = document.document_element() {
        let _ = root.set_attribute("lang", locale);
    }
    document.set_title(i18n::tr(locale, "document_title"));
    if let Some(select) = document
        .get_element_by_id("site-language")
        .and_then(|element| element.dyn_into::<HtmlSelectElement>().ok())
    {
        select.set_value(locale);
        let label = i18n::tr(locale, "language_label");
        let _ = select.set_attribute("aria-label", label);
        let _ = select.set_attribute("title", label);
    }
    if let Ok(nodes) = document.query_selector_all("[data-ui]") {
        for index in 0..nodes.length() {
            let Some(element) = nodes
                .item(index)
                .and_then(|node| node.dyn_into::<Element>().ok())
            else {
                continue;
            };
            if let Some(key) = element.get_attribute("data-ui") {
                element.set_text_content(Some(i18n::tr(locale, &key)));
            }
        }
    }
    if let Ok(nodes) = document.query_selector_all("[data-ui-placeholder]") {
        for index in 0..nodes.length() {
            let Some(element) = nodes
                .item(index)
                .and_then(|node| node.dyn_into::<Element>().ok())
            else {
                continue;
            };
            if let Some(key) = element.get_attribute("data-ui-placeholder") {
                let _ = element.set_attribute("placeholder", i18n::tr(locale, &key));
            }
        }
    }
    if let Ok(nodes) = document.query_selector_all("[data-ui-aria-label]") {
        for index in 0..nodes.length() {
            let Some(element) = nodes
                .item(index)
                .and_then(|node| node.dyn_into::<Element>().ok())
            else {
                continue;
            };
            if let Some(key) = element.get_attribute("data-ui-aria-label") {
                let _ = element.set_attribute("aria-label", i18n::tr(locale, &key));
            }
        }
    }
}

fn preferred_theme() -> &'static str {
    if let Some(theme) = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("recompiled-theme").ok().flatten())
    {
        if theme == "dark" {
            return "dark";
        }
        if theme == "light" {
            return "light";
        }
    }
    let prefers_dark = web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
        })
        .is_some_and(|query| query.matches());
    if prefers_dark {
        "dark"
    } else {
        "light"
    }
}

fn apply_theme(document: &Document, theme: &str) {
    let dark = theme == "dark";
    let locale = document
        .document_element()
        .and_then(|root| root.get_attribute("lang"))
        .unwrap_or_else(|| "en".into());
    if let Some(root) = document.document_element() {
        let _ = root.set_attribute("data-theme", if dark { "dark" } else { "light" });
    }
    if let Some(button) = document.get_element_by_id("theme-toggle") {
        let _ = button.set_attribute(
            "aria-label",
            i18n::tr(
                &locale,
                if dark {
                    "theme_switch_light"
                } else {
                    "theme_switch_dark"
                },
            ),
        );
        let _ = button.set_attribute("aria-pressed", if dark { "true" } else { "false" });
    }
    if let Some(icon) = document.get_element_by_id("theme-icon") {
        let name = if dark { "sun" } else { "moon" };
        icon.set_inner_html(&icon_markup(name));
    }
    set_text(
        document,
        "theme-label",
        i18n::tr(&locale, if dark { "theme_light" } else { "theme_dark" }),
    );
    if let Ok(Some(meta)) = document.query_selector("meta[name='theme-color']") {
        let _ = meta.set_attribute("content", if dark { "#111d25" } else { "#f5f3ec" });
    }
}

fn bind_language_selector(
    document: &Document,
    state: &Rc<RefCell<AppState>>,
) -> Result<(), JsValue> {
    let Some(select) = document.get_element_by_id("site-language") else {
        return Ok(());
    };
    let document = document.clone();
    let state = Rc::clone(state);
    let callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(select) = event
            .target()
            .and_then(|target| target.dyn_into::<HtmlSelectElement>().ok())
        else {
            return;
        };
        let locale = i18n::supported(&select.value()).to_string();
        if state.borrow().locale == locale {
            return;
        }
        animate_language_change(&document);
        state.borrow_mut().locale = locale.clone();
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            let _ = storage.set_item("recompiled-language", &locale);
        }
        apply_language(&document, &locale);
        let theme = document
            .document_element()
            .and_then(|root| root.get_attribute("data-theme"))
            .unwrap_or_else(|| "light".into());
        apply_theme(&document, &theme);
        if let Some(catalog) = state.borrow().catalog.as_ref() {
            populate_filters(&document, catalog, &locale);
            render_showcase(
                &document,
                &catalog.featured_projects,
                &catalog.projects,
                &locale,
            );
            process_resources::render(&document, &catalog.process_resources, &locale);
        }
        render_console_gallery_state(&document, &state);
        sync_controls(&document, &state.borrow());
        render(&document, &state.borrow());
        render_media_mode(&document, &state.borrow());
    });
    select.add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(())
}

fn animate_language_change(document: &Document) {
    let Some(app) = document.get_element_by_id("app") else {
        return;
    };
    let _ = app.class_list().remove_1("language-changing");
    if let Some(element) = app.dyn_ref::<HtmlElement>() {
        let _ = element.offset_width();
    }
    let _ = app.class_list().add_1("language-changing");

    let Some(window) = web_sys::window() else {
        let _ = app.class_list().remove_1("language-changing");
        return;
    };
    let callback = Closure::once_into_js(move || {
        let _ = app.class_list().remove_1("language-changing");
    });
    if let Ok(callback) = callback.dyn_into::<js_sys::Function>() {
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(&callback, 1_050);
    }
}

fn bind_theme_toggle(document: &Document) -> Result<(), JsValue> {
    let Some(button) = document.get_element_by_id("theme-toggle") else {
        return Ok(());
    };
    let document = document.clone();
    let callback = Closure::<dyn FnMut(Event)>::new(move |_| {
        let current = document
            .document_element()
            .and_then(|root| root.get_attribute("data-theme"))
            .unwrap_or_else(|| "light".into());
        let next = if current == "dark" { "light" } else { "dark" };
        apply_theme(&document, next);
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            let _ = storage.set_item("recompiled-theme", next);
        }
    });
    button.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(())
}

fn bind_quick_nav(document: &Document) -> Result<(), JsValue> {
    let Some(menu) = document.get_element_by_id("page-quick-nav") else {
        return Ok(());
    };
    let callback_menu = menu.clone();
    let callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        if target.closest("a[href^='#']").ok().flatten().is_some() {
            let _ = callback_menu.remove_attribute("open");
        }
    });
    menu.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(())
}

fn bind_media_mode(document: &Document, state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let dialog = document
        .get_element_by_id("media-mode-dialog")
        .ok_or_else(|| JsValue::from_str("The project reel dialog is missing"))?
        .dyn_into::<HtmlDialogElement>()?;
    let opener = document
        .query_selector("[data-open-media-mode]")?
        .ok_or_else(|| JsValue::from_str("The project reel button is missing"))?;
    let return_focus = Rc::new(RefCell::new(None::<HtmlElement>));

    let open_dialog = dialog.clone();
    let open_document = document.clone();
    let open_state = Rc::clone(state);
    let open_return_focus = Rc::clone(&return_focus);
    let open_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        event.prevent_default();
        *open_return_focus.borrow_mut() = open_document
            .active_element()
            .and_then(|element| element.dyn_into::<HtmlElement>().ok());
        if let Some(menu) = open_document.get_element_by_id("page-quick-nav") {
            let _ = menu.remove_attribute("open");
        }
        open_state.borrow_mut().media_playing = !prefers_reduced_motion();
        render_media_mode(&open_document, &open_state.borrow());
        if !open_dialog.open() && open_dialog.show_modal().is_err() {
            return;
        }
        if let Some(title) = open_document
            .get_element_by_id("media-mode-title")
            .and_then(|element| element.dyn_into::<HtmlElement>().ok())
        {
            let _ = title.focus();
        }
    });
    opener.add_event_listener_with_callback("click", open_callback.as_ref().unchecked_ref())?;
    open_callback.forget();

    let click_dialog = dialog.clone();
    let click_document = document.clone();
    let click_state = Rc::clone(state);
    let click_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        if target.id() == "media-mode-dialog" {
            click_dialog.close();
            return;
        }
        if let Ok(Some(button)) = target.closest("[data-reel-media]") {
            if let Some(position) = button
                .get_attribute("data-reel-media")
                .and_then(|value| value.parse::<usize>().ok())
            {
                click_state.borrow_mut().media_asset_position = position;
                render_media_mode(&click_document, &click_state.borrow());
            }
            return;
        }
        let Ok(Some(button)) = target.closest("[data-reel-action]") else {
            return;
        };
        let action = button.get_attribute("data-reel-action").unwrap_or_default();
        if action == "close" {
            click_dialog.close();
            return;
        }
        {
            let mut app = click_state.borrow_mut();
            match action.as_str() {
                "toggle" => app.media_playing = !app.media_playing,
                "next" => advance_media_mode(&mut app, 1),
                "previous" => advance_media_mode(&mut app, -1),
                "shuffle" => {
                    let current = app.media_projects.get(app.media_position).copied();
                    app.media_shuffle = !app.media_shuffle;
                    if app.media_shuffle {
                        shuffle_media_projects(&mut app.media_projects);
                    } else {
                        app.media_projects.sort_unstable();
                    }
                    app.media_position = current
                        .and_then(|current| {
                            app.media_projects
                                .iter()
                                .position(|project| *project == current)
                        })
                        .unwrap_or(0);
                    app.media_asset_position = 0;
                }
                _ => return,
            }
        }
        render_media_mode(&click_document, &click_state.borrow());
    });
    dialog.add_event_listener_with_callback("click", click_callback.as_ref().unchecked_ref())?;
    click_callback.forget();

    let close_state = Rc::clone(state);
    let close_return_focus = Rc::clone(&return_focus);
    let close_dialog_video = dialog.clone();
    let close_callback = Closure::<dyn FnMut(Event)>::new(move |_| {
        close_state.borrow_mut().media_playing = false;
        if let Ok(Some(video)) = close_dialog_video.query_selector("video") {
            if let Ok(video) = video.dyn_into::<HtmlVideoElement>() {
                video.pause().ok();
            }
        }
        if let Some(invoker) = close_return_focus.borrow_mut().take() {
            let _ = invoker.focus();
        }
    });
    dialog.add_event_listener_with_callback("close", close_callback.as_ref().unchecked_ref())?;
    close_callback.forget();

    let timer_dialog = dialog.clone();
    let timer_document = document.clone();
    let timer_state = Rc::clone(state);
    let timer_callback = Closure::<dyn FnMut()>::new(move || {
        if !timer_dialog.open() || !timer_state.borrow().media_playing {
            return;
        }
        if let Ok(Some(video)) = timer_dialog.query_selector("video") {
            if video
                .dyn_into::<HtmlVideoElement>()
                .is_ok_and(|video| !video.paused())
            {
                return;
            }
        }
        advance_media_mode(&mut timer_state.borrow_mut(), 1);
        render_media_mode(&timer_document, &timer_state.borrow());
    });
    if let Some(window) = web_sys::window() {
        let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
            timer_callback.as_ref().unchecked_ref(),
            9_000,
        );
    }
    timer_callback.forget();
    Ok(())
}

fn prefers_reduced_motion() -> bool {
    web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|query| query.matches())
}

fn media_project_indices(projects: &[Project]) -> Vec<usize> {
    projects
        .iter()
        .enumerate()
        .filter(|(_, project)| {
            project
                .media
                .iter()
                .any(|media| !safe_link(&media.url).is_empty())
        })
        .map(|(index, _)| index)
        .collect()
}

fn shuffle_media_projects(projects: &mut [usize]) {
    for end in (1..projects.len()).rev() {
        let next = (js_sys::Math::random() * (end + 1) as f64).floor() as usize;
        projects.swap(end, next);
    }
}

fn advance_media_mode(state: &mut AppState, direction: isize) {
    if state.media_projects.is_empty() {
        state.media_position = 0;
        state.media_asset_position = 0;
        return;
    }
    let count = state.media_projects.len() as isize;
    state.media_position = (state.media_position as isize + direction).rem_euclid(count) as usize;
    state.media_asset_position = 0;
}

fn media_stage_markup(item: &media::Media, locale: &str) -> String {
    let url = safe_link(&item.url);
    let preview = item
        .preview_url
        .as_deref()
        .map(safe_link)
        .filter(|url| !url.is_empty())
        .unwrap_or_else(|| {
            if item.kind == "video" {
                String::new()
            } else {
                url.clone()
            }
        });
    let label = escape_html(&item.label);
    let safe_url = escape_html(&url);
    let safe_preview = escape_html(&preview);
    let is_video = item.kind == "video";
    if is_video
        && url
            .split('?')
            .next()
            .is_some_and(|url| url.ends_with(".mp4"))
    {
        let poster = if preview.is_empty() {
            String::new()
        } else {
            format!(" poster=\"{safe_preview}\"")
        };
        return format!(
            "<video class=\"media-mode-video\" controls playsinline preload=\"metadata\" aria-label=\"{}\"{}><source src=\"{}\" type=\"video/mp4\">{}</video>",
            label,
            poster,
            safe_url,
            i18n::tr(locale, "media_mode_video_fallback")
        );
    }
    if is_video && url.is_empty() {
        return format!(
            "<div class=\"media-mode-placeholder\"><svg class=\"ui-icon\" aria-hidden=\"true\"><use href=\"#icon-image\" xlink:href=\"#icon-image\"></use></svg><span>{}</span></div>",
            i18n::tr(locale, "media_mode_no_preview")
        );
    }
    let action = if is_video && url.contains("youtube.com/") {
        i18n::tr(locale, "watch_youtube")
    } else if is_video {
        i18n::tr(locale, "open_video")
    } else {
        i18n::tr(locale, "open_image")
    };
    if preview.is_empty() {
        return format!(
            "<a class=\"media-mode-unpreviewed\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\"><svg class=\"ui-icon\" aria-hidden=\"true\"><use href=\"#icon-play\" xlink:href=\"#icon-play\"></use></svg><span>{}</span></a>",
            safe_url, action
        );
    }
    let overlay = if is_video {
        format!(
            "<span class=\"media-mode-play\"><svg class=\"ui-icon\" aria-hidden=\"true\"><use href=\"#icon-play\" xlink:href=\"#icon-play\"></use></svg><span>{}</span></span>",
            action
        )
    } else {
        format!(
            "<span class=\"media-mode-image-badge\">{}</span>",
            i18n::tr(locale, "media_mode_image")
        )
    };
    format!(
        "<a class=\"media-mode-preview\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" aria-label=\"{}: {}\"><img src=\"{}\" alt=\"{}\" decoding=\"async\" referrerpolicy=\"no-referrer\"><span class=\"media-mode-scanlines\" aria-hidden=\"true\"></span>{}</a>",
        safe_url, action, label, safe_preview, label, overlay
    )
}

fn render_media_mode(document: &Document, state: &AppState) {
    let locale = &state.locale;
    let Some(catalog) = state.catalog.as_ref() else {
        set_text(
            document,
            "media-mode-title",
            i18n::tr(locale, "media_mode_loading"),
        );
        return;
    };
    if state.media_projects.is_empty() {
        set_text(
            document,
            "media-mode-title",
            i18n::tr(locale, "media_mode_empty_title"),
        );
        set_text(
            document,
            "media-mode-summary",
            i18n::tr(locale, "media_mode_empty_body"),
        );
        set_html(
            document,
            "media-mode-stage",
            "<div class=\"media-mode-placeholder\"><span>RECOMPILED</span></div>",
        );
        set_html(document, "media-mode-gallery", "");
        set_text(document, "media-mode-count", "0 / 0");
        return;
    }
    let project_index = state.media_projects[state.media_position % state.media_projects.len()];
    let Some(project) = catalog.projects.get(project_index) else {
        return;
    };
    let assets = project
        .media
        .iter()
        .enumerate()
        .filter(|(_, item)| !safe_link(&item.url).is_empty())
        .collect::<Vec<_>>();
    if assets.is_empty() {
        return;
    }
    let media_position = state.media_asset_position % assets.len();
    let (_, item) = assets[media_position];
    set_html(
        document,
        "media-mode-stage",
        &media_stage_markup(item, locale),
    );
    set_text(document, "media-mode-caption", &item.label);
    set_text(document, "media-mode-title", &project.title);
    set_text(document, "media-mode-summary", &project.summary);
    let target_label = if project.target_type == "Software" {
        i18n::tr(locale, "target_software")
    } else {
        i18n::tr(locale, "target_game")
    };
    set_text(
        document,
        "media-mode-target",
        &format!("{target_label} · {} · {}", project.game, project.platform),
    );
    set_text(
        document,
        "media-mode-kind",
        localized_kind(locale, &project.kind),
    );
    set_text(
        document,
        "media-mode-status",
        i18n::tr(locale, status_key(status_bucket(&project.status))),
    );
    if let Some(link) = document.get_element_by_id("media-mode-project-link") {
        let url = safe_link(&project.project_url);
        let _ = link.set_attribute("href", if url.is_empty() { "#" } else { &url });
    }
    if let Some(link) = document.get_element_by_id("media-mode-source-link") {
        let url = safe_link(&item.source_url);
        let _ = link.set_attribute("href", if url.is_empty() { "#" } else { &url });
    }
    let thumbnails = assets
        .iter()
        .enumerate()
        .map(|(position, (_, asset))| {
            let preview = asset
                .preview_url
                .as_deref()
                .map(safe_link)
                .filter(|url| !url.is_empty())
                .unwrap_or_else(|| {
                    if asset.kind == "video" {
                        String::new()
                    } else {
                        safe_link(&asset.url)
                    }
                });
            let thumbnail = if preview.is_empty() {
                "<span class=\"media-mode-thumb-empty\"></span>".to_string()
            } else {
                format!(
                    "<img src=\"{}\" alt=\"\" loading=\"lazy\" decoding=\"async\" referrerpolicy=\"no-referrer\">",
                    escape_html(&preview)
                )
            };
            let selected = if position == media_position { "true" } else { "false" };
            format!(
                "<button type=\"button\" class=\"media-mode-thumb\" data-reel-media=\"{}\" aria-label=\"{} {}\" aria-pressed=\"{}\" title=\"{}\">{}<span class=\"media-mode-thumb-kind\">{}</span></button>",
                position,
                i18n::tr(locale, "media_mode_show_asset"),
                position + 1,
                selected,
                escape_html(&asset.label),
                thumbnail,
                if asset.kind == "video" { "▶" } else { "" }
            )
        })
        .collect::<String>();
    set_html(document, "media-mode-gallery", &thumbnails);
    set_text(
        document,
        "media-mode-count",
        &format!(
            "{:02} / {:02} {}",
            state.media_position + 1,
            state.media_projects.len(),
            i18n::tr(locale, "media_mode_projects")
        ),
    );
    if let Some(fill) = document.get_element_by_id("media-mode-progress-fill") {
        if let Some(fill) = fill.dyn_ref::<HtmlElement>() {
            let progress = (state.media_position + 1) as f64 / state.media_projects.len() as f64;
            let _ = fill
                .style()
                .set_property("width", &format!("{:.2}%", progress * 100.0));
        }
    }
    if let Some(button) = document
        .query_selector("[data-reel-action='toggle']")
        .ok()
        .flatten()
    {
        let label = i18n::tr(
            locale,
            if state.media_playing {
                "media_mode_pause"
            } else {
                "media_mode_play"
            },
        );
        let _ = button.set_attribute("aria-label", label);
        if let Some(text) = document.get_element_by_id("media-mode-play-label") {
            text.set_text_content(Some(label));
        }
        if let Ok(Some(icon)) = document.query_selector("#media-mode-play-icon") {
            let icon_name = if state.media_playing { "pause" } else { "play" };
            let _ = icon.set_attribute("href", &format!("#icon-{icon_name}"));
            let _ = icon.set_attribute("xlink:href", &format!("#icon-{icon_name}"));
        }
    }
    if let Some(button) = document
        .query_selector("[data-reel-action='shuffle']")
        .ok()
        .flatten()
    {
        let _ = button.set_attribute(
            "aria-pressed",
            if state.media_shuffle { "true" } else { "false" },
        );
    }
}

fn bind_console_gallery_controls(
    document: &Document,
    state: &Rc<RefCell<AppState>>,
) -> Result<(), JsValue> {
    let Some(container) = document.get_element_by_id("console-gallery-controls") else {
        return Ok(());
    };
    let input_document = document.clone();
    let input_state = Rc::clone(state);
    let input_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(input) = event
            .target()
            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
        else {
            return;
        };
        if input.id() != "console-search" {
            return;
        }
        input_state.borrow_mut().console_query = input.value();
        render_console_gallery_state(&input_document, &input_state);
    });
    container.add_event_listener_with_callback("input", input_callback.as_ref().unchecked_ref())?;
    input_callback.forget();

    let change_document = document.clone();
    let change_state = Rc::clone(state);
    let change_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(select) = event
            .target()
            .and_then(|target| target.dyn_into::<HtmlSelectElement>().ok())
        else {
            return;
        };
        match select.id().as_str() {
            "filter-console-maker" => change_state.borrow_mut().console_maker = select.value(),
            "filter-console-type" => change_state.borrow_mut().console_type = select.value(),
            _ => return,
        }
        render_console_gallery_state(&change_document, &change_state);
    });
    container
        .add_event_listener_with_callback("change", change_callback.as_ref().unchecked_ref())?;
    change_callback.forget();
    Ok(())
}

fn render_console_gallery_state(document: &Document, state: &Rc<RefCell<AppState>>) {
    let state = state.borrow();
    if let Some(catalog) = state.catalog.as_ref() {
        console_gallery::populate_controls(
            document,
            &catalog.console_gallery,
            &state.locale,
            &state.console_maker,
        );
        console_gallery::render(
            document,
            &catalog.console_gallery,
            &state.locale,
            &state.console_query,
            &state.console_maker,
            &state.console_type,
        );
    }
}

fn append_terminal_line(document: &Document, output: &Element, text: &str) {
    let Ok(line) = document.create_element("p") else {
        return;
    };
    line.set_class_name("terminal-line");
    line.set_text_content(Some(text));
    let _ = output.append_child(&line);
}

fn open_archive_terminal(
    document: &Document,
    dialog: &HtmlDialogElement,
    input: &HtmlInputElement,
    output: &Element,
    return_focus: &Rc<RefCell<Option<HtmlElement>>>,
    locale: &str,
) {
    if !dialog.open() {
        *return_focus.borrow_mut() = document
            .active_element()
            .and_then(|element| element.dyn_into::<HtmlElement>().ok());
        if dialog.show_modal().is_err() {
            return;
        }
    }
    append_terminal_line(document, output, i18n::tr(locale, "egg_unlock"));
    let _ = input.focus();
}

fn bind_easter_eggs(document: &Document, state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let dialog = document
        .get_element_by_id("archive-terminal")
        .ok_or_else(|| JsValue::from_str("The secret terminal is missing"))?
        .dyn_into::<HtmlDialogElement>()?;
    let input = document
        .get_element_by_id("terminal-input")
        .ok_or_else(|| JsValue::from_str("The secret terminal input is missing"))?
        .dyn_into::<HtmlInputElement>()?;
    let output = document
        .get_element_by_id("terminal-output")
        .ok_or_else(|| JsValue::from_str("The secret terminal output is missing"))?;
    let return_focus = Rc::new(RefCell::new(None::<HtmlElement>));

    let close_dialog = dialog.clone();
    let close_click = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        if target
            .closest("[data-terminal-close]")
            .ok()
            .flatten()
            .is_some()
            || target.id() == "archive-terminal"
        {
            close_dialog.close();
        }
    });
    dialog.add_event_listener_with_callback("click", close_click.as_ref().unchecked_ref())?;
    close_click.forget();

    let focus_after_close = Rc::clone(&return_focus);
    let close_focus = Closure::<dyn FnMut(Event)>::new(move |_| {
        if let Some(invoker) = focus_after_close.borrow_mut().take() {
            let _ = invoker.focus();
        }
    });
    dialog.add_event_listener_with_callback("close", close_focus.as_ref().unchecked_ref())?;
    close_focus.forget();

    let click_dialog = dialog.clone();
    let click_input = input.clone();
    let click_output = output.clone();
    let click_document = document.clone();
    let click_focus = Rc::clone(&return_focus);
    let click_state = Rc::clone(state);
    let tap_count = Rc::new(RefCell::new(0_u8));
    let tap_position = Rc::clone(&tap_count);
    let click_secret = Closure::<dyn FnMut(Event)>::new(move |_| {
        let next = {
            let mut count = tap_position.borrow_mut();
            *count = count.saturating_add(1);
            if *count >= 7 {
                *count = 0;
                true
            } else {
                false
            }
        };
        if next {
            let locale = click_state.borrow().locale.clone();
            open_archive_terminal(
                &click_document,
                &click_dialog,
                &click_input,
                &click_output,
                &click_focus,
                &locale,
            );
        }
    });
    document
        .get_element_by_id("hero-secret-button")
        .ok_or_else(|| JsValue::from_str("The arcade bonus button is missing"))?
        .add_event_listener_with_callback("click", click_secret.as_ref().unchecked_ref())?;
    click_secret.forget();

    let key_dialog = dialog.clone();
    let key_input = input.clone();
    let key_output = output.clone();
    let key_document = document.clone();
    let key_focus = Rc::clone(&return_focus);
    let key_state = Rc::clone(state);
    let key_progress = Rc::new(RefCell::new(0_usize));
    let sequence_progress = Rc::clone(&key_progress);
    let keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        if event.repeat() {
            return;
        }
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        if target
            .closest("input, textarea, select, [contenteditable='true']")
            .ok()
            .flatten()
            .is_some()
        {
            return;
        }
        let sequence = [
            "arrowup",
            "arrowup",
            "arrowdown",
            "arrowdown",
            "arrowleft",
            "arrowright",
            "arrowleft",
            "arrowright",
            "b",
            "a",
        ];
        let key = event.key().to_ascii_lowercase();
        let mut progress = sequence_progress.borrow_mut();
        if key == sequence[*progress] {
            *progress += 1;
        } else {
            *progress = usize::from(key == sequence[0]);
        }
        if *progress == sequence.len() {
            *progress = 0;
            event.prevent_default();
            let locale = key_state.borrow().locale.clone();
            open_archive_terminal(
                &key_document,
                &key_dialog,
                &key_input,
                &key_output,
                &key_focus,
                &locale,
            );
        }
    });
    document.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
    keydown.forget();

    let submit_document = document.clone();
    let submit_output = output.clone();
    let submit_dialog = dialog.clone();
    let submit_input = input.clone();
    let submit_state = Rc::clone(state);
    let form = document
        .get_element_by_id("terminal-form")
        .ok_or_else(|| JsValue::from_str("The secret terminal form is missing"))?;
    let submit = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        event.prevent_default();
        let locale = submit_state.borrow().locale.clone();
        let raw = submit_input
            .value()
            .trim()
            .chars()
            .take(120)
            .collect::<String>();
        if raw.is_empty() {
            return;
        }
        let command = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        let normalized = command.to_ascii_lowercase();
        let echo = i18n::tr(&locale, "egg_echo").replace("{command}", &command);
        append_terminal_line(&submit_document, &submit_output, &echo);
        submit_input.set_value("");
        let response = match normalized.as_str() {
            "help" => Some(i18n::tr(&locale, "egg_help").to_string()),
            "whoami" => Some(i18n::tr(&locale, "egg_whoami").to_string()),
            "ls" => Some(i18n::tr(&locale, "egg_ls").to_string()),
            "sudo recompile" => Some(i18n::tr(&locale, "egg_sudo").to_string()),
            "matrix" => {
                if let Some(root) = submit_document.document_element() {
                    let _ = root.set_attribute("data-egg-mode", "matrix");
                }
                Some(i18n::tr(&locale, "egg_matrix").to_string())
            }
            "newgame" => {
                if let Some(root) = submit_document.document_element() {
                    let _ = root.set_attribute("data-egg-mode", "player-two");
                }
                Some(i18n::tr(&locale, "egg_newgame").to_string())
            }
            "crt" => {
                if let Some(root) = submit_document.document_element() {
                    let _ = root.set_attribute("data-egg-mode", "crt");
                }
                Some(i18n::tr(&locale, "egg_crt").to_string())
            }
            "idkfa" => Some(i18n::tr(&locale, "egg_idkfa").to_string()),
            "roll" => {
                let roll = (js_sys::Math::random() * 20.0).floor() as u8 + 1;
                Some(i18n::tr(&locale, "egg_roll").replace("{roll}", &roll.to_string()))
            }
            "reset" => {
                if let Some(root) = submit_document.document_element() {
                    let _ = root.remove_attribute("data-egg-mode");
                }
                Some(i18n::tr(&locale, "egg_reset").to_string())
            }
            "clear" => {
                submit_output.set_text_content(None);
                Some(i18n::tr(&locale, "egg_clear").to_string())
            }
            "exit" | "quit" => {
                submit_dialog.close();
                None
            }
            _ => Some(i18n::tr(&locale, "egg_unknown").to_string()),
        };
        if let Some(response) = response {
            append_terminal_line(&submit_document, &submit_output, &response);
        }
    });
    form.add_event_listener_with_callback("submit", submit.as_ref().unchecked_ref())?;
    submit.forget();
    Ok(())
}

fn bind_controls(document: &Document, state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let catalog = document
        .get_element_by_id("catalog")
        .ok_or_else(|| JsValue::from_str("Catalog is missing"))?;
    let dialog = document
        .get_element_by_id("project-details-dialog")
        .ok_or_else(|| JsValue::from_str("Project details dialog is missing"))?
        .dyn_into::<web_sys::HtmlDialogElement>()?;
    let dialog_title = document
        .get_element_by_id("project-details-title")
        .ok_or_else(|| JsValue::from_str("Project details dialog title is missing"))?;
    let dialog_content = document
        .get_element_by_id("project-details-content")
        .ok_or_else(|| JsValue::from_str("Project details dialog content is missing"))?;
    let dialog_invoker = Rc::new(RefCell::new(None::<HtmlElement>));

    let close_dialog = dialog.clone();
    let close_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        if target
            .closest("[data-dialog-close]")
            .ok()
            .flatten()
            .is_some()
        {
            close_dialog.close();
        }
    });
    dialog.add_event_listener_with_callback("click", close_callback.as_ref().unchecked_ref())?;
    close_callback.forget();

    let return_focus_invoker = Rc::clone(&dialog_invoker);
    let close_event_callback = Closure::<dyn FnMut(Event)>::new(move |_| {
        if let Some(button) = return_focus_invoker.borrow_mut().take() {
            let _ = button.set_attribute("aria-expanded", "false");
            let _ = button.focus();
        }
    });
    dialog
        .add_event_listener_with_callback("close", close_event_callback.as_ref().unchecked_ref())?;
    close_event_callback.forget();

    let search_state = Rc::clone(state);
    let search_document = document.clone();
    let input_scheduled = Rc::new(Cell::new(false));
    let callback_scheduled = Rc::clone(&input_scheduled);
    let input_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(input) = event
            .target()
            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
        else {
            return;
        };
        if input.id() != "search" {
            return;
        }
        search_state.borrow_mut().query = input.value();
        search_state.borrow_mut().page = 1;
        if input_scheduled.replace(true) {
            return;
        }
        let scheduled = Rc::clone(&callback_scheduled);
        let document = search_document.clone();
        let state = Rc::clone(&search_state);
        let callback = Closure::once_into_js(move || {
            scheduled.set(false);
            render(&document, &state.borrow());
        });
        let Ok(callback) = callback.dyn_into::<js_sys::Function>() else {
            input_scheduled.set(false);
            render(&search_document, &search_state.borrow());
            return;
        };
        let Some(window) = web_sys::window() else {
            input_scheduled.set(false);
            let _ = callback.call0(&JsValue::NULL);
            return;
        };
        if window.request_animation_frame(&callback).is_err() {
            input_scheduled.set(false);
            let _ = callback.call0(&JsValue::NULL);
        }
    });
    catalog.add_event_listener_with_callback("input", input_callback.as_ref().unchecked_ref())?;
    input_callback.forget();

    let change_state = Rc::clone(state);
    let change_document = document.clone();
    let change_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let value = if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
            select.value()
        } else if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
            input.value()
        } else {
            return;
        };
        let mut state = change_state.borrow_mut();
        match target.id().as_str() {
            "sort" => state.sort = value,
            "search-mode" => {
                state.search_mode = if value == "verbatim" {
                    value
                } else {
                    "fuzzy".into()
                }
            }
            "page-size" => state.page_size = parse_page_size(&value),
            id => set_filter_value(&mut state, id, value),
        }
        state.page = 1;
        drop(state);
        sync_controls(&change_document, &change_state.borrow());
        render(&change_document, &change_state.borrow());
    });
    catalog.add_event_listener_with_callback("change", change_callback.as_ref().unchecked_ref())?;
    change_callback.forget();

    let click_state = Rc::clone(state);
    let click_document = document.clone();
    let click_dialog = dialog.clone();
    let click_dialog_title = dialog_title.clone();
    let click_dialog_content = dialog_content.clone();
    let click_dialog_invoker = Rc::clone(&dialog_invoker);
    let click_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        if let Ok(Some(button)) = target.closest("[data-details]") {
            if click_dialog.open() {
                return;
            }
            let Some(detail_id) = button.get_attribute("data-details") else {
                return;
            };
            let Some(row) = click_document.get_element_by_id(&detail_id) else {
                return;
            };
            let Some(detail_body) = row.query_selector(".detail-body").ok().flatten() else {
                return;
            };
            click_dialog_content.set_inner_html(&detail_body.inner_html());
            let title = button.get_attribute("aria-label").unwrap_or_default();
            click_dialog_title.set_text_content(Some(&title));
            *click_dialog_invoker.borrow_mut() = button.clone().dyn_into::<HtmlElement>().ok();
            let _ = button.set_attribute("aria-expanded", "true");
            if click_dialog.show_modal().is_ok() {
                if let Ok(title) = click_dialog_title.clone().dyn_into::<HtmlElement>() {
                    let _ = title.focus();
                }
            } else {
                *click_dialog_invoker.borrow_mut() = None;
                let _ = button.set_attribute("aria-expanded", "false");
            }
            return;
        }
        let mut state = click_state.borrow_mut();
        if let Ok(Some(button)) = target.closest("[data-kind]") {
            let kind = button
                .get_attribute("data-kind")
                .unwrap_or_else(|| "all".into());
            if kind == "all" {
                state.kinds.clear();
            } else if state.kinds.remove(&kind) { /* Toggle the active type off. */
            } else if supported_project_kind(&kind) {
                state.kinds.insert(kind);
            }
            state.page = 1;
        } else if let Ok(Some(button)) = target.closest("[data-sort]") {
            let key = button
                .get_attribute("data-sort")
                .unwrap_or_else(|| "title".into());
            state.sort = format!(
                "{}-{}",
                key,
                if state.sort == format!("{key}-asc") {
                    "desc"
                } else {
                    "asc"
                }
            );
            state.page = 1;
        } else if let Ok(Some(button)) = target.closest("[data-page]") {
            state.page = button
                .get_attribute("data-page")
                .and_then(|value| value.parse().ok())
                .unwrap_or(1);
        } else if let Ok(Some(_)) = target.closest("#clear-filters") {
            state.query.clear();
            state.kinds.clear();
            state.platform.clear();
            state.strategy.clear();
            state.status.clear();
            state.source_available.clear();
            state.target_type.clear();
            state.language.clear();
            state.programming_language.clear();
            state.genre.clear();
            state.original_platform.clear();
            state.first_found_since.clear();
            state.project_started_by.clear();
            state.media_kind.clear();
            state.page = 1;
        } else {
            return;
        }
        drop(state);
        sync_controls(&click_document, &click_state.borrow());
        render(&click_document, &click_state.borrow());
        if target.closest("[data-page]").ok().flatten().is_some() {
            if let Some(top) = click_document
                .get_element_by_id("table-top")
                .and_then(|node| node.dyn_into::<HtmlElement>().ok())
            {
                let _ = top.focus();
            }
        }
    });
    catalog.add_event_listener_with_callback("click", click_callback.as_ref().unchecked_ref())?;
    click_callback.forget();
    Ok(())
}

fn search_tokens(text: &str) -> Vec<String> {
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
fn search_matches(
    project_text: &str,
    project_terms: &[String],
    profile_fields: Option<&[String]>,
    query: &str,
    mode: &str,
) -> bool {
    let query_terms = search_tokens(query);
    let profile_terms = profile_fields.map(|fields| {
        fields
            .iter()
            .flat_map(|field| search_tokens(field))
            .collect::<Vec<_>>()
    });
    search_matches_prepared(
        project_text,
        project_terms,
        profile_fields,
        profile_terms.as_deref(),
        query,
        &query_terms,
        mode,
    )
}

fn search_matches_prepared(
    project_text: &str,
    project_terms: &[String],
    profile_fields: Option<&[String]>,
    profile_terms: Option<&[String]>,
    query: &str,
    query_terms: &[String],
    mode: &str,
) -> bool {
    if mode == "verbatim" {
        return project_text.contains(query)
            || profile_fields
                .is_some_and(|fields| fields.iter().any(|field| field.contains(query)));
    }

    !query_terms.is_empty()
        && query_terms.iter().all(|query_term| {
            project_terms
                .iter()
                .any(|candidate| fuzzy_word_matches(query_term, candidate))
                || profile_terms.is_some_and(|terms| {
                    terms
                        .iter()
                        .any(|candidate| fuzzy_word_matches(query_term, candidate))
                })
        })
}

fn fuzzy_word_matches(query: &str, candidate: &str) -> bool {
    if candidate.contains(query) {
        return true;
    }

    let query_chars: Vec<char> = query.chars().collect();
    let candidate_chars: Vec<char> = candidate.chars().collect();
    if query_chars.len() < 3 {
        return false;
    }

    let max_distance = if query_chars.len() <= 7 { 1 } else { 2 };
    if query_chars.len().abs_diff(candidate_chars.len()) > max_distance {
        return false;
    }

    let mut two_back = vec![0; candidate_chars.len() + 1];
    let mut previous: Vec<usize> = (0..=candidate_chars.len()).collect();
    let mut current = vec![0; candidate_chars.len() + 1];
    for (query_index, query_char) in query_chars.iter().enumerate() {
        current[0] = query_index + 1;
        for (candidate_index, candidate_char) in candidate_chars.iter().enumerate() {
            let row = query_index + 1;
            let column = candidate_index + 1;
            let substitution_cost = usize::from(query_char != candidate_char);
            let mut distance = (previous[column] + 1)
                .min(current[column - 1] + 1)
                .min(previous[column - 1] + substitution_cost);
            if row > 1
                && column > 1
                && query_chars[row - 1] == candidate_chars[column - 2]
                && query_chars[row - 2] == candidate_chars[column - 1]
            {
                distance = distance.min(two_back[column - 2] + 1);
            }
            current[column] = distance;
        }
        std::mem::swap(&mut two_back, &mut previous);
        std::mem::swap(&mut previous, &mut current);
    }
    previous[candidate_chars.len()] <= max_distance
}

async fn load_catalog(document: Document, state: Rc<RefCell<AppState>>) {
    let result = async {
        let window =
            web_sys::window().ok_or_else(|| JsValue::from_str("Browser window is unavailable"))?;
        let response = JsFuture::from(window.fetch_with_str("./catalog.json")).await?;
        let response: Response = response.dyn_into()?;
        if !response.ok() {
            return Err(JsValue::from_str("Could not load the project index"));
        }
        let text = JsFuture::from(response.text()?).await?;
        let raw = text
            .as_string()
            .ok_or_else(|| JsValue::from_str("Catalog response is not text"))?;
        let mut catalog = serde_json::from_str::<Catalog>(&raw)
            .map_err(|error| JsValue::from_str(&format!("Catalog data is invalid: {error}")))?;
        catalog.prepare_indexes();
        Ok(catalog)
    }
    .await;

    match result {
        Ok(catalog) => {
            let locale = state.borrow().locale.clone();
            populate_filters(&document, &catalog, &locale);
            render_directory(&document, &catalog.directory_items, &locale);
            render_showcase(
                &document,
                &catalog.featured_projects,
                &catalog.projects,
                &locale,
            );
            process_resources::render(&document, &catalog.process_resources, &locale);
            state.borrow_mut().media_projects = media_project_indices(&catalog.projects);
            state.borrow_mut().catalog = Some(catalog);
            render_console_gallery_state(&document, &state);
            sync_controls(&document, &state.borrow());
            set_status(&document, &locale, "status_online");
            if let Some(kicker) = document.get_element_by_id("status-line") {
                if let Some(parent) = kicker.parent_element() {
                    let _ = parent.class_list().add_1("is-live");
                }
            }
            render(&document, &state.borrow());
            render_media_mode(&document, &state.borrow());
        }
        Err(error) => {
            web_sys::console::error_1(&error);
            let locale = state.borrow().locale.clone();
            set_html(&document, "projects", &format!("<tr><td colspan=\"8\"><div class=\"empty-state\"><h3>{}</h3><p>{}</p></div></td></tr>", i18n::tr(&locale, "catalog_failed"), i18n::tr(&locale, "catalog_reload")));
            set_status(&document, &locale, "status_unavailable");
        }
    }
}

fn render(document: &Document, state: &AppState) {
    let Some(catalog) = state.catalog.as_ref() else {
        return;
    };
    let needle = state.query.trim().to_lowercase();
    let query_terms = search_tokens(&needle);
    let mut projects: Vec<&Project> = catalog
        .projects
        .iter()
        .filter(|project| {
            if !(state.kinds.is_empty() || state.kinds.contains(&project.kind))
                || !(state.platform.is_empty() || project.platform == state.platform)
                || !(state.strategy.is_empty() || project.strategy == state.strategy)
                || !(state.status.is_empty() || status_bucket(&project.status) == state.status)
                || !(state.source_available.is_empty()
                    || project.source_available == state.source_available)
                || !(state.target_type.is_empty() || project.target_type == state.target_type)
                || !(state.language.is_empty() || project.language == state.language)
                || !(state.programming_language.is_empty()
                    || project.programming_language == state.programming_language)
                || !(state.media_kind.is_empty()
                    || if state.media_kind == "none" {
                        project.media.is_empty()
                    } else {
                        project
                            .media
                            .iter()
                            .any(|item| item.kind == state.media_kind)
                    })
                || !(state.first_found_since.is_empty()
                    || project
                        .first_found
                        .as_deref()
                        .is_some_and(|value| value >= state.first_found_since.as_str()))
                || !(state.project_started_by.is_empty()
                    || project
                        .project_started
                        .as_deref()
                        .is_some_and(|value| value <= state.project_started_by.as_str()))
            {
                return false;
            }

            let game_profile = if state.genre.is_empty() && state.original_platform.is_empty() {
                None
            } else {
                catalog.profile_for(&project.game)
            };
            if (!state.genre.is_empty() && !profile_has_value(game_profile, "genre", &state.genre))
                || (!state.original_platform.is_empty()
                    && !profile_has_value(
                        game_profile,
                        "original_platform",
                        &state.original_platform,
                    ))
            {
                return false;
            }

            if needle.is_empty() {
                return true;
            }
            search_matches_prepared(
                &project.search_text,
                &project.search_terms,
                catalog
                    .profile_search
                    .get(project.game.as_str())
                    .map(Vec::as_slice),
                catalog
                    .profile_search_terms
                    .get(project.game.as_str())
                    .map(Vec::as_slice),
                &needle,
                &query_terms,
                &state.search_mode,
            )
        })
        .collect();

    let (sort_key, direction) = state.sort.rsplit_once('-').unwrap_or(("updated", "desc"));
    let descending = direction == "desc";
    projects.sort_by(|a, b| {
        let comparison = match sort_key {
            "updated" => {
                return date_order(&a.updated, &b.updated, descending)
                    .then_with(|| a.title.cmp(&b.title))
            }
            "found" => {
                return date_order(&a.first_found, &b.first_found, descending)
                    .then_with(|| a.title.cmp(&b.title))
            }
            "title" => a.title_sort_key.cmp(&b.title_sort_key),
            "kind" => a.kind.cmp(&b.kind),
            "platform" => a.platform.cmp(&b.platform),
            "strategy" => a.strategy.cmp(&b.strategy),
            "status" => status_bucket(&a.status).cmp(status_bucket(&b.status)),
            "language" => a.language.cmp(&b.language),
            "target-type" => a.target_type.cmp(&b.target_type),
            _ => a.title.cmp(&b.title),
        };
        (if descending {
            comparison.reverse()
        } else {
            comparison
        })
        .then_with(|| a.title.cmp(&b.title))
    });

    set_text(
        document,
        "project-total",
        &catalog.projects.len().to_string(),
    );
    set_text(document, "target-total", &catalog.target_count.to_string());
    set_text(document, "source-total", &catalog.source_count.to_string());
    set_text(
        document,
        "catalog-updated",
        &format_date(&catalog.updated, &state.locale),
    );
    let count_label = i18n::tr(
        &state.locale,
        if projects.len() == 1 {
            "result_one"
        } else {
            "result_many"
        },
    );
    set_text(
        document,
        "result-count",
        &format!("{} {}", projects.len(), count_label),
    );

    let total_pages = projects.len().div_ceil(state.page_size).max(1);
    let page = state.page.clamp(1, total_pages);
    let offset = (page - 1) * state.page_size;
    let shown = (offset + state.page_size).min(projects.len());
    let page_info = format!(
        "{} {}–{} {} {} {}",
        i18n::tr(&state.locale, "showing"),
        if projects.is_empty() { 0 } else { offset + 1 },
        shown,
        i18n::tr(&state.locale, "of"),
        projects.len(),
        i18n::tr(&state.locale, "result_many")
    );
    set_html(
        document,
        "table-head",
        &table_header(&state.sort, &state.locale),
    );
    let pagination = pagination_markup(page, total_pages, &state.locale);
    set_html(document, "pagination-top", &pagination);
    set_html(document, "pagination-bottom", &pagination);
    let html = if projects.is_empty() {
        format!(
            "<tr><td colspan=\"8\"><div class=\"empty-state\"><h3>{}</h3><p>{}</p></div></td></tr>",
            i18n::tr(&state.locale, "no_results"),
            i18n::tr(&state.locale, "no_results_hint")
        )
    } else {
        projects
            .iter()
            .skip(offset)
            .take(state.page_size)
            .map(|project| project_markup(project, &catalog.profiles, &state.locale))
            .collect::<String>()
    };
    set_html(document, "projects", &html);
    write_url(state, page);
    set_text(document, "page-info", &page_info);
}

fn project_markup(project: &Project, profiles: &[GameProfile], locale: &str) -> String {
    let source_links = project
        .sources
        .iter()
        .map(|source| {
            format!(
                "<a href=\"{}\" target=\"_blank\" rel=\"noreferrer\">{}</a>",
                escape_html(&safe_link(&source.url)),
                escape_html(&source.label)
            )
        })
        .collect::<String>();
    let updated = project
        .updated
        .as_deref()
        .map(|date| {
            format!(
                "<span>{} {}</span>",
                i18n::tr(locale, "project_updated"),
                escape_html(&format_date(date, locale))
            )
        })
        .unwrap_or_default();
    let started = project
        .project_started
        .as_deref()
        .map(|date| {
            format!(
                "<span>{} {}</span>",
                i18n::tr(locale, "project_started"),
                escape_html(&format_date(date, locale))
            )
        })
        .unwrap_or_default();
    let id = escape_html(&project.id);
    let media_count = if project.media.is_empty() {
        String::new()
    } else {
        format!(
            " · {} {}",
            project.media.len(),
            i18n::tr(locale, "media_count")
        )
    };
    let language = if project.language.is_empty() {
        "—"
    } else {
        &project.language
    };
    let kind_class = project.kind.to_lowercase().replace(' ', "-");
    let target_label = if project.target_type == "Software" {
        i18n::tr(locale, "target_software")
    } else {
        i18n::tr(locale, "target_game")
    };
    let target_type_class = if project.target_type == "Software" {
        "software"
    } else {
        "game"
    };
    let game_profile = profiles
        .iter()
        .find(|profile| profile.game_key == project.game)
        .map(|profile| game_profile_markup(profile, locale))
        .unwrap_or_default();
    let programming_language =
        programming_language_markup(project.programming_language_evidence.as_ref(), locale);
    let project_links = project_links_markup(&project.links, locale);
    let subtype = if project.subtype.trim().is_empty() {
        String::new()
    } else {
        format!(
            "<span class=\"metadata-chip\">{}</span>",
            escape_html(&project.subtype)
        )
    };
    format!(
        "<tr class=\"catalog-row\"><th scope=\"row\"><a class=\"table-project-title\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a><small class=\"table-game\">{}</small><button class=\"details-toggle\" data-details=\"detail-{}\" aria-label=\"{} {}\" aria-expanded=\"false\" aria-controls=\"project-details-dialog\">{}{} <svg class=\"ui-icon ui-icon-small\" aria-hidden=\"true\"><use href=\"#icon-chevron\" xlink:href=\"#icon-chevron\"></use></svg></button></th><td><span class=\"card-field-label\">{}</span><span class=\"table-target-type target-type-{}\">{}</span></td><td><span class=\"card-field-label\">{}</span><span class=\"table-kind kind-text-{}\">{} {}</span></td><td><span class=\"card-field-label\">{}</span><span class=\"table-platform\">{}</span></td><td><span class=\"card-field-label\">{}</span><span class=\"table-approach\">{}</span></td><td><span class=\"card-field-label\">{}</span><span class=\"table-stage\">{} {}</span></td><td><span class=\"card-field-label\">{}</span>{}</td><td class=\"table-date\"><span class=\"card-field-label\">{}</span>{}</td></tr><tr class=\"detail-row\" id=\"detail-{}\" hidden><td colspan=\"8\"><div class=\"detail-body\"><p class=\"detail-summary\">{}</p><p class=\"detail-status\">{}</p><div class=\"project-metadata\"><span class=\"metadata-chip\"><svg class=\"ui-icon ui-icon-small\"><use href=\"#icon-check\" xlink:href=\"#icon-check\"></use></svg>{}</span><span class=\"metadata-chip\">{}</span>{}{}{}</div>{}{}<div class=\"source-links\"><span><svg class=\"ui-icon ui-icon-small\"><use href=\"#icon-link\" xlink:href=\"#icon-link\"></use></svg> {}</span>{}</div>{}{}</div></td></tr>",
        escape_html(&safe_link(&project.project_url)), escape_html(&project.title), icon_markup("external"), escape_html(&project.game),
        id, i18n::tr(locale, "details_for"), escape_html(&project.title), i18n::tr(locale, "details"), media_count,
        escape_html(i18n::tr(locale, "col_target_type")), target_type_class, escape_html(target_label),
        escape_html(i18n::tr(locale, "col_type")), escape_html(&kind_class), icon_markup(kind_icon_name(&project.kind)), escape_html(localized_kind(locale, &project.kind)),
        escape_html(i18n::tr(locale, "col_platform")), escape_html(&project.platform),
        escape_html(i18n::tr(locale, "col_approach")), escape_html(&localize_strategy(locale, &project.strategy)),
        escape_html(i18n::tr(locale, "col_stage")), icon_markup("check"), i18n::tr(locale, status_key(status_bucket(&project.status))),
        escape_html(i18n::tr(locale, "col_language")), escape_html(language),
        escape_html(i18n::tr(locale, "col_found")), escape_html(&project.first_found.as_deref().map(|date| format_date(date, locale)).unwrap_or_else(|| "—".into())),
        id, escape_html(&project.summary), escape_html(&project.status), escape_html(localize_source_type(locale, &project.source_available)), escape_html(target_label), updated, started, subtype,
        programming_language, game_profile, i18n::tr(locale, "project_sources"), source_links, media::gallery(&project.media, locale), project_links
    )
}

fn programming_language_markup(
    details: Option<&ProgrammingLanguageEvidence>,
    locale: &str,
) -> String {
    let Some(details) = details else {
        return String::new();
    };
    let source = safe_link(&details.source_url);
    if source.is_empty() {
        return String::new();
    }
    let breakdown = details
        .breakdown
        .as_ref()
        .map(|items| {
            let total: f64 = items.values().sum();
            let mut sorted = items.iter().collect::<Vec<_>>();
            sorted.sort_by(|a, b| b.1.total_cmp(a.1));
            sorted
                .into_iter()
                .take(5)
                .map(|(name, bytes)| {
                    let percent = if total == 0.0 {
                        0.0
                    } else {
                        (*bytes / total) * 100.0
                    };
                    format!("{} {}%", escape_html(name), percent.round() as u64)
                })
                .collect::<Vec<_>>()
                .join(" · ")
        })
        .unwrap_or_default();
    let heading = details
        .predominant_language
        .as_deref()
        .filter(|name| !name.is_empty())
        .map(|name| {
            format!(
                "{} {}",
                i18n::tr(locale, "predominant_language"),
                escape_html(name)
            )
        })
        .or_else(|| details.reason_unknown.as_deref().map(escape_html))
        .unwrap_or_else(|| i18n::tr(locale, "unknown_repository_language").into());
    let breakdown = if breakdown.is_empty() {
        String::new()
    } else {
        format!(" <span class=\"language-breakdown\">{}</span>", breakdown)
    };
    format!(
        "<p class=\"programming-language-evidence\"><strong><svg class=\"ui-icon ui-icon-small\" aria-hidden=\"true\"><use href=\"#icon-code\" xlink:href=\"#icon-code\"></use></svg>{}</strong>{} <a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a><small>{} · {} {}</small></p>",
        heading, breakdown, escape_html(&source), i18n::tr(locale, "source_languages"), icon_markup("external"), escape_html(&details.evidence), i18n::tr(locale, "verified"), escape_html(&details.verified_at)
    )
}

fn project_links_markup(links: &[ProjectLink], locale: &str) -> String {
    let entries = links.iter().filter_map(|link| {
        let url = safe_link(&link.url);
        let source = safe_link(&link.source_url);
        if url.is_empty() || source.is_empty() { return None; }
        Some(format!(
            "<li><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} <span>{} {}</span></a><small>{} <a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a>{}</small></li>",
            escape_html(&url), escape_html(&link.label), escape_html(&link.kind.to_uppercase()), icon_markup("external"), i18n::tr(locale, "linked_from"), escape_html(&source), i18n::tr(locale, "project_source"), icon_markup("external"),
            if link.notes.is_empty() { String::new() } else { format!(" · {}", escape_html(&link.notes)) }
        ))
    }).collect::<String>();
    if entries.is_empty() {
        return String::new();
    }
    format!(
        "<details class=\"project-resources\"><summary><svg class=\"ui-icon disclosure-icon\" aria-hidden=\"true\"><use href=\"#icon-folder\" xlink:href=\"#icon-folder\"></use></svg>{} <span>{} {}</span><svg class=\"ui-icon disclosure-chevron\" aria-hidden=\"true\"><use href=\"#icon-chevron\" xlink:href=\"#icon-chevron\"></use></svg></summary><ul class=\"resource-links-list\">{}</ul></details>",
        i18n::tr(locale, "more_resources"), links.len(), i18n::tr(locale, "links"), entries
    )
}

fn render_directory(document: &Document, items: &[DirectoryItem], locale: &str) {
    for (id, item_type) in [("directory-tools", "Tool"), ("directory-sites", "Website")] {
        let html = items
            .iter()
            .filter(|item| item.item_type == item_type)
            .filter_map(|item| directory_card_markup(item, locale))
            .collect::<String>();
        set_html(document, id, &html);
    }
}

fn render_showcase(
    document: &Document,
    featured: &[FeaturedProject],
    projects: &[Project],
    locale: &str,
) {
    let html = featured
        .iter()
        .filter_map(|entry| {
            let project = projects
                .iter()
                .find(|project| project.id == entry.project_id)?;
            showcase_card_markup(entry, project, locale)
        })
        .collect::<String>();
    set_html(document, "featured-projects", &html);
}

fn showcase_card_markup(
    entry: &FeaturedProject,
    project: &Project,
    locale: &str,
) -> Option<String> {
    let repository = safe_link(&project.project_url);
    let source = safe_link(&entry.source_url);
    let ranking = safe_link(&entry.ranking_url);
    let expected_ranking = repository.strip_prefix("https://github.com/").map(|path| {
        format!(
            "https://api.github.com/repos/{}",
            path.trim_end_matches('/')
        )
    });
    if !repository.starts_with("https://github.com/")
        || repository.trim_end_matches('/') != source.trim_end_matches('/')
        || expected_ranking.as_deref() != Some(ranking.as_str())
    {
        return None;
    }
    let kind_key = match project.kind.as_str() {
        "Decompilation" => "kind_decomp",
        "Recompilation" => "kind_recomp",
        "API Translation" => "kind_api",
        "Fan Localization" => "kind_fan",
        "Reimplementation" => "kind_reimplementation",
        "Emulator" => "kind_emulator",
        _ => return None,
    };
    let method = if project.strategy.trim().is_empty() {
        project.platform.as_str()
    } else {
        project.strategy.as_str()
    };
    Some(format!(
        "<article class=\"showcase-card\"><div class=\"showcase-card-topline\"><span class=\"showcase-rank\">#{:02}</span><span class=\"showcase-stars\"><svg class=\"ui-icon ui-icon-small\" aria-hidden=\"true\"><use href=\"#icon-spark\" xlink:href=\"#icon-spark\"></use></svg>{} {}</span></div><p class=\"showcase-kind\">{}</p><h3>{}</h3><h4>{}</h4><p class=\"showcase-summary\">{}</p><div class=\"showcase-tags\"><span>{}</span><span>{}</span><span>{}</span></div><a class=\"showcase-repo\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a><div class=\"showcase-evidence\"><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a><small>{} · {}</small></div></article>",
        entry.position,
        grouped_number(entry.github_stars), i18n::tr(locale, "showcase_stars"),
        i18n::tr(locale, kind_key), escape_html(&project.game), escape_html(&project.title), escape_html(&project.summary),
        escape_html(&project.platform), escape_html(method), escape_html(&project.status),
        escape_html(&repository), i18n::tr(locale, "showcase_open"), icon_markup("external"),
        escape_html(&ranking), i18n::tr(locale, "showcase_source"), icon_markup("external"),
        escape_html(&entry.verified_at), escape_html(&entry.evidence)
    ))
}

fn grouped_number(value: u64) -> String {
    let digits = value.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(character);
    }
    grouped
}

fn directory_card_markup(item: &DirectoryItem, locale: &str) -> Option<String> {
    let url = safe_link(&item.url);
    let source = safe_link(&item.source_url);
    if url.is_empty() || source.is_empty() {
        return None;
    }
    Some(format!(
        "<article class=\"directory-card\"><div class=\"directory-card-topline\"><span><svg class=\"ui-icon ui-icon-small\" aria-hidden=\"true\"><use href=\"#icon-{}\" xlink:href=\"#icon-{}\"></use></svg>{}</span><span><svg class=\"ui-icon ui-icon-small\" aria-hidden=\"true\"><use href=\"#icon-check\" xlink:href=\"#icon-check\"></use></svg>{} {}</span></div><h4><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a></h4><p>{}</p><div class=\"directory-evidence\"><a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a><small>{}</small></div></article>",
        directory_icon_name(&item.category), directory_icon_name(&item.category), escape_html(&item.category), i18n::tr(locale, "verified"), escape_html(&item.verified_at),
        escape_html(&url), escape_html(&item.name), icon_markup("external"), escape_html(&item.description), escape_html(&source), i18n::tr(locale, "maintainer_source"), icon_markup("external"), escape_html(&item.source_evidence)
    ))
}

fn game_profile_markup(profile: &GameProfile, locale: &str) -> String {
    let wikipedia = profile
        .wikipedia_url
        .as_deref()
        .map(safe_link)
        .filter(|url| url.starts_with("https://en.wikipedia.org/"));
    if profile.facts.is_empty() && wikipedia.is_none() {
        return String::new();
    }
    let facts = profile.facts.iter().filter_map(|fact| {
        let value = game_fact_value(&fact.value);
        let source = safe_link(&fact.source_url);
        if value.is_empty() || source.is_empty() { return None; }
        let granularity = fact.date_granularity.as_deref()
            .map(|value| format!(" <small class=\"fact-granularity\">{}</small>", escape_html(value)))
            .unwrap_or_default();
        Some(format!(
            "<div class=\"game-profile-fact\"><dt><svg class=\"ui-icon ui-icon-small\" aria-hidden=\"true\"><use href=\"#icon-{}\" xlink:href=\"#icon-{}\"></use></svg>{}</dt><dd>{}{} <a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a><small class=\"fact-evidence\">{}</small></dd></div>",
            game_fact_icon_name(&fact.field), game_fact_icon_name(&fact.field), i18n::tr(locale, game_fact_key(&fact.field)), escape_html(&value), granularity,
            escape_html(&source), i18n::tr(locale, "source_label"), icon_markup("external"), escape_html(&fact.evidence)
        ))
    }).collect::<String>();
    if facts.is_empty() && wikipedia.is_none() {
        return String::new();
    }
    let wikipedia_link = wikipedia.map(|url| {
        let label = if profile.wikipedia_title.is_some() { i18n::tr(locale, "wikipedia_page") } else { i18n::tr(locale, "wikipedia_search") };
        format!("<a class=\"game-wikipedia\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} {}</a>", escape_html(&url), label, icon_markup("external"))
    }).unwrap_or_default();
    format!("<section class=\"game-profile\" aria-label=\"{}\"><div class=\"game-profile-heading\"><h4>{}</h4>{}</div><dl class=\"game-profile-facts\">{}</dl></section>", i18n::tr(locale, "game_profile"), i18n::tr(locale, "game_profile"), wikipedia_link, facts)
}

fn game_fact_key(field: &str) -> &'static str {
    match field {
        "canonical_name" => "fact_name",
        "genre" => "fact_genre",
        "original_platform" => "fact_platform",
        "regional_release_date" => "fact_release",
        "developer" => "fact_developer",
        "publisher" => "fact_publisher",
        "series" => "fact_series",
        "collection_context" => "fact_collection",
        _ => "fact_name",
    }
}

fn game_fact_value(value: &Value) -> String {
    match value {
        Value::String(value) => value.clone(),
        Value::Array(values) => values
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(" · "),
        Value::Null => String::new(),
        _ => value.to_string(),
    }
}

fn date_order(a: &Option<String>, b: &Option<String>, descending: bool) -> std::cmp::Ordering {
    match (a, b) {
        (Some(a), Some(b)) => {
            if descending {
                b.cmp(a)
            } else {
                a.cmp(b)
            }
        }
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    }
}

fn table_header(sort: &str, locale: &str) -> String {
    let columns = [
        ("title", "col_project"),
        ("target-type", "col_target_type"),
        ("kind", "col_type"),
        ("platform", "col_platform"),
        ("strategy", "col_approach"),
        ("status", "col_stage"),
        ("language", "col_language"),
        ("found", "col_found"),
    ];
    let cells = columns.iter().map(|(key, label)| {
        let asc = sort == format!("{key}-asc");
        let desc = sort == format!("{key}-desc");
        let aria = if asc { "ascending" } else if desc { "descending" } else { "none" };
        let arrow = if asc { "up" } else if desc { "down" } else { "sort" };
        format!("<th scope=\"col\" aria-sort=\"{}\"><button data-sort=\"{}\">{} <svg class=\"ui-icon ui-icon-small\" aria-hidden=\"true\"><use href=\"#icon-{}\" xlink:href=\"#icon-{}\"></use></svg></button></th>", aria, key, i18n::tr(locale, label), arrow, arrow)
    }).collect::<String>();
    format!("<tr>{}</tr>", cells)
}

fn pagination_markup(page: usize, pages: usize, locale: &str) -> String {
    if pages < 2 {
        return String::new();
    }
    let page_text = match locale {
        "ja" => format!("{} / {} ページ", page, pages),
        "zh" => format!("第 {} 页，共 {} 页", page, pages),
        _ => format!(
            "{} {} {} {}",
            i18n::tr(locale, "page"),
            page,
            i18n::tr(locale, "of"),
            pages
        ),
    };
    format!("<nav class=\"pagination\" aria-label=\"{}\"><button data-page=\"{}\" {}>{} {}</button><span>{}</span><button data-page=\"{}\" {}>{} {}</button></nav>", i18n::tr(locale, "pagination_aria"), page.saturating_sub(1).max(1), if page == 1 { "disabled" } else { "" }, icon_markup("left"), i18n::tr(locale, "previous"), page_text, (page + 1).min(pages), if page == pages { "disabled" } else { "" }, i18n::tr(locale, "next"), icon_markup("right"))
}

fn icon_markup(name: &str) -> String {
    format!("<svg class=\"ui-icon\" aria-hidden=\"true\" focusable=\"false\"><use href=\"#icon-{0}\" xlink:href=\"#icon-{0}\"></use></svg>", name)
}

fn kind_icon_name(kind: &str) -> &'static str {
    match kind {
        "Decompilation" => "code",
        "Recompilation" => "cube",
        "API Translation" => "arrows",
        "Fan Localization" => "translate",
        "Emulator" => "gamepad",
        _ => "terminal",
    }
}

fn directory_icon_name(category: &str) -> &'static str {
    let category = category.to_ascii_lowercase();
    if category.contains("translation") || category.contains("localization") {
        "translate"
    } else if category.contains("compatibility")
        || category.contains("api")
        || category.contains("graphics")
    {
        "arrows"
    } else if category.contains("patch")
        || category.contains("mod")
        || category.contains("community")
    {
        "cube"
    } else if category.contains("decomp")
        || category.contains("reverse")
        || category.contains("code")
    {
        "code"
    } else {
        "terminal"
    }
}

fn game_fact_icon_name(field: &str) -> &'static str {
    match field {
        "genre" | "series" => "spark",
        "original_platform" | "collection_context" => "cube",
        "developer" | "publisher" => "code",
        _ => "terminal",
    }
}

fn set_filter_value(state: &mut AppState, id: &str, value: String) {
    match id {
        "filter-target-type" => state.target_type = value,
        "filter-platform" => state.platform = value,
        "filter-strategy" => state.strategy = value,
        "filter-status" => state.status = value,
        "filter-source" => state.source_available = value,
        "filter-language" => state.language = value,
        "filter-programming-language" => state.programming_language = value,
        "filter-genre" => state.genre = value,
        "filter-original-platform" => state.original_platform = value,
        "filter-found" => state.first_found_since = value,
        "filter-started" => state.project_started_by = value,
        "filter-media" => state.media_kind = value,
        _ => {}
    }
}

fn populate_filters(document: &Document, catalog: &Catalog, locale: &str) {
    for (id, label, values) in [
        (
            "filter-target-type",
            "any_target",
            catalog
                .projects
                .iter()
                .map(|project| project.target_type.clone())
                .collect::<BTreeSet<_>>(),
        ),
        (
            "filter-platform",
            "any_platform",
            catalog
                .projects
                .iter()
                .map(|p| p.platform.clone())
                .collect::<BTreeSet<_>>(),
        ),
        (
            "filter-strategy",
            "any_approach",
            catalog
                .projects
                .iter()
                .map(|p| p.strategy.clone())
                .filter(|v| !v.is_empty())
                .collect::<BTreeSet<_>>(),
        ),
        (
            "filter-status",
            "any_stage",
            catalog
                .projects
                .iter()
                .map(|p| status_bucket(&p.status).to_string())
                .collect::<BTreeSet<_>>(),
        ),
        (
            "filter-source",
            "any_source",
            catalog
                .projects
                .iter()
                .map(|p| p.source_available.clone())
                .filter(|v| !v.is_empty())
                .collect::<BTreeSet<_>>(),
        ),
        (
            "filter-language",
            "any_language",
            catalog
                .projects
                .iter()
                .map(|p| p.language.clone())
                .filter(|v| !v.is_empty())
                .collect::<BTreeSet<_>>(),
        ),
        (
            "filter-programming-language",
            "any_language",
            catalog
                .projects
                .iter()
                .map(|p| p.programming_language.clone())
                .filter(|v| !v.is_empty())
                .collect::<BTreeSet<_>>(),
        ),
        (
            "filter-genre",
            "any_genre",
            profile_values(&catalog.profiles, "genre"),
        ),
        (
            "filter-original-platform",
            "any_original_platform",
            profile_values(&catalog.profiles, "original_platform"),
        ),
    ] {
        let options = std::iter::once(format!(
            "<option value=\"\">{}</option>",
            i18n::tr(locale, label)
        ))
        .chain(values.into_iter().map(|value| {
            let label = if id == "filter-target-type" {
                if value == "Software" {
                    i18n::tr(locale, "target_software")
                } else {
                    i18n::tr(locale, "target_game")
                }
            } else if id == "filter-strategy" {
                localize_strategy(locale, &value)
            } else if id == "filter-status" {
                i18n::tr(locale, status_key(&value))
            } else if id == "filter-source" {
                localize_source_type(locale, &value)
            } else {
                value.as_str()
            };
            format!(
                "<option value=\"{}\">{}</option>",
                escape_html(&value),
                escape_html(label)
            )
        }))
        .collect::<String>();
        set_html(document, id, &options);
    }
    let started = catalog
        .projects
        .iter()
        .any(|project| project.project_started.is_some());
    if let Some(input) = document.get_element_by_id("filter-started") {
        if !started {
            let _ = input.set_attribute("disabled", "");
        }
    }
}

fn status_bucket(status: &str) -> &'static str {
    let value = status.to_lowercase();
    if value.contains("unclear") || value.contains("uncertain") || value.contains("verify") {
        "Unverified"
    } else if value.contains("released") && !value.contains("unreleased") {
        "Released"
    } else if value.contains("complete") {
        "Complete"
    } else if value.contains("playable") {
        "Playable"
    } else if value.contains("experimental")
        || value.contains("early")
        || value.contains("preview")
        || value.contains("alpha")
        || value.contains("bring-up")
        || value.contains("bring up")
        || value.contains("unreleased")
    {
        "Early / experimental"
    } else {
        "In progress"
    }
}

fn localize_source_type<'a>(locale: &str, source: &'a str) -> &'a str {
    let key = match source {
        "Repository" => "source_repository",
        "Public fan localization repository" => "source_public_fan_localization_repo",
        "Public fan translation/localization project" => "source_fan_localization_project",
        "Patch page" => "source_patch_page",
        "Project page / patch" => "source_project_page_patch",
        "Public project repository" => "source_project_repository",
        "Public project repository; license status is documented in the evidence" => {
            "source_project_repository_license_note"
        }
        "Project repository; see verification metadata for license and excluded game data." => {
            "source_project_repository_verification_note"
        }
        "Public source repository" => "source_public_source_repository",
        "No public source code" => "source_no_public_code",
        "Archived or community source" => "source_archived_community",
        "Source availability unclear" => "source_availability_unclear",
        "Public source repository; game ROM files are user-supplied" => {
            "source_public_source_repository_user_roms"
        }
        "Public source repository; game files/assets are not included" => {
            "source_public_source_repository_no_game_files"
        }
        "Public source repository; requires user-supplied game executable" => {
            "source_public_source_repository_user_executable"
        }
        "Official product documentation only; proprietary component" => {
            "source_official_proprietary_docs"
        }
        "Wine project documentation; integrated component" => "source_wine_integrated_docs",
        _ => return source,
    };
    i18n::tr(locale, key)
}

fn status_key(status: &str) -> &'static str {
    match status {
        "Unverified" => "stage_unverified",
        "Released" => "stage_released",
        "Complete" => "stage_complete",
        "Playable" => "stage_playable",
        "Early / experimental" => "stage_early",
        _ => "stage_progress",
    }
}

fn localized_kind<'a>(locale: &str, kind: &'a str) -> &'a str {
    let key = match kind {
        "Decompilation" => "kind_decomp",
        "Recompilation" => "kind_recomp",
        "API Translation" => "kind_api",
        "Fan Localization" => "kind_fan",
        "Reimplementation" => "kind_reimplementation",
        "Emulator" => "kind_emulator",
        _ => return kind,
    };
    i18n::tr(locale, key)
}

fn localize_strategy<'a>(locale: &str, strategy: &'a str) -> &'a str {
    let key = match strategy {
        "Annotated disassembly" => "strategy_annotated",
        "Console-to-desktop compatibility and graphics translation" => "strategy_compat",
        "Direct3D API translation" | "Direct3D/DirectDraw graphics translation" => "strategy_d3d",
        "Direct3D-to-Metal translation" | "DirectX-to-Metal translation" => "strategy_metal",
        "Direct3D-to-Vulkan translation" | "Direct3D-to-Vulkan translation (integrated)" => {
            "strategy_d3d_vulkan"
        }
        "Fan translation patch" => "strategy_fan_patch",
        "Matching decompilation" => "strategy_matching_decomp",
        "Native port from matching decompilation" => "strategy_native_port",
        "OpenGL-to-Vulkan translation" => "strategy_gl_vulkan",
        "Reverse-engineered C++ source reconstruction (non-matching)" => "strategy_re_cpp",
        "Reverse-engineered code translation / native reconstruction" => "strategy_re_native",
        "Reverse-engineered source reconstruction (non-matching)" => "strategy_re_source",
        "Runtime/in-place localization mod" => "strategy_runtime_loc",
        "Source decompilation; byte-matching not stated"
        | "Source decompilation; matching fidelity not established" => "strategy_source_decomp",
        "Static recompilation" => "strategy_static",
        "Vulkan-to-Metal implementation" => "strategy_vk_metal",
        "Windows API compatibility stack" => "strategy_windows_compat",
        "x86-64 instruction translation" => "strategy_x86",
        "x86/x86-64 instruction translation" => "strategy_x86_general",
        _ => return strategy,
    };
    i18n::tr(locale, key)
}

fn profile_values(profiles: &[GameProfile], field: &str) -> BTreeSet<String> {
    profiles
        .iter()
        .flat_map(|profile| profile.facts.iter())
        .filter(|fact| fact.field == field)
        .flat_map(|fact| match &fact.value {
            Value::String(value) => value
                .split(';')
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            Value::Array(values) => values
                .iter()
                .filter_map(Value::as_str)
                .flat_map(|value| value.split(';'))
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect()
}

fn profile_has_value(profile: Option<&GameProfile>, field: &str, expected: &str) -> bool {
    let Some(profile) = profile else {
        return false;
    };
    profile
        .facts
        .iter()
        .filter(|fact| fact.field == field)
        .any(|fact| match &fact.value {
            Value::String(value) => value
                .split(';')
                .map(str::trim)
                .any(|value| value == expected),
            Value::Array(values) => values
                .iter()
                .filter_map(Value::as_str)
                .flat_map(|value| value.split(';'))
                .map(str::trim)
                .any(|value| value == expected),
            _ => false,
        })
}

fn safe_link(value: &str) -> String {
    if value.starts_with("https://") {
        value.to_string()
    } else {
        String::new()
    }
}

fn supported_project_kind(kind: &str) -> bool {
    matches!(
        kind,
        "Decompilation"
            | "Recompilation"
            | "API Translation"
            | "Fan Localization"
            | "Reimplementation"
            | "Emulator"
    )
}

fn parse_kind_filter(value: Option<String>) -> BTreeSet<String> {
    value
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|kind| supported_project_kind(kind))
        .map(str::to_string)
        .collect()
}

fn parse_page_size(value: &str) -> usize {
    value
        .parse::<usize>()
        .ok()
        .filter(|size| (1..=1000).contains(size))
        .unwrap_or(25)
}

fn read_url(state: &mut AppState) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(query) = window.location().search() else {
        return;
    };
    let Ok(params) = UrlSearchParams::new_with_str(&query) else {
        return;
    };
    state.query = params.get("q").unwrap_or_default();
    state.search_mode = match params.get("match").as_deref() {
        Some("verbatim") => "verbatim".into(),
        _ => "fuzzy".into(),
    };
    state.kinds = parse_kind_filter(params.get("kind"));
    let sort = params.get("sort").unwrap_or_else(|| "updated-desc".into());
    state.sort = if sort.rsplit_once('-').is_some_and(|(key, direction)| {
        [
            "updated",
            "found",
            "title",
            "kind",
            "platform",
            "strategy",
            "status",
            "language",
            "target-type",
        ]
        .contains(&key)
            && ["asc", "desc"].contains(&direction)
    }) {
        sort
    } else {
        "updated-desc".into()
    };
    for (param, id) in [
        ("target-type", "filter-target-type"),
        ("platform", "filter-platform"),
        ("strategy", "filter-strategy"),
        ("stage", "filter-status"),
        ("source", "filter-source"),
        ("language", "filter-language"),
        ("program-language", "filter-programming-language"),
        ("genre", "filter-genre"),
        ("game-platform", "filter-original-platform"),
        ("found", "filter-found"),
        ("started", "filter-started"),
        ("media", "filter-media"),
    ] {
        set_filter_value(state, id, params.get(param).unwrap_or_default());
    }
    state.page = params
        .get("page")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1)
        .clamp(1, 10000);
    state.page_size = params
        .get("rows")
        .map(|value| parse_page_size(&value))
        .unwrap_or(25);
}

fn write_url(state: &AppState, page: usize) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(params) = UrlSearchParams::new() else {
        return;
    };
    for (key, value) in [
        ("q", &state.query),
        ("target-type", &state.target_type),
        ("platform", &state.platform),
        ("strategy", &state.strategy),
        ("stage", &state.status),
        ("source", &state.source_available),
        ("language", &state.language),
        ("program-language", &state.programming_language),
        ("genre", &state.genre),
        ("game-platform", &state.original_platform),
        ("found", &state.first_found_since),
        ("started", &state.project_started_by),
        ("media", &state.media_kind),
    ] {
        if !value.is_empty() {
            params.set(key, value);
        }
    }
    if state.search_mode == "verbatim" {
        params.set("match", "verbatim");
    }
    if !state.kinds.is_empty() {
        let selected = state
            .kinds
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(",");
        params.set("kind", &selected);
    }
    if state.sort != "updated-desc" {
        params.set("sort", &state.sort);
    }
    if page > 1 {
        params.set("page", &page.to_string());
    }
    if state.page_size != 25 {
        params.set("rows", &state.page_size.to_string());
    }
    let query: String = params.to_string().into();
    let url = format!(
        "{}{}{}{}",
        window.location().pathname().unwrap_or_default(),
        if query.is_empty() { "" } else { "?" },
        query,
        window.location().hash().unwrap_or_default()
    );
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&url));
    }
}

fn sync_controls(document: &Document, state: &AppState) {
    for (id, value) in [
        ("search", state.query.as_str()),
        ("search-mode", state.search_mode.as_str()),
        ("filter-target-type", state.target_type.as_str()),
        ("sort", state.sort.as_str()),
        ("filter-platform", state.platform.as_str()),
        ("filter-strategy", state.strategy.as_str()),
        ("filter-status", state.status.as_str()),
        ("filter-source", state.source_available.as_str()),
        ("filter-language", state.language.as_str()),
        (
            "filter-programming-language",
            state.programming_language.as_str(),
        ),
        ("filter-genre", state.genre.as_str()),
        ("filter-original-platform", state.original_platform.as_str()),
        ("filter-found", state.first_found_since.as_str()),
        ("filter-started", state.project_started_by.as_str()),
        ("filter-media", state.media_kind.as_str()),
    ] {
        if let Some(element) = document.get_element_by_id(id) {
            if let Ok(select) = element.clone().dyn_into::<HtmlSelectElement>() {
                select.set_value(value);
            }
            if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                input.set_value(value);
            }
        }
    }
    if let Some(input) = document
        .get_element_by_id("page-size")
        .and_then(|node| node.dyn_into::<HtmlInputElement>().ok())
    {
        input.set_value(&state.page_size.to_string());
    }
    if let Ok(buttons) = document.query_selector_all("[data-kind]") {
        for index in 0..buttons.length() {
            if let Some(button) = buttons
                .item(index)
                .and_then(|node| node.dyn_into::<Element>().ok())
            {
                let kind = button.get_attribute("data-kind").unwrap_or_default();
                let selected = if kind == "all" {
                    state.kinds.is_empty()
                } else {
                    state.kinds.contains(&kind)
                };
                let _ =
                    button.set_attribute("aria-pressed", if selected { "true" } else { "false" });
            }
        }
    }
    if !state.media_kind.is_empty()
        || !state.platform.is_empty()
        || !state.strategy.is_empty()
        || !state.status.is_empty()
        || !state.language.is_empty()
        || !state.programming_language.is_empty()
        || !state.genre.is_empty()
        || !state.original_platform.is_empty()
        || !state.source_available.is_empty()
        || !state.target_type.is_empty()
        || !state.first_found_since.is_empty()
        || !state.project_started_by.is_empty()
    {
        if let Ok(Some(drawer)) = document.query_selector(".filter-drawer") {
            let _ = drawer.set_attribute("open", "");
        }
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn format_date(value: &str, locale: &str) -> String {
    let date = value.split('T').next().unwrap_or(value);
    let mut parts = date.split('-');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(year), Some(month), Some(day)) if year.len() == 4 => {
            let month_index = match month {
                "01" => 0,
                "02" => 1,
                "03" => 2,
                "04" => 3,
                "05" => 4,
                "06" => 5,
                "07" => 6,
                "08" => 7,
                "09" => 8,
                "10" => 9,
                "11" => 10,
                "12" => 11,
                _ => return value.to_string(),
            };
            let day = day.trim_start_matches('0');
            if locale == "ja" || locale == "zh" {
                return format!("{year}年{}月{}日", month_index + 1, day);
            }
            let months: [&[&str; 12]; 4] = [
                &[
                    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
                    "Dec",
                ],
                &[
                    "ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sep", "oct", "nov",
                    "dic",
                ],
                &[
                    "janv.", "févr.", "mars", "avr.", "mai", "juin", "juil.", "août", "sept.",
                    "oct.", "nov.", "déc.",
                ],
                &[
                    "jan.", "fev.", "mar.", "abr.", "mai", "jun.", "jul.", "ago.", "set.", "out.",
                    "nov.", "dez.",
                ],
            ];
            let language_index = match locale {
                "es" => 1,
                "fr" => 2,
                "pt" => 3,
                _ => 0,
            };
            let month_name = months[language_index][month_index];
            if locale == "en" {
                format!("{month_name} {day}, {year}")
            } else {
                format!("{day} {month_name} {year}")
            }
        }
        _ => value.to_string(),
    }
}

fn set_text(document: &Document, id: &str, value: &str) {
    if let Some(element) = document.get_element_by_id(id) {
        element.set_text_content(Some(value));
    }
}

fn set_status(document: &Document, locale: &str, key: &str) {
    if let Some(element) = document.get_element_by_id("status-line") {
        let _ = element.set_attribute("data-ui", key);
        element.set_text_content(Some(i18n::tr(locale, key)));
    }
}

fn set_html(document: &Document, id: &str, value: &str) {
    if let Some(element) = document.get_element_by_id(id) {
        element.set_inner_html(value);
    }
}

#[cfg(test)]
mod catalog_tests {
    use super::{
        media, media_project_indices, media_stage_markup, parse_kind_filter, parse_page_size,
        project_markup, search_matches, search_tokens, status_bucket, Catalog,
    };
    use std::collections::BTreeSet;

    #[test]
    fn published_catalog_deserializes() {
        serde_json::from_str::<Catalog>(include_str!("../docs/catalog.json")).unwrap_or_else(
            |error| panic!("published catalog does not match the WASM data model: {error}"),
        );
    }

    #[test]
    fn compact_catalog_cards_keep_all_seven_metadata_labels_localized() {
        let catalog = serde_json::from_str::<Catalog>(include_str!("../docs/catalog.json"))
            .expect("published catalog should deserialize");
        let project = catalog
            .projects
            .iter()
            .find(|project| project.id == "oot-decomp")
            .expect("Ocarina of Time decompilation should be in the catalog");
        let markup = project_markup(project, &catalog.profiles, "fr");

        assert_eq!(markup.matches("class=\"card-field-label\"").count(), 7);
        assert!(markup.contains("class=\"table-platform\""));
        assert!(markup.contains("class=\"table-approach\""));
        for label in [
            "Type de cible",
            "Type de projet",
            "Plateforme",
            "Approche",
            "État",
            "Langue",
            "Ajout à l’index",
        ] {
            assert!(
                markup.contains(&format!("<span class=\"card-field-label\">{label}</span>")),
                "mobile card is missing an actual localized field label {label}"
            );
        }
    }

    #[test]
    fn media_reel_has_one_entry_per_project_with_source_linked_media() {
        let catalog = serde_json::from_str::<Catalog>(include_str!("../docs/catalog.json"))
            .expect("published catalog should deserialize");
        let projects = media_project_indices(&catalog.projects);
        let expected = catalog
            .projects
            .iter()
            .enumerate()
            .filter(|(_, project)| {
                project
                    .media
                    .iter()
                    .any(|item| item.url.starts_with("https://"))
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        assert_eq!(projects, expected);
        assert!(
            projects.len() >= 40,
            "the reel should feature the sourced media catalog"
        );
    }

    #[test]
    fn media_reel_embeds_direct_video_without_autoplay_and_links_hosted_video() {
        let mp4 = media::Media {
            kind: "video".into(),
            label: "Gameplay trailer".into(),
            url: "https://example.org/trailer.mp4".into(),
            preview_url: Some("https://example.org/poster.jpg".into()),
            source_url: "https://example.org/project".into(),
        };
        let markup = media_stage_markup(&mp4, "en");
        assert!(markup.contains("controls"));
        assert!(markup.contains("poster=\"https://example.org/poster.jpg\""));
        assert!(!markup.contains("autoplay"));

        let hosted = media::Media {
            kind: "video".into(),
            label: "Gameplay \"preview\"".into(),
            url: "https://www.youtube.com/watch?v=example".into(),
            preview_url: Some("https://img.example.org/poster.jpg".into()),
            source_url: "https://example.org/project".into(),
        };
        let markup = media_stage_markup(&hosted, "en");
        assert!(markup.contains("target=\"_blank\""));
        assert!(markup.contains("&quot;preview&quot;"));
        assert!(!markup.contains("<iframe"));
    }

    #[test]
    fn unreleased_alpha_is_not_bucketed_as_released() {
        assert_eq!(
            status_bucket("Alpha and unreleased"),
            "Early / experimental"
        );
        assert_eq!(status_bucket("Bring-up stage"), "Early / experimental");
        assert_eq!(
            status_bucket("Released; project ships a stable build"),
            "Released"
        );
    }

    #[test]
    fn fuzzy_search_accepts_small_typos_while_verbatim_requires_the_phrase() {
        let text = "the legend of zelda ocarina of time";
        let terms = search_tokens(text);
        let profile_fields = vec!["action adventure game".to_string()];

        assert!(search_matches(
            text,
            &terms,
            Some(&profile_fields),
            "ocarina tiem",
            "fuzzy"
        ));
        assert!(!search_matches(
            text,
            &terms,
            Some(&profile_fields),
            "ocarina tiem",
            "verbatim"
        ));
        assert!(search_matches(
            text,
            &terms,
            Some(&profile_fields),
            "ocarina of time",
            "verbatim"
        ));
        assert!(!search_matches(
            text,
            &terms,
            Some(&profile_fields),
            "ocarina time",
            "verbatim"
        ));
        assert!(search_matches(
            text,
            &terms,
            Some(&profile_fields),
            "acton adventure",
            "fuzzy"
        ));
    }

    #[test]
    fn both_search_modes_work_with_a_real_catalog_entry() {
        let mut catalog = serde_json::from_str::<Catalog>(include_str!("../docs/catalog.json"))
            .expect("published catalog should deserialize");
        catalog.prepare_indexes();
        let project = catalog
            .projects
            .iter()
            .find(|project| project.id == "oot-decomp")
            .expect("Ocarina of Time decompilation should be in the catalog");
        let profile_fields = catalog
            .profile_search
            .get(project.game.as_str())
            .map(Vec::as_slice);

        assert!(search_matches(
            &project.search_text,
            &project.search_terms,
            profile_fields,
            "ocarina of tiem",
            "fuzzy"
        ));
        assert!(search_matches(
            &project.search_text,
            &project.search_terms,
            profile_fields,
            "legend of zelda: ocarina of time",
            "verbatim"
        ));
        assert!(!search_matches(
            &project.search_text,
            &project.search_terms,
            profile_fields,
            "legend zelda",
            "verbatim"
        ));
    }

    #[test]
    fn catalog_precomputes_render_indexes_without_changing_catalog_counts() {
        let mut catalog = serde_json::from_str::<Catalog>(include_str!("../docs/catalog.json"))
            .expect("published catalog should deserialize");
        let expected_targets = catalog
            .projects
            .iter()
            .map(|project| project.game.as_str())
            .collect::<BTreeSet<_>>()
            .len();
        let expected_sources = catalog
            .projects
            .iter()
            .flat_map(|project| project.sources.iter().map(|source| source.url.as_str()))
            .collect::<BTreeSet<_>>()
            .len();

        catalog.prepare_indexes();

        assert_eq!(catalog.target_count, expected_targets);
        assert_eq!(catalog.source_count, expected_sources);
        assert_eq!(catalog.profile_indices.len(), catalog.profiles.len());
        assert_eq!(catalog.profile_search_terms.len(), catalog.profiles.len());
        assert!(catalog.projects.iter().all(|project| {
            !project.search_text.is_empty()
                && project.search_text.contains(&project.game.to_lowercase())
                && project.title_sort_key == project.game.to_lowercase()
        }));
    }

    #[test]
    fn every_game_target_has_a_wikipedia_reference() {
        let catalog = serde_json::from_str::<Catalog>(include_str!("../docs/catalog.json"))
            .expect("published catalog should deserialize");
        let game_keys = catalog
            .projects
            .iter()
            .filter(|project| project.target_type == "Game")
            .map(|project| project.game.as_str())
            .collect::<BTreeSet<_>>();
        assert!(!game_keys.is_empty(), "catalog should include game targets");
        for game_key in game_keys {
            let profile = catalog
                .profiles
                .iter()
                .find(|profile| profile.game_key == game_key)
                .unwrap_or_else(|| panic!("{game_key} is missing its game profile"));
            let url = profile.wikipedia_url.as_deref().unwrap_or_else(|| {
                panic!("{} is missing its Wikipedia reference", profile.game_key)
            });
            assert!(
                url.starts_with("https://en.wikipedia.org/"),
                "{} has an unexpected Wikipedia URL: {url}",
                profile.game_key,
            );
        }
    }

    #[test]
    fn featured_projects_have_ordered_stars_and_real_repository_sources() {
        let catalog = serde_json::from_str::<Catalog>(include_str!("../docs/catalog.json"))
            .expect("published catalog should deserialize");
        assert_eq!(
            catalog.featured_projects.len(),
            3,
            "the showcase should contain three entries"
        );
        for (index, entry) in catalog.featured_projects.iter().enumerate() {
            assert_eq!(
                entry.position,
                index + 1,
                "showcase position should be contiguous"
            );
            let project = catalog
                .projects
                .iter()
                .find(|project| project.id == entry.project_id)
                .unwrap_or_else(|| {
                    panic!(
                        "featured project {} is missing from the index",
                        entry.project_id
                    )
                });
            assert!(
                project.project_url.starts_with("https://github.com/"),
                "{} should link to its GitHub repository",
                project.id
            );
            assert_eq!(
                entry.source_url, project.project_url,
                "{} should cite its actual project repository",
                project.id
            );
            let repo_path = project
                .project_url
                .strip_prefix("https://github.com/")
                .unwrap_or_else(|| panic!("{} should have a GitHub repository URL", project.id));
            assert_eq!(
                entry.ranking_url.trim_end_matches('/'),
                format!(
                    "https://api.github.com/repos/{}",
                    repo_path.trim_end_matches('/')
                ),
                "{} should link to its repository's GitHub star-count endpoint",
                project.id
            );
            assert!(
                entry.github_stars > 0,
                "{} should have a verified popularity count",
                project.id
            );
        }
        assert!(
            catalog
                .featured_projects
                .windows(2)
                .all(|pair| pair[0].github_stars >= pair[1].github_stars),
            "featured projects should be ranked by descending stars"
        );
    }

    #[test]
    fn type_filter_keeps_multiple_selected_kinds_and_ignores_invalid_values() {
        assert_eq!(
            parse_kind_filter(Some("Recompilation,API Translation,Recompilation".into())),
            ["API Translation".to_string(), "Recompilation".to_string()]
                .into_iter()
                .collect()
        );
        assert_eq!(
            parse_kind_filter(Some("all,Retired category,Emulator".into())),
            ["Emulator".to_string()].into_iter().collect()
        );
        assert!(parse_kind_filter(None).is_empty());
    }

    #[test]
    fn page_size_accepts_custom_positive_values_and_bounds_invalid_values() {
        assert_eq!(parse_page_size("37"), 37);
        assert_eq!(parse_page_size("1"), 1);
        assert_eq!(parse_page_size("1000"), 1000);
        assert_eq!(parse_page_size("0"), 25);
        assert_eq!(parse_page_size("1001"), 25);
        assert_eq!(parse_page_size("many"), 25);
    }
}
