import React, { createContext, useReducer, useEffect } from "react";
import type { Reducer as IReducer, Dispatch, FC } from "react";

import { loadClient, loadContract, web3Enable } from "../lib/api";
import type { Api, Account } from "../lib/api";

interface State {
  api?: Api;
  account?: Account;
  web3_enabled: boolean;
}

type Action =
  | { type: "set_api"; api: Api }
  | { type: "web3_enabled" }
  | { type: "set_account"; account: Account };

const Reducer: IReducer<State, Action> = (state, action) => {
  switch (action.type) {
    case "set_api":
      return { ...state, api: action.api };
    case "web3_enabled":
      return { ...state, web3_enabled: true };
    case "set_account":
      return { ...state, account: action.account };
    default:
      return state;
  }
};

const initialState = { web3_enabled: false };

export const ApiContext = createContext<[State, Dispatch<Action>]>([
  initialState,
  () => {},
]);

export const ApiContextProvider: FC = ({ children }) => {
  const [state, dispatch] = useReducer(Reducer, initialState);

  useEffect(() => {
    loadClient().then((client) => {
      const nft = loadContract(client);

      dispatch({ type: "set_api", api: { client, nft } });
    });
  }, []);

  useEffect(() => {
    setTimeout(() => {
      web3Enable("pokenft").then(() => dispatch({ type: "web3_enabled" }));
    }, 1000);
  }, [state.api]);

  if (!!state.api && state.web3_enabled) {
    return (
      <ApiContext.Provider value={[state, dispatch]}>
        {children}
      </ApiContext.Provider>
    );
  } else {
    return <div>Loading</div>;
  }
};
