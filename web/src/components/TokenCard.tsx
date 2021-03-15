import React, { FC, useState, useEffect, useContext, useCallback } from "react";
import { ApiContext } from "./ApiContext";

interface Props {
  token: string;
}

const TypeColors: Record<string, string> = {
  normal: "#A8A77A",
  fire: "#EE8130",
  water: "#6390F0",
  electric: "#F7D02C",
  grass: "#7AC74C",
  ice: "#96D9D6",
  fighting: "#C22E28",
  poison: "#A33EA1",
  ground: "#E2BF65",
  flying: "#A98FF3",
  psychic: "#F95587",
  bug: "#A6B91A",
  rock: "#B6A136",
  ghost: "#735797",
  dragon: "#6F35FC",
  dark: "#705746",
  steel: "#B7B7CE",
  fairy: "#D685AD",
};

const typeColor = (poke: any): string => {
  return TypeColors[(poke.types[0] as any).type.name] as string;
};

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

  const cry = useCallback(() => {
    new Audio(`https://pokemoncries.com/cries-old/${id}.mp3`).play();
  }, [id]);

  if (!!poke) {
    return (
      <div
        className="rounded-lg  p-2 hover:bg-red-200 transition cursor-pointer flex flex-col content-center"
        onClick={cry}
        style={{ backgroundColor: typeColor(poke) }}
      >
        <div>
          #{id} - {poke.name}
        </div>
        <img src={poke.sprites.front_default} alt={poke.name} />
      </div>
    );
  } else {
    return <div>Loading {id}</div>;
  }
};

export default TokenCard;
