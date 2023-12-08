import React from "react";
import PropTypes from "prop-types";
import { useAppSelector } from "../main";
import { CellStyle, RowStyle, SheetStyle } from "./editor.css";

export const Editor = () => {
  const isPlaying = useAppSelector((state) => state.editor.isPlaying);

  return (
    <div>
      <h1> Editor </h1>
      <Sheet />
      {isPlaying ? <div>Playing</div> : <div>Not playing</div>}
    </div>
  );
};

export const Sheet = () => {
  return (
    <div className={SheetStyle}>
      {[...Array(10)].map((_, idx) => (
        <Row key={idx} columnCount={10} />
      ))}
    </div>
  );
};

export const Row = ({ columnCount }: { columnCount: number }) => {
  return <div className={RowStyle}>
    {[...Array(columnCount)].map((_, idx) => <Cell key={idx} />)}
  </div>;
};

Row.propTypes = {
  columnCount: PropTypes.number.isRequired,
};

export const Cell = () => {
  return <div className={CellStyle}>Cell</div>;
};
