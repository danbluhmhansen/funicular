import { Signal, useSignal } from "@preact/signals";
import { Game } from "~api-models";
import { Button } from "~components/button.tsx";
import { Audit, auditAdd } from "~models/audit.ts";
import Dialog from "~islands/dialog.tsx";

interface GameGridProps {
  audits: Signal<Audit<Game>[]>;
}

export default function GameGrid({ audits }: GameGridProps) {
  const show = useSignal(false);
  const newName = useSignal("");

  return (
    <>
      <div class="flex flex-row items-center justify-between space-x-4 p-4">
        <Dialog show={show}>
          <h3 class="text-lg font-medium leading-6 text-white">
            Title
          </h3>
          <div>
            <input
              type="text"
              name="name"
              onChange={(n) => newName.value = n.currentTarget.value}
              class="
              bg(gray-50 dark:gray-700)
              border(& gray-300 dark:gray-600 focus:primary-600 dark:focus:primary-500)
              text(sm gray-900 dark:white)
              ring(focus:primary-600 dark:focus:primary-500)
              rounded-lg
              block
              w-full
              p-2.5
              dark:placeholder-gray-400
              "
            />
          </div>
          <p class="mt-2 text-sm text-white">Description</p>
          <div class="mt-4">
            <Button
              onClick={() => {
                audits.value = [
                  auditAdd({ id: crypto.randomUUID(), name: newName.value }),
                  ...audits.value,
                ];
                show.value = false;
              }}
            >
              Add
            </Button>
          </div>
        </Dialog>
      </div>
      {audits.value.length > 0
        ? (
          <table class="table-auto border-collapse mx-auto">
            <thead>
              <tr class="px-4 py-2">
                <th>Name</th>
                <th>Created</th>
                <th>Description</th>
                <th>State</th>
              </tr>
            </thead>
            <tbody>
              {audits.value.map((a) => (
                <tr key={a.post?.id} class="px-4 py-2">
                  <td>
                    <a href={`games/${a.post?.name}`} class="hover:underline">
                      {a.post?.name}
                    </a>
                  </td>
                  <td>
                    {a.post?.created &&
                      new Date(a.post?.created).toLocaleString()}
                  </td>
                  <td>{a.post?.description}</td>
                  <td>{a.state}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )
        : <p>No games...</p>}
    </>
  );
}
