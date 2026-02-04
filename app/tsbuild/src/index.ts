import type { Config } from "./lib/config/parser.js"

export { assetsPlugin } from "./lib/plugin/assets/assets.js"
export { declarationPlugin } from "./lib/plugin/declaration/declaration.js"
export { reactPlugin } from "./lib/plugin/react/plugin.js"
export { reactRefreshPlugin } from "./lib/plugin/react-refresh/plugin.js"
export { reactRouterPlugin } from "./lib/plugin/react-router/plugin.js"
export { servePlugin } from "./lib/plugin/serve/plugin.js"
export { stylexPlugin } from "./lib/plugin/stylex/plugin.js"
export { workerPlugin } from "./lib/plugin/worker/worker.plugin.js"
export { outputPlugin } from "./lib/plugin/output/output.plugin.js"

export const defineConfig = (config: Config): Config => {
    return config
}
