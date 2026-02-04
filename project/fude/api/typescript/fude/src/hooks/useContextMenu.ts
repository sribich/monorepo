import type { PointerEvent, SyntheticEvent } from "react"

export interface ContextMenuProps {
    onContextMenu?: (event: SyntheticEvent<Element, PointerEvent>) => void
}

export const useContextMenu = (props: ContextMenuProps) => {
    const { onContextMenu } = props

    return {
        contextMenuProps: {
            onContextMenu,
        },
    } as const
}
