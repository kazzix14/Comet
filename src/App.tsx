import { useEffect } from "react";

import styled from "styled-components";
import React from "react";
import { useAppDispatch, useAppSelector } from "./main";
import { down, up } from "./editor/editor.slice";
import { pushKey, flushKey } from "./app.slice";
import { StyledEditor } from "./editor/editor.component";

const useKeys = (): void => {
  const dispatch = useAppDispatch();

  const keyDownListener = async (event: KeyboardEvent) => {
    dispatch(pushKey(event.key));
  };

  useEffect(() => {
    window.addEventListener("keydown", keyDownListener);

    return () => {
      window.removeEventListener("keydown", keyDownListener);
    };
  }, []);
};

type Command = "You've done" | "Yeah" | "clear";

const COMMAND_LOOKUP_TABLE: CommandLookupTable = {
  Escape: "clear",
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

const App = () => {
  useKeys();

  const keyBuffer = useAppSelector((state) => state.app.keyBuffer);

  const focusedRow = useAppSelector((state) => state.editor.focusedRow);
  const dispatch = useAppDispatch();

  useEffect(() => {
    let current_command_lookup_table = COMMAND_LOOKUP_TABLE;
    for (const key of keyBuffer) {
      const lookup = current_command_lookup_table[key];
      if (typeof lookup === "string") {
        dispatch(flushKey());
        break;
      } else if (lookup === undefined) {
        dispatch(flushKey());
        break;
      }
      current_command_lookup_table = lookup;
    }
  });

  return (
    <div>
      <div>counter: {keyBuffer}</div>
      <div>counter: {focusedRow}</div>
      <StyledEditor></StyledEditor>
    </div>
  );
};

export const StyledApp = styled(App)``;
