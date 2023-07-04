import { ErrorPageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";

export default function Page({ error }: ErrorPageProps) {
  const { message } = error as Error;
  return (
    <>
      <Head>
        <title>Funicular - Error</title>
      </Head>
      <div class="mx-auto">
        <p class="max-w-xl">{message}</p>
      </div>
    </>
  );
}
