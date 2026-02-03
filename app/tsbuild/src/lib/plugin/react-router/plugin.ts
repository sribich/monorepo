import { Generator, getConfig } from "@tanstack/router-generator"

import type { Plugin } from "../plugin.js"

export const reactRouterPlugin = (): Plugin => {
    return async (context) => {
        const repository = context.repository
        const project = await repository.getCurrentProject()

        if (!project.dependencies.includes("@tanstack/react-router")) {
            return null
        }

        const routerConfig = getConfig({
            target: "react",
            generatedRouteTree: "./src/generated/routeTree.ts",
        })

        const generator = new Generator({
            config: routerConfig,
            root: context.build.rootDirectory,
        })

        return {
            onBuild: async () => {
                await generator.run()
            },
        }
    }
}
