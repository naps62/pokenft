import React, { FC, useState, useEffect, useContext, useCallback } from "react";
import { ApiContext } from "./ApiContext";
import Select from "react-select";
import { useToasts } from "react-toast-notifications";

import { loadAccounts } from "../lib/api";
import type { Account } from "../lib/api";

const toOptions = (accounts: Account[]) => {
  return accounts.map((account) => {
    const { address, meta } = account;
    return { value: account, label: `${meta.name} - ${address}` };
  });
};

const Header: FC = () => {
  const [accounts, setAccounts] = useState<null | Account[]>(null);
  const [state, dispatch] = useContext(ApiContext);
  const { addToast } = useToasts();

  useEffect(() => {
    if (!state.account) {
      return;
    }

    addToast(`Using account ${state.account.meta.name}`, {
      appearance: "success",
    });
  }, [addToast, state.account]);

  useEffect(() => {
    if (!state.web3_enabled) {
      return;
    }

    loadAccounts().then((accounts) => {
      setAccounts(accounts);
      dispatch({ type: "set_account", account: accounts[0] });
    });
  }, [state.web3_enabled, dispatch]);

  const onChange = useCallback(
    (option) => {
      dispatch({ type: "set_account", account: option.value });
    },
    [dispatch]
  );

  if (accounts == null) {
    return <div>Loading Accounts</div>;
  } else {
    return (
      <div>
        <Select
          onChange={onChange}
          options={toOptions(accounts)}
          placeholder="Choose your account"
        />
      </div>
    );
  }
};

export default Header;
