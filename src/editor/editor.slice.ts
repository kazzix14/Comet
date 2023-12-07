import { createSlice } from "@reduxjs/toolkit";

interface editorState {
  focusedRow: number;
  isPlaying: boolean;
}

export const editorSlice = createSlice({
  name: "editor",
  initialState: {
    focusedRow: 0,
    isPlaying: false,
  } as editorState,
  reducers: {
    down: (state) => {
      state.focusedRow += 1;
    },
    up: (state) => {
      state.focusedRow -= 1;
    },
    play: (state) => {
      state.isPlaying = true;
    },
    stop: (state) => {
      state.isPlaying = false;
    },
  },
});

export const { down, up, play, stop } = editorSlice.actions;
