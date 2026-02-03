import { makeStyles, useStyleProps, useStyles, type StyleProps } from "@sribich/fude"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { create } from "@stylexjs/stylex"
import { useMemo } from "react"
import type { Accent } from "../../../generated/rpc-client/dictionary_GetExactWord"

//==============================================================================
// Accents
//==============================================================================
export namespace Accents {
    export interface Props {
        accents: Accent[]
    }
}

export const AccentList = ({ accents }: Accents.Props) => {
    const { styles } = useStyles(accentsStyles, {})

    const partitionedAccents = useMemo(() => {
        return Object.entries(Object.groupBy(accents, ({ dictionary }) => dictionary)).map(
            ([dictionary, accent]) => {
                const components = accent.map((accent) => (
                    <li key={accent.id}>
                        <PitchAccent word={accent.reading} position={accent.accent} />
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
// PitchAccentItems
//==============================================================================
export namespace PitchAccentItems {
    export interface Props extends StyleProps {
        items: PitchAccent.Props[]
    }
}

export const PitchAccentItems = (props: PitchAccentItems.Props) => {
    const styleProps = useStyleProps(props, {})

    const pitchAccentComponents = props.items.map(({ word, position }) => (
        <PitchAccent key={position} word={word} position={position} />
    ))

    return <div {...styleProps}>{pitchAccentComponents}</div>
}

//==============================================================================
// PitchAccent
//==============================================================================
export namespace PitchAccent {
    export interface Props {
        word: string
        position: number
    }
}

export const PitchAccent = ({ word, position }: PitchAccent.Props) => {
    const { styles } = useStyles(pitchAccentStyles, {})

    const chars = useMemo(() => {
        const heiban = position === 0
        const accentPos = position - 1

        return word.split("").map((char, i) => {
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
    }, [word, position])

    return <span>{chars}</span>
}

const pitchAccentStyles = makeStyles({
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
            left: 0,
            right: "-1px",
            height: "6px",
        },
    }),
    variants: {},
    defaultVariants: {},
})
