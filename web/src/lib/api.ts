import { ApiPromise, WsProvider } from "@polkadot/api";
import { ContractPromise, Abi } from "@polkadot/api-contract";
import { web3Enable, web3Accounts } from "@polkadot/extension-dapp";
import { createTestKeyring } from "@polkadot/keyring/testing";

import ABI from "./abi.json";

export interface Api {
  client: ApiPromise;
  nft: ContractPromise;
}

const ContractAddress = "5FgZfTd5JkQqP3E9hGyAtFK4TadqwKCxDJryAtUSQLrnkzxq";

export const loadClient = async (): Promise<ApiPromise> => {
  const wsProvider = new WsProvider("ws://127.0.0.1:9944");
  return ApiPromise.create({ provider: wsProvider });
};

export const loadContract = (api: ApiPromise): ContractPromise => {
  const abi = new Abi(ABI);
  return new ContractPromise(api, abi, ContractAddress);
};

export { web3Enable };

export interface Account {
  address: string;
  meta: {
    name?: string;
  };
}

export const loadAccounts = async (): Promise<Account[]> => {
  if (process.env.NODE_ENV === "production") {
    return web3Accounts();
  } else {
    return createTestKeyring().getPairs();
  }
};
