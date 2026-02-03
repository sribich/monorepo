import { Button, Divider, makeStyles, Select, SelectItem, useStyles } from "@sribich/fude"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"
import { use, useEffect, useMemo, useState } from "react"
import { AudioPlayer } from "../../../components/AudioPlayer/AudioPlayer"
import { getWord } from "../../../generated/rpc-client/dictionary_GetWord"
import { getPronunciation } from "../../../generated/rpc-client/pronunciation_GetPronunciation"
import { ApiHostContext } from "../../../hooks/useApiPort"

//==============================================================================
// DictionaryLookupPage
//==============================================================================
export namespace DictionaryLookupPage {
    export type Props = Record<string, never>
}

export const DictionaryLookupPage = (props: DictionaryLookupPage.Props) => {
    return null
}

/*
import { Button, Divider, makeStyles, Select, SelectItem, TextField, useStyles } from "@sribich/fude"
import { createFileRoute } from "@tanstack/react-router"
import { useEffect, useMemo, useState } from "react"
import { create } from "@stylexjs/stylex"
import {
    getWord,
    type Accent as _Accent,
    type Definition,
    type DefinitionEntry,
} from "../../../generated/rpc-client/dictionary_GetWord"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { getPronunciation } from "../../../generated/rpc-client/pronunciation_GetPronunciation"
import { AudioPlayer } from "../../../components/AudioPlayer/AudioPlayer"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"

export const Route = createFileRoute("/_app/dictionary/")({
    component: () => <RouteComponent />,
})

const RouteComponent = () => {
    const [searchTerm, setSearchTerm] = useState("")
    const [result, setResult] = useState("")

    const { data, refetch } = getWord([searchTerm], { word: searchTerm }, { enabled: false })

    const components = (data?.definitions ?? []).map((definition) => (
        <DictionaryEntry key={`${definition.word}${definition.reading}`} definition={definition} />
    ))

    const search = () => {
        refetch()
    }

    return (
        <div>
            <div>
                <TextField value={searchTerm} onChange={setSearchTerm} />
                <Button onClick={search}>Search</Button>
            </div>
            {components}
        </div>
    )
}



//
//
//




*/

export namespace PronunciationSelection {
    export interface Props {
        word: string
        reading: string
        onIdChange: (_: string) => void
    }
}

export const PronunciationSelection = (props: PronunciationSelection.Props) => {
    const [speakerId, setSpeakerId] = useState(undefined)

    const { host } = use(ApiHostContext)

    const { data, isLoading, error } = getPronunciation(["pronunciation", props.word], {
        word: props.word,
    })

    const { data: speaker } = getPronunciation(
        ["pronunciation", props.word, speakerId],
        {
            word: props.word,
            // reading: props.reading,
            speakerId: speakerId,
        },
        {
            enabled: !!speakerId,
        },
    )

    const changeSpeaker = (id) => {
        setSpeakerId(id)
        props.onIdChange(id)
    }

    useEffect(() => {
        if (data?.pronunciations[0]) {
            changeSpeaker(data.pronunciations[0].id)
        }
    }, [data])

    if (isLoading || !data?.pronunciations?.[0]) {
        return null
    }

    const speakerData = (speaker ?? data)?.pronunciations?.[0]

    return (
        <>
            {speakerData && (
                <AudioPlayer src={`${host}/rpc/pronunciation/${speakerData.id}/play`} autoPlay>
                    ...
                </AudioPlayer>
            )}
            <Select
                items={data.pronunciations}
                onSelectionChange={changeSpeaker}
                selectedKey={speakerId}
                label="Speaker"
            >
                {(item) => {
                    return (
                        <SelectItem id={item.id} aria-label={item.speaker}>
                            {item.speaker} ({item.sex})
                        </SelectItem>
                    )
                }}
            </Select>
        </>
    )
}

export const DefinitionLookup = ({ searchTerm, addCard }) => {
    const { data, refetch } = getWord([searchTerm], { word: searchTerm }, {})

    const components = (data?.definitions ?? []).map((definition) => (
        <DictionaryEntry
            key={`${definition.word}${definition.reading}`}
            definition={definition}
            addCard={addCard}
        />
    ))

    return components
}

//==============================================================================
// DictionaryEntry
//==============================================================================
export namespace DictionaryEntry {
    export interface Props {
        addCard?: (reading: string) => void
        definition: DefinitionEntry
    }
}

export const DictionaryEntry = ({ addCard, definition }: DictionaryEntry.Props) => {
    const { styles } = useStyles(dictionaryEntryStyles, {})

    const [audioId, onIdChange] = useState("")

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

    const test = () => {
        console.log("IN TEST")
        console.log(addCard)
        addCard?.(definition.reading, audioId)
    }

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

                    <div {...styles.buttons()}>
                        <PronunciationSelection
                            word={definition.word}
                            reading={definition.reading}
                            onIdChange={onIdChange}
                        />
                        <Button onPress={test}>Add Card</Button>
                    </div>
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
        buttons: {
            display: "flex",
            gap: newSpacing["4"],
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

//==============================================================================
// Accents
//==============================================================================
export namespace Accents {
    export interface Props {
        accents: _Accent[]
    }
}

export const Accents = ({ accents }: Accents.Props) => {
    const { styles } = useStyles(accentsStyles, {})

    const partitionedAccents = useMemo(() => {
        return Object.entries(Object.groupBy(accents, ({ dictionary }) => dictionary)).map(
            ([dictionary, accent]) => {
                const components = accent.map((accent) => (
                    <li key={accent.id}>
                        <Accent
                            title={accent.reading}
                            reading={accent.reading}
                            position={accent.accent}
                        />
                    </li>
                ))

                return (
                    <div key={accent.id}>
                        <span {...styles.dictionaryHeader()}>{dictionary}</span>
                        <ul {...styles.accent()}>{components}</ul>
                    </div>
                )
            },
        )
    }, [accents])

    return <div>{partitionedAccents}</div>
}

const accentsStyles = makeStyles({
    slots: create({
        wrapper: {
            position: "relative",
        },
        dictionaryHeader: {
            borderRadius: "8px",
            backgroundColor: "#6640be",
            padding: "4px",
            color: "#fff",
            display: "inline-block",
            fontWeight: 600,
        },
        accent: {
            paddingTop: "4px",
            fontSize: fontSize.xl,
            fontWeight: 400,
        },
    }),
    variants: {},
    defaultVariants: {},
})

//==============================================================================
// Accent
//==============================================================================
export namespace Accent {
    export interface Props {
        title: string
        reading: string
        position: number
    }
}

export const Accent = ({ title, reading, position }: Accent.Props) => {
    const { styles } = useStyles(accentStyles, {})

    const chars = useMemo(() => {
        const heiban = position === 0
        const accentPos = position - 1

        return reading.split("").map((char, i) => {
            if (i === accentPos) {
                return (
                    <span {...styles.wrapper()}>
                        {char}
                        <span {...styles.accentEnd()} />
                    </span>
                )
            }

            if (i !== 0 && (heiban || i < accentPos)) {
                return (
                    <span {...styles.wrapper()}>
                        {char}
                        <span {...styles.accent()} />
                    </span>
                )
            }

            return <span>{char}</span>
        })
    }, [reading, position])

    return <span>{chars}</span>
}

const accentStyles = makeStyles({
    slots: create({
        wrapper: {
            position: "relative",
        },
        accent: {
            position: "absolute",
            borderTopWidth: "2px",
            borderTopStyle: "solid",
            borderTopColor: "#000",
            left: 0,
            right: "-1px",
        },
        accentEnd: {
            position: "absolute",
            borderTopWidth: "2px",
            borderTopStyle: "solid",
            borderTopColor: "#000",
            borderRightWidth: "2px",
            borderRightStyle: "solid",
            borderRightColor: "#000",
            left: 0,
            right: "-1px",
            height: "6px",
        },
    }),
    variants: {},
    defaultVariants: {},
})
