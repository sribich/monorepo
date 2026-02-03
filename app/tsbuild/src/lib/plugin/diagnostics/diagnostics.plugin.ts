import { logger } from "../../logger.js"
import type { Plugin } from "../plugin.js"

let traceId = 0
let activeTrace: DiagnosticTrace | null = null

export const diagnostics: {
    startTrace: () => DiagnosticTrace
    span: <T extends Array<unknown>, U>(
        info: SpanInfo,
        fn: (span: DiagnosticSpan, ...args: T) => U | Promise<U>,
    ) => (...args: T) => Promise<U>
} = {
    startTrace: (): DiagnosticTrace => {
        activeTrace = new DiagnosticTrace(traceId++)

        return activeTrace
    },
    span: <T extends Array<unknown>, U>(
        info: SpanInfo,
        fn: (span: DiagnosticSpan, ...args: T) => U | Promise<U>,
    ) => {
        return async (...args: T): Promise<U> => {
            const span = new DiagnosticSpan(info)

            activeTrace?.addSpan(span)

            span.start()
            const result = await fn(span, ...args)
            span.end()

            return result
        }
    },
} as const

export const diagnosticsPlugin: Plugin = async (context) => {
    const projectName = context.repository.project.name

    let time: number

    return {
        onBuildStart() {
            time = new Date().getTime()

            logger.info(`Build started for ${projectName}`)
        },
        onBuildEnd() {
            logger.info(`Build ended for ${projectName}: ${new Date().getTime() - time}ms`)
        },
        postBuild() {},
    }
}

class DiagnosticTrace {
    private spans: Map<(typeof spanPhases)[number], DiagnosticSpan[]> = new Map()

    public readonly traceId: number

    constructor(traceId: number) {
        this.traceId = traceId
    }

    addSpan(span: DiagnosticSpan): void {
        const spanSet = this.spans.get(span.info.phase)

        if (!spanSet) {
            this.spans.set(span.info.phase, [span])
        } else {
            spanSet.push(span)
        }
    }

    printDiagnostics(): void {
        if (activeTrace?.traceId !== this.traceId) {
            return
        }

        for (const phase of spanPhases) {
            const spans = this.spans.get(phase)

            if (spans) {
                this.printPhase(phase, spans)
            }
        }
    }

    // TODO: We need to be able to handle multiple different "runs" of the same
    //       backend for different formats.
    printPhase(phase: (typeof spanPhases)[number], spans: DiagnosticSpan[]): void {
        let totalTime = 0
        let phaseBody = ""

        const concurrentSpans: Map<
            string,
            { min: number; max: number; avg: number; count: number }
        > = new Map()
        const isConcurrent = ["onLoad", "onResolve"].includes(phase)

        // TODO: Sort this
        for (const span of spans) {
            const time = span.getTime()

            if (isConcurrent) {
                const value = concurrentSpans.get(span.info.name) || {
                    min: Infinity,
                    max: 0,
                    avg: 0,
                    count: 0,
                }

                value.avg += time
                value.max = Math.max(value.max, time)
                value.min = Math.min(value.min, time)
                value.count += 1

                concurrentSpans.set(span.info.name, value)
            } else {
                totalTime += time
                phaseBody += `  ${span.info.name} (${time}ms)\n`
            }
        }

        totalTime += Array.from(concurrentSpans.values()).reduce((acc, sum) => acc + sum.max, 0)

        console.log(`${phase} - ${totalTime}ms`)
        console.log(phaseBody)

        for (const [name, value] of concurrentSpans.entries()) {
            console.log(
                `  ${name} (min=${value.min}, max=${value.max}, avg=${
                    value.avg / value.count
                }, runs=${value.count})`,
            )
        }
    }
}

class DiagnosticSpan {
    private started = -1
    private elapsed = -1

    public readonly info: SpanInfo

    constructor(info: SpanInfo) {
        this.info = info
    }

    start(): void {
        if (this.started !== -1) {
            throw new Error("The start() method may not be called multiple times on a span.")
        }

        this.started = Date.now()
    }

    end(): void {
        if (this.elapsed !== -1) {
            throw new Error("The end() method may not be called multiple times on a span.")
        }

        this.elapsed = Date.now() - this.started
    }

    getTime(): number {
        if (this.elapsed === -1) {
            throw new Error("getTime() was called before the span was ended.")
        }

        return this.elapsed
    }
}

interface SpanInfo {
    name: string
    phase: (typeof spanPhases)[number]
}

export const spanPhases = [
    "preBuild",
    "onBuildStart",
    "onBuild",
    "onStart",
    "onResolve",
    "onLoad",
    "onEnd",
    "onBuildEnd",
    "postBuild",
] as const
