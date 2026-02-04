import { filterDOMProps } from "@react-aria/utils"
import type { AriaListBoxProps } from "react-aria"
import { type ListState, type Node, useListState } from "react-stately"

import {
    type CollectionProps,
    useCachedChildren,
    useCollection,
} from "../../utils/collection/hooks"
import type { RenderProps, StyleProps } from "../../utils/props"
import type { RefObject } from "react"

////////////////////////////////////////////////////////////////////////////////
/// ListPrimitive
////////////////////////////////////////////////////////////////////////////////
export interface ListPrimitiveProps<T>
    extends Omit<AriaListBoxProps<T>, "children">,
        CollectionProps<T>,
        StyleProps {
    ref?: RefObject<HTMLDivElement>
}

export const ListPrimitive = <T extends object>(props: ListPrimitiveProps<T>) => {
    const { collection, portal } = useCollection(props)

    const state = useListState({ ...props, collection, children: [], items: [] })

    return (
        <>
            {portal}
            <ListView props={props} state={state} />
        </>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// ListItemPrimitive
////////////////////////////////////////////////////////////////////////////////
export interface ListItemPrimitiveProps extends RenderProps<{}> {
    ref?: RefObject<HTMLDivElement>
}

export const ListItemPrimitive = (props: ListItemPrimitiveProps) => {
    return <CollectionItem {...props} />
}

////////////////////////////////////////////////////////////////////////////////
/// ListView
////////////////////////////////////////////////////////////////////////////////
interface ListViewProps<T> {
    props: ListPrimitiveProps<T>
    state: ListState<T>
}

const ListView = <T,>({ props, state }: ListViewProps<T>) => {
    const children = useCachedChildren({
        items: state.collection,
        children: (item) => {
            return <ListItemInner item={item} />
        },
    })

    return <div {...filterDOMProps(props)}>{children}</div>
}

////////////////////////////////////////////////////////////////////////////////
/// ListItemView
////////////////////////////////////////////////////////////////////////////////
type ListItemViewProps<T> = {
    item: Node<T>
}

const ListItemInner = <T,>({ item }: ListItemViewProps<T>) => {
    return <div {...item.props} />
}
