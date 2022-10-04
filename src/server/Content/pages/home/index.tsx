import { Title } from "@funicular/shared";
import Page from "page";

export default function Index() {
  return <Title size={1}>Hello, World!</Title>;
}

Page(<Index />);
