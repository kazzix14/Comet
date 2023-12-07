import { Key } from "react";
import { Command } from "./@types/backend/command";
import { RootState } from "./main";

export const COMMAND_LOOKUP_TABLE: CommandLookupTable = {
  Escape: { type: "HealthCheck" },
  h: { type: "HealthCheck" },
  " ": (state: RootState): Command => {
    if (state.editor.isPlaying) {
      return { type: "ControllerCommand", content: { type: "Pause" } };
    } else {
      return { type: "ControllerCommand", content: { type: "Play" } };
    }
  },
  s: {
    p: { type: "ControllerCommand", content: { type: "Play" } },
    s: { type: "ControllerCommand", content: { type: "Pause" } },
  },
  x: {
    d: { type: "ControllerCommand", content: { type: "HealthCheck" } },
  },
};

export interface CommandLookupTable {
  [key: Key]: CommandLookupTable | AbleToBeCommand | undefined;
}

type CommandFunction = (state: RootState) => Command;
type AbleToBeCommand = Command | CommandFunction;

export const commandLookup = (keys: Array<Key>, state: RootState): Command | null => {
  return recursiveCommandLookup(keys, state, COMMAND_LOOKUP_TABLE);
};

const recursiveCommandLookup = (keys: Array<Key>, state: RootState, commandLookupTable: CommandLookupTable): Command | null => {
  const key = keys.shift();

  if (key === undefined) {
    return null;
  }

  const lookupResult = commandLookupTable[key];

  if (lookupResult === undefined) {
    return null;
  } else if (isCommandFunction(lookupResult)) {
    return lookupResult(state);
  } else if (isCommand(lookupResult)) {
    return lookupResult;
  } else {
    return recursiveCommandLookup(keys, state, lookupResult);
  }
};

export const displayCandidateKeys = (commandLookupTable: CommandLookupTable, current = ""): Array<string> => {
  return Object.keys(commandLookupTable).flatMap((key) => {
    const result = commandLookupTable[key];

    const appended = `${current}${key}`;

    if (result === undefined || isCommand(result) || isCommandFunction(result)) {
      return appended;
    }

    return displayCandidateKeys(result, appended);
  });
};

const isCommandFunction = (maybeCommand: AbleToBeCommand | CommandLookupTable): maybeCommand is CommandFunction => {
  if (typeof maybeCommand === "function") {
    return true;
  } else {
    return false;
  }
};

const isCommand = (maybeCommand: AbleToBeCommand | CommandLookupTable): maybeCommand is Command => {
  if (typeof maybeCommand !== "function" && maybeCommand.type !== undefined) {
    return true;
  } else {
    return false;
  }
};
