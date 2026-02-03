import chokidar, { type FSWatcher } from "chokidar"
import { cwd } from "node:process"

import type { Config } from "../config/parser.js"
import picomatch, { type Matcher } from "picomatch"
import { Context } from "./context.js"
import type { BuildContext } from "./build.js"

type OnChange = (path: string) => void

export class WatchContext extends Context {
    private onChangeHandlers: OnChange[] = []
    private watcher: FSWatcher

    private excludePatterns: Matcher[] = []

    private buildContext: BuildContext

    constructor(config: Config, buildContext: BuildContext) {
        super()

        this.buildContext = buildContext

        this.excludePatterns = (config.watch?.excludeGlobs || []).map((glob) => picomatch(glob))

        this.watcher = chokidar.watch([cwd(), ...(config.watch?.additionalGlobs ?? [])], {
            persistent: true,
            ignoreInitial: true,
            ignorePermissionErrors: true,
            followSymlinks: true,
            alwaysStat: true,
            ignored: [`${cwd()}/dist`, "**/node_modules/*"],
        })

        this.watcher.on("change", (path) => {
            console.log(path)

            // console.log(path)
            this.invoke(path)
        })
    }

    public onChange(handler: OnChange): void {
        this.onChangeHandlers.push(handler)
    }

    override async initialise(): Promise<void> {}

    override async terminate(): Promise<void> {
        this.watcher.close()
    }

    public addWatchPath(path: string): void {
        this.watcher.add(path)
    }

    public addIgnorePath(path: string): void {
        this.watcher.unwatch(path)
    }

    private invoke = (() => {
        let timer: NodeJS.Timeout

        return (path: string) => {
            clearTimeout(timer)

            timer = setTimeout(() => {
                let shortPath = path.replace(this.buildContext.rootDirectory, "")

                if (shortPath.startsWith("/")) {
                    shortPath = shortPath.slice(1)
                }

                for (const pattern of this.excludePatterns) {
                    if (pattern(path) || pattern(shortPath)) {
                        return
                    }
                }

                for (const handler of this.onChangeHandlers) {
                    // TODO: This should be a promise that we only invoke once until
                    //       the promises are all resolved from handlers
                    handler(path)
                }
            }, 250)
        }
    })()
}

/*
private async queueBuild(): Promise<void> {
        if (this.building) {
            this.buildOnComplete = true
            return
        }

        this.compilationUnits.map((it) => it.compile())

        this.building = false
        if (this.buildOnComplete) {
            this.queueBuild()
        }
        this.buildOnComplete = false
    }

private async watch(): Promise<void> {
        const watcher = chokidar.watch(cwd())
        let initialised = false

        watcher.on("add", (_path: string) => {
            if (initialised) {
                throw new Error("TODO")
            }
        })
        watcher.on("change", (_path: string) => {
            if (initialised) {
                console.log("change")
                // this.queueBuild()
            }
        })

        await new Promise((resolve) => {
            watcher.once("ready", () => {
                initialised = true
                resolve(void 0)
            })
        })


        /*
        const watcher = chokidar.watch(files.map(path.dirname))
        let initialised = false

        watcher.on("add", (_path: string) => {
            if (initialised) {
                throw new Error("TODO")
            }
        })
        watcher.on("change", (_path: string) => {
            if (initialised) {
                this.queueBuild()
            }
        })

        await new Promise((resolve) => {
            watcher.once("ready", () => {
                initialised = true
                resolve(void 0)
            })
        })
        *
    }
*/
