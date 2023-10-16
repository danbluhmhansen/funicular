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
                script[type="module"] {
                    "import initUnocssRuntime from 'https://cdn.jsdelivr.net/npm/@unocss/runtime@0.56.5/+esm';import presetUno from 'https://cdn.jsdelivr.net/npm/@unocss/preset-uno@0.56.5/+esm';import { presetForms } from 'https://cdn.jsdelivr.net/npm/@julr/unocss-preset-forms@0.0.5/+esm';import presetIcons from 'https://cdn.jsdelivr.net/npm/@unocss/preset-icons@0.56.5/+esm';initUnocssRuntime({defaults:{presets:[presetUno({dark:'media'}),presetForms(),presetIcons({cdn:'https://esm.sh/'})]}});"
                }
                script[src="https://unpkg.com/htmx.org@1.9.6"]{}
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
            }
        }
    }
}

markup::define! {
    NotFound {
        h1."text-xl"."font-bold" { "Not found" }
    }
}
