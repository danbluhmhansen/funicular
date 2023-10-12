import { Head } from "$fresh/runtime.ts";

export default function Error500() {
  return (
    <>
      <Head>
        <title>500 - Internal server error</title>
      </Head>
      <h1 class="text-4xl font-bold">500 - Internal server error</h1>
    </>
  );
}
