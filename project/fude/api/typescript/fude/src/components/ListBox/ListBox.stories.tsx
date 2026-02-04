import preview from "@/preview"

import { ListBox, ListBoxItem } from "./ListBox"
import { Divider } from "../Divider/Divider"

const meta = preview.meta({
    title: "Navigation/ListBox",
    component: ListBox,
})

/**
 * # Header lmao
 * A ListBox displays a list of options and allows a user to select one
 * or more of them.
 *
 * Items in a ListBox should not be interactive. If interaction is needed,
 * use a `GridList`.
 */
export const Overview = meta.story((props) => (
    <div style={{ width: "200px" }}>
        <ListBox>
            <ListBoxItem>Open</ListBoxItem>
            <Divider />
            <ListBoxItem>Cut</ListBoxItem>
            <ListBoxItem>Copy</ListBoxItem>
            <Divider />
            <ListBoxItem>Delete</ListBoxItem>
            <ListBoxItem>Rename</ListBoxItem>
            <Divider />
            <ListBoxItem>Properties</ListBoxItem>
        </ListBox>
    </div>
))
