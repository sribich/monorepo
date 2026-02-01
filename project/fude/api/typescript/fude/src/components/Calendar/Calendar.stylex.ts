import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { newSpacing, spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"

import { createGenericContext } from "../../hooks/context"
import { type CachedStyles, makeStyles } from "../../theme/props"

export const calendarStyles = makeStyles({
    slots: create({
        wrapper: {
            borderStyle: "solid",
            borderColor: colors.borderLayout,
            borderWidth: "1px",
        },
        calendarHeader: {
            padding: newSpacing["4"],
            display: "grid",
            gridTemplateColumns: "repeat(7, minmax(0, 1fr))",
            gridGap: "1px",
            // backgroundColor: colors.borderLayout,
            borderStyle: "solid",
            borderColor: colors.borderLayout,
            borderBottomWidth: borderWidth.sm,
        },
        calendarBody: {
            display: "grid",
            gridTemplateColumns: "repeat(7, minmax(0, 1fr))",
            backgroundColor: colors.borderLayout,
            gridGap: "1px",
            // gridTemplateRows: "repeat(7, minmax(0, 1fr))",
        },
        day: {
            padding: newSpacing["4"],
            minHeight: spacing["24"],
            backgroundColor: colors.background,
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})

export const [useCalendarStyles, CalendarStyleProvider] =
    createGenericContext<CachedStyles<typeof calendarStyles>>()
