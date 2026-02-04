import { GridList, GridListItem, useDragAndDrop, useListData } from "@sribich/fude"

import type { DatatableSchemaContext } from "../../../processor/processors/datatable-schema"
import { AddPropertyMenu } from "../../../schema/property/AddPropertyMenu"
import { PropertyConfig } from "../../../schema/property/PropertyConfig"
import { PropertyField } from "../../../schema/property/PropertyField"
import { propertyComponents } from "../../../schema/property/property-components"
import { useMountContext } from "../../hooks/useMountContext"
import { useSchema } from "../../hooks/useSchema"

////////////////////////////////////////////////////////////////////////////////
/// PropertyList
////////////////////////////////////////////////////////////////////////////////
export interface PropertyListProps {}

export const PropertyList = () => {
    const { proxy } = useMountContext<DatatableSchemaContext>()
    const schema = useSchema()

    const properties = schema.property.getAll()

    const { dragAndDropHooks } = useDragAndDrop({
        /*
        getItems(keys) {
            return Array.from(keys).map((key) => {
                // const item = list.getItem(key)
                const item = list.items.find((it) => it.id === key)

                return {
                    "text/plain": item.property.name,
                }
            })
        },
        onReorder(event) {
            const activeIndex = properties.findIndex((it) => it.uuid === [...event.keys][0])
            const overIndex = properties.findIndex((it) => it.uuid === event.target.key)

            if (event.target.dropPosition === "before") {
                schema.propertyEditor.move(activeIndex, overIndex - 1)
            } else if (event.target.dropPosition === "after") {
                schema.propertyEditor.move(activeIndex, overIndex)
            }
        },
        */
    })

    return (
        <>
            <GridList
                items={properties}
                aria-label={`${schema.tableName} properties`}
                dragAndDropHooks={dragAndDropHooks}
            >
                {(item) => (
                    <GridListItem id={item.uuid}>
                        <div className="grid w-full grid-cols-[min-content_1fr]">
                            <PropertyConfig property={item} />
                            <PropertyField property={item} document={proxy.document} />
                        </div>
                    </GridListItem>
                )}
            </GridList>
            <AddPropertyMenu />
        </>
    )
}
