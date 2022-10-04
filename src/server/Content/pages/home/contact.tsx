import { Title } from "@funicular/shared";
import Page from "page";

export default function Contact() {
  return (
    <>
      <Title size={2}>Contact.</Title>
      <Title size={3} sub>
        Your contact page.
      </Title>
      <address>
        One Microsoft Way
        <br />
        Redmond, WA 98052-6399
        <br />
        <abbr title="Phone">P:</abbr>
        425.555.0100
      </address>
      <address>
        <strong>Support:</strong>{" "}
        <a href="mailto:Support@example.com">Support@example.com</a>
        <br />
        <strong>Marketing:</strong>{" "}
        <a href="mailto:Marketing@example.com">Marketing@example.com</a>
      </address>
    </>
  );
}

Page(<Contact />);
