import React, { FC, useState, useCallback, useContext } from "react";
import { ApiContext } from "./ApiContext";
import { useToasts } from "react-toast-notifications";

import { signAndSend } from "../lib/api";

const Buy: FC = () => {
  const [seed, setSeed] = useState<string>("");
  const [state, dispatch] = useContext(ApiContext);
  const { addToast } = useToasts();

  const onSubmit = useCallback(
    (e) => {
      e.preventDefault();
      const value = 0; // only useful on isPayable messages
      const gasLimit = 60000 * 1000000;

      const tx = state.api!.nft.tx.mint({ value, gasLimit }, seed);

      addToast("[Buy] Sending transaction", { appearance: "info" });
      signAndSend(tx, state.account!, (r: any) => {
        dispatch({ type: "token_bought" });
        if (r.status.isInBlock) {
          addToast("[Buy] in a Block", { appearance: "success" });
        } else if (r.status.isFinalized) {
          addToast("[Buy] Finalized", { appearance: "success" });
        }
      });
    },
    [seed, state.account, state.api]
  );

  const onInputChange = useCallback((e) => {
    e.preventDefault();
    setSeed(e.target.value);
  }, []);

  return (
    <form onSubmit={onSubmit}>
      <h2 className="text-lg mb-5">Buy a Pokemon</h2>
      <div className="flex flex-col">
        <input
          type="text"
          onChange={onInputChange}
          placeholder="Seed"
          className="rounded-t-md border border-solid border-red-500 p-2"
        />
        <br />
        <input
          type="submit"
          value="Buy"
          className="rounded-b-md border border-solid border-red-500 bg-red-500 border-t-0 p-2 text-white hover:bg-red-700 cursor-pointer"
        />
      </div>
    </form>
  );
};

export default Buy;
