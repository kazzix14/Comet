import { COMMAND_LOOKUP_TABLE, CommandLookupTable } from "./command";

import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export type Key = string;

export const inputSlice = createSlice({
  name: "input",
  initialState: {
    commandLookupTable: COMMAND_LOOKUP_TABLE,
  },
  reducers: {
    updateCommandLookupTable: (state, action: PayloadAction<CommandLookupTable>) => {
      state.commandLookupTable = action.payload;
    },
  },
});

export const { updateCommandLookupTable } = inputSlice.actions;
