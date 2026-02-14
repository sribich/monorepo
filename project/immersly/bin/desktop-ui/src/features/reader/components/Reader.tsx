import { Flex } from "@sribich/fude"
import { useCallback, useEffect, useLayoutEffect, useRef } from "react"

import type { AudioPlayer } from "../../../components/AudioPlayer/AudioPlayer"
import { ReaderAudio } from "./ReaderAudio"
import type { BookEntry } from "../../library/hooks/useBook"
import { ReaderPage } from "./ReaderPage"
import { AudioRefContext } from "../context/AudioRefContext"
import { useSingleKey } from "../../../hooks/useSingleKey"

//==============================================================================
// Reader
//==============================================================================
export namespace Reader {
    export interface Props {
        bookId: string
        bookAudioId: string

        entries: BookEntry[]
        timestamp?: number | undefined
    }
}

export const Reader = (props: Reader.Props) => {
    const audioRef = useRef<AudioPlayer.ImperativeHandle>(null)

    const toggleAudio = useCallback(() => {
        audioRef.current?.toggle()
    }, [])

    useSingleKey(" ", toggleAudio)

    // Start the audio player and sync it to the correct time.
    useEffect(() => {
        ;(async () => {
            await audioRef.current?.play()

            const audioStart =
                props.entries.find((item) => !Number.isNaN(item.timestamp.start))?.timestamp
                    .start ?? 0
            const timestamp = props.timestamp ?? audioStart
            const offset = Math.round(timestamp / 1000)

            if (offset) {
                audioRef.current?.seek(offset)
            }
        })()
    }, [props.timestamp, props.entries])
    console.log(props)
    return (
        <Flex direction="column" style={{ height: "100%" }}>
            <AudioRefContext value={audioRef}>
                <ReaderPage audioRef={audioRef} entries={props.entries} />
                <ReaderAudio audioRef={audioRef} bookId={props.bookId} audioId={props.bookAudioId} />
            </AudioRefContext>
        </Flex>
    )
}

/*
    const [playing, setPlaying] = useState(true)

    const toggleAudio = useCallback(
        (value?: boolean) => {
            const nextValue = !(value || playing)

            if (nextValue) {
                audioRef.current.play()
            } else {
                audioRef.current.pause()
            }

            setPlaying(nextValue)
        },
        [playing],
    )

    const { styles } = useStyles(testStyles, {})


}
*/
