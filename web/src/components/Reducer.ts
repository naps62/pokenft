import { Reducer as IReducer } from "react";

import { ApiContext } from "../lib/api";

type State = {
  ctx: ApiContext | null;
};

const Reducer: IReducer<State, any> = (state, action) => {
  switch (action.type) {
    case "SET_CONTEXT":
      return {
        ...state,
        ctx: action.payload,
      };
    default:
      return state;
  }
};

export default Reducer;
