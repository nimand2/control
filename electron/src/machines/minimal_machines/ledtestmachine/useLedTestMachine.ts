import { useEffect, useMemo } from "react";
import { produce } from "immer";
import { z } from "zod";
import { useMachineMutate } from "@/client/useClient";
import { toastError } from "@/components/Toast";
import { useStateOptimistic } from "@/lib/useStateOptimistic";
import { MachineIdentificationUnique } from "@/machines/types";
import { ledTestMachine } from "@/machines/properties";
import { ledTestMachineSerialRoute } from "@/routes/routes";
import {
  LedTestState,
  useLedTestMachineNamespace,
} from "./ledTestMachineNamespace";

export function useLedTestMachine() {
  const { serial: serialString } = ledTestMachineSerialRoute.useParams();
  const machineIdentification: MachineIdentificationUnique = useMemo(() => {
    const serial = parseInt(serialString, 10);
    if (Number.isNaN(serial)) {
      toastError("Invalid Serial Number", `"${serialString}" is not valid.`);
      return {
        machine_identification: { vendor: 0, machine: 0 },
        serial: 0,
      };
    }
    return {
      machine_identification: ledTestMachine.machine_identification,
      serial,
    };
  }, [serialString]);

  const { state, liveValues } = useLedTestMachineNamespace(
    machineIdentification,
  );
  const optimisticState = useStateOptimistic<LedTestState>();
  const { setReal } = optimisticState;
  const { request: sendMutation } = useMachineMutate(
    z.object({ action: z.string(), value: z.any() }),
  );

  useEffect(() => {
    if (state) setReal(state);
  }, [state, setReal]);

  const setOutput = (index: number, on: boolean) => {
    const current = optimisticState.value;
    if (current) {
      optimisticState.setOptimistic(
        produce(current, (draft) => {
          draft.outputs[index] = on;
        }),
      );
    }
    sendMutation({
      machine_identification_unique: machineIdentification,
      data: { action: "SetOutput", value: { index, on } },
    });
  };

  const setAllOutputs = (on: boolean) => {
    const current = optimisticState.value;
    if (current) {
      optimisticState.setOptimistic(
        produce(current, (draft) => {
          draft.outputs = Array(8).fill(on);
        }),
      );
    }
    sendMutation({
      machine_identification_unique: machineIdentification,
      data: { action: "SetAllOutputs", value: { on } },
    });
  };

  return { state: optimisticState.value, liveValues, setOutput, setAllOutputs };
}
