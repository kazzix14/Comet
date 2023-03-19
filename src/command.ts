import { Key } from "react";

export type Command = string;

export const COMMAND_LOOKUP_TABLE: CommandLookupTable = {
  Escape: "clear",
  h: "HealthCheck",
  " ": "command",
  a: {
    b: {
      c: "You've done",
    },
  },
  x: {
    d: "Yeah",
  },
};

export interface CommandLookupTable {
  [key: Key]: CommandLookupTable | Command | undefined;
}

export const lookup = (key: Key, commandLookupTable: CommandLookupTable): [Command | null, CommandLookupTable] => {
  const lookupResult = commandLookupTable[key];

  if (lookupResult === undefined) {
    return [null, COMMAND_LOOKUP_TABLE];
  } else if (typeof lookupResult === "string") {
    commandLookupTable = COMMAND_LOOKUP_TABLE;
    return [lookupResult, COMMAND_LOOKUP_TABLE];
  } else {
    return [null, lookupResult];
  }
};
