import { StyledEditor } from "./editor/editor.component";
import { useInput } from "./input.hook";

import React, { useEffect } from "react";
import styled from "styled-components";
import { listen } from "@tauri-apps/api/event";
import { useAppDispatch, useAppSelector } from "./main";
import { push } from "./backend.slice";
import { Notification } from "./@types/backend/notification";
import { display } from "./command";

const App = () => {
  const dispatch = useAppDispatch();
  const event = useAppSelector((state) => state.backend.eventQueue);
  const commandLookupTable = useAppSelector((state) => state.input.commandLookupTable);

  useInput(dispatch, commandLookupTable);
  useEffect(() => {
    listen<Notification>("backend:notification", (event) => {
      console.log("backend notification", event);
      dispatch(push(event.payload));
    });
  }, []);

  const notification = event.at(event.length - 1);
  let t = null;
  let c = null;
  if (notification !== undefined) {
    t = notification.type;
    if (notification.type !== "HealthCheck") {
      c = notification.content.type;
    }
  }

  return (
    <div>
      <div>{t}</div>
      <div>{c}</div>
      <ul>
        {display(commandLookupTable).map((rest, idx) => (
          <li key={idx}>{rest}</li>
        ))}
      </ul>
      <StyledEditor></StyledEditor>
    </div>
  );
};

export const StyledApp = styled(App)``;
