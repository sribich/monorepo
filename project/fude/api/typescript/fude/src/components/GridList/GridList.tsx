import { mergeProps } from "react-aria"
import {
    GridList as AriaGridList,
    GridListItem as AriaGridListItem,
    type GridListItemProps as AriaGridListItemProps,
    type GridListProps as AriaGridListProps,
} from "react-aria-components"

import { createNewControlledContext } from "../../hooks/context"
import { useStyles, type VariantProps } from "../../theme/props"
import { GridListStyleProvider, gridListStyles } from "./GridList.styles"

//==============================================================================
// GridList
//==============================================================================
const GridListContext = createNewControlledContext<GridList.Props<any>, HTMLDivElement>()

export namespace GridList {
    export interface Props<T> extends AriaGridListProps<T>, VariantProps<typeof gridListStyles> {}
}

export const GridList = <T,>(rawProps: GridList.Props<T>) => {
    const [props, ref] = GridListContext.useContext(rawProps)

    const styles = useStyles(gridListStyles, props)

    return (
        <GridListStyleProvider value={styles}>
            <AriaGridList {...props} ref={ref} />
        </GridListStyleProvider>
    )
}

//==============================================================================
// GridList
//==============================================================================
export namespace GridListItem {
    export interface Props<T> extends AriaGridListItemProps<T> {}
}

export const GridListItem = <T extends object>(props: GridListItem.Props<T>) => {
    const { styles } = GridListStyleProvider.useContext()

    return <AriaGridListItem {...mergeProps(props, styles.item())} />
}
