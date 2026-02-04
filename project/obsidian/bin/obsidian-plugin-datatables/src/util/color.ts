// import * as colors from "@ant-design/colors"

/*
import colors from "tailwindcss/colors"

export const colorMap = {
    red: colors.red.primary, // #f5222d
    volcano: colors.volcano.primary, // #fa541c
    gold: colors.gold.primary, // #faad14
    orange: colors.orange.primary, // #fa8c16
    yellow: colors.yellow.primary, // #fadb14
    lime: colors.lime.primary, // #a0d911
    green: colors.green.primary, // #52c41a
    cyan: colors.cyan.primary, // #13c2c2
    blue: colors.blue.primary, // #1677ff
    geekblue: colors.geekblue.primary, // #2f54eb
    purple: colors.purple.primary, // #722ed1
    magenta: colors.magenta.primary, // #eb2f96
    grey: colors.grey.primary, // #666666
} as const

export const functionalColor = {
    success: colorMap.green,
    warning: colorMap.gold,
    error: colorMap.red,

    inProgress: colorMap.green,
}

const contrastCache = new Map<string, string>()

/**
 * https://stackoverflow.com/a/56678483
 *
export const getBestContractColor = (backgroundColor: string) => {
    if (contrastCache.has(backgroundColor)) {
        return contrastCache.get(backgroundColor)
    }

    const rgb = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(backgroundColor)
    const r = parseInt(rgb?.[1] ?? "00", 16)
    const g = parseInt(rgb?.[2] ?? "00", 16)
    const b = parseInt(rgb?.[3] ?? "00", 16)

    const vR = r / 255
    const vG = g / 255
    const vB = b / 255

    const luminance = 0.2126 * sRGBtoLin(vR) + 0.7152 * sRGBtoLin(vG) + 0.0722 * sRGBtoLin(vB)
    const perceivedBrightness = luminanceToPerceivedBrightness(luminance)

    contrastCache.set(backgroundColor, perceivedBrightness > 50 ? "#000000" : "#ffffff")

    return contrastCache.get(backgroundColor)

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

export const rainbowColor = (numSteps: number, step: number) => {
    // This function generates vibrant, "evenly spaced" colours (i.e. no clustering). This is ideal for creating easily distinguishable vibrant markers in Google Maps and other apps.
    // Adam Cole, 2011-Sept-14
    // HSV to RBG adapted from: http://mjijackson.com/2008/02/rgb-to-hsl-and-rgb-to-hsv-color-model-conversion-algorithms-in-javascript
    let r = 0
    let g = 0
    let b = 0
    const h = step / numSteps
    const i = ~~(h * 6)
    const f = h * 6 - i
    const q = 1 - f
    switch (i % 6) {
        case 0:
            r = 1
            g = f
            b = 0
            break
        case 1:
            r = q
            g = 1
            b = 0
            break
        case 2:
            r = 0
            g = 1
            b = f
            break
        case 3:
            r = 0
            g = q
            b = 1
            break
        case 4:
            r = f
            g = 0
            b = 1
            break
        case 5:
            r = 1
            g = 0
            b = q
            break
    }

    return (
        "#" +
        ("00" + (~~(r * 255)).toString(16)).slice(-2) +
        ("00" + (~~(g * 255)).toString(16)).slice(-2) +
        ("00" + (~~(b * 255)).toString(16)).slice(-2)
    )
}
