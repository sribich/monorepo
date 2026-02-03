import type { IncomingMessage } from "node:http"

import { scope, type Type } from "arktype"

export interface ServePluginConfig {
    host?: string
    port?: number
    proxy?: Record<
        string,
        {
            target: string
            bypass?: (req: IncomingMessage) => boolean
        }
    >
    entrypoint?: string
    reload?: boolean
    nodeArgs?: string[]
}
