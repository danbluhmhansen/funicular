import type { ReactElement } from "react";
import { useToggle } from "../hooks/use-toggle";

interface NavItem {
  path: string;
  name: string;
}

export function Navbar({
  children,
  primaryButton,
  secondaryButton,
}: {
  children: ReactElement | ReactElement[] | undefined;
  primaryButton: NavItem | undefined;
  secondaryButton: NavItem | undefined;
}) {
  const { path: pPath, name: pName } = { ...primaryButton };
  const { path: sPath, name: sName } = { ...secondaryButton };
  const [active, toggleActive] = useToggle();

  return (
    <nav className="navbar">
      <div className="navbar-brand">
        <a className="navbar-burger" onClick={toggleActive}>
          <span />
          <span />
          <span />
        </a>
      </div>
      <div className={"navbar-menu" + (active ? " is-active" : "")}>
        <div className="navbar-start">{children}</div>
        {primaryButton && secondaryButton && (
          <div className="navbar-end">
            <div className="navbar-item">
              <div className="buttons">
                {
                  <a href={pPath} className="button is-primary">
                    {pName}
                  </a>
                }
                {
                  <a href={sPath} className="button is-light">
                    {sName}
                  </a>
                }
              </div>
            </div>
          </div>
        )}
      </div>
    </nav>
  );
}
