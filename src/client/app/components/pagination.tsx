import { useSearchParams } from "@remix-run/react";
import { useEffect } from "react";

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
  const page = +(params.get("page") ?? "1");
  const pageSize = +(params.get("pageSize") ?? "10");

  const pageCount = count ? Math.ceil(count / pageSize) : 1;
  const pages = GetCurrentPages(
    Array.from(Array(pageCount + 1).keys()).slice(2, -1),
    page
  );

  useEffect(() => {
    if (page > pageCount) setPage(pageCount);
  }, [pageCount]);

  function setPage(page: number) {
    params.set("page", page + "");
    setParams(params);
  }

  function setPageSize(pageSize: number) {
    params.set("pageSize", pageSize + "");
    setParams(params);
  }

  return (
    <nav className="pagination">
      <a
        className={"pagination-previous" + (page === 1 ? " is-disabled" : "")}
        onClick={() => setPage(page - 1 < 1 ? 1 : page - 1)}
      >
        Prev
      </a>
      <a
        className={
          "pagination-next" + (page === pageCount ? " is-disabled" : "")
        }
        onClick={() => setPage(page + 1 > pageCount ? pageCount : page + 1)}
      >
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
