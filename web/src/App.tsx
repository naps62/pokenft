import React from "react";
import { HashRouter as Router, Route, Switch } from "react-router-dom";
import "./App.css";

import Header from "./components/Header";
import EnsureApi from "./components/EnsureApi";
import Me from "./components/Me";

function App() {
  return (
    <EnsureApi>
      <Router>
        <Header />
        <Switch>
          <Route path="/me" component={Me} />
          <Route path="/" component={Index} />
        </Switch>
      </Router>
    </EnsureApi>
  );
}

function Index() {
  return <div>Hello</div>;
}

export default App;
