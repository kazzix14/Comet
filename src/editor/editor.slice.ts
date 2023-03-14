import { createSlice } from "@reduxjs/toolkit";

export const editorSlice = createSlice({
  name: "editor",
  initialState: {
    focusedRow: 0,
  },
  reducers: {
    down: (state) => {
      state.focusedRow += 1;
    },
    up: (state) => {
      state.focusedRow -= 1;
    },
  },
});

export const { down, up } = editorSlice.actions;
