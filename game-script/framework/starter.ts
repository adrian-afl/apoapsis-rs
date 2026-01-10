import { spawn } from "node:child_process";

export interface ProcessStartResult {
  kill: () => void;
  stdout: string;
  stderr: string;
  writeStdin: (data: string) => Promise<void>;
  exitedWithCode: number | null;
}

async function startProcess(
  path: string,
  cwd: string,
  args: string[],
): Promise<ProcessStartResult> {
  const ls = spawn(path, args, { cwd });

  const result: ProcessStartResult = {
    kill: () => ls.kill(9),
    stdout: "",
    stderr: "",
    writeStdin: (data) =>
      new Promise<void>((resolve, reject) =>
        ls.stdin.write(data, (error) => (error ? reject(error) : resolve())),
      ),
    exitedWithCode: null,
  };

  ls.stdout.on("data", (data) => {
    console.log(data.toString("utf-8"));
    result.stdout += data.toString("utf-8");
  });

  ls.stderr.on("data", (data) => {
    console.log(data.toString("utf-8"));
    result.stderr += data.toString("utf-8");
  });

  ls.on("close", (code) => {
    result.exitedWithCode = code;
  });

  process.on("exit", () => {
    ls.kill(9);
  });

  return result;
}

export function startNATSServer(port: number): Promise<ProcessStartResult> {
  return startProcess("nats-server.exe", "../../nats-server", [
    "--port",
    port.toString(),
  ]);
}

export function startGame(
  mode: "release" | "debug",
  natsPort: number,
  headless: boolean,
): Promise<ProcessStartResult> {
  return startProcess(
    `target/${mode}/planetdraw-rs.exe`,
    `../../`,
    headless
      ? ["--headless", "--nats-address", `nats://localhost:${natsPort}`]
      : ["--nats-address", `nats://localhost:${natsPort}`],
  );
}
