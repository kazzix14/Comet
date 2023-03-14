import { PayloadAction, createSlice } from "@reduxjs/toolkit";

type Key = string;

export const appSlice = createSlice({
  name: "input",
  initialState: {
    keyBuffer: [] as Array<Key>,
  },
  reducers: {
    pushKey: (state, action: PayloadAction<Key>) => {
      state.keyBuffer.push(action.payload);
    },
    flushKey: (state) => {
      state.keyBuffer = [];
    },
  },
});

export const { pushKey, flushKey } = appSlice.actions;
