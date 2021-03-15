import React, { FC, useState, useEffect, useContext } from "react";
import { ApiContext } from "./ApiContext";

interface Props {
  token: string;
}

const TokenCard: FC<Props> = ({ token }) => {
  const [state] = useContext(ApiContext);
  const [id, setId] = useState<number | undefined>();
  const [poke, setPoke] = useState<any | undefined>();

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

  useEffect(() => {
    if (!id) {
      return;
    }

    fetch(`https://pokeapi.co/api/v2/pokemon/${id}`)
      .then((resp) => resp.json())
      .then((data) => setPoke(data));
  }, [id]);

  if (!!poke) {
    return (
      <div>
        <div>
          #{id} - {poke.name}
        </div>
        <div>{token}</div>
        <img src={poke.sprites.front_default} alt={poke.name} />
      </div>
    );
  } else {
    return <div>Loading {id}</div>;
  }
};

export default TokenCard;
