import { List, ListItem } from "../List"

const meta = {
    title: "Data Display/List",
    component: List,
    tags: ["autodocs"],
} satisfies Meta<typeof List>

export default meta

type Story = StoryObj<typeof meta>

export const Basic = (props) => (
    <List>
        <ListItem>
            <div>a</div>
        </ListItem>
        <ListItem>
            <div>b</div>
        </ListItem>
        <ListItem>
            <div>c</div>
        </ListItem>
    </List>
)
