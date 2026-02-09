import "./vars/fonts.stylex"
import "./vars/colors.stylex.js"
import "./vars/shadow.stylex.js"
import "./vars/sizing.stylex.js"
import "./vars/zindex.stylex.js"
import "./vars/spacing.stylex.js"
import "./vars/fontSize.stylex.js"
import "./vars/boxShadow.stylex.js"
import "./vars/lineHeight.stylex.js"
import "./vars/borderWidth.stylex.js"
import "./vars/borderRadius.stylex.js"

import "./markers.stylex.js"

import { createTheme } from "@stylexjs/stylex"
import { colors } from "./vars/colors.stylex.js"

export * from "./fonts.js"

export const darkColors = {
    // Background Colors
    background: "#111111",
    backgroundHover: "#636363",
    backgroundSecondary: "#2c2c2c",
    backgroundSecondaryHover: "#7e7e7e",

    // Semantic Colors
    primary: "#0073ea",
    primaryHover: "#0060b9",
    primarySelected: "#133774",
    primarySelectedHover: "#0d2e65",
    success: "#00854d",
    successHover: "#017038",
    successSelected: "#015231",
    successSelectedHover: "#194733",
    warning: "#ffcb04",
    warningHover: "#eaaa16",
    warningSelected: "#fceba1",
    warningSelectedHover: "#f2d973",
    danger: "#d83a52",
    dangerHover: "#b63546",
    dangerSelected: "#642830",
    dangerSelectedHover: "#5a2027",

    // Foreground Colors
    foreground: "#ffffff",
    secondaryForeground: "#aaaaaa",
    backgroundHoverForeground: "#ffffff",
    primaryForeground: "#ffffff",
    primarySelectedForeground: "#000000",
    successForeground: "#ffffff",
    successSelectedForeground: "#ffffff",
    warningForeground: "#000000",
    warningSelectedForeground: "#ffffff",
    dangerForeground: "#ffffff",
    dangerSelectedForeground: "#ffffff",

    // Border Colors
    borderUi: "#8d8d8d",
    borderLayout: "#636363",

    // TODO
    focus: "#0073ea",
}

export const lightTheme = createTheme(colors, colors)
export const darkTheme = createTheme(colors, darkColors)
