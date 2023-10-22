use crate::routes;

markup::define! {
    Layout<R: markup::Render>(content: R) {
        @markup::doctype()
        html[lang="en",class="overflow-auto h-full"] {
            head {
                meta[charset="utf-8"];
                meta[name="viewport",contet="width=device-width,initial-scale=1.0"];
                title { "Funicular" }
                link[
                    rel="icon",
                    type="image/svg+xml",
                    href="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxZW0iIGhlaWdodD0iMWVtIiB2aWV3Qm94PSIwIDAgMjQgMjQiPjxwYXRoIGZpbGw9Im5vbmUiIHN0cm9rZT0iY3VycmVudENvbG9yIiBkPSJNNy40NzggMTguMTQ5YTEuNSAxLjUgMCAwIDEtMi45NTQuNTJtMTEuOTk5LTIuMjVhMS41IDEuNSAwIDAgMCAyLjk1NC0uNTJNOCAxMS43NThWNC42MzZtOCA1LjY0OFYzLjE4Mm02Ljk3IDYuMjNjLjAxOS0uNDc3LjAzLS45OC4wMy0xLjUwM0MyMyA0LjQxIDIyLjUgMiAyMi41IDJsLTIxIDMuODE4UzEgOC40MSAxIDExLjkxYzAgLjUyMy4wMTEgMS4wMjIuMDMgMS40OTJtMjEuOTQtMy45OUMyMi44NjIgMTIuMTI3IDIyLjUgMTQgMjIuNSAxNGwtMjEgMy44MThzLS4zNjItMS43NDMtLjQ3LTQuNDE3bTIxLjk0LTMuOTljLTEwLjY1Ni45NzMtMjEuMzAyIDMuODE4LTIxLjk0IDMuOTlNMjMgMTlMMSAyMyIvPjwvc3ZnPg=="
                ];
                link[rel="stylesheet",type="text/css",href="/tailwind-compat-68cebf45fb05bc34.css"];
                link[rel="stylesheet",type="text/css",href="/site.css"];
                script[src="/index.js",defer]{}
            }
            body[
                "hx-boost"="true",
                "hx-select"="#main",
                "hx-target"="#main",
                class="overflow-auto h-full dark:text-white dark:bg-slate-900",
            ] {
                nav[class="py-4"] {
                    ul[class="flex flex-col gap-4 justify-center items-center sm:flex-row"] {
                        li { a[href=routes::IndexPath.to_string(),class="hover:text-violet"] { "Home" } }
                        li { a[href=routes::games::Path.to_string(),class="hover:text-violet"] { "Games" } }
                    }
                }
                main #main[class="container flex flex-col gap-4 justify-center items-center mx-auto"] { @content }
            }
        }
    }
}

markup::define! { NotFound() { h1[class="text-xl font-bold"] { "Not found" } } }
