import { ControlCard } from "@/control/ControlCard";
import { Page } from "@/components/Page";
import { Label } from "@/control/Label";
import { Badge } from "@/components/ui/badge";
import React from "react";
import { useEL1014TestMachine } from "./useEL1014TestMachine";

export function EL1014TestMachineControlPage() {
  const { state } = useEL1014TestMachine();

  const safeState = state ?? {
    inputs: [false, false, false, false],
  };

  return (
    <Page>
      <ControlCard title="EL1014 Digital Inputs">
        <div className="grid grid-cols-2 gap-4">
          {safeState.inputs.map((input, index) => (
            <Label key={index} label={`Input ${index + 1}`}>
              <Badge variant={input ? "outline" : "destructive"}>
                {input ? "ON" : "OFF"}
              </Badge>
            </Label>
          ))}
        </div>
      </ControlCard>
    </Page>
  );
}
