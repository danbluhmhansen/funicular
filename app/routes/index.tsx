import { defineRoute } from "$fresh/server.ts";
import { signal } from "@preact/signals";
import Counter from "../islands/Counter.tsx";

export default defineRoute(() => {
  const count = signal(3);
  return (
    <div class="px-4 py-8 mx-auto">
      <div class="max-w-screen-md mx-auto flex flex-col items-center justify-center">
        <h1 class="text-4xl font-bold">Welcome to Fresh</h1>
        <p class="my-4">
          Try updating this message in the
          <code class="mx-2">./routes/index.tsx</code> file, and refresh.
        </p>
        <Counter count={count} />
      </div>
    </div>
  );
});
