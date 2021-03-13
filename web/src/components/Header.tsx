import React, { FC } from "react";
import { Link } from "react-router-dom";

const Header: FC = (props) => {
  return (
    <nav>
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
