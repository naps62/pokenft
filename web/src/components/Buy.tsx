import React, { FC, useState, useCallback, useContext } from "react";
import { ApiContext } from "./ApiContext";

import { signAndSend } from "../lib/api";

const Buy: FC = () => {
  const [seed, setSeed] = useState<string>("");
  const [state] = useContext(ApiContext);

  console.log(state.account);
  const onSubmit = useCallback(() => {
    const value = 0; // only useful on isPayable messages
    const gasLimit = 20000 * 1000000;

    const tx = state.api!.nft.tx.mint({ value, gasLimit }, seed);

    signAndSend(tx, state.account!, (r: any) => {
      if (r.status.isInBlock) {
        console.log("in a block");
      } else if (r.status.isFinalized) {
        console.log("finalized");
      }
    });
  }, [seed, state.account, state.api]);

  const onInputChange = useCallback((e) => {
    e.preventDefault();
    setSeed(e.target.value);
  }, []);

  return (
    <form onSubmit={onSubmit}>
      <input type="text" onChange={onInputChange} placeholder="Seed" />
      <input type="submit" value="Buy" />
    </form>
  );
};

export default Buy;
