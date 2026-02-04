import { type MouseEventHandler, type ReactNode, use } from "react"
import { DialogTrigger, RootMenuTriggerStateContext } from "react-aria-components"

export namespace ContextMenu {
    export interface Props {
        children?: ReactNode
    }
}

export const ContextMenu = (props: ContextMenu.Props) => {
    return (
        <DialogTrigger>
            <InnerMenu>{props.children}</InnerMenu>
        </DialogTrigger>
    )
}

//==============================================================================
// InnerMenu
//==============================================================================
namespace InnerMenu {
    export interface Props {
        children: ReactNode
    }
}

const InnerMenu = (props: InnerMenu.Props) => {
    const state = use(RootMenuTriggerStateContext)

    const openContextMenu: MouseEventHandler<HTMLDivElement> = (e) => {
        e.preventDefault()
        state?.open()
    }

    return <div onContextMenu={openContextMenu}>{props.children}</div>
}
