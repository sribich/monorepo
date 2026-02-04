import { Editor, MarkdownView } from "obsidian"

export type Keybind = {
    id: string
    chord: string
    trigger(editor: Editor, view: MarkdownView): void | Promise<void>
}
export type Keymap = Record<string, Keybind>

export enum Modifier {
    Ctrl = "C",
    Shift = "S",
}
export const MODIFIER_VALUES = Object.values(Modifier)
export const MODIFIER_SORT_ORDER = [Modifier.Ctrl, Modifier.Shift]

export type KeyMap = {
    [key: string]: KeyMap | Keybind
}

export type Chord = {
    sequence: Hotkey[]
}

export type Hotkey = {
    stringified: string

    key: string

    ctrl: boolean
    shift: boolean
}

export const sortModifiers = (modifiers: Modifier[]): Modifier[] =>
    modifiers.sort((a, b) => MODIFIER_SORT_ORDER.indexOf(a) - MODIFIER_SORT_ORDER.indexOf(b))
