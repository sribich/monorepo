import { createTheme } from "@stylexjs/stylex"

import { fonts } from "./vars/fonts.stylex.js"

export const font = createTheme(fonts, fonts)

export const fontSerif = createTheme(fonts, {
    display: '"Noto Serif Display", serif',
    default: '"Noto Serif", serif',
})

export const fontJapanese = createTheme(fonts, {
    display: '"Noto Sans Japanese", "Noto Sans", serif',
    default: '"Noto Sans Japanese", "Noto Sans", serif',
})

export const fontJapaneseSerif = createTheme(fonts, {
    display: '"Noto Serif Japanese", "Noto Serif", serif',
    default: '"Noto Serif Japanese", "Noto Serif", serif',
})

export const fontThemes = {
    en: {
        sans: font,
        serif: fontSerif,
    },
    jp: {
        sans: fontJapanese,
        serif: fontJapaneseSerif,
    },
}
