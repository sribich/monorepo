import { Button } from "@sribich/fude"
import type { SentenceData } from "../hooks/useBook"
import { Word } from "./Word"
import { Play } from "lucide-react"

export namespace Sentence {
    export interface Props {
        bookId: string
        audioId: string
        data: SentenceData
    }
}

export const Sentence = (props: Sentence.Props) => {
    const sentence = props.data[2].map((it) => it[0]).join("")

    const words = props.data[2].map((segment, index) => {
        return (
            <Word
                bookId={props.bookId}
                word={segment[0]}
                dictWord={segment[1]}
                wordTs={[0, 0]}
                sentenceTs={[props.data[0][0], props.data[0][1]]}
                sentenceAudio={props.audioId}
                sentence={sentence}
                freq={segment[2]}
            />
        )
    })

    return (
        <>
            <span style={{ display: "inline-block", marginBottom: "4px" }}>{words}</span>
            <br />
        </>
    )
}

/*
        return payload.map((entry) => {
                            let textContent = ""

            const segments = entry.segments.map((segment, i) => {
                            textContent += segment.word

                return (
                        <Word
                            data={i}
                            key={i}
                            bookId={id}
                        />
                        )
            })

                        const element = <p style={{ paddingBottom: "4px" }}>{segments}</p>

                        return {
                            timestamp: entry.t0,
                        element,
                        measurer: <span {...styles.text()}>{textContent}</span>,
            }
        })
                        */
