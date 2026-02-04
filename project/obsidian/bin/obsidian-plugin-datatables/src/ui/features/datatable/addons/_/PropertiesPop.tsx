// @ts-nocheck
import { DragEndEvent } from "@dnd-kit/core"
import { Eye, EyeOff } from "lucide-react"
import { createElement } from "react"

import { propertyComponents } from "../../../../../schema/property/property-components"
import { Button } from "../../../../components/button/Button"
import { Menu } from "../../../../components/menu/Menu"
import { Typography } from "../../../../components/typography/Typography"
import { useSchema } from "../../../../hooks/useSchema"
import { ViewScope } from "../../../../hooks/useViewScope"

interface Props {
    viewScope: ViewScope
}

export const PropertiesPop = ({ viewScope }: Props) => {
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

    const onDragEnd = ({ active, over }: DragEndEvent) => {}

    return (
        <div className="max-h-[50vh] w-72">
            <Typography.Muted>Properties</Typography.Muted>
            <Menu>
                <Menu.OrderedSection onDragEnd={onDragEnd}>{items}</Menu.OrderedSection>
            </Menu>
        </div>
    )
}
