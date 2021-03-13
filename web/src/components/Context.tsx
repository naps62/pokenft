import React, { FC, createContext, useReducer } from "react";
import Reducer from "./reducer";

const initialState = {
  ctx: {
    api: null,
    nft: null,
  },
};

const Store: FC = ({ children }) => {
  const [state, dispatch] = useReducer(Reducer, initialState);

  return (
    <Context.Provider value={[state, dispatch]}>{children}</Context.Provider>
  );
};

export const Context = createContext(initialState);
export default Store;
