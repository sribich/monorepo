import { Card, makeStyles, useStyles } from "@sribich/fude"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"
import { Text } from "../../../components/Text"
import { getExactWord } from "../../../generated/rpc-client/dictionary_GetExactWord"
import type { Review } from "../../../generated/rpc-client/scheduler_reviewCard"
import { ReviewWord } from "./ReviewWord"
import { Scores } from "./Scores"
import { use, useState } from "react"

//==============================================================================
// CardBack
//==============================================================================
export namespace CardBack {
    export interface Props {
        review: Review
    }
}

export const CardBack = (props: CardBack.Props) => {
    const { card } = props.review

    const { styles } = useStyles(componentStyles, {})

    const { data, isLoading } = getExactWord([card.word, card.reading], {
        word: card.word,
        reading: card.reading,
    })

    if (isLoading) {
        return null
    }

    if (!data?.word) {
        return null
    }

    const [definition, ...otherDefinitions] = data.word.definitions

    return (
        <div {...styles.container()}>
            <div {...styles.card()}>
                <ReviewWord card={card} word={data.word} />
                <ReviewImage card={card} />
            </div>

            <div {...styles.body()}>
                <Text language="jp">{card.sentence}</Text>

                <Card>
                    <Definition definition={definition} />
                </Card>

                <Card>
                    <Definition definition={data.word.bilingual_definition} />
                </Card>
            </div>

            <div {...styles.scores()}>
                <Scores review={props.review} />
            </div>
        </div>
    )
}

interface ImageItem {
    id: number
    url: string
    name: string
}
import { useDragAndDrop, isFileDropItem, GridList, GridListItem } from "@sribich/fude"
import { addCardImage } from "../../../generated/rpc-client/scheduler_addCardImage"
import { ApiHostContext } from "../../../hooks/useApiPort"

const ReviewImage = ({ card }: { card: Review["card"] }) => {
    /*
    import {GridList, GridListItem} from './GridList';
import {useDragAndDrop, isFileDropItem, Text} from 'react-aria-components';
import {useState} from 'react';

interface ImageItem {
  id: number,
  url: string,
  name: string
}

function DroppableGridList() {
*/
    const { host } = use(ApiHostContext)

    if (card.image_id) {
        return (
            <img
                src={`${host}/rpc/scheduler:playAudio/${card.id}/image`}
                style={{ minWidth: 0, flex: "1 1 auto" }}
            />
        )
    }

    const [items, setItems] = useState<ImageItem[]>([])
    const { mutateAsync } = addCardImage([])

    const { dragAndDropHooks } = useDragAndDrop({
        acceptedDragTypes: ["image/jpeg", "image/png"],
        async onRootDrop(e) {
            let items = await Promise.all(
                e.items.filter(isFileDropItem).map(async (item) => {
                    const file = await item.getFile()
                    const url = URL.createObjectURL(file)

                    const blob = file as Blob
                    const reader = new FileReader()

                    reader.readAsDataURL(blob)

                    const data = await new Promise((resolve) => {
                        reader.onloadend = () => {
                            const dataUrlPrefix = `data:${file.type};base64,`
                            const base64WithDataUrlPrefix = reader.result

                            const base64 = base64WithDataUrlPrefix.replace(dataUrlPrefix, "")

                            resolve(base64)
                        }
                    })

                    await mutateAsync({
                        id: card.id,
                        image: data,
                        mimeType: file.type,
                    })

                    return {
                        id: Math.random(),
                        url,
                        name: item.name,
                    }
                }),
            )
            setItems(items)
        },
    })

    return (
        <GridList
            aria-label="Droppable list"
            items={items}
            dragAndDropHooks={dragAndDropHooks}
            renderEmptyState={() => "Drop images here"}
            style={{ height: 250 }}
            data-size="small"
        >
            {(item) => (
                <GridListItem textValue={item.name}>
                    <img src={item.url} style={{ minWidth: 0, flex: "1 1 auto" }} />
                    <Text>{item.name}</Text>
                </GridListItem>
            )}
        </GridList>
    )
}

const componentStyles = makeStyles({
    slots: create({
        container: {
            display: "flex",
            flexDirection: "column",
            // justifyContent: "center",
            // alignItems: "center",
            height: "100%",
            width: "100%",
            maxWidth: newSpacing["768"],
        },
        card: {
            minWidth: newSpacing["384"],
            minHeight: newSpacing["112"],
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
        },
        word: {
            fontFamily: "Shippori Mincho",
            fontSize: fontSize["5xl"],
        },
        body: {
            flex: "1",
            overflow: "auto",
        },
        scores: {},
    }),
    variants: {},
    defaultVariants: {},
})

const Definition = (props) => {
    if (!props.definition) {
        return null
    }

    return <div dangerouslySetInnerHTML={{ __html: props.definition.definition }} />
}
