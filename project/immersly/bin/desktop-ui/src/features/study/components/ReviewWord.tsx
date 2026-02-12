import {
    Card,
    ContextMenu,
    makeStyles,
    Menu,
    MenuItem,
    Popover,
    useObjectRef,
    useStyles,
} from "@sribich/fude"
import type { DictionaryWord } from "../../../generated/rpc-client/dictionary_GetExactWord"
import { create } from "@stylexjs/stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { Ruby } from "./Ruby"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { PitchAccentItems } from "./PitchAccent"
import { use, useState } from "react"
import { ApiHostContext } from "../../../hooks/useApiPort"
import { getPronunciation } from "../../../generated/rpc-client/pronunciation_GetPronunciation"
import { AudioPlayer, PlayButton } from "../../../components/AudioPlayer/AudioPlayer"
import type { Card as ScheduledCard } from "../../../generated/rpc-client/scheduler_reviewCard"
import type { HTMLMediaControls } from "../../../hooks/useAudio"

//==============================================================================
// ReviewWord
//==============================================================================
export namespace ReviewWord {
    export interface Props {
        card: ScheduledCard
        word: DictionaryWord
    }
}

export const ReviewWord = (props: ReviewWord.Props) => {
    const { host } = use(ApiHostContext)

    const { styles } = useStyles(componentStyles, {})

    const [speakerId, setSpeakerId] = useState(undefined)

    const { data, isLoading, error } = getPronunciation(["study_pronunciation", props.word.word], {
        word: props.word.word,
    })

    const accents = props.word.accents.map(({ reading, accent }) => ({
        word: reading,
        position: accent,
    }))

    const audioRef = useObjectRef<HTMLMediaControls>()

    return (
        <Card {...styles.card()}>
            <Ruby {...styles.reading()}>{props.word.reading_ruby ?? props.word.word}</Ruby>

            <PitchAccentItems {...styles.pitchAccents()} items={accents} />

            {speakerId && (
                <AudioPlayer src={`${host}/rpc/pronunciation/${speakerId}/play`} autoPlay />
            )}

            <div {...styles.audio()}>
                {props.card.readingAudio && (
                    <ContextMenu>
                        <AudioPlayer
                            key={1}
                            src={`${host}/rpc/scheduler:playAudio/${props.card.id}/reading`}
                            autoPlay
                            onEnded={() => {
                                audioRef.current.play()
                            }}
                        >
                            <PlayButton />
                        </AudioPlayer>
                        <Popover placement={"bottom start"}>
                            <Menu items={data?.pronunciations ?? []}>
                                {(item) => (
                                    <MenuItem id={item.id} onAction={() => setSpeakerId(item.id)}>
                                        {item.speaker}
                                    </MenuItem>
                                )}
                            </Menu>
                        </Popover>
                    </ContextMenu>
                )}
                {props.card.sentenceAudio && (
                    <AudioPlayer
                        key={2}
                        src={`/rpc/scheduler:playAudio/${props.card.id}/sentence`}
                        ref={audioRef}
                    >
                        <PlayButton />
                    </AudioPlayer>
                )}
            </div>
        </Card>
    )
}

export const componentStyles = makeStyles({
    slots: create({
        card: {
            minWidth: newSpacing["384"],
            maxWidth: "50%",
            flex: "1 0 auto",
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
        },
        reading: {
            fontSize: fontSize["4xl"],
        },
        pitchAccents: {
            paddingTop: newSpacing["4"],
        },
        audio: {
            display: "flex",
            flexDirection: "row",
            justifyContent: "center",
        },
    }),
    variants: {},
    defaultVariants: {},
})
