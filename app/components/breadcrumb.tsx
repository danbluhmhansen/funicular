import { ComponentChild, JSX } from "preact";
import { TbChevronRight } from "react-icons/tb";

interface BreadcrumbProps extends JSX.HTMLAttributes {
  path: string;
  separator?: JSX.Element | undefined;
}

export function Breadcrumb(props: BreadcrumbProps) {
  const children = props.children as ComponentChild[];
  const { path, separator } = props;
  const sep = separator ?? <TbChevronRight />;

  const segments = path.split("/").slice(1).map((_p, i, s) => {
    return "/" + s.slice(0, i + 1).join("/");
  }).slice(1).map((href, i) => {
    return { href, child: children[i] };
  });

  const links = segments.slice(0, -1);
  const current = segments[segments.length - 1].child;

  return (
    <nav class="flex">
      <ol class="inline-flex items-center">
        {links.map(({ href, child }, i) => (
          <li key={i}>
            {i === 0
              ? (
                <a href={href} class="inline-flex items-center hover:underline">
                  {child}
                </a>
              )
              : (
                <div class="flex items-center">
                  {sep}
                  <a
                    href={href}
                    class="inline-flex items-center hover:underline"
                  >
                    {child}
                  </a>
                </div>
              )}
          </li>
        ))}
        <li>
          <div class="flex items-center">
            {sep}
            {current}
          </div>
        </li>
      </ol>
    </nav>
  );
}
