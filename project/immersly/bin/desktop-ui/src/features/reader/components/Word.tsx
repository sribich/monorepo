import {
    Dialog,
    DialogTrigger,
    MultiProvider,
    makeStyles,
    mergeProps,
    Popover,
    useStyles,
} from "@sribich/fude"
import { create } from "@stylexjs/stylex"
import { use, useCallback, useRef } from "react"
import { Pressable, useHover } from "react-aria"
import { KnownWords } from "../../../context/knownWords"
import { getWord } from "../../../generated/rpc-client/dictionary_GetWord"
import { createCard } from "../../../generated/rpc-client/scheduler_createCard"
import { DefinitionLookup } from "../../dictionary/pages/DictionaryLookupPage"
import { AudioRefContext } from "../context/AudioRefContext"

export namespace Word {
    export interface Props {
        bookId: string
        word: string
        wordTs: [number, number]
        dictWord: string
        sentence: string
        sentenceAudio: string
        sentenceTs: [number, number]
        freq: number | null
    }
}

export const Word = (props: Word.Props) => {
    const { mutateAsync } = createCard(["add_card", props.dictWord])
    // const { mutateAsync } = addCard(["add_card", props.dictWord])

    const onClick = useCallback(
        function onC(reading: string, speaker: string) {
            console.log(reading, speaker, props)

            mutateAsync({
                // bookId: props.bookId,

                word: props.dictWord,

                reading,
                readingAudio: speaker === "" ? undefined : speaker,

                sentence: props.sentence,
                sentenceAudio: props.sentenceAudio,
                sentenceTimestamp: props.sentenceTs,

                /*
            book: props.bookId,
            word: props.dictWord,
            reading: props.dictWord,

            definition,
            definitionNative,
            */
            })
        },
        [mutateAsync],
    )

    const audioRef = use(AudioRefContext)
    const triggerRef = useRef(null)

    const { hoverProps, isHovered } = useHover({})

    const { styles } = useStyles(wordStyles, {})

    //    const toggleAudio = use(AudioToggleContext)

    const jumper =
        props.data === 0 ? (
            <button
                style={{ height: "4px", width: "4px" }}
                onClick={() => {
                    audioRef.current.seek(props.sentenceTs[0] / 1000 - 1)
                }}
            >
                .
            </button>
        ) : (
            <></>
        )

    return (
        <MultiProvider
            values={
                [
                    // [DialogContext, overlayProps],
                    // [PopoverContext, { triggerRef }],
                ]
            }
        >
            {jumper}
            <DialogTrigger>
                <Pressable>
                    <span
                        {...mergeProps(
                            hoverProps,
                            styles.span(isHovered && styles.span.hovered),
                            // props,
                        )}
                        role="button"
                        tabIndex={0}
                    >
                        <FreqWord word={props.word} dictWord={props.dictWord} freq={props.freq} />
                    </span>
                </Pressable>
                <Popover>
                    <Dialog>
                        <div padding="2" shadow="md" rounded="md" {...styles.container()}>
                            <DefinitionLookup addCard={onClick} searchTerm={props.dictWord} />
                        </div>
                    </Dialog>
                </Popover>
            </DialogTrigger>
        </MultiProvider>
    )
    /*
    <div>
                            {props.word} ({props.dictWord})
                        </div>
                        <div>{props.sentence}</div>
                        <div>{JSON.stringify(props.sentenceTs)}</div>
                        <div>{JSON.stringify(props.wordTs)}</div>
*/

    // <Input value={definition} placeholder="definition tl" onChange={(e) => setDefinition(e.target.value)} />
    // <Input value={definitionNative} placeholder="definition native" onChange={(e) => setDefinitionNative(e.target.value)} />
}

const FreqWord = (props) => {
    const knownWords = use(KnownWords)

    // if (props.word === "見る") {}

    if (!props.freq || props.dictWord in knownWords[1]) {
        return <span style={{ backgroundColor: "#cdcdcd20" }}>{props.word}</span>
    }

    if (props.freq < knownWords[0] * 1.5) {
        return <span style={{ backgroundColor: "#00ff0020" }}>{props.word}</span>
    }

    return <span style={{ backgroundColor: "#ff000010" }}>{props.word}</span>

    // return <span style={{ backgroundColor: "#ff000020" }}>{props.word}</span>

    /*
                    {props.word in knownWords[1] ? <span>yes</span> : <span>no</span>}
                {props.word}
                    </FreqWord>
    */
}

const Definition = (props) => {
    const { data } = getWord([props.word], { word: props.word })

    let item = (data?.definitions ?? []).map((it) => (
        <p dangerouslySetInnerHTML={{ __html: it }}></p>
    ))

    return <p>{item}</p>
}

export const wordStyles = makeStyles({
    slots: create({
        span: {
            fontFamily: "Shippori Mincho",
            fontSize: 18,
            whiteSpace: "nowrap",
        },
        container: {
            maxWidth: "600px",
            maxHeight: "400px",
            overflow: "scroll",
            borderRadius: "12px",
            padding: "8px",
            boxShadow:
                "0px 0px 5px 0px rgb(0 0 0 / 0.02), 0px 2px 10px 0px rgb(0 0 0 / 0.06), 0px 0px 1px 0px rgb(0 0 0 / 0.3)",
            backgroundColor: "#fff",
        },
    }),
    modifiers: {
        hovered: create({
            span: {
                backgroundColor: "#cdcdcd",
            },
        }),
    },
    variants: {},
    defaultVariants: {},
})
