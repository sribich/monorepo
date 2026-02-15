import "@total-typescript/ts-reset"

import { StrictMode } from "react"
import { createRoot } from "react-dom/client"

import { App } from "./App"

import "@sribich/fude/reset.css"
import "@sribich/fude-theme/style.css"
import "./main.css"

const element = document.getElementById("root")

if (element) {
    createRoot(element).render(
        <StrictMode>
            <App />
        </StrictMode>,
    )
}
