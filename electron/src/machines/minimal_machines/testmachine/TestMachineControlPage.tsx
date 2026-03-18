import { ControlCard } from "@/control/ControlCard";
import { Page } from "@/components/Page";
import React from "react";
import { ControlGrid } from "@/control/ControlGrid";
import { SelectionGroup } from "@/control/SelectionGroup";
import { Label } from "@/control/Label";
import { Badge } from "@/components/ui/badge";
import { useTestMachine } from "./useTestMachine";

export function TestMachineControlPage() {
  const { state, setLed, setAllLeds } = useTestMachine();

  const safeState = state ?? {
    digital_output: Array(32).fill(false),
    digital_input: Array(16).fill(false),
  };

  return (
    <Page>
      <ControlGrid columns={2}>
        {[0, 1, 2, 3].map((terminalIndex) => (
          <ControlCard key={terminalIndex} title={`EL2008 ${terminalIndex + 1}`}>
            <div className="grid grid-cols-2 gap-4">
              {safeState.digital_output
                .slice(terminalIndex * 8, terminalIndex * 8 + 8)
                .map((output, localIndex) => {
                  const index = terminalIndex * 8 + localIndex;
                  return (
                    <Label key={index} label={`DO ${localIndex + 1}`}>
                      <SelectionGroup<"On" | "Off">
                        value={output ? "On" : "Off"}
                        orientation="vertical"
                        className="grid h-full grid-cols-2 gap-2"
                        options={{
                          Off: {
                            children: "Off",
                            icon: "lu:CirclePause",
                            isActiveClassName: "bg-red-600",
                            className: "h-full",
                          },
                          On: {
                            children: "On",
                            icon: "lu:CirclePlay",
                            isActiveClassName: "bg-green-600",
                            className: "h-full",
                          },
                        }}
                        onChange={(value) => setLed(index, value === "On")}
                      />
                    </Label>
                  );
                })}
            </div>
          </ControlCard>
        ))}

        {[0, 1].map((terminalIndex) => (
          <ControlCard key={`input-${terminalIndex}`} title={`EL1008 ${terminalIndex + 1}`}>
            <div className="grid grid-cols-2 gap-4">
              {safeState.digital_input
                .slice(terminalIndex * 8, terminalIndex * 8 + 8)
                .map((input, localIndex) => (
                  <Label
                    key={`${terminalIndex}-${localIndex}`}
                    label={`DI ${localIndex + 1}`}
                  >
                    <Badge variant={input ? "outline" : "destructive"}>
                      {input ? "On" : "Off"}
                    </Badge>
                  </Label>
                ))}
            </div>
          </ControlCard>
        ))}

        <ControlCard title="All Outputs">
          <SelectionGroup<"On" | "Off">
            value={safeState.digital_output.every(Boolean) ? "On" : "Off"}
            orientation="horizontal"
            options={{
              Off: { children: "Turn All Off" },
              On: { children: "Turn All On" },
            }}
            onChange={(value) => setAllLeds(value === "On")}
          />
        </ControlCard>

        <ControlCard title="Input Overview">
          <div className="grid grid-cols-2 gap-3">
            <Label label="Active Inputs">
              <Badge variant="outline">
                {safeState.digital_input.filter(Boolean).length} / {safeState.digital_input.length}
              </Badge>
            </Label>
            <Label label="Active Outputs">
              <Badge variant="outline">
                {safeState.digital_output.filter(Boolean).length} / {safeState.digital_output.length}
              </Badge>
            </Label>
          </div>
        </ControlCard>
      </ControlGrid>
    </Page>
  );
}
