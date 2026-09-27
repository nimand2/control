import { act, renderHook } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { useLedTestMachine } from "@/machines/minimal_machines/ledtestmachine/useLedTestMachine";

const backend = vi.hoisted(() => ({
  state: { outputs: Array<boolean>(8).fill(false) },
  liveValues: { inputs: Array<boolean>(8).fill(false) },
  request: vi.fn(),
}));

vi.mock("@/client/useClient", () => ({
  useMachineMutate: () => ({ request: backend.request }),
}));
vi.mock("@/components/Toast", () => ({ toastError: vi.fn() }));
vi.mock("@/machines/properties", () => ({
  ledTestMachine: { machine_identification: { vendor: 1, machine: 99 } },
}));
vi.mock("@/routes/routes", () => ({
  ledTestMachineSerialRoute: { useParams: () => ({ serial: "42" }) },
}));
vi.mock(
  "@/machines/minimal_machines/ledtestmachine/ledTestMachineNamespace",
  () => ({
    useLedTestMachineNamespace: () => backend,
  }),
);

describe("LED test machine controls", () => {
  beforeEach(() => {
    backend.state = { outputs: Array<boolean>(8).fill(false) };
    backend.request.mockClear();
  });

  it("keeps a single output optimistic until a new server state arrives", () => {
    const { result, rerender } = renderHook(() => useLedTestMachine());
    act(() => result.current.setOutput(4, true));
    expect(result.current.state?.outputs[4]).toBe(true);
    expect(backend.request).toHaveBeenCalledWith({
      machine_identification_unique: {
        machine_identification: { vendor: 1, machine: 99 },
        serial: 42,
      },
      data: { action: "SetOutput", value: { index: 4, on: true } },
    });
    backend.liveValues = { inputs: Array<boolean>(8).fill(true) };
    rerender();
    expect(result.current.state?.outputs[4]).toBe(true);
    backend.state = { outputs: Array<boolean>(8).fill(false) };
    rerender();
    expect(result.current.state?.outputs[4]).toBe(false);
  });

  it("keeps all outputs optimistic while awaiting the server", () => {
    const { result } = renderHook(() => useLedTestMachine());
    act(() => result.current.setAllOutputs(true));
    expect(result.current.state?.outputs).toEqual(Array(8).fill(true));
  });
});
