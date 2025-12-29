import { execSync } from "node:child_process";
import * as path from "node:path";

export interface ApiExportedCommand {
  name: string;
  inputType: string;
  returnType: string;
  importRs: string;
}

export interface ApiExportedEvent {
  name: string;
  payloadType: string;
  importRs: string;
}

export interface ReadApiExportsResult {
  commands: ApiExportedCommand[];
  events: ApiExportedEvent[];
}

function pathToImportRS(inputPath: string): string {
  // example: use crate::remote_api::api::serialize_world::serialize_world;
  // for which path is ./packages/core/src/remote_api/api/deserialize_world.rs

  // another example from external crate would be
  // use nats::{NATS_CONNECTION, OutgoingRemoteIOMessage, send_event};
  // ./packages/nats/src/lib.rs

  const parts = inputPath.split("/");
  const crate = parts[2] === "core" ? "crate" : parts[2]; // special for things inside core
  const isLibRs = inputPath.endsWith("lib.rs");

  console.log(inputPath);

  return (isLibRs ? inputPath.replace("/lib.rs", "") : inputPath)
    .replace("/src", "")
    .replace("/packages", "")
    .replaceAll("/", "::")
    .replace(".rs", "")
    .replace(new RegExp(`.*${parts[2]}`), `use ${crate}`);
}

export function readApiExports(): ReadApiExportsResult {
  const grepResult = execSync(
    "find . -type d | grep 'src$' | xargs grep -rE \"api_(command|event)\"",
    {
      shell: "sh",
      cwd: path.join("..", ".."),
    },
  )
    .toString("utf-8")
    .split("\n");

  const commandRegex =
    /(.*?):\/\/ @api_command ([A-z0-9_]+?)\(([A-z0-9_]+?)\):[ ]*([A-z0-9_]+?)$/;

  const eventRegex = /(.*?):\/\/ @api_event ([A-z0-9_]+?)\(([A-z0-9_]+?)\)$/;

  const result: ReadApiExportsResult = { commands: [], events: [] };

  for (const line of grepResult) {
    const commandMatch = line.match(commandRegex);
    const eventMatch = line.match(eventRegex);
    if (commandMatch) {
      result.commands.push({
        importRs: `${pathToImportRS(commandMatch[1])}::${commandMatch[2]};`,
        name: commandMatch[2],
        inputType: commandMatch[3],
        returnType: commandMatch[4],
      });
    } else if (eventMatch) {
      result.events.push({
        importRs: `${pathToImportRS(eventMatch[1])}::${eventMatch[2]};`,
        name: eventMatch[2],
        payloadType: eventMatch[3],
      });
    }
  }

  // console.log(result);

  return result;
}

// readApiExports();
