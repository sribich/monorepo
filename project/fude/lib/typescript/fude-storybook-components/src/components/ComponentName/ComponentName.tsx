import { useStyles } from "@sribich/fude"

import { componentNameStyles } from "./ComponentName.styles"

export interface ComponentNameProps {
    children: string
    className: string
}

export const ComponentName = (props: ComponentNameProps) => {
    const { styles } = useStyles(componentNameStyles, {})

    return (
        <div {...styles.container()}>
            <h1 {...styles.content()}>{props.children}</h1>
        </div>
    )
}
