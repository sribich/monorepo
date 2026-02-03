//==============================================================================
// DictionaryLookupPage
//==============================================================================
export namespace DictionarySettingsPage {
    export type Props = Record<string, never>
}

export const DictionarySettingsPage = (props: DictionarySettingsPage.Props) => {
    return null
}

/*
import { Button, GridList, GridListItem, Tab, Tabs, useDragAndDrop, useListData } from "@sribich/fude"
import { useMutation, useQuery } from "@tanstack/react-query"
import { createFileRoute } from "@tanstack/react-router"
import { useEffect } from "react"
import { importDictionary } from "../../../generated/rpc-client/dictionary_ImportDictionary"
import { listDictionaries } from "../../../generated/rpc-client/dictionary_ListDictionaries"

export const Route = createFileRoute("/_app/dictionary/settings")({
    component: () => <DictionarySettings />,
})

//==============================================================================
//
//==============================================================================
const DictionarySettings = () => {
    return (
        <Tabs>
            <Tab id="monolingual" title="Monolingual">
                <DictionaryTab kind="monolingual" />
            </Tab>
            <Tab id="bilingual" title="Bilingual">
                <DictionaryTab kind="bilingual" />
            </Tab>
            <Tab id="kanji" title="Kanji">
                <DictionaryTab kind="bilingual" />
            </Tab>
            <Tab id="names" title="Names">
                <DictionaryTab kind="bilingual" />
            </Tab>
            <Tab id="frequency" title="Frequency">
                <DictionaryTab kind="frequency" />
            </Tab>
            <Tab id="pitchaccent" title="Pitch Accent">
                <DictionaryTab kind="pitchAccent" />
            </Tab>
            <Tab id="Grammar" title="Grammar">
                <DictionaryTab kind="bilingual" />
            </Tab>
        </Tabs>
    )
}

//==============================================================================
//
//==============================================================================
namespace DictionaryTab {
    export interface Props {
        kind: ImportDictionaryKind
    }
}

const DictionaryTab = (props: DictionaryTab.Props) => {
    return (
        <div>
            <div>...</div>
            <div>
                <ImportDictionary kind={props.kind} />
                <ListDictionaries kind={props.kind} />
            </div>
        </div>
    )
}

//==============================================================================
//
//==============================================================================
namespace ImportDictionary {
    export interface Props {
        kind: ImportDictionaryKind
    }
}

const ImportDictionary = (props: ImportDictionary.Props) => {
    const { mutateAsync } = importDictionary(["import_dictionary"])

    const doImportDictionary = async () => {
        const result = await open()

        if (result) {
            await mutateAsync({
                path: typeof result === "string" ? result : result.path,
            })
        }
    }

    return (
        <Button onPress={doImportDictionary} color="primary">
            Import
        </Button>
    )
}

//==============================================================================
// ListDictionaries
//==============================================================================
namespace ListDictionaries {
    export interface Props {
        kind: ImportDictionaryKind
    }
}

const ListDictionaries = (props: ListDictionaries.Props) => {
    const list = useListData({
        initialItems: [],
    })

    const { data, isSuccess } = listDictionaries(["TOOD"], {})

    useEffect(() => {
        if (isSuccess) {
            list.append(...(data.dictionaries ?? []))
        }
    }, [isSuccess])

    const { dragAndDropHooks } = useDragAndDrop({
        getItems(keys) {
            return Array.from(keys).map((key) => {
                return {
                    "text/plain": list.getItem(key).title,
                }
            })
        },
        onReorder(event) {
            console.log("reorder")
            if (event.target.dropPosition === "before") {
                list.moveBefore(event.target.key, event.keys)
            } else if (event.target.dropPosition === "after") {
                list.moveAfter(event.target.key, event.keys)
            }
        },
    })

    return (
        <GridList aria-label="Dictionaries" items={list.items} dragAndDropHooks={dragAndDropHooks}>
            {(item) => (
                <GridListItem id={item.id}>
                    {item.title}
                    <Button>Test</Button>
                </GridListItem>
            )}
        </GridList>
    )
}

*/
