import {
    type IncomingMessage,
    type Server as HttpServer,
    type ServerResponse,
    createServer,
    request,
} from "node:http"
import { join, resolve } from "node:path"
import { stat } from "node:fs/promises"
import send from "send"

import { exists } from "../../../util/fs.js"
import { Server } from "../server.js"
import assert from "node:assert"

export class WebServer extends Server {
    private server: HttpServer | undefined

    async start(): Promise<void> {
        if (!this.server) {
            this.createServer()
        }

        if (!this.server?.listening) {
            this.server?.listen(this.config.port ?? 3000)
        }
    }

    private createServer() {
        const buildContext = this.context.build

        this.server = createServer(async (req, res) => {
            await this.getLock()

            // TODO: Remove this. Figure out how to handle shit.
            res.setHeader("Access-Control-Allow-Origin", "*")

            // PROXY
            const handled = false // this.tryProxy(req, res)

            assert(req.url)

            // We don't care about the actual hostname here, we just need
            // to parse out query parameters.
            const baseUrl = new URL(req.url, "http://localhost")
            const basePath = baseUrl.pathname

            const urlPath = basePath === "/" ? "index.html" : req.url
            const url = new URL(urlPath ?? "index.html", "http://localhost")
            const pathname = url.pathname
            const path = resolve(join(buildContext.outputDirectory, pathname))

            const proxiedRequest = this.maybeProxyRequest(pathname, req, res)

            if (proxiedRequest) {
                return
            }

            const fileExists = await exists(path)

            // If the file does not exist treat it as the entrypoint.
            if (!fileExists) {
                const url = new URL("index.html", "http://localhost")
                send(req, url.pathname, {
                    root: buildContext.outputDirectory,
                }).pipe(res)

                return
            }

            const stats = await stat(path)

            if (stats.isDirectory()) {
                res.statusCode = 404
                res.end()
                return
            }

            console.log(url, buildContext.outputDirectory)

            send(req, url.pathname, {
                root: buildContext.outputDirectory,
            }).pipe(res)
        })
    }

    private maybeProxyRequest(path: string, req: IncomingMessage, res: ServerResponse) {
        let protocol = "http:"

        let targetHost = this.config.host ?? "127.0.0.1"
        let targetPort = this.config.port ?? 3000

        const proxy = this.config.proxy ?? {}
        const proxyKeys = Object.keys(proxy)

        let proxied = false

        for (const proxyKey of proxyKeys) {
            if (path.startsWith(proxyKey)) {
                const bypass = proxy[proxyKey]?.bypass
                if (bypass && !bypass(req)) {
                    continue
                }

                const url = new URL(proxy[proxyKey]?.target ?? "")

                protocol = url.protocol ?? protocol
                targetHost = url.hostname
                targetPort = Number(url.port) ?? targetPort

                proxied = true

                break
            }
        }

        if (!proxied) {
            return false
        }

        const options = {
            protocol,
            hostname: targetHost,
            port: targetPort,
            path: req.url,
            method: req.method,
            headers: req.headers,
        }

        try {
            const proxyRequest = request(options, (proxyResponse): void => {
                if (proxyResponse.statusCode === 404) {
                    // If esbuild 404s the request, assume it's a route needing to
                    // be handled by the JS bundle, so forward a second attempt to `/`.
                    this.maybeProxyRequest("/", req, res)
                }

                res.writeHead(proxyResponse.statusCode || 200, proxyResponse.headers)

                proxyResponse.pipe(res, { end: true })
            })

            proxyRequest.on("error", (err) => {
                if (err && "code" in err && err.code === "ECONNREFUSED") {
                    console.warn("Proxied request failed -- server is still starting")
                }
            })

            req.pipe(proxyRequest, { end: true })
        } catch (e) {
            console.log(e)
        }

        return true
    }
}
