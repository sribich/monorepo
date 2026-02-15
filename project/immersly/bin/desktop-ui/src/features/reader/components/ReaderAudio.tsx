import { Card, CardBody, makeStyles, useObjectRef, useStyles } from "@sribich/fude"
import { type RefObject, use, useEffect } from "react"
import {
    AudioPlayer,
    AudioPlayerTime,
    PlayButton,
} from "../../../components/AudioPlayer/AudioPlayer"
import { setProgress } from "../../../generated/rpc-client/library_SetProgress"
import { ApiHostContext } from "../../../hooks/useApiPort"
import { create } from "@stylexjs/stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"

export namespace ReaderAudio {
    export interface Props {
        audioRef?: RefObject<AudioPlayer.ImperativeHandle>

        bookId: string
        audioId: string
    }
}

export const ReaderAudio = (props: ReaderAudio.Props) => {
    const { host } = use(ApiHostContext)

    const { mutateAsync: setProgressAsync } = setProgress([props.bookId], {})

    const audioRef = useObjectRef(props.audioRef)

    useEffect(() => {
        const interval = setInterval(() => {
            if (!audioRef.current) {
                return
            }

            setProgressAsync({
                bookId: props.bookId,
                progress: Math.floor(audioRef.current.getTime() * 1000),
            })
        }, 5000)

        return () => clearInterval(interval)
    }, [props.bookId, setProgressAsync, props.audioRef])

    const { styles } = useStyles(readerAudioStyles, {})

    return (
        <Card {...styles.wrapper()} fullWidth rounded="lg">
            <CardBody>
                <AudioPlayer src={`${host}/rpc/resource/${props.audioId}`} type="audio/wav" ref={audioRef}>
                    <div {...styles.controls()}>
                        <PlayButton />
                    </div>
                    <AudioPlayerTime />
                </AudioPlayer>
            </CardBody>
        </Card>
    )
}

const readerAudioStyles = makeStyles({
    slots: create({
        wrapper: {
            backgroundColor: colors.backgroundSecondary,
            padding: newSpacing["12"],
        },
        controls: {
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
        },
    }),
    variants: {},
    defaultVariants: {},
})
