import React from "react";
import { Badge } from "@/components/ui/badge";
import { Page } from "@/components/Page";
import { ControlCard } from "@/control/ControlCard";
import { ControlGrid } from "@/control/ControlGrid";
import { Label } from "@/control/Label";
import { SelectionGroup } from "@/control/SelectionGroup";
import { useLedTestMachine } from "./useLedTestMachine";

const offOutputs = Array(8).fill(false);
const offInputs = Array(8).fill(false);

export function LedTestMachineControlPage() {
  const { state, liveValues, setOutput, setAllOutputs } = useLedTestMachine();
  const outputs = state?.outputs ?? offOutputs;
  const inputs = liveValues?.inputs ?? offInputs;

  return (
    <Page>
      <ControlGrid columns={2}>
        <ControlCard title="LED Outputs">
          <div className="grid grid-cols-2 gap-4">
            {outputs.map((on, index) => (
              <Label key={index} label={`EL2004 Output ${index + 1}`}>
                <SelectionGroup<"On" | "Off">
                  value={on ? "On" : "Off"}
                  options={{
                    Off: { children: "Off", icon: "lu:CirclePause" },
                    On: { children: "On", icon: "lu:CirclePlay" },
                  }}
                  onChange={(value) => setOutput(index, value === "On")}
                />
              </Label>
            ))}
          </div>
          <div className="mt-4">
            <SelectionGroup<"On" | "Off">
              value={
                outputs.every(Boolean)
                  ? "On"
                  : outputs.every((on) => !on)
                    ? "Off"
                    : undefined
              }
              options={{
                Off: { children: "All Off", icon: "lu:CirclePause" },
                On: { children: "All On", icon: "lu:CirclePlay" },
              }}
              onChange={(value) => setAllOutputs(value === "On")}
            />
          </div>
        </ControlCard>

        <ControlCard title="EL1014 Inputs">
          <div className="grid grid-cols-2 gap-4">
            {inputs.map((active, index) => (
              <Label key={index} label={`Input ${index + 1}`}>
                <div className="flex h-10 items-center justify-center">
                  <Badge className={active ? "bg-green-600" : "bg-gray-500"}>
                    {active ? "HIGH" : "LOW"}
                  </Badge>
                </div>
              </Label>
            ))}
          </div>
        </ControlCard>
      </ControlGrid>
    </Page>
  );
}
