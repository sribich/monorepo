import { useStyles } from "@sribich/fude"

import { titleStyles } from "./Title.styles"

export interface TitleProps {
    children: string
    className: string
}

export const Title = (props: TitleProps) => {
    const { styles } = useStyles(titleStyles, {})

    return (
        <div {...styles.container()}>
            <h3 {...styles.content()}>{props.children}</h3>
        </div>
    )
}
