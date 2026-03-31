import { Topbar } from "@/components/Topbar";
import { el1014TestMachineSerialRoute } from "@/routes/routes";
import React from "react";

export function EL1014TestMachinePage() {
  const { serial } = el1014TestMachineSerialRoute.useParams();
  return (
    <Topbar
      pathname={`/_sidebar/machines/el1014testmachine/${serial}`}
      items={[
        {
          link: "control",
          activeLink: "control",
          title: "Control",
          icon: "lu:CirclePlay",
        },
      ]}
    />
  );
}
