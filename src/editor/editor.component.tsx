import React from "react";
import styled from "styled-components";
import { useAppDispatch, useAppSelector } from "../main";

const Editor = () => {
  const isPlaying = useAppSelector((state) => state.editor.isPlaying);

  return (
    <div>
      <h1> Editor </h1>
      {isPlaying ? <div>Playing</div> : <div>Not playing</div>}
    </div>
  );
};

export const StyledEditor = styled(Editor)``;
