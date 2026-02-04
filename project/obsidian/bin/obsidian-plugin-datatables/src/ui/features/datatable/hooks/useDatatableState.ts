import { createGenericContext } from "@sribich/fude"
import { useState } from "react"

export interface DatatableState {
    filterBarVisible: boolean
    toggleFilterBar: () => void
    sortBarVisible: boolean
    toggleSortBar: () => void
}

/**
 * TODO: Make this handle initialState
 */
export const useDatatableState = (): DatatableState => {
    const [state, setState] = useState({
        filterBarVisible: true,
        sortBarVisible: true,
    })

    return {
        filterBarVisible: state.filterBarVisible,
        toggleFilterBar: () => setState({ ...state, filterBarVisible: !state.filterBarVisible }),
        sortBarVisible: state.sortBarVisible,
        toggleSortBar: () => setState({ ...state, sortBarVisible: !state.sortBarVisible }),
    }
}

export const [useDatatableStateContext, DatatableStateProvider] =
    createGenericContext<DatatableState>()
