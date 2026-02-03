import {
    Card,
    ContextMenu,
    Menu,
    MenuItem,
    Popover,
    type VariantProps,
    makeStyles,
    useStyles,
} from "@sribich/fude"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { newSpacing, spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"
import { use, useState } from "react"
import {
    type DictionaryWord,
    type GetExactWordQueryResponse,
    getExactWord,
} from "../generated/rpc-client/dictionary_GetExactWord"
import { getPronunciation } from "../generated/rpc-client/pronunciation_GetPronunciation"
import { AudioPlayer, PlayButton } from "./AudioPlayer/AudioPlayer"
import { Text } from "./Text"
import { ApiHostContext } from "../hooks/useApiPort"

//==============================================================================
// StudyView
//==============================================================================
export namespace StudyView {
    export interface Props {
        cardId: string
        word: string
        reading: string
        readingAudio: boolean
        sentence?: string | undefined
        sentenceAudio: boolean
    }
}
export const StudyView = (props: StudyView.Props) => {
    const { styles } = useStyles(studyViewStyles, {})

    const { data, isLoading, refetch } = getExactWord([props.word, props.reading], {
        word: props.word,
        reading: props.reading,
    })

    if (isLoading) {
        return null
    }

    if (!data?.word) {
        return null
    }

    const [definition, ...otherDefinitions] = data.word.definitions

    return (
        <div {...styles.wrapper()}>
            <div {...styles.wordMeta()}>
                <StudyWord
                    cardId={props.cardId}
                    wordMeta={data.word}
                    readingAudio={props.readingAudio}
                    sentenceAudio={props.sentenceAudio}
                />
            </div>

            <Text language="jp">{props.sentence}</Text>

            <Card>
                <Definition definition={definition} />
                <Definition definition={data.word.bilingual_definition} />
            </Card>
        </div>
    )
}

const studyViewStyles = makeStyles({
    slots: create({
        wrapper: {
            maxWidth: newSpacing["768"],
        },
        wordMeta: {
            display: "flex",
            justifyContent: "center",
        },
    }),
    variants: {},
    defaultVariants: {},
})

//==============================================================================
// Definition
//==============================================================================
const Definition = (props) => {
    if (!props.definition) {
        return null
    }

    return <div dangerouslySetInnerHTML={{ __html: props.definition.definition }} />
}

/*
export const DictionaryEntry = ({ addCard, definition }: DictionaryEntry.Props) => {
    const { styles } = useStyles(dictionaryEntryStyles, {})

    const frequencies = definition.frequencies.map((frequency) => (
        <Frequency key={frequency.id} source={frequency.dictionary} value={frequency.freq} />
    ))

    const definitions = definition.definitions.map((definition) => {
        return (
            <li key={definition.id}>
                <span {...styles.definitionHeader()}>{definition.dictionary}</span>
                <div dangerouslySetInnerHTML={{ __html: definition.definition }} />
            </li>
        )
    })

    return (
        <>
            <Divider {...styles.divider()} />
            <div>
                <div {...styles.headerWrapper()}>
                    <span {...styles.reading()}>
                        <ruby>
                            {definition.word}
                            <rt>{definition.reading}</rt>
                        </ruby>
                    </span>
                    <Button onClick={() => addCard?.(definition.reading)}>Add Card</Button>
                </div>

                <div {...styles.frequencyList()}>{frequencies}</div>

                <Accents accents={definition.accents} />

                <ol>{definitions}</ol>
            </div>
        </>
    )
}

const dictionaryEntryStyles = makeStyles({
    slots: create({
        divider: {
            margin: "8px",
        },
        reading: {
            fontSize: fontSize["4xl"],
        },
        headerWrapper: {
            display: "flex",
            flexDirection: "row",
            justifyContent: "space-between",
        },
        frequencyList: {
            display: "flex",
            flexDirection: "row",
            gap: "4px",
            paddingBottom: "4px",
        },
        definitionHeader: {
            borderRadius: "8px",
            backgroundColor: "#9057ad",
            padding: "4px",
            color: "#fff",
            display: "inline-block",
            fontWeight: 600,
        },
    }),
    variants: {},
    defaultVariants: {},
})

//==============================================================================
// Frequency
//==============================================================================
export namespace Frequency {
    export interface Props {
        source: string
        value: string
    }
}

const Frequency = (props: Frequency.Props) => {
    const { styles } = useStyles(frequencyStyles, {})

    return (
        <div {...styles.wrapper()}>
            <div {...styles.left()}>{props.source}</div>
            <div {...styles.right()}>{props.value}</div>
        </div>
    )
}

const frequencyStyles = makeStyles({
    slots: create({
        multiWrapper: {
            display: "flex",
            flexDirection: "row",
            gap: "8px",
        },
        wrapper: {
            display: "flex",
            flexDirection: "row",
            border: "1px solid #ff5733",
            borderRadius: "8px",
            maxWidth: "fit-content",
            overflow: "hidden",
            backgroundColor: "#ff5733",
        },
        left: {
            color: "#ffffff",
            padding: "2px 8px",
            fontWeight: "700",
        },
        right: {
            padding: "2px 8px",
            backgroundColor: "#ffffff",
        },
    }),
    variants: {},
    defaultVariants: {},
})


*/
