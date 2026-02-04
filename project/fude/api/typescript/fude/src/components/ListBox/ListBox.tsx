import { type RefObject, use } from "react"
import { mergeProps } from "react-aria"
import {
    ListBox as AriaListBox,
    ListBoxItem as AriaListBoxItem,
    type ListBoxSectionProps as AriaListBoxSectionProps,
    type ListBoxItemProps,
    type ListBoxProps,
} from "react-aria-components"

import { useStyles, type VariantProps } from "../../theme/props"
import { ListBoxStyles, listBoxStyles } from "./ListBox.stylex"

//==============================================================================
// ListBox
//==============================================================================
export namespace ListBox {
    export interface Props<T extends object>
        extends ListBoxProps<T>,
            VariantProps<typeof listBoxStyles> {
        ref?: RefObject<HTMLDivElement>
    }
}

export const ListBox = <T extends object>(props: ListBox.Props<T>) => {
    const { styles, values } = useStyles(listBoxStyles, props)

    return (
        <ListBoxStyles value={{ styles, values }}>
            <div {...styles.wrapper()}>
                <AriaListBox {...mergeProps(props, styles.list())} />
            </div>
        </ListBoxStyles>
    )
}

//==============================================================================
// ListBoxItem
//==============================================================================
export namespace ListBoxItem {
    export interface Props extends ListBoxItemProps {
        ref?: RefObject<HTMLDivElement>
    }
}

export const ListBoxItem = (props: ListBoxItem.Props) => {
    const { styles } = use(ListBoxStyles)

    return <AriaListBoxItem {...mergeProps(props, styles.itemWrapper())} />
}

//==============================================================================
// ListBoxSection
//==============================================================================
export namespace ListBoxSection {
    export interface Props<T> extends AriaListBoxSectionProps<T> {}
}

export const ListBoxSection = <T,>(props: ListBoxSection.Props<T>) => {
    return null
}
