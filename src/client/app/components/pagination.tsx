import { useSearchParams } from "@remix-run/react";
import { useEffect, useState } from "react";

function GetCurrentPages(items: number[], selected: number) {
  if (items.length < 4) return items;
  else if (selected < 4) return items.slice(0, 5);
  else if (selected > items.at(-4)!) return items.slice(-5);
  else return items.slice(selected - 3, selected + 2);
}

export default function Pagination({
  count,
  pageSizes,
}: {
  count: number;
  pageSizes: number[];
}) {
  const [params, setParams] = useSearchParams();
  const [page, setPage] = useState(+(params.get("page") ?? "1"));
  const [pageSize, setPageSize] = useState(+(params.get("pageSize") ?? "10"));

  if (!pageSizes.some((ps) => ps === pageSize)) setPageSize(10);

  const pageCount = count ? Math.ceil(count / pageSize) : 1;
  const pages = GetCurrentPages(
    Array.from(Array(pageCount + 1).keys()).slice(2, -1),
    page
  );

  useEffect(() => {
    params.set("page", page.toString());
    params.set("pageSize", pageSize.toString());
    setParams(params);
  }, [page, pageSize]);

  return (
    <nav className="pagination">
      <a className="pagination-previous" onClick={() => setPage((p) => p - 1)}>
        Prev
      </a>
      <a className="pagination-next" onClick={() => setPage((p) => p + 1)}>
        Next
      </a>
      <ul className="pagination-list">
        <li>
          <a
            className={"pagination-link" + (page === 1 ? " is-current" : "")}
            onClick={() => setPage(1)}
          >
            1
          </a>
        </li>
        {pages.map((p) => (
          <li key={p}>
            <a
              className={"pagination-link" + (p === page ? " is-current" : "")}
              onClick={() => setPage(p)}
            >
              {p}
            </a>
          </li>
        ))}
        <li>
          <a
            className={
              "pagination-link" + (page === pageCount ? " is-current" : "")
            }
            onClick={() => setPage(pageCount)}
          >
            {pageCount}
          </a>
        </li>
      </ul>
      <div className="select">
        <select
          value={pageSize}
          onChange={(event) => setPageSize(+event.target.value)}
        >
          {pageSizes.map((ps) => (
            <option key={ps}>{ps}</option>
          ))}
        </select>
      </div>
    </nav>
  );
}
