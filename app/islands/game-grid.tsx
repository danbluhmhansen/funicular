import { Signal } from "@preact/signals";
import { Game } from "~api-models";
import { Button } from "~components/button.tsx";
import { Audit, auditAdd } from "~models/audit.ts";
import Dialog from "~islands/dialog.tsx";
import { Dialog as Modal } from "@headlessui/react";

interface GameGridProps {
  audits: Signal<Audit<Game>[]>;
}

export default function GameGrid({ audits }: GameGridProps) {
  return (
    <>
      <div class="flex flex-row items-center justify-between space-x-4 p-4">
        <Button
          onClick={() =>
            audits.value = [
              auditAdd({ id: crypto.randomUUID(), name: "" }),
              ...audits.value,
            ]}
        >
          Add
        </Button>
        <Dialog>
          <Modal.Title
            as="h3"
            /* @ts-ignore */
            class="text-lg font-medium leading-6 text-white"
          >
            Title
          </Modal.Title>
          <Modal.Description class="mt-2 text-sm text-white">
            Description
          </Modal.Description>

          <div class="mt-4">
            <Button>
              Ok
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
