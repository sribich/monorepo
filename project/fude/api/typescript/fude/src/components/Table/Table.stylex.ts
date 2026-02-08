import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"

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
        columnWrapper: {
            ":hover": {
                background: colors.backgroundHover,
            },
        },
        column: {
            display: "flex",
            flexDirection: "column",
        },
        columnResizer: {
            // flexGrow: 1,
            placeSelf: "end",
            // justifySelf: "end",
            height: "24px",
            cursor: "ew-resize",
            width: "4px",
            backgroundColor: "black",
            paddingBlock: "4px",
            boxSizing: "content-box",
            "::after": {

            }
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
