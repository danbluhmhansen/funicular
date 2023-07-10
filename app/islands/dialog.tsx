import { JSX } from "preact";
import { Dialog, Transition } from "@headlessui/react";
import { Fragment } from "preact/compat";
import { Button } from "~components/button.tsx";
import { useSignal } from "@preact/signals";

interface ModalProps {
  title: JSX.Element | string;
  description: JSX.Element | string;
}

export default function Modal({ title, description }: ModalProps) {
  const show = useSignal(false);

  return (
    <>
      <Button onClick={() => show.value = true}>
        Open dialog
      </Button>

      <Transition appear show={show.value} as={Fragment}>
        <Dialog
          class="relative z-10"
          onClose={() => show.value = false}
        >
          <Transition.Child
            as={Fragment}
            enter="ease-out duration-300"
            enterFrom="opacity-0"
            enterTo="opacity-100"
            leave="ease-in duration-200"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
          >
            <div class="fixed inset-0 bg-black bg-opacity-25" />
          </Transition.Child>

          <div class="fixed inset-0 overflow-y-auto">
            <div class="flex min-h-full items-center justify-center p-4 text-center">
              <Transition.Child
                as={Fragment}
                enter="ease-out duration-300"
                enterFrom="opacity-0 scale-95"
                enterTo="opacity-100 scale-100"
                leave="ease-in duration-200"
                leaveFrom="opacity-100 scale-100"
                leaveTo="opacity-0 scale-95"
              >
                <Dialog.Panel class="
                  w-full
                  max-w-md
                  transform
                  overflow-hidden
                  rounded-2xl
                  bg-slate-900
                  p-6
                  text-left
                  align-middle
                  shadow-xl
                  transition-all">
                  <Dialog.Title
                    as="h3"
                    /* @ts-ignore */
                    class="text-lg font-medium leading-6 text-white"
                  >
                    {title}
                  </Dialog.Title>
                  <Dialog.Description class="mt-2 text-sm text-white">
                    {description}
                  </Dialog.Description>

                  <div class="mt-4">
                    <Button onClick={() => show.value = false}>
                      Ok
                    </Button>
                  </div>
                </Dialog.Panel>
              </Transition.Child>
            </div>
          </div>
        </Dialog>
      </Transition>
    </>
  );
}
