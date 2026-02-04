import type { Immutable } from "@sribich/ts-utils"

import type { Document } from "../../index/document"
import { propertyComponents } from "./property-components"
import type { PropertySchema } from "./property-schema"

export interface PropertyFieldProps {
    property: Immutable<PropertySchema>
    document: Immutable<Document>
}

export const PropertyField = (props: PropertyFieldProps) => {
    const component = propertyComponents[props.property.kind]

    return (
        <div className="flex-autorounded flex h-9 w-full cursor-pointer items-center p-0 p-2 hover:bg-white hover:bg-opacity-5">
            {component.field({
                property: props.property,
                document: props.document,
            })}
        </div>
    )

    //
    // return (
    //     <div
    //         className={cn(
    //             "flex-autorounded flex h-9 cursor-pointer items-center",
    //             /*!tableView &&*/ "p-2 hover:bg-white hover:bg-opacity-5 ",
    //         )}
    //     >
    //         {component.field({ property, page })}
    //     </div>
    // )
}

/*
{component.value.defaultOverlay ?? true ? (
    <Overlay
        render={({ hide }) => component.value.edit({ ...props, hide })}
        hideOnEnter={component.value.blurOnEnter ?? false}
    >
*/
