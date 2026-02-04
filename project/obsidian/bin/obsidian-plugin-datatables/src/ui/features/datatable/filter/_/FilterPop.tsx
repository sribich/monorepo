// @ts-nocheck
import { useState } from "react"

import { Schema } from "../../../../schema/schema.editor"
import { ViewData } from "../../view/useView"
import { FilterPopAddItem } from "./FilterPopAddItem"
import { FilterPopView } from "./FilterPopView"

export interface FilterPopProps {
    close: () => void
    editor: Schema
    view: ViewData
}

export const FilterPop = ({ editor, view }: FilterPopProps) => {
    const [isAddingItem, setAddingItem] = useState(false)

    const addItem = (uuid: string) => {
        schema.view.filter.add(view.schema, uuid)
    }

    return (
        <div className="w-72">
            {isAddingItem ? (
                <FilterPopAddItem addItem={addItem} view={view} />
            ) : (
                <FilterPopView addItem={() => setAddingItem(true)} />
            )}
        </div>
    )
}
