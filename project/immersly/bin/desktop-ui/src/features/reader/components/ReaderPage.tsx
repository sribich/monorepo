import { use, useLayoutEffect, useMemo, useRef, useState, type RefObject } from "react"
import type { AudioPlayer } from "../../../components/AudioPlayer/AudioPlayer"
import type { BookEntry } from "../../library/hooks/useBook"
import { useElementSize } from "../../../hooks/useElementSize"
import { useMeasuredElements } from "../../../hooks/useMeasuredElements"
import { Button } from "@sribich/fude"
import { Play, Redo2 } from "lucide-react"
import { AudioRefContext } from "../context/AudioRefContext"

//==============================================================================
// ReaderPage
//==============================================================================
export namespace ReaderPage {
    export interface Props {
        audioRef: RefObject<AudioPlayer.ImperativeHandle>
        entries: BookEntry[]
    }
}

export const ReaderPage = (props: ReaderPage.Props) => {
    const contentRef = useRef<HTMLDivElement>(null)
    const { elementRef, elementSize } = useElementSize({
        watchForResizes: true,
    })

    const { measurer, ...measuredElements } = useMeasuredElements({
        items: props.entries,
        fromIndex: 0,
        size: elementSize,
    })

    const [subLines, setSubLines] = useState(0)

    const [startingIndex, setStartingIndex] = useState(() => {
        const currentTime = props.audioRef.current?.getTime() * 1000

        return Math.max(
            0,
            props.entries.findIndex((item) => item.timestamp.start > currentTime) - 1,
        )
    })

    const doResize = () => {
        const currentTime = props.audioRef.current?.getTime() * 1000
        const index = Math.max(
            0,
            props.entries.findIndex((item) => item.timestamp.start > currentTime) - 1,
        )

        setStartingIndex(index)
    }

    const audioRef = use(AudioRefContext)

    // const renderedElements = useMemo(() => {
    const elementList = measuredElements.fillForward(
        startingIndex,
        elementSize.width,
        elementSize.height,
        subLines,
    )

    const renderedElements = elementList
        .map((item) => (
            <span key={item.id} style={{ display: "flex", alignItems: "start" }}>
                <div style={{ height: "31px", display: "flex", alignItems: "center" }}>
                    <Button
                        size="xs"
                        variant="ghost"
                        style={{ position: "absolute", left: "-24px" }}
                        onPress={() => {
                            if (item.timestamp.start) {
                                audioRef.current?.seek(item.timestamp.start / 1000)
                            }
                        }}
                    >
                        <Redo2 size={12} />
                    </Button>
                </div>
                {item.component}
            </span>
        ))
        .slice()
    // }, [])

    useLayoutEffect(() => {
        if (!contentRef.current) {
            return
        }

        const { height } = contentRef.current.getBoundingClientRect()

        if (height > elementSize.height) {
            // We've got too many lines and we're overflowing the container. Force a reset.
            setSubLines((prev) => prev + 1)
        }
    })

    useLayoutEffect(() => {
        if (measuredElements.isMeasuringSync) {
            return
        }

        const interval = setInterval(() => {
            if (elementList.length === 0) {
                return
            }

            const currentTime = props.audioRef.current?.getTime() * 1000

            if (currentTime > elementList[elementList.length - 1]?.timestamp.end) {
                setStartingIndex(
                    Math.max(
                        0,
                        props.entries.findIndex((item) => item.timestamp.start > currentTime) - 1,
                    ),
                )
                setSubLines(0)
            } else if (currentTime < elementList[0]?.timestamp.start) {
                const newValue = Math.max(
                    0,
                    props.entries.findIndex((item) => item.timestamp.start > currentTime) - 1,
                )

                if (newValue !== startingIndex) {
                    setStartingIndex(newValue)
                    setSubLines(0)
                }
            }
        }, 25)

        return () => clearInterval(interval)
    }, [measuredElements.isMeasuringSync, props.audioRef.current, props.entries, startingIndex])

    return (
        <div ref={elementRef} style={{ flexGrow: 1, padding: "12px", paddingLeft: "24px" }}>
            {measurer}
            <div ref={contentRef} style={{ position: "relative" }}>
                {renderedElements}
            </div>
            <button style={{ position: "absolute", bottom: 0, right: 0 }} onClick={doResize}>
                R
            </button>
        </div>
    )
}

//
