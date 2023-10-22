import "htmx.org";
import "@unocss/reset/tailwind-compat.css";
import Alpine from "alpinejs";

window.htmx = require("htmx.org");

window.Alpine = Alpine;
Alpine.start();

window.htmx.config.defaultSwapStyle = "outerHTML";
