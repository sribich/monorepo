import preview from "@/preview"
import { stylesToArgTypes } from "../../theme/props.js"
import { Button } from "../Button/Button.js"
import { Flex } from "./Flex.js"
import { flexStyles } from "./Flex.stylex.js"

const meta = preview.meta({
    title: "Layout/Flex",
    component: Flex,
    ...stylesToArgTypes(flexStyles),
})

export const Overview = meta.story((props) => (
    <Flex {...props}>
        <Button>First Item</Button>
        <Button size="lg">Second Item</Button>
        <Button>Third Item</Button>
    </Flex>
))
