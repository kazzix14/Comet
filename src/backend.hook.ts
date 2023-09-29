import { listen } from "@tauri-apps/api/event";
import { Notification } from "./@types/backend/notification";
import { play, stop } from "./editor/editor.slice";
import { useEffect } from "react";
import { AppDispatch } from "./main";

export const useBackend = (dispatch: AppDispatch) => {
  useEffect(() => {
    const unlisten = listen<Notification>("backend:notification", (event) => {
      console.log("backend notification", event);
      // ここでnotificationの内容に応じてstateを更新する

      const payload = event.payload;

      if (payload.type !== "HealthCheck") {
        if (payload.type === "SequencerNotification") {
          switch (payload.content.type) {
            case "Play":
              dispatch(play());
              break;
            case "Stop":
              dispatch(stop());
              break;
          }
        }
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);
};
