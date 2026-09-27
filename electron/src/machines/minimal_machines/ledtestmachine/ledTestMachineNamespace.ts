import { useMemo } from "react";
import { create } from "zustand";
import { StoreApi } from "zustand";
import { z } from "zod";
import {
  createNamespaceHookImplementation,
  Event,
  EventHandler,
  eventSchema,
  handleUnhandledEventError,
  NamespaceId,
  ThrottledStoreUpdater,
} from "@/client/socketioStore";
import { MachineIdentificationUnique } from "@/machines/types";

const stateDataSchema = z.object({ outputs: z.array(z.boolean()).length(8) });
const liveValuesDataSchema = z.object({ inputs: z.array(z.boolean()).length(8) });
const stateSchema = eventSchema(stateDataSchema);
const liveValuesSchema = eventSchema(liveValuesDataSchema);

export type LedTestState = z.infer<typeof stateDataSchema>;
export type LedTestLiveValues = z.infer<typeof liveValuesDataSchema>;

type LedTestNamespaceStore = {
  state: LedTestState | null;
  liveValues: LedTestLiveValues | null;
};

const createLedTestStore = (): StoreApi<LedTestNamespaceStore> =>
  create<LedTestNamespaceStore>(() => ({ state: null, liveValues: null }));

const createLedTestMessageHandler = (
  store: StoreApi<LedTestNamespaceStore>,
  throttledUpdater: ThrottledStoreUpdater<LedTestNamespaceStore>,
): EventHandler => {
  void store;
  return (event: Event<any>) => {
    if (event.name === "StateEvent") {
      const parsed = stateSchema.parse(event);
      throttledUpdater.updateWith((current) => ({ ...current, state: parsed.data }));
    } else if (event.name === "LiveValuesEvent") {
      const parsed = liveValuesSchema.parse(event);
      throttledUpdater.updateWith((current) => ({ ...current, liveValues: parsed.data }));
    } else {
      handleUnhandledEventError(event.name);
    }
  };
};

const useLedTestNamespaceImplementation =
  createNamespaceHookImplementation<LedTestNamespaceStore>({
    createStore: createLedTestStore,
    createEventHandler: createLedTestMessageHandler,
  });

export function useLedTestMachineNamespace(
  machine_identification_unique: MachineIdentificationUnique,
): LedTestNamespaceStore {
  const namespaceId = useMemo<NamespaceId>(
    () => ({ type: "machine", machine_identification_unique }),
    [machine_identification_unique],
  );

  return useLedTestNamespaceImplementation(namespaceId);
}