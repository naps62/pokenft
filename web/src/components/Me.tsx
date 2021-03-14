import React, { FC, useState, useEffect, useContext } from "react";
import { ApiContext } from "./ApiContext";

const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

const Me: FC = (props) => {
  const [tokens, setTokens] = useState<any[]>([]);
  const [state] = useContext(ApiContext);

  useEffect(() => {
    if (!state.account) {
      return;
    }

    const { address } = state.account!;

    state
      .api!.nft.query.tokensOf(address, { value: 0, gasLimit: 0 }, address)
      .then(({ output }: any) => {
        setTokens(output);
      });
  }, [state.account, state.api]);

  return (
    <section>
      <h1>Tokens</h1>
      <ul>
        {tokens.map((token) => (
          <li key={token}>{token}</li>
        ))}
      </ul>
    </section>
  );
};

export default Me;
