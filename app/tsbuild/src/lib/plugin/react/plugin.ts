import { RunnerMode } from "../../runner.js"
import type { Plugin } from "../plugin.js"

export const reactPlugin = (): Plugin => {
    return async (context) => {
        return {
            modifyConfig(config) {
                config.esbuild = {
                    ...config.esbuild,
                    jsx: "automatic",
                    jsxImportSource: "react",
                    jsxDev: context.build.mode === "dev",
                }

                config.rolldown ??= {}
                config.rolldown.input ??= {}

                /*
                config.rolldown.input.jsx ??= {}

                if (typeof config.rolldown.input.jsx === "string") {
                    throw new Error()
                }

                config.rolldown.input.jsx = {
                    ...config.rolldown.input.jsx,
                    mode: "automatic",
                }
                */

                config.rolldown.input.transform ??= {}
                config.rolldown.input.transform.jsx ??= {}

                if (typeof config.rolldown.input.transform.jsx === "string") {
                    throw new Error()
                }

                config.rolldown.input.transform.jsx = {
                    runtime: "automatic",
                    development: context.build.mode === RunnerMode.DEV,
                }
            },
        }
    }
}
