import { Menu, MenuItem, TextField } from "@sribich/fude"
import { File } from "lucide-react"
import { type Key, useMemo, useState } from "react"

import type { DatatableContext } from "../../../processor/processors/datatable"
import { useMountContext } from "../../hooks/useMountContext"
import { getPrettyTagName } from "../../util/obsidian"

export const DatatableSourceConfigurator = () => {
    const { proxy, proxyMut, loader } = useMountContext<DatatableContext>()

    const [search, setSearch] = useState("")

    const onAction = (source: Key) => {
        proxyMut.codeBlock.source = String(source)
    }

    const sources = useMemo(() => {
        const tags = loader.index.tags.getAllTags()

        /*
        <Menu.Item.Icon>
            <File />
        </Menu.Item.Icon>
        <Menu.Item.Text>
            {getPrettyTagName(it)}
            <Typography.Muted className="ml-1">({it})</Typography.Muted>
        </Menu.Item.Text>
        */
        return [...tags].map((it) => ({
            tag: it,
            prettyTag: getPrettyTagName(it),
            component: (
                <MenuItem id={it} key={it}>
                    <File />
                    {getPrettyTagName(it)}({it})
                </MenuItem>
            ),
        }))
    }, [loader])

    const filteredSources = sources
        .filter((it) => it.tag.includes(search) || it.prettyTag.includes(search))
        .map((it) => it.component)

    return (
        <div>
            <TextField label="Search" value={search} onChange={(event) => setSearch(event)} />
            {/*<Typography.Muted>Please select a datatable source</Typography.Muted>*/}
            <Menu onAction={onAction}>{filteredSources}</Menu>
        </div>
    )
}
