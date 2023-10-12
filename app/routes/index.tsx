import { defineRoute } from "$fresh/server.ts";
import { signal } from "@preact/signals";
import Counter from "~islands/counter.tsx";

export default defineRoute(() => {
  const count = signal(3);
  return (
    <div class="py-8 px-4 mx-auto">
      <div class="flex flex-col justify-center items-center mx-auto max-w-screen-md">
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
