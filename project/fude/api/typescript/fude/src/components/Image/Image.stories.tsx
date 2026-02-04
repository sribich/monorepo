import preview from "@/preview"
import { stylesToArgTypes } from "../../theme/props.js"
import { Image } from "./Image.js"
import { imageStyles } from "./Image.stylex.js"

const meta = preview.meta({
    component: Image,
    argTypes: stylesToArgTypes(imageStyles),
})

export const Overview = meta.story((props) => (
    <Image width="300" zoom blur src="https://heroui.com/images/hero-card-complete.jpeg" />
))
