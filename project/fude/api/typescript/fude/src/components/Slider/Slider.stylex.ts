import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { newSpacing, spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"

import { createNewGenericContext } from "../../hooks/context"
import { type CachedStyles, makeStyles } from "../../theme/props"

export const sliderStyles = makeStyles({
    slots: create({
        slider: {
            // zIndex: 0,
            display: "flex",
            flexDirection: "column",
            width: "100%",
            // userSelect: "none",
            // touchAction: "none",
            // minInlineSize: spacing["48"],
            // flexShrink: 0,
            gap: newSpacing["4"],
        },
        labelWrapper: {
            // display: "flex",
            position: "relative",
            inlineSize: "auto",
            // TODO: Font color
            // TODO: Line height??

            display: "grid",
            gridTemplateAreas: "'label help value'",
            gridTemplateColumns: "auto 1fr auto",
            justifyItems: "start",
        },
        labelTitle: {
            gridArea: "label",
        },
        labelValue: {
            gridArea: "value",
        },
        labelValueSideWrapper: {},

        trackWrapper: {
            position: "relative",
            display: "flex",
            gap: newSpacing["8"],
        },
        track: {
            display: "flex",
            width: "100%",
            position: "relative",

            background: "var(--track-empty)",
            borderRadius: borderRadius.full,
            // boxShadow: boxShadow["inset"],
            // boxSizing: "border-box",
        },
        thumb: {
            borderRadius: borderRadius.full,

            display: "flex",
            justifyContent: "center",
            alignItems: "center",

            backgroundColor: "var(--track-fill)",
            cursor: "grab",

            "::before": {
                content: "",
                position: "absolute",
                inlineSize: newSpacing["10"],
                blockSize: newSpacing["10"],
                borderRadius: borderRadius.full,
            },
            "::after": {
                content: "",
                boxShadow: boxShadow.sm,
                backgroundColor: colors.background,
                borderRadius: borderRadius.full,
            },
        },
    }),
    // modifiers: {
    //     /*
    //     labelTop: create({
    //         slider: {
    //             display: "inline-flex",
    //             flexDirection: "column",
    //         },
    //     }),
    //     labelSide: create({
    //         slider: {
    //             display: "inline-flex",
    //             alignItems: "center",
    //             width: "4px",
    //         },
    //     }),
    //     isSelected: create({
    //         thumb: {
    //             // cursor: "grabbing",
    //             "::after": {
    //                 transform: "scale(0.8)",
    //             },
    //         },
    //     }),
    //     trackGradient: create({
    //         track: (gradient) => ({
    //             "track-empty": gradient,
    //         }),
    //     }),
    //     customTrack: create({
    //         track: {
    //             backgroundColor: "transparent",
    //         },
    //     }),
    //     */
    // },
    // conditions: {
    //     isVertical: {
    //         true: create({}),
    //         false: create({
    //             thumb: {
    //                 top: "calc(1/2 * 100%)",
    //             },
    //             trackWrapper: {
    //                 alignItems: "center",
    //             },
    //             track: {
    //                 borderInlineColor: "transparent",
    //             },
    //         }),
    //     },
    // },
    variants: {
        // isDisabled: {
        //     // true: create({
        //     //     slider: {
        //     //         cursor: "not-allowed",
        //     //     },
        //     //     trackWrapper: {
        //     //         cursor: "not-allowed",
        //     //         pointerEvents: "none",
        //     //     },
        //     // }),
        //     // false: create({}),
        // },
        color: {
            primary: create({
                trackWrapper: {
                    "--track-fill": colors.primary,
                    "--track-empty": colors.primarySelected,
                },
            }),
            success: create({
                trackWrapper: {
                    "--track-fill": colors.success,
                    "--track-empty": colors.successSelected,
                },
            }),
            warning: create({
                trackWrapper: {
                    "--track-fill": colors.warning,
                    "--track-empty": colors.warningSelected,
                },
            }),
            danger: create({
                trackWrapper: {
                    "--track-fill": colors.danger,
                    "--track-empty": colors.dangerSelected,
                },
            }),
        },
        // radius: {
        //     // none: create({
        //     //     track: {
        //     //         borderRadius: borderRadius["none"],
        //     //     },
        //     // }),
        //     // sm: create({
        //     //     track: {
        //     //         borderRadius: borderRadius["sm"],
        //     //     },
        //     // }),
        //     // md: create({
        //     //     track: {
        //     //         borderRadius: borderRadius["md"],
        //     //     },
        //     // }),
        // },
        size: {
            sm: create({
                // label: {
                //     fontSize: fontSize.sm,
                // },
                // value: {
                //     fontSize: fontSize.sm,
                // },
                trackWrapper: {
                    "--thumb-size": newSpacing["20"],
                },
                // track: {
                //     blockSize: spacing["1"],
                // },
                thumb: {
                    // insetBlockStart: `calc(${spacing["1"]}/2)`,
                    inlineSize: newSpacing["20"],
                    blockSize: newSpacing["20"],
                    "::after": {
                        inlineSize: newSpacing["16"],
                        blockSize: newSpacing["16"],
                    },
                },
            }),
            md: create({
                // label: {
                //     fontSize: fontSize.md,
                // },
                // value: {
                //     fontSize: fontSize.md,
                // },
                trackWrapper: {
                    "--thumb-size": spacing["6"],
                },
                // track: {
                //     blockSize: spacing["3"],
                // },
                thumb: {
                    // insetBlockStart: `calc(${spacing["3"]}/2)`,
                    inlineSize: newSpacing["24"],
                    blockSize: newSpacing["24"],
                    "::after": {
                        inlineSize: newSpacing["20"],
                        blockSize: newSpacing["20"],
                    },
                },
            }),
            lg: create({
                label: {
                    fontSize: fontSize.md,
                },
                value: {
                    fontSize: fontSize.md,
                },
                trackWrapper: {
                    "--thumb-size": spacing["7"],
                },
                // track: {
                //     blockSize: spacing["7"],
                // },
                thumb: {
                    insetBlockStart: `calc(${spacing["7"]}/2)`,
                    inlineSize: spacing["7"],
                    blockSize: spacing["7"],
                    "::after": {
                        inlineSize: spacing["5"],
                        blockSize: spacing["5"],
                    },
                },
            }),
        },
    },
    defaultVariants: {
        color: "primary",
        radius: "md",
        size: "sm",
        isDisabled: false,
        isVertical: false,
    },
    compounds: [
        {
            conditions: {
                isVertical: false,
            },
            modify: {
                size: {
                    sm: create({
                        track: {
                            blockSize: newSpacing["4"],
                            marginBlock: `calc((var(--thumb-size) - ${newSpacing["4"]}) / 2)`,
                            borderInlineWidth: `calc(var(--thumb-size) / 2)`,
                        },
                    }),
                    md: create({
                        track: {
                            blockSize: newSpacing["12"],
                        },
                    }),
                    lg: create({
                        track: {
                            blockSize: newSpacing["28"],
                        },
                    }),
                },
            },
        },
    ],
})

/*
These are the old styles which keep the thumb within the bounds of the component, but has
issues with custom track renderers

export const sliderStyles = {
    slots: stylex.create({
        slider: {
            zIndex: 0,
            display: "block",
            position: "relative",
            userSelect: "none",
            touchAction: "none",
            minInlineSize: spacing["48"],
            flexShrink: 0,
            //
            gap: spacing["3"],
        },
        labelWrapper: {
            // display: "flex",
            position: "relative",
            inlineSize: "auto",
            // TODO: Font color
            // TODO: Line height??

            display: "grid",
            gridTemplateAreas: "'label help value'",
            gridTemplateColumns: "auto 1fr auto",
            justifyItems: "start",
        },
        labelTitle: {
            gridArea: "label",
        },
        labelValue: {
            gridArea: "value",
        },
        labelValueSideWrapper: {},

        trackWrapper: {
            zIndex: "auto",
            display: "inline-block",
            boxSizing: "border-box",
            position: "relative",

            "--thumb-margin": `calc(var(--thumb-size)/2)`,

            // inlineSize: `100%`,
            inlineSize: "calc(100% - var(--thumb-margin))",
            marginInlineStart: "var(--thumb-margin)",

            verticalAlign: "top",

            // "::before": {
            //     content: "",
            //     display: "block",
            //     position: "absolute",
            //     inlineSize: "var(--thumb-margin)",
            //     blockSize: "100%",
            //     insetInlineStart: "calc(var(--thumb-margin) * -1)",
            //     backgroundColor: "var(--track-empty)",
            //     borderStartStartRadius: borderRadius.full,
            //     borderEndStartRadius: borderRadius.full,
            // },
            // "::after": {
            //     content: "",
            //     display: "block",
            //     position: "absolute",
            //     top: 0,
            //     inlineSize: "var(--thumb-margin)",
            //     blockSize: "100%",
            //     insetInlineEnd: "calc(var(--thumb-margin) * -1)",
            //     backgroundColor: "var(--track-empty)",
            //     borderStartEndRadius: borderRadius.full,
            //     borderEndEndRadius: borderRadius.full,
            // },
        },

        track: {
            zIndex: 1,
            blockSize: spacing["1"],
            width: `calc(100% - calc(var(--thumb-margin)))`,
            marginLeft: "calc(var(--thumb-margin) / 2)",
            // borderRadius: borderRadius.full,
            background: "var(--track-empty)",

            position: "absolute",
            left: 0,
            right: 0,
        },

        thumb: {
            zIndex: 2,
            borderRadius: borderRadius.full,
            // borderStyle: "solid",
            // borderWidth: borderWidth.sm,

            position: "absolute",
            // display: "inline-block",

            display: "flex",
            justifyContent: "center",
            alignItems: "center",

            boxSizing: "border-box",
            backgroundColor: "var(--track-fill)",

            // cursor: "grab",

            "::before": {
                content: "",
                position: "absolute",
                inlineSize: spacing["10"],
                blockSize: spacing["10"],
            },
            "::after": {
                content: "",
                boxShadow: boxShadow.sm,
                backgroundColor: colors.background,
                borderRadius: borderRadius.full,
            },
        },
    }),
    conditions: {
        slider: stylex.create({
            labelTop: {
                display: "inline-flex",
                flexDirection: "column",
            },
            labelSide: {
                display: "inline-flex",
                alignItems: "center",
                width: "4px",
            },
        }),
        thumb: stylex.create({
            isSelected: {
                // cursor: "grabbing",
                "::after": {
                    transform: "scale(0.8)",
                },
            },
        }),
        track: stylex.create({
            trackGradient: (gradient) => ({
                "track-empty": gradient,
            }),
        }),
    },
    variants: {
        isDisabled: {
            true: stylex.create({
                slider: {
                    cursor: "not-allowed",
                },
                trackWrapper: {
                    cursor: "not-allowed",
                    pointerEvents: "none",
                },
            }),
            false: stylex.create({}),
        },
        color: {
            primary: stylex.create({
                trackWrapper: {
                    "--track-fill": colors.primary,
                    "--track-empty": colors.primarySelected,
                },
            }),
            success: stylex.create({
                trackWrapper: {
                    "--track-fill": colors.success,
                    "--track-empty": colors.successSelected,
                },
            }),
            warning: stylex.create({
                trackWrapper: {
                    "--track-fill": colors.warning,
                    "--track-empty": colors.warningSelected,
                },
            }),
            danger: stylex.create({
                trackWrapper: {
                    "--track-fill": colors.danger,
                    "--track-empty": colors.dangerSelected,
                },
            }),
        },
        size: {
            sm: stylex.create({
                label: {
                    fontSize: fontSize.sm,
                },
                value: {
                    fontSize: fontSize.sm,
                },
                trackWrapper: {
                    "--thumb-size": spacing["5"],
                },
                track: {
                    blockSize: spacing["1"],
                },
                thumb: {
                    insetBlockStart: `calc(${spacing["1"]}/2)`,
                    inlineSize: spacing["5"],
                    blockSize: spacing["5"],
                    "::after": {
                        inlineSize: spacing["4"],
                        blockSize: spacing["4"],
                    },
                },
            }),
            md: stylex.create({
                label: {
                    fontSize: fontSize.md,
                },
                value: {
                    fontSize: fontSize.md,
                },
                trackWrapper: {
                    "--thumb-size": spacing["6"],
                },
                track: {
                    blockSize: spacing["3"],
                },
                thumb: {
                    insetBlockStart: `calc(${spacing["3"]}/2)`,
                    inlineSize: spacing["6"],
                    blockSize: spacing["6"],
                    "::after": {
                        inlineSize: spacing["5"],
                        blockSize: spacing["5"],
                    },
                },
            }),
            lg: stylex.create({
                label: {
                    fontSize: fontSize.md,
                },
                value: {
                    fontSize: fontSize.md,
                },
                trackWrapper: {
                    "--thumb-size": spacing["7"],
                },
                track: {
                    blockSize: spacing["7"],
                },
                thumb: {
                    insetBlockStart: `calc(${spacing["7"]}/2)`,
                    inlineSize: spacing["7"],
                    blockSize: spacing["7"],
                    "::after": {
                        inlineSize: spacing["5"],
                        blockSize: spacing["5"],
                    },
                },
            }),
        },
    },
}
*/

export const SliderStyleContext = createNewGenericContext<CachedStyles<typeof sliderStyles>>()
