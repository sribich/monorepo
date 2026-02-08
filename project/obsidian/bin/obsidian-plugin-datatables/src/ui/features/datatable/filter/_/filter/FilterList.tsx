// @ts-nocheck
import { DragEndEvent } from "@dnd-kit/core"
import { Plus } from "lucide-react"

import { Menu } from "../../../components/menu/Menu"
import { Popover } from "../../../components/popover/Popover"
import { Typography } from "../../../components/typography/Typography"
import { ViewScope } from "../../../hooks/useViewScope"
import { AddFilterMenu } from "./AddFilterMenu"

interface Props {
    viewScope: ViewScope
}

export const FilterList = ({ viewScope }: Props) => {
    const onDragEnd = ({ active, over }: DragEndEvent) => {}

    return (
        <div className="max-h-[50vh] w-72">
            <Typography.Muted>Filters</Typography.Muted>
            <Menu>
                <Menu.OrderedSection onDragEnd={onDragEnd}></Menu.OrderedSection>
                {viewScope.data.filters.map((it) => (
                    <Menu.OrderedItem key={it.uuid} dragId={it.uuid}>
                        <Menu.Item.Text>
                            {
                                viewScope.data.tableProperties.find((p) => p.uuid === it.property)
                                    ?.name
                            }
                        </Menu.Item.Text>
                    </Menu.OrderedItem>
                ))}
                <Menu.Section />

                <Popover>
                    <Popover.Trigger>
                        <Menu.Item>
                            <Menu.Item.Icon>
                                <Plus />
                            </Menu.Item.Icon>
                            <Menu.Item.Text>Add Filter</Menu.Item.Text>
                        </Menu.Item>
                    </Popover.Trigger>
                    <Popover.Content>
                        <AddFilterMenu viewScope={viewScope} />
                    </Popover.Content>
                </Popover>
            </Menu>
        </div>
    )
}

/*
const schema = useSchema()

    const toggleProperty = (uuid: string) => {
        schema.view.toggleProperty(viewScope.view, uuid)
    }

    const items = viewScope.data.tableProperties.map((it) => {
        const propertyComponent = propertyComponents[it.kind]

        return (
            <Menu.OrderedItem key={it.uuid} dragId={it.uuid}>
                <Menu.Item.Icon>{createElement(propertyComponent.icon, {})}</Menu.Item.Icon>
                <Menu.Item.Text>{propertyComponent.name}</Menu.Item.Text>
                <Menu.Item.Extra>
                    <Button variant="ghost" size="icon" onClick={() => toggleProperty(it.uuid)}>
                        {viewScope.data.viewProperties.some((prop) => prop.uuid === it.uuid) ? (
                            <Eye size="16" />
                        ) : (
                            <EyeOff size="16" />
                        )}
                    </Button>
                </Menu.Item.Extra>
            </Menu.OrderedItem>
        )
    })

    const onDragEnd = ({ active, over }: DragEndEvent) => {
        console.log("hi")
    }

    return (
        <div className="w-72 max-h-[50vh]">
            <Typography.Muted>Properties</Typography.Muted>
            <Menu>
                <Menu.OrderedSection onDragEnd={onDragEnd}>{items}</Menu.OrderedSection>
            </Menu>
        </div>
    )
    */
