import { ApiPromise, WsProvider } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
import ABI from "./abi.json";

export interface ApiContext {
  api: ApiPromise;
  nft: ContractPromise;
}

const ContractAddress = "5G5GkJA88wXHX2iDhgVhtifsAnUqgsaYQeE8X9pHhEyoJeBz";

const load = async (): Promise<ApiPromise> => {
  const wsProvider = new WsProvider("ws://127.0.0.1:9944");
  return ApiPromise.create({ provider: wsProvider });
};

const loadContract = (api: ApiPromise): ContractPromise =>
  new ContractPromise(api, ABI, ContractAddress);

export { load, loadContract };
