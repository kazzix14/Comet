import { style } from "@vanilla-extract/css";

export const editorStyle = style({
});

export const SheetStyle = style({
  display: "flex",
  flexDirection: "column",
  gap: "0.25rem",
});

export const RowStyle = style({
  display: "flex",
  flexDirection: "row",
  gap: "0.25rem",
});

export const CellStyle = style({
  padding: "0 0.4rem",
  border: "1px solid #fff",
});
