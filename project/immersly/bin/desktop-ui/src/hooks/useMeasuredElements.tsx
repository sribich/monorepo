import { mergeProps, useStyleProps, type StyleProps } from "@sribich/fude"
import {
    createContext,
    useCallback,
    useEffect,
    useLayoutEffect,
    useMemo,
    useRef,
    useState,
    type ReactNode,
} from "react"
import { slice } from "../util/slice"
import type { AbsoluteSize } from "../features/library/hooks/useBook"

export const MeasuringContext = createContext(false)

type Task = { kind: "measure"; from: number; to: number } | { kind: "queue_work" }

interface UnitOfWork {
    /**
     * Whether this work needs to run synchronously.
     */
    synchronous: boolean
    /**
     *
     */
    task: Task
}

export interface MeasurableComponent {
    size: AbsoluteSize
    id: number
    component: ReactNode
}

export interface UseMeasuredElementsResult<T extends MeasurableComponent> {
    /**
     * A component which will hold the elements as they are being measured. This
     * must be mounted in the consuming component.
     */
    measurer: ReactNode
    /**
     * Whether the measurer is currently performing a synchronous measurement.
     *
     * Content cannot be rendered during this time, as it means elements were
     * requested before they could be measured asynchronously.
     */
    isMeasuringSync: boolean
    /**
     * Returns a slice of the set of components that will fill the container.
     *
     * This may force a synchronous render if the components are not yet
     * measured.
     */
    fillForward: (from: number, x: number, y: number, subLines: number) => T[]
    /**
     * Returns a slice of the set of components that will fill the container.
     *
     * This may force a synchronous render if the components are not yet
     * measured.
     */
    fillBackward: (from: number, x: number, y: number) => T[]
}

export interface UseMeasuredElementsOptions<T extends MeasurableComponent> extends StyleProps {
    /**
     * The items that need to be measured.
     */
    items: T[]
    /**
     * Starts measurement from the provided index.
     */
    fromIndex: number
    /**
     * Synchronously measures [`preloadCount`] elements to ensure that the screen
     * can be painted.
     *
     * @default 50
     */
    preloadCount?: number
    /**
     * TODO
     *
     * @default 3
     */
    chunkSize?: number

    size: { width: number; height: number }
}

export const useMeasuredElements = <T extends MeasurableComponent>(
    options: UseMeasuredElementsOptions<T>,
): UseMeasuredElementsResult<T> => {
    const { chunkSize = 10, preloadCount = 25 } = options

    const hiddenContainerRef = useRef<HTMLDivElement>(null)

    const [isMeasuring, setMeasuring] = useState(false)
    const [isMeasuringSync, setMeasuringSync] = useState(false)

    const [measuringIndexes, setMeasuringIndexes] = useState<[number, number]>([0, 0])

    useLayoutEffect(() => {
        if (hiddenContainerRef.current) {
            for (const child of hiddenContainerRef.current.children) {
                const measuredNode = child.children?.[0]

                const id = Number(child.getAttribute("data-id"))
                const item = options.items[id]

                if (!item || item.size || !measuredNode) {
                    continue
                }

                const size = measuredNode.getBoundingClientRect()

                const computedStyles = getComputedStyle(measuredNode)

                const padding = {
                    x:
                        Number.parseFloat(computedStyles.paddingLeft) +
                        Number.parseFloat(computedStyles.paddingRight),
                    y:
                        Number.parseFloat(computedStyles.paddingTop) +
                        Number.parseFloat(computedStyles.paddingBottom),
                }
                const margin = {
                    x:
                        Number.parseFloat(computedStyles.marginLeft) +
                        Number.parseFloat(computedStyles.marginRight),
                    y:
                        Number.parseFloat(computedStyles.marginTop) +
                        Number.parseFloat(computedStyles.marginBottom),
                }
                const border = {
                    x:
                        Number.parseFloat(computedStyles.borderLeftWidth) +
                        Number.parseFloat(computedStyles.borderRightWidth),
                    y:
                        Number.parseFloat(computedStyles.borderTopWidth) +
                        Number.parseFloat(computedStyles.borderBottomWidth),
                }

                item.size = {
                    x: size.width - padding.x - border.x,
                    y: size.height - padding.y - border.y,
                    xSpacing: padding.x + margin.x + border.x,
                    ySpacing: padding.y + margin.y + border.y,
                }
            }

            setMeasuring(false)
            setMeasuringSync(false)
            setMeasuringIndexes([0, 0])
        }
    })

    const [queue, setQueue] = useState<UnitOfWork[]>([])

    /**
     * This is the queue worker / scheduler.
     */
    useEffect(() => {
        const timeout = setTimeout(() => {
            const unitOfWork = queue.at(0)

            if (!unitOfWork || unitOfWork.synchronous || isMeasuring) {
                return
            }

            switch (unitOfWork.task.kind) {
                case "measure": {
                    const { from, to } = unitOfWork.task
                    const needsMeasuring = slice(options.items, from, to).find((item) => !item.size)

                    if (needsMeasuring) {
                        // Run asynchronous measurements as a macrotask to give the application
                        // more time to do work.
                        setTimeout(() => {
                            setMeasuring(true)
                            setMeasuringSync(unitOfWork.synchronous)
                            setMeasuringIndexes([from, to])
                        }, 0)
                    }

                    break
                }
                case "queue_work": {
                    const chunkWork = new Array(Math.ceil(options.items.length / chunkSize))
                        .fill(null)
                        .map(
                            (_, index) =>
                                ({
                                    synchronous: false,
                                    task: {
                                        kind: "measure",
                                        from: index * chunkSize,
                                        to: Math.min((index + 1) * chunkSize, options.items.length),
                                    },
                                }) as UnitOfWork,
                        )

                    setQueue((prev) => [...prev, ...chunkWork])

                    break
                }
            }

            setQueue((prev) => prev.slice(1))
        }, 50)

        return () => clearTimeout(timeout)
    }, [options.items, queue, isMeasuring, chunkSize])

    /**
     * Handle synchronous tasks
     */
    useLayoutEffect(() => {
        const unitOfWork = queue.at(0)

        if (
            isMeasuring ||
            !unitOfWork ||
            !unitOfWork.synchronous ||
            unitOfWork.task.kind !== "measure"
        ) {
            return
        }

        const { from, to } = unitOfWork.task
        const needsMeasuring = slice(options.items, from, to).find((item) => !item.size)

        if (needsMeasuring) {
            setMeasuring(true)
            setMeasuringSync(unitOfWork.synchronous)
            setMeasuringIndexes([from, to])
        }

        setQueue((prev) => prev.slice(1))
    })

    const measurer =
        measuringIndexes && (measuringIndexes[0] !== 0 || measuringIndexes[1] !== 0) ? (
            <MeasuringContext value={true}>
                <div
                    ref={hiddenContainerRef}
                    style={{ position: "absolute", visibility: "hidden" }}
                >
                    {slice(options.items, ...measuringIndexes).map((item, id) =>
                        !item.size ? (
                            <div data-id={measuringIndexes[0] + id} style={{ width: "99999px" }}>
                                {item.component}
                            </div>
                        ) : null,
                    )}
                </div>
            </MeasuringContext>
        ) : null

    return {
        measurer,
        isMeasuringSync,
        fillForward: useCallback(
            (from: number, x: number, y: number, subLines: number): MeasurableComponent[] => {
                let filledHeight = 0

                if (isMeasuringSync) {
                    return []
                }

                for (let i = from; i < options.items.length; i++) {
                    const component = options.items[i]

                    if (!component) {
                        continue
                    }

                    if (!component.size) {
                        // Only queue the work if we do not have it already queued.
                        if (
                            !queue.find(
                                (item) => item.synchronous && item.task.kind === "measure",
                            ) &&
                            !isMeasuringSync
                        ) {
                            setQueue(() => [
                                {
                                    synchronous: true,
                                    task: {
                                        kind: "measure",
                                        from: from,
                                        to: Math.min(from + preloadCount, options.items.length),
                                    },
                                },
                                {
                                    synchronous: false,
                                    task: {
                                        kind: "queue_work",
                                    },
                                },
                            ])
                        }

                        // TODO: We need to perform a preload here
                        return []
                    }

                    const componentHeight =
                        component.size.y *
                            Math.ceil(component.size.x / (x - component.size.xSpacing)) +
                        component.size.ySpacing

                    if (filledHeight + componentHeight > y) {
                        return slice(options.items, from, i + 1 - subLines)
                    }

                    filledHeight += componentHeight
                }

                return []
            },
            [isMeasuringSync, options, queue],
        ),
        fillBackward: (from: number, x: number, y: number): MeasurableComponent[] => {
            return []
        },
    }
}
