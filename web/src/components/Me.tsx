import React, { FC, useState, useEffect, useContext } from "react";
import { ApiContext } from "./ApiContext";

import TokenCard from "./TokenCard";
import Buy from "./Buy";

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
  }, [state.account, state.api, state.tokens_bought]);

  return (
    <div>
      <section>
        <Buy />
      </section>
      <section>
        <h2 className="text-lg mb-5">Your Tokens</h2>
        <ul className="grid gap-4 grid-cols-5">
          {tokens.map((token) => (
            <li key={token}>
              <TokenCard token={token} />
            </li>
          ))}
        </ul>
      </section>
    </div>
  );
};

export default Me;
