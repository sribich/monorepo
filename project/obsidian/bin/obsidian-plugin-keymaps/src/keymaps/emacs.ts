import { Editor, MarkdownView } from "obsidian"

import { Keymap } from "../keymap"

export const emacs = {
    moveUp: {
        id: "emacs.move-down",
        chord: "C-p",
        trigger: (editor: Editor) => {
            const currentCursor = editor.getCursor()
            const nextLine = editor.getLine(currentCursor.line - 1)

            if (nextLine.length < currentCursor.ch) {
                currentCursor.ch = nextLine.length
            }

            editor.setCursor(currentCursor.line - 1, currentCursor.ch)
        },
    },
    moveDown: {
        id: "emacs.move-up",
        chord: "C-n",
        trigger: (editor: Editor) => {
            const currentCursor = editor.getCursor()
            const nextLine = editor.getLine(currentCursor.line + 1)

            if (nextLine.length < currentCursor.ch) {
                currentCursor.ch = nextLine.length
            }

            editor.setCursor(currentCursor.line + 1, currentCursor.ch)
        },
    },
    save: {
        id: "emacs.save",
        chord: "C-x C-s",
        trigger: async (_: Editor, view: MarkdownView) => {
            await view.save()
        },
    },
} satisfies Keymap
