import { createRef, JSX } from "preact";

interface CheckboxProps extends JSX.HTMLAttributes<HTMLInputElement> {
  noSet?: boolean;
}

export default function Checkbox(props: CheckboxProps) {
  const { checked, noSet } = props;
  const ref = createRef();
  return (
    <input
      {...props}
      ref={ref}
      type="checkbox"
      onClick={() => {
        if (typeof checked === "object" && !noSet) checked.value = ref.current.checked;
      }}
    />
  );
}
