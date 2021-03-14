import React, { FC } from "react";
import { Link } from "react-router-dom";

import AccountPicker from "./AccountPicker";

const Header: FC = () => {
  return (
    <nav>
      <AccountPicker />
      <ul>
        <li>
          <Link to="/">Home</Link>
        </li>
        <li>
          <Link to="/me">Me</Link>
        </li>
      </ul>
    </nav>
  );
};

export default Header;
