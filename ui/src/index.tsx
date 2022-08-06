import React from "react";
import ReactDOM from "react-dom/client";

import init from "sorting-visualiser";
import App from "./App";

init()
  .then((wasmInit) => {
    const root = ReactDOM.createRoot(
      document.getElementById("root") as HTMLElement
    );
    root.render(<App wasm={wasmInit} />);
  })
  .catch((err) => {
    console.error(err);
  });
