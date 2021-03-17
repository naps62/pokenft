import React from "react";
import { ToastProvider } from "react-toast-notifications";

import "./App.css";

import Header from "./components/Header";
import Me from "./components/Me";

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
            <Header />
            <div className="mb-5" />
            <Me />
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
