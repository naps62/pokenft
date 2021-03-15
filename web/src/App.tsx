import React from "react";
import { HashRouter as Router, Route, Switch } from "react-router-dom";
import { ToastProvider } from "react-toast-notifications";

import "./App.css";

import Header from "./components/Header";
import Me from "./components/Me";
import Buy from "./components/Buy";

import { ApiContextProvider } from "./components/ApiContext";

function App() {
  return (
    <div>
      <div className="container  mx-auto px-4 p-4">
        <ApiContextProvider>
          <ToastProvider
            placement="bottom-right"
            autoDismiss={true}
            autoDismissTimeout={2000}
          >
            <Router>
              <Header />
              <div className="mb-5" />
              <Switch>
                <Route path="/me" component={Me} />
                <Route path="/buy" component={Buy} />
                <Route path="/" component={Index} />
              </Switch>
            </Router>
          </ToastProvider>
        </ApiContextProvider>
      </div>
    </div>
  );
}

function Index() {
  return <div>Hello</div>;
}

export default App;
