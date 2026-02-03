import { Outlet, createFileRoute } from "@tanstack/react-router"
import { Sidebar } from "../components/Sidebar"
import { makeStyles, useStyles } from "@sribich/fude"
import { create } from "@stylexjs/stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"

const appStyles = makeStyles({
    slots: create({
        container: {
            display: "flex",
            height: "100vh",
            width: "100vw",
            maxHeight: "100vh",
            maxWidth: "100vw",
            flexDirection: "row",
        },
        sidebar: {
            display: "flex",
            flex: "0 0 auto",
            overflow: "scroll",
        },
        content: {
            display: "flex",
            flexDirection: "column",
            flex: "1 1 auto",
            overflow: "scroll",
            backgroundColor: colors.background,
            // padding: spacing["4"],
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})

export function AppLayout() {
    const { styles } = useStyles(appStyles, {})

    return (
        <div {...styles.container()}>
            <div {...styles.sidebar()}>
                <Sidebar />
            </div>
            <div {...styles.content()}>
                <Outlet />
            </div>
        </div>
    )
}

export const Route = createFileRoute("/_app")({
    component: AppLayout,
})
