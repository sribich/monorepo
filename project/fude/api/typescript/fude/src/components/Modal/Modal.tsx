import { create } from "@stylexjs/stylex"
import type { ReactNode } from "react"
import { Dialog, ModalOverlay, Modal as RACModal } from "react-aria-components"
import { type ExportedStyles, makeStyles, useStyles, type VariantProps } from "../../theme/props"

export namespace Modal {
    export interface Props extends VariantProps<typeof modalStyles> {
        children?: ReactNode
    }
}

export const Modal = (props: Modal.Props) => {
    const { styles } = useStyles(modalStyles, props)

    return (
        <ModalOverlay {...styles.overlay()}>
            <div {...styles.center()}>
                <RACModal {...styles.modal()}>
                    <Dialog>{props.children}</Dialog>
                </RACModal>
            </div>
        </ModalOverlay>
    )
}

const modalStyles = makeStyles({
    slots: create({
        overlay: {
            position: "absolute",
            top: 0,
            left: 0,
            width: "100%",
            height: "100dvh",
            isolation: "isolate",
            textAlign: "center",
            backgroundColor: "#00000020",
            backdropFilter: "blur(2px)",
        },
        center: {
            position: "sticky",
            top: 0,
            left: 0,
            width: "100%",
            height: "100dvh",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            boxSizing: "border-box",
        },
        modal: {
            backgroundColor: "transparent",
        },
    }),
    modifiers: {},
    conditions: {},
    variants: {},
    defaultVariants: {},
}) satisfies ExportedStyles

// font-sans w-full max-w-[min(90vw,450px)] max-h-[calc(var(--visual-viewport-height)*.9)] rounded-2xl bg-white dark:bg-neutral-800/70 dark:backdrop-blur-2xl dark:backdrop-saturate-200 forced-colors:bg-[Canvas] text-left align-middle text-neutral-700 dark:text-neutral-300 shadow-2xl bg-clip-padding border border-black/10 dark:border-white/10
