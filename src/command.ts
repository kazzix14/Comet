import { Key } from "react";
import { Command } from "./@types/backend/command";

export const COMMAND_LOOKUP_TABLE: CommandLookupTable = {
  Escape: { type: "HealthCheck" },
  h: { type: "HealthCheck" },
  " ": { type: "SequencerCommand", content: { type: "HealthCheck" } },
  a: {
    b: {
      c: { type: "SequencerCommand", content: { type: "Play" } },
    },
  },
  x: {
    d: { type: "ControllerCommand", content: { type: "HealthCheck" } },
  },
};

export interface CommandLookupTable {
  [key: Key]: CommandLookupTable | Command | undefined;
}

export const lookup = (key: Key, commandLookupTable: CommandLookupTable): [Command | null, CommandLookupTable] => {
  const lookupResult = commandLookupTable[key];

  if (lookupResult === undefined) {
    return [null, COMMAND_LOOKUP_TABLE];
  } else if (isCommand(lookupResult)) {
    return [lookupResult, COMMAND_LOOKUP_TABLE];
  } else {
    return [null, lookupResult];
  }
};

export const display = (commandLookupTable: CommandLookupTable, current = ""): Array<string> => {
  return Object.keys(commandLookupTable).flatMap((key) => {
    const result = commandLookupTable[key];

    const appended = `${current}${key}`;

    if (result === undefined || isCommand(result)) {
      return appended;
    }

    return display(result, appended);
  });
};

const isCommand = (maybeCommand: Command | CommandLookupTable): maybeCommand is Command => {
  if (maybeCommand.type !== undefined) {
    return true;
  } else {
    return false;
  }
};
