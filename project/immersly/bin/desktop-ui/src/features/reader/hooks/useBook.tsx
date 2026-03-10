import { useMemo, type ReactNode } from "react"
import { ArkErrors, scope, type } from "arktype"

import { readBook } from "../../../generated/rpc-client/library_ReadBook"
import { Sentence } from "../components/Sentence"

export const useBookKey = (bookId: string): string => {
    return ["read_book", bookId]
}

export const useBook = (bookId: string) => {
    const { data, error, isLoading } = readBook(["read_book", bookId], { id: bookId })

    const entries: BookEntry[] = useMemo(() => {
        if (!data) {
            return []
        }

        const payload = v2PayloadParser(JSON.parse(data.text))

        // TODO: This should probably be a development only error, but we need
        //       to display the error.
        if (payload instanceof ArkErrors) {
            throw new Error(payload.summary)
        }

        return payload.map((item, id) => {
            item[0][0] = Math.floor(item[0][0] * 1000)
            item[0][1] = Math.floor(item[0][1] * 1000)

            return {
                id,
                component: <Sentence bookId={bookId} audioId={data.audioId} data={item} />,
                // length: item.segments.reduce((acc, curr) => acc + curr.word.length, 0),
                timestamp: {
                    start: item[0][0],
                    end: item[0][1],
                },
            }
        })
    }, [data])
    console.log("timestamp", data?.timestamp)
    return {
        entries,
        timestamp: data?.timestamp,
        isLoading,
        bookAudioId: data?.audioId,
        error,
    }
}

export interface BookEntry {
    id: number
    component: ReactNode
    timestamp: TimeFrame
    size?: AbsoluteSize
}

/**
 * The size of an element with its spacing properties preserved separately.
 *
 * When these sizes are measured, they should be done in a container that has
 * no limit on its width. This will allow us to accurately determine the size
 * of the component when the viewport changes.
 *
 * To preserve spacing characteristics, we only need to account for the total
 * directional spacing, as the box the content occupies will not change based
 * on the specific direction the spacing is defined.
 */
export interface AbsoluteSize {
    x: number
    y: number

    xSpacing: number
    ySpacing: number
}

export interface TimeFrame {
    start: number | null
    end: number | null
}

const v2PayloadParser = scope({
    timestamp: ["number | null", "number | null"],
    word: ["string", "string", "number | null"],
    part: ["timestamp", "string", "word[]"],
    payload: "part[]",
}).export().payload

const payloadParser = scope({
    segment: {
        word: "string",
        base: "string",
        freq: "number | null",
    },
    part: {
        t0: "number | null",
        t1: "number | null",
        kind: "'Chapter' | 'Paragraph'",
        segments: "segment[]",
    },
    payload: "part[]",
}).export().payload

type Payload = {
    t0: number
    t1: number

    x: number
    y: number

    xSpace: number
    ySpace: number

    component: ReactNode
}

export type SentenceData = (typeof v2PayloadParser.infer)[number]
