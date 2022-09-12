export default function Navbar() {
  return (
    <nav
      className="
          navbar
          navbar-expand-lg
          navbar-light
          relative
          flex
          w-full
          flex-wrap
          items-center
          justify-between
          bg-gray-900
          py-3
          text-gray-200
          shadow-lg"
    >
      <div className="container-fluid flex w-full flex-wrap items-center justify-between px-6">
        <div className="container-fluid">
          <a className="text-xl text-white" href="#">
            Navbar
          </a>
        </div>
      </div>
    </nav>
  );
}
