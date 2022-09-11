export default function Index() {
  return (
    <>
      <nav
        className="
          relative
          w-full
          flex
          flex-wrap
          items-center
          justify-between
          py-3
          bg-gray-100
          text-gray-500
          hover:text-gray-700
          focus:text-gray-700
          shadow-lg"
      >
        <div className="container-fluid w-full flex flex-wrap items-center justify-between px-6">
          <div className="container-fluid">
            <a className="text-xl text-black" href="#">
              Navbar
            </a>
          </div>
        </div>
      </nav>
      <div style={{ fontFamily: "system-ui, sans-serif", lineHeight: "1.4" }}>
        <h1>Welcome to Remix</h1>
        <ul>
          <li>
            <a
              target="_blank"
              href="https://remix.run/tutorials/blog"
              rel="noreferrer"
            >
              15m Quickstart Blog Tutorial
            </a>
          </li>
          <li>
            <a
              target="_blank"
              href="https://remix.run/tutorials/jokes"
              rel="noreferrer"
            >
              Deep Dive Jokes App Tutorial
            </a>
          </li>
          <li>
            <a target="_blank" href="https://remix.run/docs" rel="noreferrer">
              Remix Docs
            </a>
          </li>
        </ul>
      </div>
    </>
  );
}
