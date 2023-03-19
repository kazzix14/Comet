import { Key } from "react";

export type Command = "player:play" | "You've done" | "Yeah" | "clear";

export const COMMAND_LOOKUP_TABLE: CommandLookupTable = {
  Escape: "clear",
  " ": "player:play",
  a: {
    b: {
      c: "You've done",
    },
  },
  x: {
    d: "Yeah",
  },
};

interface CommandLookupTable {
  [key: string]: CommandLookupTable | Command | undefined;
}

export const lookup = (
  keySequence: Array<Key>,
  lookupTable = COMMAND_LOOKUP_TABLE,
  head = 0
): Command | null => {
  const lookupResult = lookupTable[keySequence[head]];

  if (lookupResult === undefined) {
    return null;
  } else if (typeof lookupResult === "string") {
    return lookupResult;
  }

  return lookup(keySequence, lookupResult, head + 1);
};
