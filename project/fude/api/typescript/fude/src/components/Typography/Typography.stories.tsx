import preview from "@/preview"
import { Text } from "./Typography"

const meta = preview.meta({
    component: Text,
})

export const Overview = meta.story(() => <Text>Hi</Text>)
