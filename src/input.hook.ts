import { AppDispatch } from "./main";
import { updateCommandLookupTable } from "./input.slice";

import { useEffect } from "react";
import { emit } from "@tauri-apps/api/event";
import { lookup, CommandLookupTable } from "./command";

export const useInput = (dispatch: AppDispatch, currentCommandLookupTable: CommandLookupTable) => {
  useEffect(() => {
    const keyDownListener = async (event: KeyboardEvent) => {
      const key = event.key;

      const [command, nextCommandLookupTable] = lookup(key, currentCommandLookupTable);

      if (command != null) {
        emit("command", command);
      }

      dispatch(updateCommandLookupTable(nextCommandLookupTable));
    };

    window.addEventListener("keydown", keyDownListener);
    return () => {
      window.removeEventListener("keydown", keyDownListener);
    };
  }, [currentCommandLookupTable]);
};
