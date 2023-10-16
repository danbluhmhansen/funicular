markup::define! {
    Layout<Main: markup::Render>(main: Main) {
        @markup::doctype()
        html {
            head {
                meta[charset="utf-8"];
                meta[name="viewport",contet="width=device-width,initial-scale=1.0"];
                title { "Funicular" }
                link[rel="icon",type="image/svg+xml",href="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxZW0iIGhlaWdodD0iMWVtIiB2aWV3Qm94PSIwIDAgMjQgMjQiPjxwYXRoIGZpbGw9Im5vbmUiIHN0cm9rZT0iY3VycmVudENvbG9yIiBkPSJNNy40NzggMTguMTQ5YTEuNSAxLjUgMCAwIDEtMi45NTQuNTJtMTEuOTk5LTIuMjVhMS41IDEuNSAwIDAgMCAyLjk1NC0uNTJNOCAxMS43NThWNC42MzZtOCA1LjY0OFYzLjE4Mm02Ljk3IDYuMjNjLjAxOS0uNDc3LjAzLS45OC4wMy0xLjUwM0MyMyA0LjQxIDIyLjUgMiAyMi41IDJsLTIxIDMuODE4UzEgOC40MSAxIDExLjkxYzAgLjUyMy4wMTEgMS4wMjIuMDMgMS40OTJtMjEuOTQtMy45OUMyMi44NjIgMTIuMTI3IDIyLjUgMTQgMjIuNSAxNGwtMjEgMy44MThzLS4zNjItMS43NDMtLjQ3LTQuNDE3bTIxLjk0LTMuOTljLTEwLjY1Ni45NzMtMjEuMzAyIDMuODE4LTIxLjk0IDMuOTlNMjMgMTlMMSAyMyIvPjwvc3ZnPg=="];
                style { "[un-cloak]{display:none;}@media(prefers-color-scheme:dark){html{background-color:#0f172a;}}" }
                link[rel="stylesheet",href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind-compat.min.css"];
                script[src="https://unpkg.com/htmx.org@1.9.6",defer]{}
            }
            body."dark:text-white"."dark:bg-slate-900"["un-cloak","hx-boost"="true"] {
                nav."py-4" {
                    ul.flex."flex-col"."gap-4"."justify-center"."items-center"."sm:flex-row" {
                        li { a."hover:text-violet"[href={crate::routes::IndexPath.to_string()}] { "Home" } }
                        li { a."hover:text-violet"[href={crate::routes::games::Path.to_string()}] { "Games" } }
                    }
                }
                main.container.flex."flex-col"."gap-4"."justify-center"."items-center"."mx-auto" {
                    @main
                }
                script[type="module"] {
                    "import init from 'https://esm.sh/@unocss/runtime';import presetUno from 'https://esm.sh/@unocss/preset-uno';import {presetForms} from 'https://esm.sh/@julr/unocss-preset-forms';import presetIcons from 'https://esm.sh/@unocss/preset-icons/browser';init({defaults:{presets:[presetUno({dark:'media'}),presetForms(),presetIcons({cdn:'https://esm.sh/'})]}});"
                }
            }
        }
    }
}

markup::define! {
    NotFound {
        h1."text-xl"."font-bold" { "Not found" }
    }
}
