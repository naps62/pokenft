import React from "react";
import { HashRouter as Router, Route, Switch } from "react-router-dom";
import "./App.css";

import Header from "./components/Header";
import Me from "./components/Me";

import { ApiContextProvider } from "./components/ApiContext";

function App() {
  return (
    <ApiContextProvider>
      <Router>
        <Header />
        <Switch>
          <Route path="/me" component={Me} />
          <Route path="/" component={Index} />
        </Switch>
      </Router>
    </ApiContextProvider>
  );
}

function Index() {
  return <div>Hello</div>;
}

export default App;
