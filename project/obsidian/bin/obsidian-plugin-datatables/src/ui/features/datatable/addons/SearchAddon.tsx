import { Button, TextField } from "@sribich/fude"
import { Search } from "lucide-react"
import { type KeyboardEvent, useState } from "react"

export const SearchAddon = () => {
    const [isSearching, setSearching] = useState(false)

    const onKeyUp = ({ key, target }: KeyboardEvent) => {
        if (key === "Escape" && target instanceof HTMLInputElement) {
            setSearching(false)
        }
    }

    return (
        <>
            <Button size="sm" variant="light" onClick={() => setSearching(true)}>
                <Search size="16" />
            </Button>
            {isSearching && (
                <TextField autoFocus onKeyUp={onKeyUp} placeholder="Type to search..." />
            )}
        </>
    )
}
