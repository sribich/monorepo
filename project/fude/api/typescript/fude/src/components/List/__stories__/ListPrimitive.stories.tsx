import { ListItemPrimitive, ListPrimitive } from "../ListPrimitive"

const meta = {
    title: "Data Display/List/Primitive",
    component: ListPrimitive,
    tags: ["autodocs"],
} satisfies Meta<typeof ListPrimitive>

export default meta

type Story = StoryObj<typeof meta>

export const Basic = (props) => (
    <ListPrimitive>
        <ListItemPrimitive>
            <div>a</div>
        </ListItemPrimitive>
        <ListItemPrimitive>
            <div>b</div>
        </ListItemPrimitive>
        <ListItemPrimitive>
            <div>c</div>
        </ListItemPrimitive>
    </ListPrimitive>
)
