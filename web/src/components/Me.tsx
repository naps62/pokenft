import React, { FC, useState, useEffect } from "react";
import { ApiContext } from "../lib/api";

interface Props {
  ctx: ApiContext;
}

const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

const Me: FC<Props> = (props) => {
  const [tokens, setTokens] = useState<any[]>([]);

  console.log(props);
  useEffect(() => {
    (props.ctx.nft as any)
      .tokens_of(ALICE, { value: 0, gasLimit: 0 }, ALICE)
      .then(setTokens);
  }, []);

  return (
    <ul>
      {tokens.map((token) => (
        <li>{token}</li>
      ))}
    </ul>
  );
};

export default Me;
