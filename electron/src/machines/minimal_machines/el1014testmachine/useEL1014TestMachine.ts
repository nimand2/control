import { toastError } from "@/components/Toast";
import { useStateOptimistic } from "@/lib/useStateOptimistic";
import { el1014TestMachineSerialRoute } from "@/routes/routes";
import { MachineIdentificationUnique } from "@/machines/types";
import {
  StateEvent,
  useEL1014TestMachineNamespace,
} from "./el1014TestMachineNamespace";
import { useEffect, useMemo } from "react";
import { el1014TestMachine } from "@/machines/properties";

export function useEL1014TestMachine() {
  const { serial: serialString } = el1014TestMachineSerialRoute.useParams();

  const machineIdentification: MachineIdentificationUnique = useMemo(() => {
    const serial = parseInt(serialString);

    if (isNaN(serial)) {
      toastError(
        "Invalid Serial Number",
        `"${serialString}" is not a valid serial number.`,
      );

      return {
        machine_identification: { vendor: 0, machine: 0 },
        serial: 0,
      };
    }

    return {
      machine_identification: el1014TestMachine.machine_identification,
      serial,
    };
  }, [serialString]);

  const { state } = useEL1014TestMachineNamespace(machineIdentification);

  const stateOptimistic = useStateOptimistic<StateEvent>();

  useEffect(() => {
    if (state) stateOptimistic.setReal(state);
  }, [state, stateOptimistic]);

  return {
    state: stateOptimistic.value,
  };
}
