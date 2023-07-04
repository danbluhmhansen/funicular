import { UnknownPageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";

export default function Page({ url }: UnknownPageProps) {
  return (
    <>
      <Head>
        <title>Funicular - Not found</title>
      </Head>
      <div class="mx-auto">
        <p>404 not found: {url.pathname}</p>
      </div>
    </>
  );
}
