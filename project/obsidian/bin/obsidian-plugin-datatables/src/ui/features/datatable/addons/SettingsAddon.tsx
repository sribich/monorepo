import {
    Box,
    Button,
    Dialog,
    DialogTrigger,
    Divider,
    GridList,
    GridListItem,
    Popover,
    TextField,
    TypographyText,
    useDragAndDrop,
} from "@sribich/fude"
import { MoreHorizontal, Plus } from "lucide-react"
import { type KeyboardEvent, createElement } from "react"

import { viewComponents } from "../../../../schema/view/components"
import { useAsyncAction } from "../../../hooks/useAsyncAction"
import { useSchema } from "../../../hooks/useSchema"
import { useView } from "../../../hooks/useView"
import { ViewScopeProvider, useViewScope, useViewScopeContext } from "../../../hooks/useViewScope"

////////////////////////////////////////////////////////////////////////////////
/// SettingsAddon
////////////////////////////////////////////////////////////////////////////////
export const SettingsAddon = () => {
    return (
        <DialogTrigger>
            <Button size="sm" variant="light">
                <MoreHorizontal size="16" />
            </Button>
            <Popover placement="bottom end">
                <SettingsDialog />
            </Popover>
        </DialogTrigger>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// SettingsDialog
////////////////////////////////////////////////////////////////////////////////
const SettingsDialog = () => {
    const { view } = useView()
    // TODO: We need to not be calling useViewScope twice. We should figure out
    //       how to consolodate this one.
    const viewScope = useViewScope(view!)

    return (
        <Dialog>
            <Box color="secondary" padding="2" rounded="sm">
                <ViewScopeProvider value={viewScope}>
                    <TypographyText color="secondary" size="sm">
                        View options
                    </TypographyText>

                    <ViewNameInput />
                    <div className="mb-2" />

                    <ViewTabInfo />
                </ViewScopeProvider>
            </Box>
        </Dialog>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// ViewNameInput
////////////////////////////////////////////////////////////////////////////////
const ViewNameInput = () => {
    const schema = useSchema()
    const viewScope = useViewScopeContext()

    const onKeyUp = ({ key, target }: KeyboardEvent) => {
        if (key === "Enter" && target instanceof HTMLInputElement) {
            schema.view.rename(viewScope.view, target.value)
        }
    }

    return <TextField onKeyUp={onKeyUp} defaultValue={viewScope.schema.name} />
}

////////////////////////////////////////////////////////////////////////////////
///
////////////////////////////////////////////////////////////////////////////////
const ViewTabInfo = () => {
    const schema = useSchema()

    const allViews = schema.view.getAll()

    /*
    const { dragAndDropHooks } = useDragAndDrop({
        getItems: (keys) =>
            [...keys].map((key) => ({ "text/plain": allViews.find((it) => it.uuid === key)?.name ?? key })),
        onReorder: ({ target, keys }) => {
            const source = [...keys][0]

            if (
                !source ||
                typeof source !== "string" ||
                target.dropPosition === "on" ||
                typeof target.key !== "string"
            ) {
                return
            }

            schema.viewEditor.reorderViews(source, target.dropPosition, target.key)
        },
    })
    */

    // TODO: Use isAddingView to make the button a loader
    const [isAddingView, addView] = useAsyncAction(async () => {
        await schema.view.createView("table")
    })

    return (
        <DialogTrigger>
            <Button size="sm" variant="light" fullWidth>
                Views
            </Button>
            <Popover placement="bottom end">
                <Dialog>
                    <Box padding="2" rounded="sm">
                        <GridList
                            size="sm"
                            aria-label="Views"
                            items={allViews} /*dragAndDropHooks={dragAndDropHooks}*/
                        >
                            {(view) => (
                                <GridListItem id={view.uuid} textValue={view.name}>
                                    <div className="flex w-full items-center">
                                        {createElement(viewComponents[view.kind].icon, {
                                            size: 16,
                                            className: "mr-1",
                                        })}
                                        <span className="flex-1">{view.name}</span>
                                        <Button
                                            radius="sm"
                                            size="sm"
                                            variant="light"
                                            className="flex-0 p-0"
                                        >
                                            <MoreHorizontal size="16" />
                                        </Button>
                                    </div>
                                </GridListItem>
                            )}
                        </GridList>
                        <Divider />
                        <Button variant="light" size="sm" fullWidth onClick={addView}>
                            <Plus size="20" />
                            Add new view
                        </Button>
                    </Box>
                </Dialog>
            </Popover>
        </DialogTrigger>
    )
}

/*
import { ArrowDownUp, Filter, List } from "lucide-react"

import { Menu } from "../../../components/menu/Menu"
import { Popover } from "../../../components/popover/Popover"
import { Typography } from "../../../components/typography/Typography"
import { ViewScope } from "../../../hooks/useViewScope"
import { FilterList } from "../filter/FilterList"
import { Properties } from "./edit-items/Properties"

type Props = {
    viewScope: ViewScope
}

export const DatatableEditPop = ({ viewScope }: Props) => {
    return (
        <div className="flex w-72 flex-col">
            <Typography.Muted>View options</Typography.Muted>
            <Menu>
                <Menu.Item>Layout</Menu.Item>
                <Menu.Section />

                <Properties viewScope={viewScope} />

                <Popover>
                    <Popover.Trigger asChild>
                        <Menu.Item>
                            <Menu.Item.Icon>
                                <Filter size="20" />
                            </Menu.Item.Icon>
                            <Menu.Item.Text>Filter</Menu.Item.Text>
                            <Menu.Item.Extra arrow>
                                <Typography.Muted>{viewScope.data.filters.length} filters</Typography.Muted>
                            </Menu.Item.Extra>
                        </Menu.Item>
                    </Popover.Trigger>
                    <Popover.Content align="end">
                        <FilterList viewScope={viewScope} />
                    </Popover.Content>
                </Popover>

                <Menu.Item>
                    <Menu.Item.Icon>
                        <ArrowDownUp size="20" />
                    </Menu.Item.Icon>
                    Sort
                </Menu.Item>
                <Menu.Section />
                <Menu.Item>Notifications</Menu.Item>
                <Menu.Item>Delete</Menu.Item>
            </Menu>
        </div>
    )
}

*/
