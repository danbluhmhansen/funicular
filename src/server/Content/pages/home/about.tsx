import { Title } from "@funicular/shared";
import Page from "page";

export default function About() {
  return (
    <>
      <Title size={2}>About.</Title>
      <Title size={3} sub>
        Your application description page.
      </Title>
      <p>Use this area to provide additional information.</p>
    </>
  );
}

Page(<About />);
