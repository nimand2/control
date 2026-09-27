import React from "react";
import { render, screen } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { LedTestMachineControlPage } from "@/machines/minimal_machines/ledtestmachine/LedTestMachineControlPage";

vi.mock("@/components/Icon", () => ({ Icon: () => null }));
vi.mock("@/machines/minimal_machines/ledtestmachine/useLedTestMachine", () => ({
  useLedTestMachine: () => ({
    state: { outputs: [true, false, false, false, false, false, false, false] },
    liveValues: { inputs: Array(8).fill(false) },
    setOutput: vi.fn(),
    setAllOutputs: vi.fn(),
  }),
}));

it("does not show All Off or All On as active for mixed outputs", () => {
  render(<LedTestMachineControlPage />);
  expect(screen.getByRole("button", { name: "All Off" })).not.toHaveClass(
    "bg-primary",
  );
  expect(screen.getByRole("button", { name: "All On" })).not.toHaveClass(
    "bg-primary",
  );
});
