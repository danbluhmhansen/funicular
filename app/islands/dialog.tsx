import { useSignal } from "@preact/signals";
import { useEffect, useRef } from "preact/hooks";
import { Button } from "~components/button.tsx";

export default function Dialog() {
  const show = useSignal(false);
  const ref = useRef<HTMLDialogElement | null>(null);

  useEffect(() => {
    if (show.value) ref.current?.show();
  }, [show.value]);

  return (
    <>
      <dialog ref={ref}>
        <p>Greetings, one and all!</p>
        <form method="dialog">
          <button>OK</button>
        </form>
      </dialog>
      <Button onClick={() => show.value = true}>
        Foo
      </Button>
    </>
  );
}
