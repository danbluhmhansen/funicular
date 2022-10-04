import type { ReactNode } from "react";

export function Title({
  size,
  sub,
  spaced,
  children,
}: {
  size: 1 | 2 | 3 | 4 | 5 | 6;
  sub?: boolean | undefined;
  spaced?: boolean | undefined;
  children?: ReactNode | undefined;
}) {
  const className =
    `${sub ? "subtitle" : "title"} is-${size}` + (spaced ? " is-spaced" : "");
  switch (size) {
    case 1:
      return <h1 className={className}>{children}</h1>;
    case 2:
      return <h2 className={className}>{children}</h2>;
    case 3:
      return <h3 className={className}>{children}</h3>;
    case 4:
      return <h4 className={className}>{children}</h4>;
    case 5:
      return <h5 className={className}>{children}</h5>;
    case 6:
      return <h6 className={className}>{children}</h6>;
  }
}
