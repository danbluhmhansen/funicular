import type { ReactNode } from "react";
import { createRoot } from "react-dom/client";

export default function Page(page: ReactNode, query?: string | undefined) {
  createRoot(
    document.querySelector(query ?? "#container") as HTMLElement
  ).render(page);
}
