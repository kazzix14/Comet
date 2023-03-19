import { StyledEditor } from "./editor/editor.component";
import { useInput } from "./input.hook";

import React, { useEffect } from "react";
import styled from "styled-components";
import { listen } from "@tauri-apps/api/event";
import { useAppDispatch, useAppSelector } from "./main";
import { push } from "./backend.slice";

const App = () => {
  const dispatch = useAppDispatch();
  const event = useAppSelector((state) => state.backend.eventQueue);
  const commandLookupTable = useAppSelector((state) => state.input.commandLookupTable);

  useInput(dispatch, commandLookupTable);
  useEffect(() => {
    listen("backend:notification", (event) => {
      console.log("backend notification", event);
      dispatch(push(event));
    });
  }, []);

  console.log(event);

  const t = event[event.length - 1]?.payload?.type;

  return (
    <div>
      <div>{t}</div>
      <StyledEditor></StyledEditor>
    </div>
  );
};

export const StyledApp = styled(App)``;
