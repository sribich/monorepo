import { create } from "@stylexjs/stylex"

import { createGenericContext } from "../../hooks/context"
import { type CachedStyles, makeStyles } from "../../theme/props"
import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"

export const datePickerStyles = makeStyles({
    slots: create({
        wrapper: {
            isolation: "isolate",
            display: "block",
        },
        headerGroup: {
            display: "grid",
            gap: spacing["4"],
            gridAutoColumns: "1fr",
            gridAutoFlow: "column",
        },
        headerItem: {
            display: "grid",
            width: "100%",
            gridTemplateColumns: "1fr auto 1fr",
            gridTemplateAreas: "'prev text next'",
            minWidth: `calc(${spacing["9"]} * 7)`,
            maxWidth: `calc(${spacing["9"]} * 7)`,
            boxSizing: "border-box",
            alignItems: "center",
            alignContent: "center",
        },
        headerPrev: {
            gridArea: "prev",
            justifySelf: "start",
        },
        headerText: {
            gridArea: "text",
            // TODO: headerText: "[grid-area:title] text-ellipsis overflow-hidden whitespace-nowrap text-center text-lg font-bold self-center", // w-full leading-[theme(height.10)]
        },
        headerNext: {
            gridArea: "next",
            justifySelf: "end",
        },
        calendarGroup: {
            display: "grid",
            gridAutoFlow: "column",
            gridAutoColumns: "1fr",
            gap: spacing["4"],
            alignItems: "flex-start",
        },
        calendarItem: {
            tableLayout: "fixed",
            width: "100%",
            borderCollapse: "collapse",
            borderSpacing: 0,
            userSelect: "none",
            minWidth: `calc(${spacing["9"]} * 7)`,
            maxWidth: `calc(${spacing["9"]} * 7)`,
        },
        calendarCell: {
            height: spacing["9"],
            width: spacing["9"],
            padding: 0,
            position: "relative",
            textAlign: "center",
            // TODO: outline-0 (none)
        },
        calendarDayOfWeek: {
            display: "flex",
            height: "100%",
            width: "100%",
            flexDirection: "column",
            justifyContent: "flex-end",
            fontSize: fontSize.sm,
            // TODO: fontWeight: fontWeight.md
            textTransform: "uppercase",
        },
        calendarDay: {
            position: "absolute",
            top: 0,
            left: 0,
            display: "block",
            width: "100%",
            whiteSpace: "nowrap",
            boxSizing: "border-box",
            lineHeight: spacing["9"],
            "--margin": `calc((100%-${spacing["9"]})/2)`,
            "::before": {
                content: "",
                display: "block",
                position: "absolute",
                top: `-calc(50%-calc(${spacing["9"]}/2))`,
                left: `-calc(50%-calc(${spacing["9"]}/2))`,
                width: `calc(${spacing["9"]} + 1px)`,
                height: `calc(${spacing["9"]} + 1px)`,
                margin: 0,
                borderRadius: borderRadius.full,
            },
            "::after": {
                content: "",
                display: "block",
                position: "absolute",
                // top: `-calc(50%-calc(${spacing["9"]}/2))`,
                top: 0,
                left: `-calc(50%-calc(${spacing["9"]}/2))`,
                width: spacing["9"],
                height: spacing["9"],
                borderRadius: borderRadius.full,
                zIndex: -1,
                backgroundColor: "var(--calendar-bg)",
            },
            ":hover": {
                "--calendar-bg": colors["primarySelectedHover"],
            },
        },
        calendarDayInner: {
            display: "block",
            width: spacing["9"],
            margin: "0 auto",
            marginInlineStart: "auto",
            marginInlineEnd: "var(--margin)",
        },
    }),
    conditions: {
        outsideMonth: create({
            calendarDay: {
                visibility: "hidden",
            },
        }),
        isSelected: create({
            calendarDay: {},
        }),
        isSelectionStart: create({
            calendarDay: {
                borderStartStartRadius: borderRadius["full"],
                borderEndStartRadius: borderRadius["full"],
                // backgroundColor: `color-mix(in srgb, ${colors["primarySelected"]} 75%, transparent)`,
                // "--calendar-bg": colors["primary"],
                zIndex: 10,
                "::before": {
                    borderStartStartRadius: borderRadius["full"],
                    borderEndStartRadius: borderRadius["full"],
                    backgroundColor: colors["primarySelected"],
                    zIndex: -2,
                },
                "::after": {
                    backgroundColor: colors["primary"],
                    zIndex: -1,
                },
            },
        }),
        isSelectionEnd: create({
            calendarDay: {
                borderStartEndRadius: borderRadius["full"],
                borderEndEndRadius: borderRadius["full"],
                zIndex: 10,
                "::before": {
                    borderStartEndRadius: borderRadius["full"],
                    borderEndEndRadius: borderRadius["full"],
                    backgroundColor: colors["primarySelected"],
                    zIndex: -2,
                },
                "::after": {
                    backgroundColor: colors["primary"],
                    zIndex: -1,
                },
            },
        }),
        isRangeStart: create({
            calendarDay: {
                borderStartStartRadius: borderRadius["full"],
                borderEndStartRadius: borderRadius["full"],
            },
        }),
        isRange: create({
            calendarDay: {
                backgroundColor: colors["primarySelected"],
            },
        }),
        isRangeEnd: create({
            calendarDay: {
                borderStartEndRadius: borderRadius["full"],
                borderEndEndRadius: borderRadius["full"],
            },
        }),
    },
    variants: {},
    defaultVariants: {},
})

export const [useDatePickerStyles, DatePickerStyleProvider] =
    createGenericContext<CachedStyles<typeof datePickerStyles>>()

/*
const calendarVariants = tv({
    slots: {
        day: [
            // "data-[selection-range=true]:after:bg-blue-200",
            // "data-[selection-range=true]:rounded-none",


            "data-[selection-range=true]:bg-blue-500/25",

            "data-[selection-start=true]:after:bg-blue-600",
            "data-[selection-end=true]:after:bg-blue-600",


            // "data-[selection-range=true]:rounded-none",

            // "data-[selection-start=true]:[border-end-start-radius:theme(width.9)]",

        ],

    },
})
*/
