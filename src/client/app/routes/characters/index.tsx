import type { ErrorBoundaryComponent, LoaderFunction } from "@remix-run/node";
import { useLoaderData, useSearchParams } from "@remix-run/react";
import { fetchGraphQl } from "~/lib/graphql";
import type Character from "~/models/character";
import type { GraphQlSort } from "~/models/graphql/graphql-sort";
import type { GraphQlResponse } from "~/models/graphql/graphql-response";
import type { PaginationState } from "@tanstack/react-table";
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { useEffect, useMemo, useState } from "react";

export const ErrorBoundary: ErrorBoundaryComponent = ({ error }) => {
  return (
    <article className="message is-danger">
      <div className="message-header">
        <p>{error.name}</p>
      </div>
      <div className="message-body">
        <p>{error.message}</p>
        <p>{error.stack && error.stack}</p>
      </div>
    </article>
  );
};

export const loader: LoaderFunction = async ({ request }) => {
  const url = new URL(request.url);
  const pageIndex = +(url.searchParams.get("pageIndex") ?? "0");
  const pageSize = +(url.searchParams.get("pageSize") ?? "10");

  return await fetchGraphQl({
    query: `
      query CharactersQuery($skip: Int, $top: Int, $orderby: [Orderby]) {
        characters(count: true, skip: $skip, top: $top, orderby: $orderby) {
          id
          name
          strength
          dexterity
          constitution
          intelligence
          wisdom
          charisma
        }
      }`,
    variables: {
      skip: pageIndex * pageSize,
      top: pageSize,
      orderby: [{ field: "name" }],
    },
  });
};

const columnHelper = createColumnHelper<Character>();

const columns = [
  columnHelper.accessor((row) => row.name, {
    id: "name",
    cell: (info) => info.getValue(),
    header: "Name",
  }),
  columnHelper.accessor((row) => row.strength, {
    id: "strength",
    cell: (info) => info.getValue(),
    header: "Strength",
  }),
  columnHelper.accessor((row) => row.dexterity, {
    id: "dexterity",
    cell: (info) => info.getValue(),
    header: "Dexterity",
  }),
  columnHelper.accessor((row) => row.constitution, {
    id: "constitution",
    cell: (info) => info.getValue(),
    header: "Constitution",
  }),
  columnHelper.accessor((row) => row.intelligence, {
    id: "intelligence",
    cell: (info) => info.getValue(),
    header: "Intelligence",
  }),
  columnHelper.accessor((row) => row.wisdom, {
    id: "wisdom",
    cell: (info) => info.getValue(),
    header: "Wisdom",
  }),
  columnHelper.accessor((row) => row.charisma, {
    id: "charisma",
    cell: (info) => info.getValue(),
    header: "Charisma",
  }),
];

export default function Index() {
  const { data, extensions } = useLoaderData<GraphQlResponse>();
  const [params, setParams] = useSearchParams();

  const [{ pageIndex, pageSize }, setPagination] = useState<PaginationState>({
    pageIndex: +(params.get("page") ?? "0"),
    pageSize: +(params.get("pageSize") ?? "10"),
  });
  const pagination = useMemo(
    () => ({ pageIndex, pageSize }),
    [pageIndex, pageSize]
  );

  const table = useReactTable({
    data: data.characters,
    columns: columns,
    getCoreRowModel: getCoreRowModel(),
    pageCount: extensions.count ? Math.ceil(extensions.count / pageSize) : 1,
    state: {
      pagination,
    },
    onPaginationChange: setPagination,
    manualPagination: true,
  });

  useEffect(() => {
    params.set("pageIndex", "" + pageIndex);
    params.set("pageSize", "" + pageSize);
    setParams(params);
  }, [pageIndex, pageSize]);

  const orderby = params
    .getAll("orderby")
    .map((o) => o.split(" "))
    .map(([field, desc]) => ({
      field: field.toLowerCase(),
      desc: desc !== undefined && desc.toLowerCase() === "desc",
    }))
    .filter(({ field }, i, s) => s.findIndex((o) => o.field === field) === i);

  function setOrderby(sort: GraphQlSort) {
    params.delete("orderby");
    const existing = orderby.findIndex((o) => o.field === sort.field);
    if (existing !== -1) orderby.splice(existing, 1);
    orderby.push(sort);
    orderby.forEach((o) =>
      params.append("orderby", o.field + (o.desc ? " desc" : ""))
    );
    setParams(params);
  }

  return (
    <div className="container">
      <h3 className="title">Characters</h3>
      <table className="table">
        <thead>
          {table.getHeaderGroups().map((row) => (
            <tr key={row.id}>
              {row.headers.map((header) => (
                <th key={header.id}>
                  {header.isPlaceholder
                    ? null
                    : flexRender(
                        header.column.columnDef.header,
                        header.getContext()
                      )}
                </th>
              ))}
            </tr>
          ))}
        </thead>
        <tfoot>
          <tr>
            <th colSpan={table.getAllColumns().length}>
              <nav className="pagination">
                <a
                  className={
                    "pagination-previous" +
                    (table.getCanPreviousPage() ? "" : " is-disabled")
                  }
                  onClick={() => table.previousPage()}
                >
                  Prev
                </a>
                <a
                  className={
                    "pagination-next" +
                    (table.getCanNextPage() ? "" : " is-disabled")
                  }
                  onClick={() => table.nextPage()}
                >
                  Next
                </a>
              </nav>
            </th>
          </tr>
        </tfoot>
        <tbody>
          {table.getRowModel().rows.map((row) => (
            <tr key={row.id}>
              {row.getVisibleCells().map((cell) => (
                <td key={cell.id}>
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
