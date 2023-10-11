import { Signal } from "@preact/signals";
import { createRef, JSX } from "preact";

interface CheckboxProps extends JSX.HTMLAttributes<HTMLInputElement> {
  checked: Signal<boolean>;
  readonly?: boolean;
}

export default function Checkbox(props: CheckboxProps) {
  const ref = createRef();
  return (
    <input
      {...props}
      ref={ref}
      type="checkbox"
      checked={props.checked.value}
      onClick={() => {
        if (!props.readonly) props.checked.value = ref.current.checked;
      }}
      class="bg-transparent"
    />
  );
}
