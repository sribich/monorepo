// @ts-nocheck
import { PlusOutlined } from "@ant-design/icons"
import { Divider } from "antd"

import { BasaltButton } from "../../../components/basalt/button/BasaltButton"

export interface FilterPopViewProps {
    addItem: () => void
}

export const FilterPopView = ({ addItem }: FilterPopViewProps) => {
    return (
        <>
            <Divider className="my-2" />
            <BasaltButton
                block
                className="px-2 text-left hover:bg-white hover:bg-opacity-5"
                icon={<PlusOutlined />}
                onClick={addItem}
            >
                Add filter
            </BasaltButton>
        </>
    )
}
