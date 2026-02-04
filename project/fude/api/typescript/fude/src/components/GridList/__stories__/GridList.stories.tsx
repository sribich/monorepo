import { useListData } from "react-stately"
import { useDragAndDrop } from "../../../hooks/useDragAndDrop"
import { GridList, GridListItem } from "../GridList"
import { Button } from "../../Button/Button"
import { type Meta, StoryObj } from "@storybook/react-vite"

const meta = {
    title: "Navigation/GridList",
    component: GridList,
} satisfies Meta<typeof GridList>
export default meta
type Story = StoryObj<typeof meta>

export const Overview = (props) => {
    const list = useListData({
        initialItems: [
            { id: 1, name: "First" },
            { id: 2, name: "Second" },
            { id: 3, name: "Third" },
            { id: 4, name: "Fourth" },
            { id: 5, name: "Fifth" },
        ],
    })

    const { dragAndDropHooks } = useDragAndDrop({
        getItems(keys) {
            return Array.from(keys).map((key) => {
                return {
                    "text/plain": list.getItem(key).name,
                }
            })
        },
        onReorder(event) {
            if (event.target.dropPosition === "before") {
                list.moveBefore(event.target.key, event.keys)
            } else if (event.target.dropPosition === "after") {
                list.moveAfter(event.target.key, event.keys)
            }
        },
    })

    return (
        <GridList
            aria-label="items"
            selectionMode="multiple"
            items={list.items}
            dragAndDropHooks={dragAndDropHooks}
        >
            {(item) => (
                <GridListItem id={item.id}>
                    {item.name}
                    <Button>...</Button>
                </GridListItem>
            )}
        </GridList>
    )
}
