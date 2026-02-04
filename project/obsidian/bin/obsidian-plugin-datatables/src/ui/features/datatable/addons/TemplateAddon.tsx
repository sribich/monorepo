import {
    Box,
    Button,
    Dialog,
    DialogTrigger,
    Divider,
    GridList,
    GridListItem,
    Popover,
    TypographyText,
} from "@sribich/fude"
import { File, MoreHorizontal, Plus } from "lucide-react"

import { useSchema } from "../../../hooks/useSchema"
import { useView } from "../../../hooks/useView"

export const TemplateAddon = () => {
    return (
        <DialogTrigger>
            <Button size="sm" color="primary" variant="solid">
                New
            </Button>
            <Popover placement="bottom end">
                <TemplateDialog />
            </Popover>
        </DialogTrigger>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// TemplateDialog
////////////////////////////////////////////////////////////////////////////////
const TemplateDialog = () => {
    const schema = useSchema()

    const { source } = useView()

    const templates = schema.getTemplates()

    const newTemplate = () => {
        schema.createTemplate()
    }

    const onAction = (key: string | number) => {
        if (typeof key === "number") {
            throw new Error(`InvariantError`)
        }

        schema.instantiateTemplate(key)
    }

    return (
        <Dialog>
            <Box color="secondary" padding="2" rounded="sm" shadow="md" className="w-72">
                <TypographyText color="secondary" size="sm">
                    Templates for{" "}
                </TypographyText>
                <TypographyText size="sm">{source}</TypographyText>

                <GridList
                    aria-label="Templates"
                    items={templates?.options ?? []}
                    size="sm"
                    highlightChildren
                    onAction={onAction}
                >
                    {(item) => (
                        <GridListItem id={item.uuid}>
                            <File size="20" className="mr-1" />
                            <span className="flex-1">{item.name}</span>
                            <TemplateOptions />
                        </GridListItem>
                    )}
                </GridList>

                <Divider />

                <Button size="sm" fullWidth onClick={newTemplate}>
                    <Plus size="20" />
                    New Template
                </Button>
            </Box>
        </Dialog>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// TemplateOptions
////////////////////////////////////////////////////////////////////////////////
const TemplateOptions = () => {
    return (
        <Button size="xs" className="flex-0">
            <MoreHorizontal size="12" />
        </Button>
    )
}
