import chalk from "chalk"

const syncHandlers = new Set<CleanupHandler<void>>()
const asyncHandlers = new Set<CleanupHandler<Promise<void>>>()

let isConfigured = false
let isExiting = false

export type CleanupHandler<T> = () => T
export type CleanupRegistry<T> = (handler: CleanupHandler<T>) => void

const configureSignalHandlers = () => {
    if (isConfigured) {
        return
    }

    isConfigured = true

    process.on("beforeExit", exitAsync)
    process.on("exit", exit)

    process.on("uncaughtException", (e) => {
        console.error(e)
        exit(1)
    })
    process.on("unhandledRejection", (e) => {
        console.error(e)
        exit(1)
    })

    // When a signal is successfully handled the exit code should be
    // EXIT_SUCCESS. If the program exits abnormally as a result of
    // receiving a signal then `128 + signum` should be returned.
    //
    // For a program like tsbuild, any signal based exit should be
    // considered a failure.
    process.on("SIGHUP", () => exitAsync(129, "SIGHUP"))
    process.on("SIGINT", () => exitAsync(130, "SIGINT"))
    process.on("SIGTERM", () => exitAsync(143, "SIGTERM"))
}

const checkExit = (reason?: string) => {
    if (isExiting) {
        if (reason) {
            console.info(`Received ${reason}. Forcefully exiting.`)
            process.exit(1)
        }

        return true
    }

    isExiting = true

    if (reason) {
        console.warn(`\n${chalk.yellow(`Received ${reason}. Performing cleanup.`)}`)
    }

    console.warn("You may press CTRL+C at any time to forcefully exit.\n")

    return false
}

const exit = (status?: number) => {
    if (checkExit()) {
        return
    }

    if (asyncHandlers.size > 0) {
        console.error(`
The process has exited in an uncontrolled manner and ${asyncHandlers.size}
registered asynchronous cleanup handlers have not run.

To shutdown gracefully, use exitAsync().
`)
    }

    for (const handler of syncHandlers) {
        handler()
    }

    if (status !== undefined) {
        process.exit(status)
    }
}

/**
 * Runs all sync and async callbacks before safely exiting the application.
 */
export const exitAsync = async (status = 0, reason?: string): Promise<void> => {
    if (checkExit(reason)) {
        return
    }

    for (const handler of syncHandlers) {
        handler()
    }

    for (const handler of asyncHandlers) {
        await handler()
    }

    process.exit(status)
}

export const addCleanupHandler: CleanupRegistry<void> = (handler) => {
    configureSignalHandlers()

    syncHandlers.add(handler)
}

export const addAsyncCleanupHandler: CleanupRegistry<Promise<void>> = (handler) => {
    configureSignalHandlers()

    asyncHandlers.add(handler)
}
