import { Command as BackendCommand } from "./@types/backend/command";

type Key = string;

type Command = "HealthCheck" | "Play" | "Toggle" | "Pause";

const toBackendCommand = (command: Command, condition: CommandLookupCondition): BackendCommand => {
  switch (command) {
    case "HealthCheck":
      return { type: "HealthCheck" };
    case "Play":
      return { type: "ControllerCommand", content: { type: "Play" } };
    case "Pause":
      return { type: "ControllerCommand", content: { type: "Pause" } };
    case "Toggle":
      if (condition.isPlaying) {
        return { type: "ControllerCommand", content: { type: "Pause" } };
      } else {
        return { type: "ControllerCommand", content: { type: "Play" } };
      }
    default:
      throw new Error("Unknown command");
  }
};

export const COMMAND_LOOKUP_TABLE: CommandLookupTable = {
  Escape: "HealthCheck",
  " ": "Toggle",
  s: {
    s: "Play",
    p: "Pause",
  },
};

export interface CommandLookupTable {
  [key: Key]: CommandLookupTable | Command | undefined;
}

export interface CommandLookupCondition {
  isPlaying: boolean;
}

export const commandLookup = (keys: Array<Key>, condition: CommandLookupCondition): CommandLookupResult => {
  return recursiveCommandLookup(keys, condition, COMMAND_LOOKUP_TABLE);
};

export interface Identified {
  type: "Identified";
  command: BackendCommand | null;
}
export const isIdentified = (maybeIdentified: CommandLookupResult): maybeIdentified is Identified => {
  if (maybeIdentified.type === "Identified") {
    return true;
  } else {
    return false;
  }
};

export interface Unidentified {
  type: "Unidentified";
  rest: CommandLookupTable;
}

type CommandLookupResult = Identified | Unidentified;

const recursiveCommandLookup = (
  keys: Array<Key>,
  condition: CommandLookupCondition,
  commandLookupTable: CommandLookupTable
): CommandLookupResult => {
  const key = keys.shift();

  if (key === undefined) {
    return { type: "Unidentified", rest: commandLookupTable };
  }

  const lookupResult = commandLookupTable[key];
  console.log(lookupResult);

  if (lookupResult === undefined) {
    return { type: "Identified", command: null };
  } else if (typeof lookupResult === "string") {
    return { type: "Identified", command: toBackendCommand(lookupResult, condition) };
  } else {
    return recursiveCommandLookup(keys, condition, lookupResult);
  }
};

export const displayCandidateKeys = (commandLookupTable: CommandLookupTable, current = ""): Array<string> => {
  return Object.keys(commandLookupTable).flatMap((key) => {
    const result = commandLookupTable[key];

    const appended = `${current}${key}`;

    if (result === undefined || typeof result === "string") {
      return appended;
    }

    return displayCandidateKeys(result, appended);
  });
};

// const isCommandFunction = (maybeCommand: Command | CommandLookupTable): maybeCommand is Command => {
//   if (maybeCommand.type !== "undefined") {
//     return true;
//   } else {
//     return false;
//   }
// };
