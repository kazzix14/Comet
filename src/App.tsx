import { Editor } from "./editor/editor.component";
import { useInput } from "./input.hook";
import { useAppDispatch, useAppSelector } from "./main";
import { useBackend } from "./backend.hook";
import { COMMAND_LOOKUP_TABLE, displayCandidateKeys } from "./command";

import React from "react";

export const App = () => {
  const dispatch = useAppDispatch();

  useInput(dispatch);
  useBackend(dispatch);

  const currentInputs = useAppSelector((state) => state.input.currentInputs);

  return (
    <div>
      <div>hello</div>
      <div>
        <h2>Possible Keys</h2>
        <ul>
          {currentInputs}
          {displayCandidateKeys(COMMAND_LOOKUP_TABLE).map((rest, idx) => (
            <li key={idx}>{rest}</li>
          ))}
        </ul>
      </div>
      <Editor />
    </div>
  );
};
