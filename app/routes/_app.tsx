import { AppProps } from "$fresh/server.ts";

export default function App({ Component }: AppProps) {
  return (
    <div class="flex flex-col min-h-screen dark:bg-slate-900 dark:text-white">
      <header class="mx-auto">
        <nav>
          <ul class="flex flex-row">
            <li>
              <a href="/">Home</a>
            </li>
            <li>
              <a href="/actors">Actors</a>
            </li>
          </ul>
        </nav>
      </header>
      <Component />
    </div>
  );
}
