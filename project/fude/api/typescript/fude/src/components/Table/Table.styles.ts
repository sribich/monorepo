import { create } from "@stylexjs/stylex"

import { createGenericContext } from "../../hooks/context"
import { type CachedStyles, type MadeStyles, makeStyles } from "../../theme/props"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"

export const tableStyles = makeStyles({
    slots: create({
        table: {
            position: "relative",
            background: colors.background,
            borderCollapse: "collapse",
            borderSpacing: 0,
            width: "100%",
        },
        head: {
            boxSizing: "border-box",
            border: 0,
            borderBottomWidth: borderWidth.md,
            borderStyle: "solid",
            borderColor: colors.borderLayout,
        },
        headRow: {},
        column: {
            ":hover": {
                background: colors.backgroundHover,
            },
        },
        columnResizer: {
            flexGrow: 1,
            placeSelf: "end",
            justifySelf: "end",
        },
        body: {},
        row: {},
        cell: {},
        cellFlexWrapper: {
            alignItems: "center",
            display: "flex",
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})

export const [useTableStyles, TableStyleProvider] =
    createGenericContext<CachedStyles<typeof tableStyles>>()
