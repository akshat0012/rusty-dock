import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import store from './store'
import { Provider } from 'react-redux'

import QuickWindowMainComponent from "./src-window/quick_window"
import { HashRouter, Routes, Route } from "react-router-dom";

ReactDOM.createRoot(document.getElementById("root")).render(
    <Provider store={ store }>
        <HashRouter>
        <Routes>
            <Route path="/" element={<App />} />
            <Route path="/quick-window" element={<QuickWindowMainComponent />} />
        </Routes>
        </HashRouter>
    </Provider>
);
