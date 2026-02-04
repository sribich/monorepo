import type { Config } from "jest"

import { getJestConfig } from "../../jest.config.js"

const config: Config = getJestConfig("astro-plugin-docs")

export default config
