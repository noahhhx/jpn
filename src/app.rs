use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{A, Redirect, Route, Router, Routes},
    hooks::use_params_map,
    path,
};

use crate::trip::{self, Block, DAYS, Date, INFO, Section};

/// Applies the saved theme before first paint and handles the toggle button.
const THEME_SCRIPT: &str = r#"(function () {
  var root = document.documentElement;
  try { var saved = localStorage.getItem("theme"); if (saved) root.dataset.theme = saved; } catch (e) {}
  document.addEventListener("click", function (e) {
    if (!e.target.closest("[data-theme-toggle]")) return;
    var dark = root.dataset.theme
      ? root.dataset.theme === "dark"
      : matchMedia("(prefers-color-scheme: dark)").matches;
    var next = dark ? "light" : "dark";
    root.dataset.theme = next;
    try { localStorage.setItem("theme", next); } catch (e) {}
  });
})();"#;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script inner_html=THEME_SCRIPT></script>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/jpn.css"/>
        <Title text="Japan"/>
        <Router>
            <header class="site-header">
                <div class="wrap site-header-inner">
                    <A href="/days" attr:class="brand">
                        <span class="brand-mark" aria-hidden="true"></span>
                        "Japan"
                    </A>
                    <nav class="site-nav">
                        <A href="/days">"All days"</A>
                        <A href="/info">"Info"</A>
                        <button class="theme-toggle" type="button" data-theme-toggle aria-label="Toggle light and dark mode">
                            <span aria-hidden="true"></span>
                        </button>
                    </nav>
                </div>
            </header>
            <main class="wrap">
                <Routes fallback=NotFound>
                    <Route path=StaticSegment("") view=|| view! { <Home redirect=true/> }/>
                    <Route path=path!("/days") view=|| view! { <Home/> }/>
                    <Route path=path!("/day/:n") view=DayPage/>
                    <Route path=path!("/info") view=InfoPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Overview of every day. When `redirect` is set and today is a trip day,
/// jumps straight to that day's page instead.
#[component]
fn Home(#[prop(optional)] redirect: bool) -> impl IntoView {
    let today = Date::today_in_japan();
    let today_n = trip::today_number();
    let redirect = redirect
        .then_some(today_n)
        .flatten()
        .map(|n| view! { <Redirect path=format!("/day/{n}")/> });

    let first = trip::date_of(0);
    let last = trip::date_of(trip::last());

    view! {
        {redirect}
        <section class="intro">
            <p class="eyebrow">"Itinerary"</p>
            <h1>"Japan " {first.ymd().0}</h1>
            <p class="meta">
                {first.long()} " – " {last.long()} <span class="sep">"/"</span> {DAYS.len()} " days"
            </p>
        </section>
        <ol class="day-grid">
            {DAYS
                .iter()
                .enumerate()
                .map(|(n, day)| {
                    let date = trip::date_of(n);
                    let is_today = today_n == Some(n);
                    let is_past = today.is_some_and(|t| date < t);
                    view! {
                        <li>
                            <A
                                href=format!("/day/{n}")
                                attr:class="day-tile"
                                attr:data-today=is_today.then_some("true")
                                attr:data-past=is_past.then_some("true")
                            >
                                <span class="day-tile-top">
                                    <span class="day-num">{format!("{n:02}")}</span>
                                    <span class="day-date">{date.short()}</span>
                                </span>
                                <span class="day-city">{day.city}</span>
                                <span class="day-title">{day.title}</span>
                            </A>
                        </li>
                    }
                })
                .collect_view()}
        </ol>
    }
}

#[component]
fn DayPage() -> impl IntoView {
    let params = use_params_map();
    let n = move || params.read().get("n").and_then(|s| s.parse::<usize>().ok());

    move || {
        let Some((n, day)) = n().and_then(|n| trip::day(n).map(|d| (n, d))) else {
            return view! { <NotFound/> }.into_any();
        };
        let date = trip::date_of(n);
        let prev = n.checked_sub(1).and_then(|p| trip::day(p).map(|d| (p, d)));
        let next = trip::day(n + 1).map(|d| (n + 1, d));

        view! {
            <Title text=format!("Day {n} · {} — Japan", day.city)/>
            <article class="day">
                <header class="day-header">
                    <p class="eyebrow">
                        "Day " {format!("{n:02}")} <span class="sep">"/"</span> {format!("{:02}", trip::last())}
                    </p>
                    <h1>{day.title}</h1>
                    <p class="meta">
                        {day.city} <span class="sep">"/"</span> {date.weekday()} " " {date.long()}
                    </p>
                </header>

                {if day.sections.is_empty() {
                    view! { <p class="empty">"Nothing planned yet."</p> }.into_any()
                } else {
                    view! { <Sections sections=day.sections/> }.into_any()
                }}

                <nav class="pager">
                    {match prev {
                        Some((p, d)) => view! {
                            <A href=format!("/day/{p}") attr:class="pager-link">
                                <span class="pager-label">"← Day " {p}</span>
                                <span class="pager-title">{d.title}</span>
                            </A>
                        }
                            .into_any(),
                        None => view! { <span class="pager-link pager-empty"></span> }.into_any(),
                    }}
                    {match next {
                        Some((p, d)) => view! {
                            <A href=format!("/day/{p}") attr:class="pager-link pager-next">
                                <span class="pager-label">"Day " {p} " →"</span>
                                <span class="pager-title">{d.title}</span>
                            </A>
                        }
                            .into_any(),
                        None => view! { <span class="pager-link pager-empty"></span> }.into_any(),
                    }}
                </nav>
            </article>
        }
            .into_any()
    }
}

#[component]
fn InfoPage() -> impl IntoView {
    view! {
        <Title text="Info — Japan"/>
        <article class="day">
            <header class="day-header">
                <p class="eyebrow">"Info"</p>
                <h1>"Before & during"</h1>
                <p class="meta">"Links, apps, rules and what to bring"</p>
            </header>
            <Sections sections=INFO/>
        </article>
    }
}

#[component]
fn Sections(sections: &'static [Section]) -> impl IntoView {
    view! {
        <div class="sections">
            {sections
                .iter()
                .map(|section| {
                    view! {
                        <section class="section">
                            {(!section.heading.is_empty())
                                .then(|| view! { <h2 class="section-heading">{section.heading}</h2> })}
                            <div class="section-body">
                                {section.blocks.iter().map(block).collect_view()}
                            </div>
                        </section>
                    }
                })
                .collect_view()}
        </div>
    }
}

fn block(block: &'static Block) -> AnyView {
    match block {
        Block::Text(text) => view! { <p class="text">{*text}</p> }.into_any(),
        Block::Note(text) => view! { <p class="note">{*text}</p> }.into_any(),
        Block::List(entries) => view! {
            <ul class="entries">
                {entries
                    .iter()
                    .map(|e| {
                        let name = if e.url.is_empty() {
                            view! { <span class="entry-name">{e.name}</span> }.into_any()
                        } else {
                            view! {
                                <a class="entry-name entry-link" href=e.url target="_blank" rel="noopener">
                                    {e.name} <span aria-hidden="true">" ↗"</span>
                                </a>
                            }
                                .into_any()
                        };
                        view! {
                            <li class="entry">
                                {name}
                                {(!e.note.is_empty()).then(|| view! { <span class="entry-note">{e.note}</span> })}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        }
            .into_any(),
        Block::Checklist(items) => view! {
            <ul class="checklist">
                {items.iter().map(|i| view! { <li>{*i}</li> }).collect_view()}
            </ul>
        }
            .into_any(),
        Block::Schedule(steps) => view! {
            <ol class="timeline">
                {steps
                    .iter()
                    .map(|s| {
                        view! {
                            <li class="timeline-item">
                                <time class="timeline-time">{s.time}</time>
                                <div>
                                    <p class="timeline-title">{s.title}</p>
                                    {(!s.note.is_empty())
                                        .then(|| view! { <p class="timeline-note">{s.note}</p> })}
                                </div>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        }
            .into_any(),
        Block::Place(place) => view! {
            <div class="place">
                <div class="place-body">
                    <p class="place-name">{place.name}</p>
                    {(!place.address.is_empty())
                        .then(|| view! { <p class="place-address">{place.address}</p> })}
                    {(!place.note.is_empty()).then(|| view! { <p class="place-note">{place.note}</p> })}
                </div>
                <a class="place-map" href=place.maps_url() target="_blank" rel="noopener">
                    "Map ↗"
                </a>
            </div>
        }
            .into_any(),
        Block::Links(links) => view! {
            <ul class="links">
                {links
                    .iter()
                    .map(|l| {
                        view! {
                            <li>
                                <a href=l.url target="_blank" rel="noopener">
                                    <span class="link-label">{l.label} " ↗"</span>
                                    <span class="link-host">{l.host()}</span>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        }
            .into_any(),
        Block::Image { src, caption } => view! {
            <figure class="figure">
                <a href=*src target="_blank" rel="noopener">
                    <img src=*src alt=*caption loading="lazy"/>
                </a>
                {(!caption.is_empty()).then(|| view! { <figcaption>{*caption}</figcaption> })}
            </figure>
        }
            .into_any(),
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
        res.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="Not found — Japan"/>
        <section class="intro">
            <p class="eyebrow">"404"</p>
            <h1>"Page not found"</h1>
            <p class="meta"><A href="/days">"Back to all days"</A></p>
        </section>
    }
}
