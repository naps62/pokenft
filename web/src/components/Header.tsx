import React, { FC } from "react";
import { Link } from "react-router-dom";

import AccountPicker from "./AccountPicker";

const Header: FC = () => {
  return (
    <nav className="flex">
      <div className="flex-grow mr-10">
        <AccountPicker />
      </div>
      <div className="flex-grow-0">
        <ul className="flex flex-grow content-center">
          <li className="p-2">
            <Link to="/" className="underline text-red-500">
              Home
            </Link>
          </li>
          <li className="p-2">
            <Link to="/buy" className="underline text-red-500">
              Buy
            </Link>
          </li>
          <li className="p-2">
            <Link to="/me" className="underline text-red-500">
              Me
            </Link>
          </li>
        </ul>
      </div>
    </nav>
  );
};

export default Header;
