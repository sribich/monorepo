import assert from "assert"
import { type ChildProcess, fork } from "child_process"
import type { BuildResult } from "esbuild"
import pidtree from "pidtree"

let childProcess: undefined | ChildProcess

export const killCurrentProcess = (signal = "SIGTERM", sync = false): void | Promise<void> => {
    if (!childProcess || !childProcess.pid) {
        return
    }

    if (sync) {
        try {
            childProcess.kill(signal as NodeJS.Signals)
            childProcess = undefined
        } catch (_) {
            void 0
        }
        return
    }

    const tree = pidtree(childProcess.pid, { root: true })

    return tree
        .then((it) =>
            it.forEach((pid) => {
                try {
                    process.kill(pid, signal)
                } catch (_) {
                    void 0
                }
            }),
        )
        .then(() => {
            assert(childProcess)

            if (!childProcess.killed) {
                try {
                    childProcess.kill(signal as NodeJS.Signals)
                } catch (_) {
                    void 0
                }
            }
        })
        .catch((_) => {
            void 0
        })
        .finally(() => {
            childProcess = undefined
        })
}

export const runProcess = (path: string, nodeArgs: string[] = []): void => {
    const execArgv = nodeArgs

    // TODO: We need to be able to point to this dynamically.
    // const outfile = Object.keys(event.metafile!.outputs).filter((it) => (!it.endsWith("LEGAL.txt") && !it.endsWith(".map")))[0]!
    // const outfile = event.outputFiles![0]!.path
    const outfile = "dist/main.js"

    const subProcess = fork(outfile, undefined /*, options.args*/, {
        execArgv,
        stdio: "inherit",
        env: {
            ...process.env,
        },
    })

    childProcess = subProcess

    // We may want to keep this and decide whether or not
    // we want to return early (or not) with a flag.
    //
    // if (!options.watch) {
    //     return new Promise((resolve, reject) => {
    //         subProcess.on("exit", (code) => {
    //             if (code === 0) {
    //                 resolve()
    //             } else {
    //                 reject()
    //             }
    //         })
    //     })
    // }
}

/*
const getExecArgv = (options: Schema): string[] => {
    const args = [...options.nodeArgs]

    if (options.inspect) {
        const inspectFlag = options.inspect.brk ? "--inspect-brk" : "--inspect"

        args.push(`${inspectFlag}=${options.inspect.host}:${options.inspect.port}`)
    }

    if (options.sourceMaps) {
        // args.push("--enable-source-maps")
    }

    // TODO: This needs to be removed after the evidence migration
    args.push("--openssl-legacy-provider")

    return args
}*/
