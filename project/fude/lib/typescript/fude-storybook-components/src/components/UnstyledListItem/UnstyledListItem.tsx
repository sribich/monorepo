import { useStyles } from "@sribich/fude"
import type { ReactNode } from "react"

import { unstyledListItemStyles } from "./UnstyledListItem.styles"

export interface UnstyledListItemProps {
    children: ReactNode
}

export const UnstyledListItem = (props: UnstyledListItemProps) => {
    const { styles } = useStyles(unstyledListItemStyles, {})
    return <li {...styles.container()}>{props.children}</li>
}
