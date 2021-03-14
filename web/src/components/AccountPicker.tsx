import React, { FC, useState, useEffect, useContext, useCallback } from "react";
import { ApiContext } from "./ApiContext";
import Select from "react-select";

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

  console.log(1);
  if (accounts == null) {
    return <div>Loading Accounts</div>;
  } else {
    return (
      <div>
        <Select onChange={onChange} options={toOptions(accounts)} />
      </div>
    );
  }
};

export default Header;
