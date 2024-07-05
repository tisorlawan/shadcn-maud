use axum::routing::get;
use maud::{html, Markup, PreEscaped};
use shadcnui_maud::button::*;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let route = axum::Router::new()
        .route("/", get(root_page))
        .nest_service("/static", ServeDir::new("./static/dist"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind server");
    axum::serve(listener, route).await.expect("serve app");
}

async fn root_page() -> Markup {
    html! {
        html lang="id" translate="no";

        head {
            link rel="stylesheet" href="/static/css/style.css" {}
        }

        body class="bg-background" {
            div class="flex flex-col items-center justify-center h-screen" {
                div class="flex gap-2" {
                    (ui_theme_toggle())

                    (Button::new()
                     .build(html! {
                        "Submit"
                    }))

                    (Button::outline()
                     .build(html! {
                         "Outline"
                     }))

                    (Button::new()
                     .variant(ButtonVariant::Ghost)
                     .build(html! {
                        "Outline"
                    }))
                }
            }
        }

        (script())
    }
}

pub fn ui_theme_toggle() -> Markup {
    html! {
        button id="theme-toggle" type="button" class="bg-primary text-primary-foreground p-2.5 text-sm bg-none rounded-full" {
            svg id="theme-toggle-dark-icon" class="hidden w-4 h-4" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" {
                path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"{}
            }
            svg id="theme-toggle-light-icon" class="hidden w-4 h-4" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" {
                path d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" fill-rule="evenodd" clip-rule="evenodd" {}
            }
        }
    }
}

fn script() -> Markup {
    html! {
        script {
            (PreEscaped(
                    r##"

                    var themeToggleDarkIcon = document.getElementById('theme-toggle-dark-icon');
                    var themeToggleLightIcon = document.getElementById("theme-toggle-light-icon");
                    var themeToggleBtn = document.getElementById("theme-toggle");

                    // Initial theme check and apply
                    if (localStorage.getItem("theme") === "dark" || (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches)) {
                        document.documentElement.classList.add("dark");
                        themeToggleLightIcon !== null && themeToggleLightIcon.classList.remove("hidden");
                    } else {
                        document.documentElement.classList.add("light");
                        themeToggleDarkIcon !== null && themeToggleDarkIcon.classList.remove("hidden");
                    }

                    // Event listener for the toggle button
                    themeToggleBtn !== null && themeToggleBtn.addEventListener("click", function() {
                        themeToggleDarkIcon !== null && themeToggleDarkIcon.classList.toggle("hidden");
                        themeToggleLightIcon !== null && themeToggleLightIcon.classList.toggle("hidden");

                        var currentTheme = localStorage.getItem("theme");

                        if (currentTheme === "light") {
                            document.documentElement.classList.remove("light");
                            document.documentElement.classList.add("dark");
                            localStorage.setItem("theme", "dark");
                        } else {
                            document.documentElement.classList.remove("dark");
                            document.documentElement.classList.add("light");
                            localStorage.setItem("theme", "light");
                        }

                        // If no theme is set in localStorage, default to the opposite theme
                        if (!currentTheme) {
                            var newTheme = document.documentElement.classList.contains("dark") ? "light" : "dark";
                            document.documentElement.classList.toggle("dark");
                            document.documentElement.classList.toggle("light");
                            localStorage.setItem("theme", newTheme);
                        }
                    });
                    "##))
        }
    }
}
