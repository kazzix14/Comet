import { AppDispatch } from "./main";
import { pushKey, clear, command, Key } from "./input.slice";
import { lookup } from "./command";

import { useEffect } from "react";
import { emit } from "@tauri-apps/api/event";

export const useInput = (dispatch: AppDispatch, keySequence: Array<Key>) => {
  useEffect(() => {
    const keyDownListener = async (event: KeyboardEvent) => {
      console.log("keyDownListener", event.key);
      dispatch(pushKey(event.key));
    };

    window.addEventListener("keydown", keyDownListener);

    if (0 < keySequence.length) {
      const lookupResult = lookup(keySequence);

      if (lookupResult !== null) {
        dispatch(command(lookupResult));
        emit(lookupResult);
      }

      dispatch(clear());
    }

    return () => {
      window.removeEventListener("keydown", keyDownListener);
    };
  }, [keySequence]);
};
