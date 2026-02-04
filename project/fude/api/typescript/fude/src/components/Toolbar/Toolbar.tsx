import type { RefObject } from "react"
import { useToolbar, type AriaToolbarProps } from "@react-aria/toolbar"
import type { Orientation } from "@react-types/shared"

import { mergeProps } from "../../utils/mergeProps"
import { useObjectRef } from "../../hooks/useObjectRef"
import type { RenderProps } from "../../utils/props"
import { useRenderProps } from "../../hooks/useRenderProps"

///=============================================================================
/// Toolbar
///=============================================================================
export namespace Toolbar {
    export interface Props extends AriaToolbarProps, RenderProps<Render> {
        ref?: RefObject<HTMLDivElement>
    }

    export interface Render {
        orientation: Orientation
    }
}

export const Toolbar = (props: Toolbar.Props) => {
    const toolbarRef = useObjectRef(props.ref)

    const { toolbarProps } = useToolbar(props, toolbarRef)

    const renderProps = useRenderProps(props, { orientation: props.orientation ?? "horizontal" })

    return (
        <div {...mergeProps(toolbarProps, renderProps)} ref={toolbarRef}>
            {renderProps.children}
        </div>
    )
}
