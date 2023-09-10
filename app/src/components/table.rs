use std::ops::Deref;

use maud::{html, Markup, Render};

use crate::{CAPTION, THEAD, TR};

pub enum TableHead<'a> {
    Checkbox(&'a str),
    Header(Markup),
}

pub enum TableData<'a> {
    Checkbox(&'a str, Option<String>),
    Data(Markup),
}

pub struct Table<'a> {
    caption: Option<Markup>,
    head: Vec<TableHead<'a>>,
    body: Vec<Vec<TableData<'a>>>,
    empty: Option<Markup>,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self {
            caption: None,
            head: vec![],
            body: vec![],
            empty: None,
        }
    }

    pub fn caption(mut self, caption: Markup) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn head(mut self, head: TableHead<'a>) -> Self {
        self.head.push(head);
        self
    }

    pub fn heads(mut self, heads: &mut Vec<TableHead<'a>>) -> Self {
        self.head.append(heads);
        self
    }

    pub fn body(mut self, body: Vec<Vec<TableData<'a>>>) -> Self {
        self.body = body;
        self
    }

    pub fn body_or(self, body: Vec<Vec<TableData<'a>>>, or: Markup) -> Self {
        self.body(body).empty(or)
    }

    pub fn empty(mut self, empty: Markup) -> Self {
        self.empty = Some(empty);
        self
    }
}

impl Render for Table<'_> {
    fn render(&self) -> Markup {
        html! {
            @if !self.body.is_empty() {
                div class="overflow-x-auto relative rounded shadow-md" {
                    table class="w-full" {
                        // TODO: avoid clone
                        @if let Some(caption) = self.caption.to_owned() {
                            caption class=(CAPTION) { (caption) }
                        }
                        @if !self.head.is_empty() {
                            thead class=(THEAD) {
                                tr {
                                    @for head in self.head.deref() {
                                        @match head {
                                            TableHead::Checkbox(name) =>
                                                th class="p-3 text-center" {
                                                    input type="checkbox" name=(name) value="true" class="bg-transparent";
                                                },
                                            TableHead::Header(children) => th class="py-3 px-6 text-left" { (children) },
                                        }
                                    }
                                }
                            }
                        }
                        tbody {
                            @for row in self.body.deref() {
                                tr class=(TR) {
                                    @for data in row {
                                        @match data {
                                            TableData::Checkbox(name, value) =>
                                                td class="p-3 text-center" {
                                                    input
                                                        type="checkbox"
                                                        name=(name)
                                                        value=[value]
                                                        class="bg-transparent";
                                                },
                                            TableData::Data(children) => td class="py-3 px-6" { (children) },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            // TODO: avoid clone
            } @else if let Some(empty) = self.empty.to_owned() {
                (empty)
            }
        }
    }
}
