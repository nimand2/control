import React from "react";
import { Topbar } from "@/components/Topbar";
import { ledTestMachineSerialRoute } from "@/routes/routes";

export function LedTestMachinePage() {
  const { serial } = ledTestMachineSerialRoute.useParams();
  return (
    <Topbar
      pathname={`/_sidebar/machines/ledtestmachine/${serial}`}
      items={[
        {
          link: "control",
          activeLink: "control",
          title: "Control",
          icon: "lu:ToggleLeft",
        },
      ]}
    />
  );
}
