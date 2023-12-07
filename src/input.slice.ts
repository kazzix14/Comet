import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export type Key = string;

interface inputState {
  currentInputs: Array<Key>;
}

export const inputSlice = createSlice({
  name: "input",
  initialState: {
    currentInputs: [],
  } as inputState,
  reducers: {
    pushKey: (state, action: PayloadAction<Key>) => {
      state.currentInputs = state.currentInputs.concat(action.payload);
    },
    clearKeys: (state) => {
      state.currentInputs = [];
    },
  },
});

export const { pushKey, clearKeys } = inputSlice.actions;
