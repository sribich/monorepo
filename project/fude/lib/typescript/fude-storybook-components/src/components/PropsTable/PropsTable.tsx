import { Controls } from "@storybook/addon-docs/blocks"
import { props } from "@stylexjs/stylex"
import type { ReactNode } from "react"

import { propsTableStyles } from "./PropsTable.styles"

export interface PropsTableProps {
    children: ReactNode
}

export const PropsTable = (_props: PropsTableProps) => {
    return <Controls {...props(propsTableStyles.container)}></Controls>
}

/*
         <PureArgsTable
                story="Overview"
                sort="alpha"
                {...props}
                {...stylex.props(propsTableStyles.container)}
            />
*/
