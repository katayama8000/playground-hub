#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/playground")]
    Playground {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Playground() -> Element {
    let programing_language = [
        ("Rust", "https://www.rust-lang.org"),
        ("JavaScript", "https://www.javascript.com"),
        ("TypeScript", "https://www.typescriptlang.org"),
        ("Python", "https://www.python.org"),
        ("Java", "https://www.java.com"),
        ("C++", "https://isocpp.org"),
        ("C#", "https://docs.microsoft.com/en-us/dotnet/csharp/"),
        ("PHP", "https://www.php.net"),
        ("Swift", "https://developer.apple.com/swift/"),
        ("Go", "https://golang.org"),
    ];

    rsx! {
        div {
            h1 { "Playground" }
            ul { style: "list-style-type: none; padding: 0; margin: 0;",
                for (lang, url) in programing_language.iter() {
                    li { style: "padding: 0.5rem; margin: 0.5rem; background-color: #f3f4f6;",
                        a { href: *url, "{lang}" }
                    }
                }
            }
            h1 { style: "color: red; font-size: 2rem; text-align: center;", "CSS" }
            a { style: "color: blue; text-decoration: none;", href: "https://www.google.com", "Google" }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        Link { to: Route::Playground {}, "Go to playground" }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
