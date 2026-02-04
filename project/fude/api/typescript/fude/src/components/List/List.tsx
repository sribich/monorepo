import type { RefObject } from "react"

import type { StyleProps } from "../../utils/props"
import { ListItemPrimitive, type ListItemPrimitiveProps, ListPrimitive } from "./ListPrimitive"

////////////////////////////////////////////////////////////////////////////////
/// Styles
////////////////////////////////////////////////////////////////////////////////
// const listStyles = tv({})

// const [useStyles, StyleProvider] = createGenericContext<ReturnType<typeof tabsVariants>>()

////////////////////////////////////////////////////////////////////////////////
/// List
////////////////////////////////////////////////////////////////////////////////
export interface ListProps extends Omit<ListItemPrimitiveProps, "className" | "style">, StyleProps {
    ref?: RefObject<HTMLDivElement>
}

export const List = (props: ListProps) => {
    return <ListPrimitive {...props} />
}

////////////////////////////////////////////////////////////////////////////////
/// ListItem
////////////////////////////////////////////////////////////////////////////////
export interface ListItemProps extends ListItemPrimitiveProps {
    ref?: RefObject<HTMLDivElement>
}

export const ListItem = (props: ListItemProps) => {
    return <ListItemPrimitive {...props} />
}
