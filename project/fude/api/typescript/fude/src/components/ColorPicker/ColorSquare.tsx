import type { Color } from "@react-types/color"

import { type VariantProps, useStyles } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import { Checkerboard } from "./Checkerboard"
import { colorSquareStyles } from "./ColorSquare.styles"

////////////////////////////////////////////////////////////////////////////////
/// ColorSquare
////////////////////////////////////////////////////////////////////////////////
interface ColorSquareProps extends VariantProps<typeof colorSquareStyles> {
    color: Color
}

export const ColorSquare = (props: ColorSquareProps) => {
    const { styles } = useStyles(colorSquareStyles, props)

    return (
        <div {...styles.colorSquareContainer()}>
            <Checkerboard />
            <div
                {...mergeProps(styles.colorSquare(), {
                    style: { backgroundColor: props.color.toString("css") },
                })}
            />
        </div>
    )
}

/* import { ComponentPropsWithoutRef, ElementRef } from "react"

import { cn } from "../../../util/utils"
import { useColor } from "../hook/useColor"
import { Checkerboard } from "./Checkerboard"

export type ColorElement = ElementRef<"div">
export type ColorProps = Omit<ComponentPropsWithoutRef<"div">, "color"> & {
    color: string
}

export const Color = ColorElement, ColorProps>(({ color, className, ...props }, ref) => {
    const { color: hslColor } = useColor(color)

    console.log(color)
    console.log(hslColor)

    return (
        <div ref={ref} className={cn("relative w-6 h-6 overflow-hidden rounded", className)} {...props}>
            <Checkerboard className="w-full h-full" />
            <div
                className="absolute top-0 bottom-0 left-0 right-0 rounded shadow-inner-sm"
                style={{
                    backgroundColor: `hsl(${hslColor.h * 360}, ${hslColor.hsl.s * 100}%, ${hslColor.hsl.l * 100}%, ${
                        hslColor.a
                    })`,
                }}
            ></div>
        </div>
    )
})

Color.displayName = "Color"
 */
