import { StyledEditor } from "./editor/editor.component";
import { useInput } from "./input.hook";

import React, { useEffect } from "react";
import styled from "styled-components";
import { emit, listen } from "@tauri-apps/api/event";
import { useAppDispatch, useAppSelector } from "./main";
import { push } from "./backend.slice";

const App = () => {
  const dispatch = useAppDispatch();
  const keySequence = useAppSelector((state) => state.input.keySequence);
  const command = useAppSelector((state) => state.input.command);
  const event = useAppSelector((state) => state.backend.eventQueue);

  useInput(dispatch, keySequence);
  useEffect(() => {
    listen("player:play:feedback", (event) => {
      dispatch(push(event));
    });
  }, []);

  return (
    <div>
      <div>{command}</div>
      <div>{event.keys()}</div>
      <StyledEditor></StyledEditor>
    </div>
  );
};

export const StyledApp = styled(App)``;
