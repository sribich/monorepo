import { Box, Button, Dialog, DialogTrigger, Overlay } from "@sribich/fude"
import { type useImperativeHandle, useRef, useState } from "react"

export interface ConfirmProps {
    open: boolean
    onConfirm: () => void
    onCancel?: () => void
}

export const Confirm = (props: ConfirmProps) => {
    const onConfirm = () => {
        props.onConfirm()
    }
    const onCancel = () => {
        props.onCancel?.()
    }

    return (
        <DialogTrigger isOpen={props.open}>
            <Dialog>
                <Overlay>
                    <Box>
                        <Button onPress={onConfirm}>Yes</Button>
                        <Button onPress={onCancel}>No</Button>
                    </Box>
                </Overlay>
            </Dialog>
        </DialogTrigger>
    )
}

export const useConfirm = () => {
    const [open, setOpen] = useState(false)

    return {
        open: () => setOpen(true),
        Confirm: (props: ConfirmProps) => {
            const onConfirm = () => {
                setOpen(false)
                props.onConfirm()
            }
            const onCancel = () => {
                setOpen(false)
                props.onCancel?.()
            }

            return <_Confirm {...props} open={open} onConfirm={onConfirm} onCancel={onCancel} />
        },
    }
}
