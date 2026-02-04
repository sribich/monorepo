export type ColorScale = Partial<{
    50: string
    100: string
    200: string
    300: string
    400: string
    500: string
    600: string
    700: string
    800: string
    900: string
    foreground: string
    DEFAULT: string
}>

export type Colors = {
    background: ColorScale
    foreground: ColorScale
    separator: ColorScale
    overlay: ColorScale
    focus: ColorScale
    content1: ColorScale
    content2: ColorScale
    content3: ColorScale
    content4: ColorScale

    default: ColorScale
    primary: ColorScale
    secondary: ColorScale
    success: ColorScale
    warning: ColorScale
    danger: ColorScale
}

export interface Layout {
    // TODO
}

export interface Theme {
    extends?: string | undefined
    layout?: Layout | undefined
    colors?: Colors | undefined
}
