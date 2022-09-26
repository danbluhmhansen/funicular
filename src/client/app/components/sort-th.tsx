import type { ReactNode } from "react";

export default function SortTh({
  children,
  action,
}: {
  children: ReactNode;
  action: () => void;
}) {
  return (
    <th onClick={action} className="is-clickable">
      {children}
    </th>
  );
}
