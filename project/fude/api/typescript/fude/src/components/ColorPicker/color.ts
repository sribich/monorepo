/**
 * https://stackoverflow.com/questions/3423214/convert-hsb-hsv-color-to-hsl
 * https://stackoverflow.com/questions/2353211/hsl-to-rgb-color-conversion
 * https://stackoverflow.com/questions/3423214/convert-hsb-hsv-color-to-hsl
 * https://github.com/vinaypillai/ac-colors/blob/master/index.js
 * https://css-tricks.com/converting-color-spaces-in-javascript/
 * https://medium.com/innovaccer-design/rgb-vs-hsb-vs-hsl-demystified-1992d7273d3a
 */
export type HexColor = { kind: "hex"; value: string }
export type RgbColor = { kind: "rgb"; value: { r: number; g: number; b: number; a?: number } }
export type HsbColor = { kind: "hsb"; value: { h: number; s: number; b: number; a?: number } }
export type HslColor = { kind: "hsl"; value: { h: number; s: number; l: number; a?: number } }

export type Colors = HexColor | RgbColor | HsbColor | HslColor

export type Color = Colors["value"]

const getColor = (color: Color) => {
    if (typeof color === "string") {
        return { kind: "hex", value: color } as HexColor
    } else if ("r" in color) {
        return { kind: "rgb", value: color } as RgbColor
    } else if ("l" in color) {
        return { kind: "hsl", value: color } as HslColor
    } else {
        return { kind: "hsb", value: color } as HsbColor
    }
}

export const convertToHsb = <TColor extends Colors["value"]>(
    originalColor: TColor,
): HsbColor["value"] => {
    const color = getColor(originalColor)

    switch (color.kind) {
        case "hex":
            return hslToHsb(hexToHsl(color.value))
        case "rgb":
            return hslToHsb(rgbToHsl(color.value))
        case "hsb":
            return color.value
        case "hsl":
            return hslToHsb(color.value)
    }
}

const hexToHsl = (hex: HexColor["value"]): HslColor["value"] => {
    const parsed = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})?$/i.exec(hex)

    if (!parsed) {
        console.error(`Unable to parse ${hex} into an rgba value.`)

        return { h: 0, s: 0, l: 0, a: 0 }
    }

    const rgba = {
        r: parseInt(parsed[1] ?? "00", 16),
        g: parseInt(parsed[2] ?? "00", 16),
        b: parseInt(parsed[3] ?? "00", 16),
        a: parseInt(parsed[4] ?? "ff", 16),
    }

    return rgbToHsl(rgba)
}

const rgbToHsl = (rgba: RgbColor["value"]): HslColor["value"] => {
    const r = rgba.r / 255
    const g = rgba.g / 255
    const b = rgba.b / 255

    const vmax = Math.max(r, g, b)
    const vmin = Math.min(r, g, b)

    let h = (vmax + vmin) / 2
    let s = h
    const l = h

    if (vmax === vmin) {
        return { h: 0, s: 0, l, a: (rgba.a ?? 255) / 255 }
    }

    const delta = vmax - vmin

    s = l > 0.5 ? 2 - vmax - vmin : delta / (vmax + vmin)

    if (vmax === r) h = (g - b) / delta + (g < b ? 6 : 0)
    if (vmax === g) h = (b - r) / delta + 2
    if (vmax === b) h = (r - g) / delta + 4

    h /= 6

    return { h, s: s, l, a: (rgba.a ?? 255) / 255 }
}

export const hsbToHsl = (hsb: HsbColor["value"]): HslColor["value"] => {
    const { h, s, b } = hsb

    const l = b - (b * s) / 2
    const m = Math.min(l, 1 - l)

    return {
        h,
        s: m ? (b - l) / m : 0,
        l,
        a: hsb.a ?? 1,
    }
}

const hslToHsb = (hsl: HslColor["value"]): HsbColor["value"] => {
    const { h, s, l } = hsl

    const b = s * Math.min(l, 1 - l) + l

    return {
        h,
        s: b ? 2 - (2 * l) / b : 0,
        b,
        a: hsl.a ?? 1,
    }
}

// let hsl2hsv = (h,s,l,v=s*Math.min(l,1-l)+l) => [h, v?2-2*l/v:0, v];

// let hsv2hsl = (h,s,v,l=v-v*s/2, m=Math.min(l,1-l)) => [h,m?(v-l)/m:0,l];
