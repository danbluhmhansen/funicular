import { createRoot } from "react-dom/client";

export default function Index() {
  return <h1 className="title">Hello, World!</h1>;
}

createRoot(document.querySelector("#container") as HTMLElement).render(
  <Index />
);
