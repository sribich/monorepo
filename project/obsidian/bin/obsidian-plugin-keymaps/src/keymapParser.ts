import {
    Token,
    apply,
    buildLexer,
    expectEOF,
    expectSingleResult,
    opt,
    rep_sc,
    rule,
    seq,
    tok,
} from "typescript-parsec"

import { Chord, Hotkey, MODIFIER_VALUES, Modifier, sortModifiers } from "./keymap"

enum TokenKind {
    Modifier,
    Key,
    Space,
}

const lexer = buildLexer([
    [true, /^[CMS]-/g, TokenKind.Modifier],
    [false, /^\s+/g, TokenKind.Space],
    [true, /^[a-zA-Z0-9]-?/g, TokenKind.Key],
])

const applyChord = (value: [Modifier[], string[]][]): Chord => {
    const chord: Chord = {
        sequence: [],
    }

    for (const [modifiers, keys] of value) {
        const baseHotkey: Omit<Hotkey, "key" | "stringified"> = {
            ctrl: false,
            shift: false,
        }

        const seenModifiers = new Set<Modifier>()

        for (const modifier of modifiers) {
            if (seenModifiers.has(modifier)) {
                throw new Error(`Duplicate modifier ${modifier}`)
            }

            switch (modifier) {
                case Modifier.Ctrl:
                    baseHotkey.ctrl = true
                    break
                case Modifier.Shift:
                    baseHotkey.shift = true
                    break
            }

            seenModifiers.add(modifier)
        }

        const sortedModifiers = sortModifiers(modifiers).map((it) => it.toString())

        for (const key of keys) {
            const hotkey: Hotkey = {
                ...baseHotkey,
                key,
                stringified: sortedModifiers.concat([key]).join("-"),
            }

            chord.sequence.push(hotkey)
        }
    }

    return chord
}

const MODIFIERS = rule<TokenKind, Modifier[]>()
const KEYS = rule<TokenKind, string[]>()

export const CHORD = rule<TokenKind, Chord>()

MODIFIERS.setPattern(
    apply(rep_sc(opt(tok(TokenKind.Modifier))), (value: (Token<TokenKind> | undefined)[]) => {
        const modifiers = value.filter(Boolean).map((it) => it.text.slice(0, 1))

        modifiers.forEach((it) => {
            if (!MODIFIER_VALUES.includes(it)) {
                throw new Error(`Unknown modifier key ${it}- does not map to any known modifier`)
            }
        })

        return modifiers as Modifier[]
    }),
)

KEYS.setPattern(
    apply(rep_sc(tok(TokenKind.Key)), (value: Token<TokenKind>[]) =>
        value.map((it) => it.text.slice(0, 1)),
    ),
)

CHORD.setPattern(apply(rep_sc(seq(MODIFIERS, KEYS)), applyChord))

export const parseChord = (input: string): Chord => {
    return expectSingleResult(expectEOF(CHORD.parse(lexer.parse(input))))
}
