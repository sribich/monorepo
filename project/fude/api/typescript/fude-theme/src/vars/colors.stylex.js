import { defineVars } from "@stylexjs/stylex"

export const hues = defineVars({
    background: "0",
    surface: "270",

    navigation: "0",

    // oklch(100% 0.50 100)

    // Semantic Colors
    primary: "256",
    success: "155",
    warning: "100",
    danger: "17",
})

export const colors = defineVars({
    //==================
    // Background Colors
    //==================
    background: `oklch(100% 0 ${hues.background})`,
    // backgroundHover: ``,
    // backgroundSelected: ``,

    //===============
    // Surface Colors
    //===============
    surface: `oklch(95% 0.0125 ${hues.surface})`,

    //============
    // Text Colors
    //============

    //==================
    // Navigation Colors
    //==================
    // f9f9f9
    navigationBackground: `oklch(97.5% 0 ${hues.navigation})`,

    //==============
    // Border Colors
    //==============

    //==============
    // Status Colors
    //==============

    //==
    // Background Colors
    //==
    // background: `oklch(100% 0 ${hues.background})`,
    backgroundHover: "oklch(52.25% 0.0266 282.77 / 10%)",

    backgroundSecondary: "oklch(95% 0 0)",
    backgroundSecondaryHover: "oklch(90% 0 0)",

    // Background color for UI elements and components
    uiBackground: "oklch(90% 0.02 275)",

    surfaceRaised: `oklch(90% 0.01, ${hues.surface})`,

    //==
    // Semantic Colors
    //==
    primary: `oklch(55% 0.20 ${hues.primary})`, // Base value
    primaryHover: `oklch(50% 0.15 ${hues.primary})`, // -5% lightness, 3/4 of the chrome value
    primarySelected: `oklch(90% 0.05 ${hues.primary})`, // +40% lightness, 1/3 of the chroma value
    primarySelectedHover: `oklch(85% 0.0375 ${hues.primary})`, // -5% lightness, 3/4 of the chroma value

    success: `oklch(55% 0.20 ${hues.success})`,
    successHover: `oklch(50% 0.15 ${hues.success})`,
    successSelected: `oklch(85% 0.04 ${hues.success})`,
    successSelectedHover: `oklch(80% 0.03 ${hues.success})`,

    warning: `oklch(90% 0.20 ${hues.warning})`,
    warningHover: `oklch(85% 0.20 ${hues.warning})`,
    warningSelected: `oklch(93.7% 0.095 ${hues.warning})`,
    warningSelectedHover: `oklch(88.55% 0.125 ${hues.warning})`,

    danger: `oklch(55% 0.20 ${hues.danger})`,
    dangerHover: `oklch(50% 0.15 ${hues.danger})`,
    dangerSelected: `oklch(90% 0.05 ${hues.danger})`,
    dangerSelectedHover: `oklch(85% 0.0375 ${hues.danger})`,

    // Foreground Colors
    foreground: "oklch(0% 0 0)",
    secondaryForeground: "oklch(52.25% 0.027 282.77)",

    backgroundHoverForeground: "oklch(0% 0 0)",

    primaryForeground: "oklch(100% 0 0)",
    primarySelectedForeground: "oklch(0% 0 0)",
    successForeground: "oklch(100% 0 0)",
    successSelectedForeground: "oklch(100% 0 0)",
    warningForeground: "oklch(0% 0 0)",
    warningSelectedForeground: "oklch(100% 0 0)",
    dangerForeground: "oklch(100% 0 0)",
    dangerSelectedForeground: "oklch(100% 0 0)",

    // Border Colors
    borderUi: "oklch(82.50% 0.02 275)",
    borderUiHover: "oklch(70% 0.02 275)",
    borderUiSelected: "oklch(50% 0.02 275)",
    borderLayout: "oklch(87.50% 0.02 275)",
    borderLayoutHover: "oklch(90% 0.02 275)",

    // UNFIGURED OUT
    focus: "oklch(57.24% 0.19977401174527706 256.57454751341163)",

    // link #1f76c2
    // placeholder text #676879
    // disabled text rgba(50, 51, 56, 0.38)
    // icon color #676879
})

/*
:root {
  --sb-primary-text-color:#323338;

  --sb-dark-background-on-secondary-color:#f6f7fb;
  --sb-text-color-on-primary:#fff;
  --sb-positive-color:#00854d;
  --sb-negative-color:#d83a52;
  --sb-color-purple:#a25ddc;
  --sb-color-winter:#9aadbd;
  --sb-color-sofia_pink:#ff158a;

  --sb-primary-hover-color:#0060b9;
  --sb-secondary-text-color:#676879;

  --sb-icon-color:#676879;
  --sb-text-color-fixed-light:#fff;
  --sb-text-color-fixed-dark:#323338;
  --sb-inverted-color-background:#323338;
  --sb-disabled-background-color:#ecedf5;
  --sb-primary-background-hover-color:hsla(237,8%,44%,.1);
  --sb-dark-background-color:#f6f7fb;
  --sb-brand-color:#5034ff
}
.dark-app-theme {
  --sb-link-color:#69a7ef;
  --sb-primary-background-color:#181b34;
  --sb-primary-text-color:#d5d8df;
  --sb-ui-border-color:#797e93;
  --sb-dark-background-on-secondary-color:#4b4e69;
  --sb-text-color-on-primary:#fff;
  --sb-positive-color:#00854d;
  --sb-negative-color:#d83a52;
  --sb-color-purple:#b57de3;
  --sb-color-winter:#aebdca;
  --sb-color-sofia_pink:#ff44a1;
  --sb-layout-border-color:#4b4e69;
  --sb-secondary-background-color:#30324e;
  --sb-primary-hover-color:#0060b9;
  --sb-secondary-text-color:#9699a6;
  --sb-primary-color:#0073ea;
  --sb-primary-selected-color:#133774;
  --sb-negative-color-selected:#642830;
  --sb-icon-color:#c3c6d4;
  --sb-text-color-fixed-light:#fff;
  --sb-text-color-fixed-dark:#323338;
  --sb-inverted-color-background:#fff;
  --sb-disabled-background-color:#3c3f59;
  --sb-primary-background-hover-color:#4b4e69;
  --sb-dark-background-color:#393b53
}
.black-app-theme {
  --sb-link-color:#69a7ef;
  --sb-primary-background-color:#111;
  --sb-primary-text-color:#eee;
  --sb-ui-border-color:#8d8d8d;
  --sb-dark-background-on-secondary-color:#4b4e69;
  --sb-text-color-on-primary:#fff;
  --sb-positive-color:#00854d;
  --sb-negative-color:#d83a52;
  --sb-color-purple:#b57de3;
  --sb-color-winter:#aebdca;
  --sb-color-sofia_pink:#ff44a1;
  --sb-layout-border-color:#636363;
  --sb-secondary-background-color:#2c2c2c;
  --sb-primary-hover-color:#0060b9;
  --sb-secondary-text-color:#aaa;
  --sb-primary-color:#0073ea;
  --sb-primary-selected-color:#133774;
  --sb-negative-color-selected:#642830;
  --sb-icon-color:#aaa;
  --sb-text-color-fixed-light:#fff;
  --sb-text-color-fixed-dark:#323338;
  --sb-inverted-color-background:#eee;
  --sb-disabled-background-color:#3a3a3a;
  --sb-primary-background-hover-color:#636363;
  --sb-dark-background-color:#2c2c2c
}
*/

// export const color = stylex.defineVars({})

/*


export const semanticColors = {
    light: {
        separator: {
            DEFAULT: "#11111130", // rgba(17, 17, 17, 0.15)
        },
        focus: {
            DEFAULT: color.blue[500],
        },
        neutral: {
            50: "#fafafa",
            100: "#f4f4f5",
            200: "#e4e4e7",
            300: "#d4d4d8",
            400: "#a1a1aa",
            500: "#71717a",
            600: "#52525b",
            700: "#3f3f46",
            800: "#27272a",
            900: "#18181b",
        },
        ///
        default: {
            ...color.zinc,
            foreground: getReadableColor(color.zinc[300]),
            DEFAULT: color.zinc[300],
        },
        primary: {
            ...color.blue,
            foreground: getReadableColor(color.blue[500]),
            DEFAULT: color.blue[500],
        },
        secondary: {
            ...color.purple,
            foreground: getReadableColor(color.purple[500]),
            DEFAULT: color.purple[500],
        },
        success: {
            ...color.green,
            foreground: getReadableColor(color.green[500]),
            DEFAULT: color.green[500],
        },
        warning: {
            ...color.yellow,
            foreground: getReadableColor(color.yellow[500]),
            DEFAULT: color.yellow[500],
        },
        danger: {
            ...color.rose,
            foreground: getReadableColor(color.rose[500]),
            DEFAULT: color.rose[500],
        },
    },
    dark: {
        background: {
            DEFAULT: "#000",
            // DEFAULT: "#191919",
        },
        foreground: {},
        separator: {
            DEFAULT: "#ffffff30", // rgba(255, 255, 255, 0.15)
        },
        focus: {
            DEFAULT: color.blue[500],
        },
        overlay: {},
        content1: {},
        content2: {},
        content3: {},
        content4: {},
        default: {
            ...swapColorValues(color.zinc),
            foreground: getReadableColor(color.zinc[700]),
            DEFAULT: color.zinc[700],
        },
        primary: {
            ...swapColorValues(color.blue),
            foreground: getReadableColor(color.blue[500]),
            DEFAULT: color.blue[500],
        },
        secondary: {
            ...swapColorValues(color.purple),
            foreground: getReadableColor(color.purple[500]),
            DEFAULT: color.purple[500],
        },
        success: {
            ...swapColorValues(color.green),
            foreground: getReadableColor(color.green[500]),
            DEFAULT: color.green[500],
        },
        warning: {
            ...swapColorValues(color.yellow),
            foreground: getReadableColor(color.yellow[500]),
            DEFAULT: color.yellow[500],
        },
        danger: {
            ...swapColorValues(color.rose),
            foreground: getReadableColor(color.rose[500]),
            DEFAULT: color.rose[500],
        },
    },
}


import color from "tailwindcss/colors.js"

import { getReadableColor } from "./rgb.js"

function swapColorValues<T extends Object>(colors: T) {
    const swappedColors = {}
    const keys = Object.keys(colors)
    const length = keys.length

    for (let i = 0; i < length / 2; i++) {
        const key1 = keys[i]
        const key2 = keys[length - 1 - i]

        // @ts-ignore
        swappedColors[key1] = colors[key2]
        // @ts-ignore
        swappedColors[key2] = colors[key1]
    }
    if (length % 2 !== 0) {
        const middleKey = keys[Math.floor(length / 2)]

        // @ts-ignore
        swappedColors[middleKey] = colors[middleKey]
    }

    return swappedColors
}







export const rgbToHsl = (
    rgba: [number, number, number, number],
): { h: number; s: number; l: number; a: number } => {
    const r = rgba[0] / 255
    const g = rgba[1] / 255
    const b = rgba[2] / 255

    const vmax = Math.max(r, g, b)
    const vmin = Math.min(r, g, b)
    const chroma = vmax - vmin

    let hue = 0

    if (chroma !== 0) {
        switch (vmax) {
            case r:
                hue = (g - b) / chroma + (g < b ? 6 : 0)
                break
            case g:
                hue = (b - r) / chroma + 2
                break
            case b:
                hue = (r - g) / chroma + 4
                break
        }
    }

    const lightness = (vmax + vmin) / 2
    const saturation = chroma === 0 ? 0 : chroma / (1 - Math.abs(2 * lightness - 1))

    return {
        h: Math.min(Math.round(hue * 60), 360),
        s: saturation,
        l: lightness,
        a: rgba[3],
    }
}
export const hexToHslString = (color: string): string => {
    const hsl = rgbToHsl(parseHexColor(color))

    return `hsl(${hsl.h}, ${hsl.s * 100}%, ${hsl.l * 100}%)`
}



export const getBestContractColor = (color: string) => {
    const [r, g, b] = parseHexColor(color)

    const vR = r / 255
    const vG = g / 255
    const vB = b / 255

    const luminance = 0.2126 * sRGBtoLin(vR) + 0.7152 * sRGBtoLin(vG) + 0.0722 * sRGBtoLin(vB)
    const perceivedBrightness = luminanceToPerceivedBrightness(luminance)

    return perceivedBrightness > 50 ? "#000" : "#fff"

    // const brightness = (r * 299 + g * 587 + b * 114) / 1000
    // return brightness > 128 ? "#000000" : "#ffffff"
}

const sRGBtoLin = (colorChannel: number) => {
    // Send this function a decimal sRGB gamma encoded color value
    // between 0.0 and 1.0, and it returns a linearized value.

    if (colorChannel <= 0.04045) {
        return colorChannel / 12.92
    } else {
        return Math.pow((colorChannel + 0.055) / 1.055, 2.4)
    }
}

const luminanceToPerceivedBrightness = (luminance: number) => {
    // Send this function a luminance value between 0.0 and 1.0,
    // and it returns L* which is "perceptual lightness"

    if (luminance <= 216 / 24389) {
        // The CIE standard states 0.008856 but 216/24389 is the intent for 0.008856451679036
        return luminance * (24389 / 27) // The CIE standard states 903.3, but 24389/27 is the intent, making 903.296296296296296
    } else {
        return Math.pow(luminance, 1 / 3) * 116 - 16
    }
}

*/

/*
{
  "light-app-theme": {



    "text-color-on-primary": "#ffffff",
    "text-color-on-inverted": "#ffffff",
    "placeholder-color": "#676879",
    "icon-color": "#676879",
    "link-color": "#1f76c2",
    "fixed-light-color": "#ffffff",
    "fixed-dark-color": "#323338",

    "secondary-background-color": "#ffffff",
    "grey-background-color": "#f6f7fb",
    "allgrey-background-color": "#f6f7fb",
    "inverted-color-background": "#323338",
    "disabled-background-color": "#ecedf5",

    "private-color": "#f65f7c",
    "shareable-color": "#a25ddc",


    "color-highlight_blue": "#cce5ff",
    "color-basic_blue": "#0073ea",
    "color-dark_blue": "#0060b9",
    "color-sky_blue": "#aed4fc",
    "color-bazooka": "#f65f7c",
    "color-snow_white": "#ffffff",
    "color-riverstone_gray": "#f6f7fb",
    "color-ui_grey": "#dcdfec",
    "color-wolf_gray": "#c3c6d4",
    "color-asphalt": "#676879",
    "color-mud_black": "#323338",
    "color-success": "#00854d",
    "color-success-hover": "#007038",
    "color-success-highlight": "#bbdbc9",
    "color-olive_green": "#b5cec0",
    "color-error": "#d83a52",
    "color-error-hover": "#b63546",
    "color-error-highlight": "#f4c3cb",
    "color-pinky_red": "#ecb7bf",
    "color-link_color": "#1f76c2",
    "color-surface": "#292f4c",
    "color-grass_green": "#037f4c",
    "color-grass_green-hover": "#116846",
    "color-grass_green-selected": "#81bfa5",
    "color-done-green": "#00c875",
    "color-done-green-hover": "#0f9b63",
    "color-done-green-selected": "#80e3ba",
    "color-bright-green": "#9cd326",
    "color-bright-green-hover": "#7ca32b",
    "color-bright-green-selected": "#cde992",
    "color-saladish": "#cab641",
    "color-saladish-hover": "#9d8f3e",
    "color-saladish-selected": "#e4daa0",
    "color-egg_yolk": "#ffcb00",
    "color-egg_yolk-hover": "#c29e11",
    "color-egg_yolk-selected": "#ffe580",
    "color-working_orange": "#fdab3d",
    "color-working_orange-hover": "#c0873c",
    "color-working_orange-selected": "#fed59e",
    "color-dark-orange": "#ff642e",
    "color-dark-orange-hover": "#c25531",
    "color-dark-orange-selected": "#ffb196",
    "color-peach": "#ffadad",
    "color-peach-hover": "#c2888a",
    "color-peach-selected": "#ffd6d6",
    "color-sunset": "#ff7575",
    "color-sunset-hover": "#c26163",
    "color-sunset-selected": "#ffbaba",
    "color-stuck-red": "#e2445c",
    "color-stuck-red-hover": "#ad3f51",
    "color-stuck-red-selected": "#f0a1ad",
    "color-dark-red": "#bb3354",
    "color-dark-red-hover": "#92334c",
    "color-dark-red-selected": "#dd99a9",
    "color-sofia_pink": "#ff158a",
    "color-sofia_pink-hover": "#c21e71",
    "color-sofia_pink-selected": "#ff8ac4",
    "color-lipstick": "#ff5ac4",
    "color-lipstick-hover": "#c24e9a",
    "color-lipstick-selected": "#fface1",
    "color-bubble": "#faa1f1",
    "color-bubble-hover": "#be80ba",
    "color-bubble-selected": "#fcd0f8",
    "color-purple": "#a25ddc",
    "color-purple-hover": "#8050ab",
    "color-purple-selected": "#d0aeed",
    "color-dark_purple": "#784bd1",
    "color-dark_purple-hover": "#6344a3",
    "color-dark_purple-selected": "#bba5e8",
    "color-berry": "#7e3b8a",
    "color-berry-hover": "#673971",
    "color-berry-selected": "#be9dc4",
    "color-dark_indigo": "#401694",
    "color-dark_indigo-hover": "#3c1f78",
    "color-dark_indigo-selected": "#a08bc9",
    "color-indigo": "#5559df",
    "color-indigo-hover": "#4b4ead",
    "color-indigo-selected": "#aaacef",
    "color-navy": "#225091",
    "color-navy-hover": "#274776",
    "color-navy-selected": "#90a7c8",
    "color-bright-blue": "#579bfc",
    "color-bright-blue-hover": "#4c7cc1",
    "color-bright-blue-selected": "#abcdfd",
    "color-dark-blue": "#0086c0",
    "color-dark-blue-hover": "#0f6d97",
    "color-dark-blue-selected": "#80c2df",
    "color-aquamarine": "#4eccc6",
    "color-aquamarine-hover": "#469e9b",
    "color-aquamarine-selected": "#a6e5e2",
    "color-chili-blue": "#66ccff",
    "color-chili-blue-hover": "#569ec3",
    "color-chili-blue-selected": "#b2e5ff",
    "color-river": "#68a1bd",
    "color-river-hover": "#588095",
    "color-river-selected": "#b3d0de",
    "color-winter": "#9aadbd",
    "color-winter-hover": "#7b8895",
    "color-winter-selected": "#ccd6de",
    "color-explosive": "#c4c4c4",
    "color-explosive-hover": "#98999a",
    "color-explosive-selected": "#e1e1e1",
    "color-american_gray": "#808080",
    "color-american_gray-hover": "#69696a",
    "color-american_gray-selected": "#bfbfbf",
    "color-blackish": "#333333",
    "color-brown": "#7f5347",
    "color-brown-hover": "#684943",
    "color-brown-selected": "#bfa9a3",
    "color-orchid": "#D974B0",
    "color-orchid-hover": "#AE5D8D",
    "color-orchid-selected": "#ECBAD7",
    "color-tan": "#AD967A",
    "color-tan-hover": "#8A7862",
    "color-tan-selected": "#D6CABC",
    "color-sky": "#A1E3F6",
    "color-sky-hover": "#81B6C5",
    "color-sky-selected": "#D0F1FA",
    "color-coffee": "#BD816E",
    "color-coffee-hover": "#976758",
    "color-coffee-selected": "#DEC0B7",
    "color-royal": "#2B76E5",
    "color-royal-hover": "#225EB7",
    "color-royal-selected": "#95BBF2",
    "color-teal": "#175A63",
    "color-teal-hover": "#12484F",
    "color-teal-selected": "#8BACB1",
    "color-lavender": "#BDA8F9",
    "color-lavender-hover": "#9786C7",
    "color-lavender-selected": "#DED4FC",
    "color-steel": "#A9BEE8",
    "color-steel-hover": "#8798BA",
    "color-steel-selected": "#D4DFF4",
    "color-lilac": "#9D99B9",
    "color-lilac-hover": "#7E7A94",
    "color-lilac-selected": "#CECCDC",
    "color-pecan": "#563E3E",
    "color-pecan-hover": "#453232",
    "color-pecan-selected": "#AB9F9F",

  },
  "dark-app-theme": {
    "primary-color": "#0073ea",
    "primary-on-secondary-color": "#0073ea",
    "primary-hover-color": "#0060b9",
    "primary-selected-color": "#133774",
    "primary-selected-hover-color": "#0d2e65",
    "primary-text-color": "#d5d8df",
    "text-color-on-primary": "#ffffff",
    "text-color-on-inverted": "#323338",
    "secondary-text-color": "#9699a6",
    "placeholder-color": "#c3c6d4",
    "icon-color": "#c3c6d4",
    "link-color": "#69a7ef",
    "fixed-light-color": "#ffffff",
    "fixed-dark-color": "#323338",
    "primary-background-color": "#181b34",
    "primary-background-hover-color": "#4b4e69",
    "secondary-background-color": "#30324e",
    "grey-background-color": "#181b34",
    "allgrey-background-color": "#30324e",
    "inverted-color-background": "#ffffff",
    "disabled-background-color": "#3c3f59",
    "positive-color": "#00854d",
    "positive-color-hover": "#007038",
    "positive-color-selected": "#025231",
    "positive-color-selected-hover": "#194733",
    "negative-color": "#d83a52",
    "negative-color-hover": "#b63546",
    "negative-color-selected": "#642830",
    "negative-color-selected-hover": "#5a2027",
    "private-color": "#f65f7c",
    "shareable-color": "#a25ddc",
    "ui-border-color": "#797e93",
    "layout-border-color": "#4b4e69",
    "color-grass_green": "#359970",
    "color-grass_green-hover": "#116846",
    "color-grass_green-selected": "#0f4f43",
    "color-done-green": "#33d391",
    "color-done-green-hover": "#0f9b63",
    "color-done-green-selected": "#0e7358",
    "color-bright-green": "#b0dc51",
    "color-bright-green-hover": "#7ca32b",
    "color-bright-green-selected": "#5c7930",
    "color-saladish": "#d5c567",
    "color-saladish-hover": "#9d8f3e",
    "color-saladish-selected": "#736a3e",
    "color-egg_yolk": "#ffd533",
    "color-egg_yolk-hover": "#c29e11",
    "color-egg_yolk-selected": "#8D751E",
    "color-working_orange": "#fdbc64",
    "color-working_orange-hover": "#c0873c",
    "color-working_orange-selected": "#8c653c",
    "color-dark-orange": "#ff7b4d",
    "color-dark-orange-hover": "#c25531",
    "color-dark-orange-selected": "#8d4134",
    "color-peach": "#ffbdbd",
    "color-peach-hover": "#c2888a",
    "color-peach-selected": "#8d6674",
    "color-sunset": "#ff9191",
    "color-sunset-hover": "#c26163",
    "color-sunset-selected": "#8d4a58",
    "color-stuck-red": "#e8697d",
    "color-stuck-red-hover": "#ad3f51",
    "color-stuck-red-selected": "#7f314b",
    "color-dark-red": "#c95c76",
    "color-dark-red-hover": "#92334c",
    "color-dark-red-selected": "#6b2947",
    "color-sofia_pink": "#ff44a1",
    "color-sofia_pink-hover": "#c21e71",
    "color-sofia_pink-selected": "#8d1a62",
    "color-lipstick": "#ff7bd0",
    "color-lipstick-hover": "#c24e9a",
    "color-lipstick-selected": "#8d3c7f",
    "color-bubble": "#fbb4f4",
    "color-bubble-hover": "#be80ba",
    "color-bubble-selected": "#8b6096",
    "color-purple": "#b57de3",
    "color-purple-hover": "#8050ab",
    "color-purple-selected": "#5f3e8b",
    "color-dark_purple": "#936fda",
    "color-dark_purple-hover": "#6344a3",
    "color-dark_purple-selected": "#4a3586",
    "color-berry": "#6645a9",
    "color-berry-hover": "#673971",
    "color-berry-selected": "#4d2d62",
    "color-dark_indigo": "#401694",
    "color-dark_indigo-hover": "#3c1f78",
    "color-dark_indigo-selected": "#2e1b67",
    "color-indigo": "#777ae5",
    "color-indigo-hover": "#4b4ead",
    "color-indigo-selected": "#383c8d",
    "color-navy": "#4e73a7",
    "color-navy-hover": "#274776",
    "color-navy-selected": "#1f3866",
    "color-bright-blue": "#79affd",
    "color-bright-blue-hover": "#4c7cc1",
    "color-bright-blue-selected": "#395d9b",
    "color-dark-blue": "#339ecd",
    "color-dark-blue-hover": "#0f6d97",
    "color-dark-blue-selected": "#0e527e",
    "color-aquamarine": "#71d6d1",
    "color-aquamarine-hover": "#469e9b",
    "color-aquamarine-selected": "#357580",
    "color-chili-blue": "#85d6ff",
    "color-chili-blue-hover": "#569ec3",
    "color-chili-blue-selected": "#41759d",
    "color-river": "#86b4ca",
    "color-river-hover": "#588095",
    "color-river-selected": "#42607c",
    "color-winter": "#aebdca",
    "color-winter-hover": "#7b8895",
    "color-winter-selected": "#5b667c",
    "color-explosive": "#d0d0d0",
    "color-explosive-hover": "#98999a",
    "color-explosive-selected": "#70717f",
    "color-american_gray": "#999999",
    "color-american_gray-hover": "#69696a",
    "color-american_gray-selected": "#4e505e",
    "color-blackish": "#5c5c5c",
    "color-brown": "#99756c",
    "color-brown-hover": "#684943",
    "color-brown-selected": "#4d3941",
    "color-orchid": "#E190C0",
    "color-orchid-hover": "#B4739A",
    "color-orchid-selected": "#B4739A",
    "color-tan": "#BDAB95",
    "color-tan-hover": "#978977",
    "color-tan-selected": "#716863",
    "color-sky": "#B4E9F8",
    "color-sky-hover": "#90BAC6",
    "color-sky-selected": "#6C8A9A",
    "color-coffee": "#CA9A8B",
    "color-coffee-hover": "#A27B6F",
    "color-coffee-selected": "#795E5D",
    "color-royal": "#5591EA",
    "color-royal-hover": "#4474BB",
    "color-royal-selected": "#375993",
    "color-teal": "#457B82",
    "color-teal-hover": "#376268",
    "color-teal-selected": "#2E4D58",
    "color-lavender": "#CAB9FA",
    "color-lavender-hover": "#A294C8",
    "color-lavender-selected": "#85597B",
    "color-steel": "#BACBED",
    "color-steel-hover": "#95A2BE",
    "color-steel-selected": "#707A95",
    "color-lilac": "#B1ADC7",
    "color-lilac-hover": "#8E8A9F",
    "color-lilac-selected": "#6B697F",
    "color-pecan": "#786565",
    "color-pecan-hover": "#605151",
    "color-pecan-selected": "#4A4148",
    "color-warning": "#FFCB00",
    "color-warning-hover": "#EAAA15",
    "color-warning-select": "#503e02",
    "color-warning-select-hover": "#402f00"
  },
  "black-app-theme": {

    "primary-on-secondary-color": "#0073ea",

    "primary-text-color": "#eeeeee",

    "text-color-on-inverted": "#111111",

    "placeholder-color": "#aaaaaa",
    "icon-color": "#aaaaaa",
    "link-color": "#69a7ef",
    "fixed-light-color": "#ffffff",
    "fixed-dark-color": "#111111",
    >>>"primary-background-color": "#111111",
    "primary-background-hover-color": "#",
    "secondary-background-color": "#2c2c2c",
    "grey-background-color": "#111111",
    "allgrey-background-color": "#2c2c2c",
    "inverted-color-background": "#eeeeee",
    "disabled-background-color": "#3a3a3a",

    "private-color": "#f65f7c",
    "shareable-color": "#a25ddc",

    "color-grass_green": "#359970",
    "color-grass_green-hover": "#116846",
    "color-grass_green-selected": "#0a482e",
    "color-done-green": "#33d391",
    "color-done-green-hover": "#0f9b63",
    "color-done-green-selected": "#096c43",
    "color-bright-green": "#b0dc51",
    "color-bright-green-hover": "#7ca32b",
    "color-bright-green-selected": "#56721b",
    "color-saladish": "#d5c567",
    "color-saladish-hover": "#9d8f3e",
    "color-saladish-selected": "#6d6329",
    "color-egg_yolk": "#ffd533",
    "color-egg_yolk-hover": "#c29e11",
    "color-egg_yolk-selected": "#886e09",
    "color-working_orange": "#fdbc64",
    "color-working_orange-hover": "#c0873c",
    "color-working_orange-selected": "#875e27",
    "color-dark-orange": "#ff7b4d",
    "color-dark-orange-hover": "#c25531",
    "color-dark-orange-selected": "#883a1f",
    "color-peach": "#ffbdbd",
    "color-peach-hover": "#c2888a",
    "color-peach-selected": "#885f5f",
    "color-sunset": "#ff9191",
    "color-sunset-hover": "#c26163",
    "color-sunset-selected": "#884343",
    "color-stuck-red": "#e8697d",
    "color-stuck-red-hover": "#ad3f51",
    "color-stuck-red-selected": "#792a36",
    "color-dark-red": "#c95c76",
    "color-dark-red-hover": "#92334c",
    "color-dark-red-selected": "#662232",
    "color-sofia_pink": "#ff44a1",
    "color-sofia_pink-hover": "#c21e71",
    "color-sofia_pink-selected": "#88134d",
    "color-lipstick": "#ff7bd0",
    "color-lipstick-hover": "#c24e9a",
    "color-lipstick-selected": "#88356a",
    "color-bubble": "#fbb4f4",
    "color-bubble-hover": "#be80ba",
    "color-bubble-selected": "#855981",
    "color-purple": "#b57de3",
    "color-purple-hover": "#8050ab",
    "color-purple-selected": "#593776",
    "color-dark_purple": "#936fda",
    "color-dark_purple-hover": "#6344a3",
    "color-dark_purple-selected": "#442e71",
    "color-berry": "#9862a1",
    "color-berry-hover": "#673971",
    "color-berry-selected": "#47264d",
    "color-dark_indigo": "#6645a9",
    "color-dark_indigo-hover": "#3c1f78",
    "color-dark_indigo-selected": "#291452",
    "color-indigo": "#777ae5",
    "color-indigo-hover": "#4b4ead",
    "color-indigo-selected": "#333578",
    "color-navy": "#4e73a7",
    "color-navy-hover": "#274776",
    "color-navy-selected": "#193151",
    "color-bright-blue": "#79affd",
    "color-bright-blue-hover": "#4c7cc1",
    "color-bright-blue-selected": "#345686",
    "color-dark-blue": "#339ecd",
    "color-dark-blue-hover": "#0f6d97",
    "color-dark-blue-selected": "#094b69",
    "color-aquamarine": "#71d6d1",
    "color-aquamarine-hover": "#469e9b",
    "color-aquamarine-selected": "#2f6e6b",
    "color-chili-blue": "#85d6ff",
    "color-chili-blue-hover": "#569ec3",
    "color-chili-blue-selected": "#3b6e88",
    "color-river": "#86b4ca",
    "color-river-hover": "#588095",
    "color-river-selected": "#3c5967",
    "color-winter": "#aebdca",
    "color-winter-hover": "#7b8895",
    "color-winter-selected": "#555f67",
    "color-explosive": "#d0d0d0",
    "color-explosive-hover": "#98999a",
    "color-explosive-selected": "#6a6a6a",
    "color-american_gray": "#999999",
    "color-american_gray-hover": "#69696a",
    "color-american_gray-selected": "#494949",
    "color-blackish": "#5c5c5c",
    "color-brown": "#99756c",
    "color-brown-hover": "#684943",
    "color-brown-selected": "#48322c",
    "color-orchid": "#e190c0",
    "color-orchid-hover": "#b4739a",
    "color-orchid-selected": "#7e516c",
    "color-tan": "#bdab95",
    "color-tan-hover": "#978977",
    "color-tan-selected": "#6a6053",
    "color-sky": "#b4e9f8",
    "color-sky-hover": "#90bac6",
    "color-sky-selected": "#65828b",
    "color-coffee": "#ca9a8b",
    "color-coffee-hover": "#a27b6f",
    "color-coffee-selected": "#71564e",
    "color-royal": "#5591ea",
    "color-royal-hover": "#4474bb",
    "color-royal-selected": "#305183",
    "color-teal": "#457b82",
    "color-teal-hover": "#376268",
    "color-teal-selected": "#274549",
    "color-lavender": "#cab9fa",
    "color-lavender-hover": "#a294c8",
    "color-lavender-selected": "#71688c",
    "color-steel": "#bacbed",
    "color-steel-hover": "#95a2be",
    "color-steel-selected": "#687185",
    "color-lilac": "#687185",
    "color-lilac-hover": "#8e8a9f",
    "color-lilac-selected": "#63616f",
    "color-pecan": "#786565",
    "color-pecan-hover": "#605151",
    "color-pecan-selected": "#433939",
    "color-warning": "#FFCB00",
    "color-warning-hover": "#EAAA15",
    "color-warning-select": "#503e02",
    "color-warning-select-hover": "#402f00"
  },
  "hacker_theme-app-theme": {
    "primary-color": "#fe78c6",
    "primary-on-secondary-color": "#fe78c6",
    "primary-hover-color": "#fe5ab9",
    "primary-hover-on-secondary-color": "#fe5ab9",
    "primary-selected-color": "#9f4077",
    "primary-selected-color-rgb": "#9f4077",
    "primary-selected-hover-color": "#0d2e65",
    "primary-selected-on-secondary-color": "#9f4077",
    "primary-text-color": "#d5d8df",
    "primary-text-on-secondary-color": "#d5d8df",
    "fixed-light-color": "#ffffff",
    "fixed-dark-color": "#323338",
    "primary-background-color": "#282a36",
    "primary-background-color-rgb": "#282a36",
    "primary-background-hover-color": "#4b4e69",
    "primary-background-hover-on-secondary-color": "#4b4e69",
    "grey-background-color": "#282a36",
    "allgrey-background-color": "#282a36",
    "inverted-color-background": "#ffffff",
    "text-color-on-inverted": "#323338",
    "modal-background-color": "#282a36",
    "color-error": "#ff5555",
    "color-success": "#50fa7b",
    "positive-color": "#00854d",
    "positive-color-hover": "#007038",
    "positive-color-selected": "#26503e",
    "positive-color-selected-hover": "#194733",
    "negative-color": "#d83a52",
    "negative-color-hover": "#b63546",
    "negative-color-selected": "#642830",
    "negative-color-selected-hover": "#5a2027",
    "private-color": "#f65f7c",
    "shareable-color": "#a358df",
    "ui-border-color": "#797e93",
    "ui-border-on-secondary-color": "#797e93",
    "layout-border-color": "#414458",
    "layout-border-on-secondary-color": "#414458",
    "placeholder-color": "#c3c6d4",
    "placeholder-on-secondary-color": "#c3c6d4",
    "icon-color": "#c3c6d4",
    "icon-on-secondary-color": "#c3c6d4",
    "disabled-background-color": "#3a3a3a",
    "disabled-background-on-secondary-color": "#3a3a3a",
    "dark-background-color": "#303241",
    "dark-background-on-secondary-color": "#595959",
    "secondary-background-color": "#30324e",
    "secondary-background-color-rgb": "#30324e",
    "dialog-background-color": "#30324e",
    "label-background-color": "#404b69",
    "label-background-on-secondary-color": "#404b69",
    "secondary-text-color": "#9699a6",
    "secondary-text-on-secondary-color": "#9699a6",
    "link-color": "#bd93f9",
    "link-on-secondary-color": "#bd93f9",
    "color-grass_green": "#359970",
    "color-grass_green-hover": "#116846",
    "color-grass_green-selected": "#0a482e",
    "color-done-green": "#33d391",
    "color-done-green-hover": "#0f9b63",
    "color-done-green-selected": "#096c43",
    "color-bright-green": "#b0dc51",
    "color-bright-green-hover": "#7ca32b",
    "color-bright-green-selected": "#56721b",
    "color-saladish": "#d5c567",
    "color-saladish-hover": "#9d8f3e",
    "color-saladish-selected": "#6d6329",
    "color-egg_yolk": "#ffd533",
    "color-egg_yolk-hover": "#c29e11",
    "color-egg_yolk-selected": "#886e09",
    "color-working_orange": "#fdbc64",
    "color-working_orange-hover": "#c0873c",
    "color-working_orange-selected": "#875e27",
    "color-dark-orange": "#ff7b4d",
    "color-dark-orange-hover": "#c25531",
    "color-dark-orange-selected": "#883a1f",
    "color-peach": "#ffbdbd",
    "color-peach-hover": "#c2888a",
    "color-peach-selected": "#885f5f",
    "color-sunset": "#ff9191",
    "color-sunset-hover": "#c26163",
    "color-sunset-selected": "#884343",
    "color-stuck-red": "#e8697d",
    "color-stuck-red-hover": "#ad3f51",
    "color-stuck-red-selected": "#792a36",
    "color-dark-red": "#c95c76",
    "color-dark-red-hover": "#92334c",
    "color-dark-red-selected": "#662232",
    "color-sofia_pink": "#ff44a1",
    "color-sofia_pink-hover": "#c21e71",
    "color-sofia_pink-selected": "#88134d",
    "color-lipstick": "#ff7bd0",
    "color-lipstick-hover": "#c24e9a",
    "color-lipstick-selected": "#88356a",
    "color-bubble": "#fbb4f4",
    "color-bubble-hover": "#be80ba",
    "color-bubble-selected": "#855981",
    "color-purple": "#b57de3",
    "color-purple-hover": "#8050ab",
    "color-purple-selected": "#593776",
    "color-dark_purple": "#936fda",
    "color-dark_purple-hover": "#6344a3",
    "color-dark_purple-selected": "#442e71",
    "color-berry": "#9862a1",
    "color-berry-hover": "#673971",
    "color-berry-selected": "#47264d",
    "color-dark_indigo": "#6645a9",
    "color-dark_indigo-hover": "#3c1f78",
    "color-dark_indigo-selected": "#291452",
    "color-indigo": "#777ae5",
    "color-indigo-hover": "#4b4ead",
    "color-indigo-selected": "#333578",
    "color-navy": "#4e73a7",
    "color-navy-hover": "#274776",
    "color-navy-selected": "#193151",
    "color-bright-blue": "#79affd",
    "color-bright-blue-hover": "#4c7cc1",
    "color-bright-blue-selected": "#345686",
    "color-dark-blue": "#339ecd",
    "color-dark-blue-hover": "#0f6d97",
    "color-dark-blue-selected": "#094b69",
    "color-aquamarine": "#71d6d1",
    "color-aquamarine-hover": "#469e9b",
    "color-aquamarine-selected": "#2f6e6b",
    "color-chili-blue": "#85d6ff",
    "color-chili-blue-hover": "#569ec3",
    "color-chili-blue-selected": "#3b6e88",
    "color-river": "#86b4ca",
    "color-river-hover": "#588095",
    "color-river-selected": "#3c5967",
    "color-winter": "#aebdca",
    "color-winter-hover": "#7b8895",
    "color-winter-selected": "#555f67",
    "color-explosive": "#d0d0d0",
    "color-explosive-hover": "#98999a",
    "color-explosive-selected": "#6a6a6a",
    "color-american_gray": "#999999",
    "color-american_gray-hover": "#69696a",
    "color-american_gray-selected": "#494949",
    "color-blackish": "#5c5c5c",
    "color-brown": "#99756c",
    "color-brown-hover": "#684943",
    "color-brown-selected": "#48322c",
    "color-orchid": "#e190c0",
    "color-orchid-hover": "#b4739a",
    "color-orchid-selected": "#7e516c",
    "color-tan": "#bdab95",
    "color-tan-hover": "#978977",
    "color-tan-selected": "#6a6053",
    "color-sky": "#b4e9f8",
    "color-sky-hover": "#90bac6",
    "color-sky-selected": "#65828b",
    "color-coffee": "#ca9a8b",
    "color-coffee-hover": "#a27b6f",
    "color-coffee-selected": "#71564e",
    "color-royal": "#5591ea",
    "color-royal-hover": "#4474bb",
    "color-royal-selected": "#305183",
    "color-teal": "#457b82",
    "color-teal-hover": "#376268",
    "color-teal-selected": "#274549",
    "color-lavender": "#cab9fa",
    "color-lavender-hover": "#a294c8",
    "color-lavender-selected": "#71688c",
    "color-steel": "#bacbed",
    "color-steel-hover": "#95a2be",
    "color-steel-selected": "#687185",
    "color-lilac": "#687185",
    "color-lilac-hover": "#8e8a9f",
    "color-lilac-selected": "#63616f",
    "color-pecan": "#786565",
    "color-pecan-hover": "#605151",
    "color-pecan-selected": "#433939",
    "color-warning": "#FFCB00",
    "color-warning-hover": "#EAAA15",
    "color-warning-select": "#503e02",
    "color-warning-select-hover": "#402f00"
  }
}
*/
