import { useStyles } from "@sribich/fude"
import { useMemo } from "react"

import { sectionNameStyles } from "./SectionName.styles"

export interface SectionNameProps {
    children: string
    className: string
}

export const SectionName = (props: SectionNameProps) => {
    const { styles } = useStyles(sectionNameStyles, {})

    const id = useMemo(() => {
        return props.children
            .toLowerCase()
            .replace(/[\’\']/g, "")
            .split(" ")
            .join("-")
    }, [props.children])

    return (
        <div id={id} {...styles.container()}>
            <h2 {...styles.content()}>{props.children}</h2>
        </div>
    )
}
