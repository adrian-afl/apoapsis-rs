import { spawn } from "node:child_process";
import { InspectColorForeground, styleText } from "node:util";

export interface ProcessStartResult {
  kill: () => void;
  stdout: string;
  stderr: string;
  writeStdin: (data: string) => Promise<void>;
  exitedWithCode: Promise<number>;
}

async function startProcess(
  color: InspectColorForeground,
  path: string,
  cwd: string,
  args: string[],
): Promise<ProcessStartResult> {
  const ls = spawn(path, args, { cwd, env: { RUST_BACKTRACE: "1" } });

  const result: ProcessStartResult = {
    kill: () => ls.kill(9),
    stdout: "",
    stderr: "",
    writeStdin: (data) =>
      new Promise<void>((resolve, reject) =>
        ls.stdin.write(data, (error) => (error ? reject(error) : resolve())),
      ),
    exitedWithCode: new Promise<number>((resolve) => {
      ls.on("close", (code) => {
        console.log(styleText(color, `Exited with code: ${code}`));
        resolve(code);
      });
    }),
  };

  ls.stdout.on("data", (data) => {
    console.log(styleText(color, data.toString("utf-8")));
    result.stdout += data.toString("utf-8");
  });

  ls.stderr.on("data", (data) => {
    console.log(styleText(color, data.toString("utf-8")));
    result.stderr += data.toString("utf-8");
  });

  process.on("exit", () => {
    ls.kill(9);
  });

  return result;
}

export function startGame(
  mode: "release" | "debug",
  port: number,
  headless: boolean,
): Promise<ProcessStartResult> {
  return startProcess(
    "green",
    `target/${mode}/planetdraw-rs.exe`,
    `../`,
    headless ? ["--headless", "--port", `${port}`] : ["--port", `${port}`],
  );
}
