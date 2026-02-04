/* import { useContext, useEffect } from "react"

import { OrderedMenuContext } from "../context/OrderedMenuContext"

export const useOrderedMenu = (menuId?: string) => {
    const { setItemIds } = useContext(OrderedMenuContext)

    useEffect(() => {
        if (!menuId) {
            return
        }

        setItemIds((prev) => {
            if (prev.includes(menuId)) {
                return prev
            }

            return prev.slice().concat(menuId)
        })
    }, [menuId, setItemIds])
}
 */
