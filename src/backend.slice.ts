import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export const backendSlice = createSlice({
  name: "backend",
  initialState: {
    eventQueue: Array<any>(),
  },
  reducers: {
    push: (state, action: PayloadAction<any>) => {
      state.eventQueue = [...state.eventQueue, action.payload];
    },
  },
});

export const { push } = backendSlice.actions;
