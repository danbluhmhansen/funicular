import { createRoot } from "react-dom/client";

export default function About() {
  return (
    <>
      <h2 className="title">About.</h2>
      <h3 className="title">Your application description page.</h3>
      <p>Use this area to provide additional information.</p>
    </>
  );
}

createRoot(document.querySelector("#container") as HTMLElement).render(
  <About />
);
