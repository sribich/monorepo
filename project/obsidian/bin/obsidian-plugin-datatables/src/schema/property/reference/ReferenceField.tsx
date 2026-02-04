export const ReferenceField = () => {
    return null
}

/*
import { FieldKind } from "../../../../schema/field/field.definition"
import { BasaltButton } from "../../../components/basalt/button/BasaltButton"
import { useFieldKind } from "../../../hooks/useFieldKind"
import { FieldProps } from "../Field"

export const ReferenceValue = ({ editor, page, ...props }: FieldProps) => {
    const field = useFieldKind(props.field, FieldKind.Reference)
    const pages = schema.field.getValue(field, page)

    const navigateTo = (path: string) => {
        const file = app.metadataCache.getFirstLinkpathDest(path, path)

        if (file) {
            app.workspace.getLeaf().openFile(file, { active: true })
        }
    }

    const references = pages.map((it) => {
        const pageTitle = it
            .split("/")
            .last()
            ?.replace(/\.[^/.]+$/, "")

        return (
            <BasaltButton key={it} onClick={() => navigateTo(it)} type="link" className="mr-1">
                {pageTitle}
            </BasaltButton>
        )
    })

    return <div>{references}</div>
}
*/
/*
<Overlay
    render={({ hide }) => <ReferenceValueEdit {...props} editor={editor} page={page} hide={hide} />}
    hideOnEnter={true}
>
*/

/*
import { useMemo, useState } from "react"

import { FileOutlined } from "@ant-design/icons"
import { Typography } from "antd"

import { FieldKind } from "../../../../schema/field/field.definition"
import { ListItem } from "../../../components/_/list/ListItem"
import { BasaltInput } from "../../../components/basalt/input/BasaltInput"
import { useFieldKind } from "../../../hooks/useFieldKind"
import { FieldProps } from "../Field"

export const ReferenceValueEdit = ({ editor, page, ...props }: FieldProps) => {
    const field = useFieldKind(props.field, FieldKind.Reference)
    const [search, setSearch] = useState("")

    const possibleReferences = useMemo(() => {
        const pages = Array.from(schema.index().tags.getPages(field.config.target))

        const addReference = (path: string) => {
            schema.updateFieldValue(field, page, (oldConfig = []) => {
                if (!oldConfig?.find((it) => it === path)) {
                    oldConfig?.push(path)
                }
                return oldConfig
            })
        }

        return pages.map((it) => ({
            page: it,
            component: (
                <ListItem key={it} onClick={() => addReference(it)}>
                    <ListItem.Left>
                        <div className="flex flex-row">
                            <span className="mr-2">
                                <FileOutlined />
                            </span>
                            <div className="flex flex-col">
                                <span>{it.split("/").last()?.replace(".md", "")}</span>
                                <Typography.Text type="secondary">{it}</Typography.Text>
                            </div>
                        </div>
                    </ListItem.Left>
                </ListItem>
            ),
        }))
    }, [editor, field, page])

    const filteredReferences = possibleReferences.filter((it) => it.page.includes(search)).map((it) => it.component)

    return (
        <div className="flex flex-col bg-[#262626]">
            <div className="flex-0 flex bg-white/[0.05] p-1.5">
                <BasaltInput value={search} onChange={(event) => setSearch(event.target.value)} />
            </div>
            <div className="flex flex-1 flex-col p-1.5">
                <ListItem.Section title="Reference a page" withDivider={false} />
                {filteredReferences}
            </div>
        </div>
    )
}
*/
