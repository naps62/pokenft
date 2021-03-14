import React, { FC, useState, useEffect, useContext } from "react";
import { ApiContext } from "./ApiContext";

interface Props {
  token: string;
}

const TokenCard: FC<Props> = ({ token }) => {
  const [state] = useContext(ApiContext);
  const [id, setId] = useState<number | undefined>();

  useEffect(() => {
    state
      .api!.nft.query.pokemonOf(
        state.account!.address,
        { value: 0, gasLimit: 0 },
        token
      )
      .then(({ output }: any) => {
        setId(output.toNumber());
      });
  }, [state.api, state.account, token]);

  return (
    <div>
      <div>{id}</div>
      <div>{token}</div>
    </div>
  );
};

export default TokenCard;
