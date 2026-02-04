import preview from "@/preview"
import { Heading, Text } from "./Typography"

const meta = preview.meta({
    component: Text,
})

export const Overview = meta.story(() => <Text>Hi</Text>)

export const Headers = meta.story(() => (
    <>
        <Heading level={1}>H1</Heading>
        <Heading level={2}>H2</Heading>
        <Heading level={3}>H3</Heading>
        <Heading level={4}>H4</Heading>
        <Heading level={5}>H5</Heading>
        <Heading level={6}>H6</Heading>
    </>
))
