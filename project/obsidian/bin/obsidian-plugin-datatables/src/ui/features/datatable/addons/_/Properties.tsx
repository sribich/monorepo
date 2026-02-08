// @ts-nocheck
import { List } from "lucide-react"

import { Menu } from "../../../../components/menu/Menu"
import { Popover } from "../../../../components/popover/Popover"
import { Typography } from "../../../../components/typography/Typography"
import { ViewScope } from "../../../../hooks/useViewScope"
import { PropertiesPop } from "./PropertiesPop"

interface Props {
    viewScope: ViewScope
}

export const Properties = ({ viewScope }: Props) => {
    return (
        <Popover>
            <Popover.Trigger asChild>
                <Menu.Item>
                    <Menu.Item.Icon>
                        <List size="20" />
                    </Menu.Item.Icon>
                    <Menu.Item.Text>Properties</Menu.Item.Text>
                    <Menu.Item.Extra arrow>
                        <Typography.Muted>
                            {viewScope.data.viewProperties.length}/
                            {viewScope.data.tableProperties.length} shown
                        </Typography.Muted>
                    </Menu.Item.Extra>
                </Menu.Item>
            </Popover.Trigger>
            <Popover.Content align="end">
                <PropertiesPop viewScope={viewScope} />
            </Popover.Content>
        </Popover>
    )
}
