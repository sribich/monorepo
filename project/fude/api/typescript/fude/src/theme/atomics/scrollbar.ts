import { create } from "@stylexjs/stylex"

export const { hideScrollbar, defaultScrollbar } = create({
    hideScrollbar: {
        scrollbarWidth: "none",
        MsOverflowStyle: "none",
        "::-webkit-scrollbar": {
            display: "none",
        },
    },
    defaultScrollbar: {
        scrollbarWidth: "auto",
        MsOverflowStyle: "auto",
        "::-webkit-scrollbar": {
            display: "block",
        },
    },
})
