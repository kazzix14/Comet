import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { Notification } from "./@types/backend/notification";

export const backendSlice = createSlice({
  name: "backend",
  initialState: {
    eventQueue: Array<Notification>(),
  },
  reducers: {
    push: (state, action: PayloadAction<Notification>) => {
      state.eventQueue = [...state.eventQueue, action.payload];
    },
  },
});

export const { push } = backendSlice.actions;
