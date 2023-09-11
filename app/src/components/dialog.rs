use std::fmt::Display;

use maud::{html, Markup, Render};

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
}

impl Render for Dialog<'_> {
    fn render(&self) -> Markup {
        html! {
            dialog
                id=[self.id.as_deref()]
                class="hidden z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm" {
                div class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900" {
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
