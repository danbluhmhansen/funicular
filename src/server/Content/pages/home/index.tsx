import { render } from "react-dom";

export default function Index() {
  return <h1>Hello, World!</h1>;
}

render(<Index />, document.querySelector("#container"));
