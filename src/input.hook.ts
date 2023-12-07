import { AppDispatch, useAppSelector } from "./main";
import { pushKey, clearKeys } from "./input.slice";

import { useEffect } from "react";
import { emit } from "@tauri-apps/api/event";
import { commandLookup } from "./command";

export const useInput = (dispatch: AppDispatch) => {
  const currentInputs = useAppSelector((state) => state.input.currentInputs);
  const isPlaying = useAppSelector((state) => state.editor.isPlaying);

  useEffect(() => {
    const keyDownListener = async (event: KeyboardEvent) => {
      const key = event.key;

      const command = commandLookup(currentInputs.concat(key), {
        isPlaying: isPlaying,
      });

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
  }, [currentInputs, isPlaying]);
};
