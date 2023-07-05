import { ComponentChild, JSX } from "preact";
import { TbChevronRight } from "react-icons/tb";

interface BreadcrumbProps extends JSX.HTMLAttributes {
  path: string;
}

export function Breadcrumb(props: BreadcrumbProps) {
  const children = props.children as ComponentChild[];
  const path = props.path;

  const segments = path.split("/").slice(1).map((_p, i, s) => {
    return "/" + s.slice(0, i + 1).join("/");
  }).slice(1).map((s, i) => {
    return { href: s, child: children[i] };
  });

  const links = segments.slice(0, -1);
  const current = segments[segments.length - 1].child;

  return (
    <nav class="flex">
      <ol class="inline-flex items-center">
        {links.map((l, i) => (
          <a href={l.href} class="inline-flex items-center">
            {i !== 0 && <TbChevronRight />}
            {l.child}
          </a>
        ))}
        <li>
          <div class="flex items-center">
            <TbChevronRight />
            {current}
          </div>
        </li>
      </ol>
    </nav>
  );
}
