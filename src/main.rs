use std::str::FromStr;

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
        Cookie {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

#[component]
fn Cookie() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "cookie",
            h4 { "ServerFn Cookie" }
            button {
                onclick: move |_| async move {
                    let data = cookie_server().await.unwrap();
                    response.set(data);
                },
                "Set Cookie"
            }

            if !response().is_empty() {
                p {
                    "Server cookied: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[derive(Debug, Clone)]
pub enum Error {
    ServerError(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ServerError(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

impl FromStr for Error {
    type Err = Self;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Error::ServerError(s.to_string()))
    }
}

#[server(CookieServer)]
#[middleware(tower_cookies::CookieManagerLayer::new())]
async fn cookie_server() -> Result<String, ServerFnError<Error>> {
    let cookie_jar: tower_cookies::Cookies = extract().await.map_err(|_| {
        ServerFnError::ServerError::<Error>("Failed to extract cookies".to_string())
    })?;
    let cookie = cookie_jar.get("cookie_name").map(|c| c.value().to_string());
    if cookie.is_none() {
        cookie_jar.add(
            tower_cookies::Cookie::build(("cookie_name", "cookie_value"))
                .http_only(true)
                .secure(true)
                .same_site(tower_cookies::cookie::SameSite::Strict)
                .into(),
        );
        return Ok("Cookie set".to_string());
    } else {
        return Ok(format!("Cookie: {}", cookie.unwrap()));
    }
}
