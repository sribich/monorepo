import { Link } from "../Link"
import { DelegateButton } from "../../Button/Button"

const meta = {
    title: "Navigation/Link",
    component: Link,
} satisfies Meta<typeof Link>

export default meta

type Story = StoryObj<typeof meta>

export const Overview = (props) => (
    <DelegateButton>
        <Link>Link Text</Link>
    </DelegateButton>
)
