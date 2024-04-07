use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(app);
}

fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let card_items = [
        (
            "Rust",
            "https://play.rust-lang.org",
            "https://www.rust-lang.org/logos/rust-logo-blk.svg",
        ),
        (
            "JavaScript",
            "https://playcode.io/javascript",
            "https://upload.wikimedia.org/wikipedia/commons/6/6a/JavaScript-logo.png",
        ),
        (
            "TypeScript",
            "https://playcode.io/typescript",
            "https://upload.wikimedia.org/wikipedia/commons/4/4c/Typescript_logo_2020.svg",
        ),
        (
            "Python",
            "https://programiz.pro/ide/python",
            "https://upload.wikimedia.org/wikipedia/commons/c/c3/Python-logo-notext.svg",
        ),
        (
            "Java",
            "https://dev.java/playground/",
            "https://upload.wikimedia.org/wikipedia/en/3/30/Java_programming_language_logo.svg",
        ),
        (
            "C",
            "https://www.tutorialspoint.com/compile_c_online.php",
            "https://upload.wikimedia.org/wikipedia/commons/1/19/C_Logo.png",
        ),
        (
            "C++",
            "https://cpp.sh/",
            "https://upload.wikimedia.org/wikipedia/commons/1/18/ISO_C%2B%2B_Logo.svg",
        ),
        (
            "C#",
            "https://dotnetfiddle.net/",
            "https://upload.wikimedia.org/wikipedia/commons/0/0d/C_Sharp_wordmark.svg",
        ),
        (
            "PHP",
            "https://php-play.dev/",
            "https://upload.wikimedia.org/wikipedia/commons/2/27/PHP-logo.svg",
        ),
        (
            "Go",
            "https://go.dev/play/",
            "https://upload.wikimedia.org/wikipedia/commons/0/05/Go_Logo_Blue.svg",
        ),
        (
            "Ruby",
            "https://repl.it/languages/ruby",
            "https://upload.wikimedia.org/wikipedia/commons/7/73/Ruby_logo.svg",
        ),
        (
            "Haskell",
            "https://repl.it/languages/haskell",
            "https://upload.wikimedia.org/wikipedia/commons/1/1c/Haskell-Logo.svg",
        ),
    ];

    rsx! {
        header {
         style: "display: flex; align-items: center; padding: 10px; background-color: #4CAF50; color: white; border-radius: 8px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);",
         h1 { style: "margin: 0; font-size: 1.5rem;", "Playground-Hub" },
         a { href: "https://github.com/katayama8000/playground-hub", target: "_blank", style: "text-decoration: none; color: inherit; margin-left: auto;",
             img { src: "https://upload.wikimedia.org/wikipedia/commons/9/91/Octicons-mark-github.svg", style: "width: 30px; height: 30px; margin-right: 10px;", alt: "GitHub Logo" }
          }
        }
        div {
            // Using grid layout for card-like appearance
            div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 1rem;",
                for (lang, url, logo_url) in card_items.iter() {
                    // Card style for each language
                    div { style: "padding: 1rem; background-color: #f3f4f6; border-radius: 8px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);",
                        a { href: *url, target: "_blank", // Open link in new tab
                            div { style: "font-weight: bold;", "{lang}" },
                            img { style: "width: 100px; height: 100px;", src: *logo_url, alt: "{lang} Logo" }
                        }
                    }
                }
            }
        }
    }
}
