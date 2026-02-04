import { useColorArea } from "@react-aria/color"
import { useColorAreaState } from "@react-stately/color"
import type { AriaColorAreaProps, Color } from "@react-types/color"
import { create } from "@stylexjs/stylex"
import { type ForwardedRef, type RefObject, useRef } from "react"
import { VisuallyHidden, mergeProps } from "react-aria"

import { useRenderProps } from "../../hooks/useRenderProps"
import { type VariantProps, makeStyles, useStyles } from "../../theme/props"
import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import type { StyleProps } from "../../utils/props"

////////////////////////////////////////////////////////////////////////////////
/// Styles
////////////////////////////////////////////////////////////////////////////////
const colorAreaStyles = makeStyles({
    slots: create({
        container: {
            zIndex: 1,
            display: "inline-block",
            position: "relative",
        },
        gradient: {
            position: "relative",
            zIndex: 1,
            height: "100%",
            width: "100%",
        },
        thumb: {
            zIndex: 2,
            display: "block",
            position: "absolute",
            boxSizing: "border-box",
            width: spacing["5"],
            height: spacing["5"],
            marginLeft: `calc(-1 * calc(${spacing["5"]}/2))`,
            marginTop: `calc(-1 * calc(${spacing["5"]}/2))`,

            borderColor: "#fff",
            borderRadius: borderRadius.full,
            borderWidth: borderWidth.lg,
            borderStyle: "solid",
            boxShadow: boxShadow.sm,
        },
        thumbColor: {
            borderRadius: borderRadius.full,
            height: "100%",
            width: "100%",
            boxShadow: boxShadow.sm,
        },
    }),
    variants: {
        rounded: {
            none: create({
                gradient: {
                    borderRadius: borderRadius.none,
                },
            }),
            sm: create({
                gradient: {
                    borderRadius: borderRadius.sm,
                },
            }),
            md: create({
                gradient: {
                    borderRadius: borderRadius.md,
                },
            }),
            lg: create({
                gradient: {
                    borderRadius: borderRadius.lg,
                },
            }),
        },
    },
    defaultVariants: {
        rounded: "none",
    },
})

////////////////////////////////////////////////////////////////////////////////
/// Shade
////////////////////////////////////////////////////////////////////////////////
export interface SaturationProps
    extends AriaColorAreaProps,
        StyleProps,
        VariantProps<typeof colorAreaStyles> {
    ref?: RefObject<HTMLDivElement>
    value: Color
}

export const ColorArea = (props: SaturationProps) => {
    const color = props.value.toFormat("hsb")

    const colorProps = {
        ...props,
        value: color,
        xChannel: "saturation",
        yChannel: "brightness",
    } satisfies SaturationProps

    const inputXRef = useRef(null)
    const inputYRef = useRef(null)
    const containerRef = useRef(null)

    const state = useColorAreaState(colorProps)
    const { colorAreaProps, thumbProps, xInputProps, yInputProps } = useColorArea(
        {
            ...colorProps,
            inputXRef,
            inputYRef,
            containerRef,
        },
        state,
    )

    const renderProps = useRenderProps(props, {})

    const { styles } = useStyles(colorAreaStyles, props)

    // <div {...mergeProps(gradientProps, styles.gradient())} />
    return (
        <div {...mergeProps(renderProps, colorAreaProps, styles.container())} ref={containerRef}>
            <div {...mergeProps(thumbProps, styles.thumb())}>
                <div
                    {...mergeProps(styles.thumbColor(), {
                        style: { backgroundColor: color.toString("css") },
                    })}
                />
                <VisuallyHidden>
                    <input {...mergeProps(xInputProps)} ref={inputXRef} />
                    <input {...mergeProps(yInputProps)} ref={inputYRef} />
                </VisuallyHidden>
            </div>
        </div>
    )
}
