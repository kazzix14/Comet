import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { CommandLookupTable } from "./command";

export type Key = string;

interface inputState {
  currentInputs: Array<Key>;
  currentCommandLookupTable: CommandLookupTable;
}

export const inputSlice = createSlice({
  name: "input",
  initialState: {
    currentInputs: [],
    currentCommandLookupTable: {},
  } as inputState,
  reducers: {
    pushKey: (state, action: PayloadAction<Key>) => {
      state.currentInputs = state.currentInputs.concat(action.payload);
    },
    clearKeys: (state) => {
      state.currentInputs = [];
    },
    setCurrentCommandLookupTable: (state, action: PayloadAction<CommandLookupTable>) => {
      state.currentCommandLookupTable = action.payload;
    },
  },
});

export const { pushKey, clearKeys, setCurrentCommandLookupTable } = inputSlice.actions;
