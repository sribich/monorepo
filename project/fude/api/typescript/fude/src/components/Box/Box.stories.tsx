import { Settings } from "lucide-react"

import preview from "@/preview"

import { Button } from "../Button/Button"
import { Card, CardBody, CardHeader } from "../Card/Card"
import { Box } from "./Box"

const meta = preview.meta({
    component: Box,
})

/**
 * The <b>Box</b> component is used as a customization entrypoint into various
 * fude components via <a href="https://react-aria.adobe.com/customization#slots">slot </a>
 * context. This is useful for cases where the component wants to manage its own styling
 * while allowing the consumer to define the content.
 *
 * In the provided example, the <b>Card</b> component provides a <b>menuArea</b> slot
 * which can be used to add configuration options to a card.
 */
export const Overview = meta.story(() => (
    <Card>
        <CardHeader>
            <Box slot="menuArea">
                <Button iconOnly size="xs">
                    <Settings />
                </Button>
            </Box>
        </CardHeader>
        <CardBody>Card Bodyx</CardBody>
    </Card>
))
