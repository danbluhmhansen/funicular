use std::fmt::Display;

use maud::{html, Markup, DOCTYPE};

use crate::DIALOG;

pub struct Link<'a> {
    pub href: &'a str,
    pub children: Markup,
}

impl<'a> Link<'a> {
    pub fn new(href: &'a str, children: Markup) -> Self {
        Self { href, children }
    }
}

pub struct Page<'a> {
    children: Markup,
    dialogs: Vec<Dialog<'a>>,
}

impl<'a> Page<'a> {
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            dialogs: vec![],
        }
    }

    pub fn dialog(mut self, dialog: Dialog<'a>) -> Self {
        self.dialogs.push(dialog);
        self
    }

    pub fn render(self) -> Markup {
        let links = vec![Link::new("/", html! { "Home" }), Link::new("/games", html! { "Games" })];
        html! {
            (DOCTYPE)
            html lang="en" class="h-full overflow-auto" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width,initial-scale=1";
                    title { "Funicular" }
                    link
                        rel="icon"
                        type="image/svg+xml"
                        href="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxZW0iIGhlaWdodD0iMWVtIiB2aWV3Qm94PSIwIDAgMjQgMjQiPjxwYXRoIGZpbGw9Im5vbmUiIHN0cm9rZT0iY3VycmVudENvbG9yIiBkPSJNNy40NzggMTguMTQ5YTEuNSAxLjUgMCAwIDEtMi45NTQuNTJtMTEuOTk5LTIuMjVhMS41IDEuNSAwIDAgMCAyLjk1NC0uNTJNOCAxMS43NThWNC42MzZtOCA1LjY0OFYzLjE4Mm02Ljk3IDYuMjNjLjAxOS0uNDc3LjAzLS45OC4wMy0xLjUwM0MyMyA0LjQxIDIyLjUgMiAyMi41IDJsLTIxIDMuODE4UzEgOC40MSAxIDExLjkxYzAgLjUyMy4wMTEgMS4wMjIuMDMgMS40OTJtMjEuOTQtMy45OUMyMi44NjIgMTIuMTI3IDIyLjUgMTQgMjIuNSAxNGwtMjEgMy44MThzLS4zNjItMS43NDMtLjQ3LTQuNDE3bTIxLjk0LTMuOTljLTEwLjY1Ni45NzMtMjEuMzAyIDMuODE4LTIxLjk0IDMuOTlNMjMgMTlMMSAyMyIvPjwvc3ZnPg==";
                    link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind-compat.min.css";
                    link rel="stylesheet" type="text/css" href="/site.css";
                }
                body class="h-full dark:text-white dark:bg-slate-900 overflow-auto" {
                    @for dialog in self.dialogs {
                        (dialog.render())
                    }
                    header class="py-4" {
                        nav {
                            ul class="flex flex-col gap-4 justify-center items-center sm:flex-row" {
                                @for link in &links {
                                    li { a href=(link.href) class="hover:text-violet-500" { (link.children) } }
                                }
                            }
                        }
                    }
                    main class="container flex flex-col gap-4 justify-center items-center mx-auto" { (self.children) }
                    footer class="py-4";
                }
            }
        }
    }
}

pub struct Dialog<'a> {
    children: Markup,
    id: Option<String>,
    title: Option<&'a str>,
}

impl<'a> Dialog<'a> {
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            id: None,
            title: None,
        }
    }

    pub fn id<D: Display>(mut self, id: D) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn render(self) -> Markup {
        html! {
            dialog id=[self.id] class=(DIALOG) {
                div class="flex z-10 flex-col gap-4 p-4 max-w-sm rounded border dark:text-white dark:bg-slate-900" {
                    div {
                        a href="#!" class="float-right w-4 h-4 i-tabler-x" {}
                        @if let Some(title) = self.title { h2 class="text-xl" { (title) } }
                    }
                    (self.children)
                }
                a href="#!" class="fixed inset-0" {}
            }
        }
    }
}
