import React, { FC } from "react";

import AccountPicker from "./AccountPicker";

const Header: FC = () => {
  return (
    <nav className="flex">
      <div className="flex-grow">
        <AccountPicker />
      </div>
    </nav>
  );
};

export default Header;
