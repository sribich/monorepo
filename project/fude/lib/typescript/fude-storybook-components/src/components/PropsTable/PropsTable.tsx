// import { PureArgsTable } from "@storybook/blocks"
import { props } from "@stylexjs/stylex"
import type { ReactNode } from "react"

import { propsTableStyles } from "./PropsTable.styles"

export interface PropsTableProps {
    children: ReactNode
}

export const PropsTable = (props: PropsTableProps) => {
    return <div {...props(propsTableStyles.container)}></div>
}

/*
         <PureArgsTable
                story="Overview"
                sort="alpha"
                {...props}
                {...stylex.props(propsTableStyles.container)}
            />
*/
