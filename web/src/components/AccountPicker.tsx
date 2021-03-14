import React, { FC, useState, useEffect, useContext, useCallback } from "react";
import { ApiContext } from "./ApiContext";

import { loadAccounts } from "../lib/api";
import type { Account } from "../lib/api";

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

  console.log(accounts);

  const onChange = useCallback(
    (e) => {
      dispatch({ type: "set_account", account: accounts![e.target.value] });
    },
    [dispatch, accounts]
  );

  if (accounts == null) {
    return <div>Loading Accounts</div>;
  } else {
    return (
      <div>
        <select onChange={onChange}>
          {accounts.map((account, i) => (
            <option key={account.address} value={i}>
              {account.meta.name} - {account.address}
            </option>
          ))}
        </select>
      </div>
    );
  }
};

export default Header;
