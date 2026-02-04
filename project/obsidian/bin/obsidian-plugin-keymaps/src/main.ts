import "@total-typescript/ts-reset"
import { MarkdownView, Plugin } from "obsidian"

import { KeyMap, Keybind, Keymap, Modifier, sortModifiers } from "./keymap"
import { parseChord } from "./keymapParser"
import { emacs } from "./keymaps/emacs"

export default class BasaltKeymapPlugin extends Plugin {
    private chords: KeyMap = {}
    private currentChordString: string
    private currentChord: KeyMap

    private statusBar: HTMLElement

    override async onload() {
        this.statusBar = this.addStatusBarItem()

        this.registerKeybinds()
        this.registerDomEvent(window, "keydown", this.keyDown.bind(this), { capture: true })

        console.log(`[${this.manifest.id}] loaded version ${this.manifest.version}`)
    }

    private async registerKeybinds() {
        const keybinds = Object.values(emacs)

        for (const keybind of keybinds) {
            this.registerKeybind(keybind)
        }

        this.currentChord = this.chords
        this.currentChordString = ""
    }

    private keyDown(event: KeyboardEvent) {
        const view = app.workspace.getActiveViewOfType(MarkdownView)
        const editor = view?.editor

        if (!view || !editor) {
            return
        }

        const modifiers: Modifier[] = []
        event.ctrlKey && modifiers.push(Modifier.Ctrl)
        event.shiftKey && modifiers.push(Modifier.Shift)

        const sortedModifiers = sortModifiers(modifiers) as string[]

        const chord = sortedModifiers.concat([event.key]).join("-")

        if (!this.currentChord[chord]) {
            this.currentChord = this.chords
            return
        }

        const nextChord = this.currentChord[chord]

        if (!nextChord) {
            this.currentChord = this.chords
            return
        }

        if ("trigger" in nextChord && nextChord["trigger"] instanceof Function) {
            nextChord.trigger(editor, view)

            event.preventDefault()
            event.stopPropagation()

            this.currentChord = this.chords
            this.currentChordString = ""

            this.updateStatus()

            return
        }

        this.currentChord = nextChord as Keymap
        this.currentChordString = [this.currentChordString, chord].filter(Boolean).join(" ")

        this.updateStatus()

        event.preventDefault()
        event.stopPropagation()
    }

    private updateStatus() {
        this.statusBar.setText(`M-x: ${this.currentChordString}`)
    }

    private registerKeybind({ id, chord, trigger }: Keybind) {
        let keyMap = this.chords as KeyMap

        for (const sequence of parseChord(chord).sequence) {
            const currentMap = (keyMap[sequence.stringified] ??= {})

            if ("trigger" in currentMap) {
                throw new Error(`Found a trigger while walking the chord tree.`)
            }

            keyMap = currentMap
        }

        if (Object.keys(keyMap).length > 0) {
            throw new Error(`Attempted to register a chord on a non-leaf node.`)
        }

        Object.assign(keyMap, {
            id,
            trigger,
        })
    }
}
