// @ts-nocheck
import { ListItem } from "../../../components/_/list/ListItem"
import { fieldToComponentMap } from "../../field/Field"
import { ViewData } from "../../view/useView"

export interface FilterPopAddItemProps {
    addItem: (uuid: string) => void
    view: ViewData
}

export const FilterPopAddItem = ({ addItem, view }: FilterPopAddItemProps) => {
    const properties = view.mappedProperties

    return (
        <>
            {properties.map((it) => (
                <ListItem key={it.field.uuid} onClick={() => addItem(it.property.uuid)}>
                    <ListItem.Left>
                        <div className="mr-2">{fieldToComponentMap[it.field.kind].typeIcon}</div>
                        <div className="overflow-hidden text-ellipsis whitespace-nowrap">
                            {it.field.name}
                        </div>
                    </ListItem.Left>
                </ListItem>
            ))}
        </>
    )
}
