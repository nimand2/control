import { StoreApi } from "zustand";
import { create } from "zustand";
import { z } from "zod";
import {
  EventHandler,
  eventSchema,
  Event,
  handleUnhandledEventError,
  NamespaceId,
  createNamespaceHookImplementation,
  ThrottledStoreUpdater,
} from "@/client/socketioStore";
import { MachineIdentificationUnique } from "@/machines/types";

export const stateEventDataSchema = z.object({
  inputs: z.array(z.boolean()).length(4),
});

export const stateEventSchema = eventSchema(stateEventDataSchema);
export type StateEvent = z.infer<typeof stateEventDataSchema>;

export type EL1014TestMachineNamespaceStore = {
  state: StateEvent | null;
};

export const createEL1014TestMachineNamespaceStore =
  (): StoreApi<EL1014TestMachineNamespaceStore> =>
    create<EL1014TestMachineNamespaceStore>(() => ({
      state: null,
    }));

export function el1014TestMachineMessageHandler(
  store: StoreApi<EL1014TestMachineNamespaceStore>,
  throttledUpdater: ThrottledStoreUpdater<EL1014TestMachineNamespaceStore>,
): EventHandler {
  return (event: Event<any>) => {
    const updateStore = (
      updater: (
        state: EL1014TestMachineNamespaceStore,
      ) => EL1014TestMachineNamespaceStore,
    ) => throttledUpdater.updateWith(updater);

    try {
      if (event.name === "StateEvent") {
        const parsed = stateEventSchema.parse(event);
        updateStore(() => ({ state: parsed.data }));
      } else {
        handleUnhandledEventError(event.name);
      }
    } catch (error) {
      console.error(`Error processing ${event.name}:`, error);
      throw error;
    }
  };
}

const useEL1014TestMachineNamespaceImplementation =
  createNamespaceHookImplementation<EL1014TestMachineNamespaceStore>({
    createStore: createEL1014TestMachineNamespaceStore,
    createEventHandler: el1014TestMachineMessageHandler,
  });

export function useEL1014TestMachineNamespace(
  machine_identification_unique: MachineIdentificationUnique,
): EL1014TestMachineNamespaceStore {
  const namespaceId: NamespaceId = {
    type: "machine",
    machine_identification_unique,
  };

  return useEL1014TestMachineNamespaceImplementation(namespaceId);
}
