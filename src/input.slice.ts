import { Command } from "./command";

import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export type Key = string;

export const inputSlice = createSlice({
  name: "input",
  initialState: {
    keySequence: Array<Key>(),
    command: null as Command | null,
  },
  reducers: {
    pushKey: (state, action: PayloadAction<Key>) => {
      state.keySequence = [...state.keySequence, action.payload];
    },
    clear: (state) => {
      state.keySequence = [];
    },
    command: (state, action: PayloadAction<Command>) => {
      state.command = action.payload;
    },
  },
});

export const { pushKey, clear, command } = inputSlice.actions;
