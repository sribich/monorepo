import { useStyles } from "@sribich/fude"
import type { ReactNode } from "react"

import { unstyledListStyles } from "./UnstyledList.styles"

export interface UnstyledListProps {
    children: ReactNode
}

export const UnstyledList = (props: UnstyledListProps) => {
    const { styles } = useStyles(unstyledListStyles, {})

    return <ul {...styles.container()}>{props.children}</ul>
}
