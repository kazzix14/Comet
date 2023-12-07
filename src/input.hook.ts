import { AppDispatch, RootState } from "./main";
import { pushKey, clearKeys } from "./input.slice";

import { useEffect } from "react";
import { emit } from "@tauri-apps/api/event";
import { commandLookup } from "./command";

export const useInput = (dispatch: AppDispatch, state: RootState) => {
  useEffect(() => {
    const keyDownListener = async (event: KeyboardEvent) => {
      const key = event.key;

      const command = commandLookup(state.input.currentInputs.concat(key), state);

      if (command != null) {
        emit("command", command);
        dispatch(clearKeys());
      } else {
        dispatch(pushKey(key));
      }
    };

    window.addEventListener("keydown", keyDownListener);
    return () => {
      window.removeEventListener("keydown", keyDownListener);
    };
  }, [state.input.currentInputs, state.editor.isPlaying]);
};
