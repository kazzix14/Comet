import { StyledEditor } from "./editor/editor.component";
import { useInput } from "./input.hook";

import React, { useEffect } from "react";
import styled from "styled-components";
import { emit, listen } from "@tauri-apps/api/event";
import { useAppDispatch, useAppSelector } from "./main";

const App = () => {
  const dispatch = useAppDispatch();
  const keySequence = useAppSelector((state) => state.input.keySequence);
  const command = useAppSelector((state) => state.input.command);

  useInput(dispatch, keySequence);
  //useEffect(() => {
  //  listen("player:play:feedback", (event) => {
  //    console.log(event);
  //  });
  //});

  return (
    <div>
      <div>{command}</div>
      <StyledEditor></StyledEditor>
    </div>
  );
};

export const StyledApp = styled(App)``;
