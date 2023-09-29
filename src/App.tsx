import { StyledEditor } from "./editor/editor.component";
import { useInput } from "./input.hook";

import React from "react";
import styled from "styled-components";
import { useAppDispatch, useAppSelector } from "./main";
import { useBackend } from "./backend.hook";
import { displayCandidateKeys } from "./command";

const App = () => {
  const dispatch = useAppDispatch();
  const commandLookupTable = useAppSelector((state) => state.input.commandLookupTable);

  useInput(dispatch, commandLookupTable);
  useBackend(dispatch);

  return (
    <div>
      <div>hello</div>
      <div>
        <h2>
          Possible Keys
        </h2>
        <ul>
          {displayCandidateKeys(commandLookupTable).map((rest, idx) => (
            <li key={idx}>{rest}</li>
          ))}
        </ul>
      </div>
      <StyledEditor></StyledEditor>
    </div>
  );
};

export const StyledApp = styled(App)``;
