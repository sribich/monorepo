import { Button, Card, CardBody, makeStyles, useStyles } from "@sribich/fude"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"

import type { Review } from "../../../generated/rpc-client/scheduler_reviewCard"
import { useSingleKey } from "../../../hooks/useSingleKey"

//==============================================================================
// CardFront
//==============================================================================
export namespace CardFront {
    export interface Props {
        review: Review
        reveal: () => void
    }
}

export const CardFront = (props: CardFront.Props) => {
    useSingleKey(" ", props.reveal)

    const { styles } = useStyles(cardFrontStyles)

    return (
        <div {...styles.container()}>
            <Card>
                <CardBody>
                    <div {...styles.card()}>
                        <h1 {...styles.word()}>{props.review.card.word}</h1>
                        <Button onPress={props.reveal}>Reveal</Button>
                    </div>
                </CardBody>
            </Card>
        </div>
    )
}

const cardFrontStyles = makeStyles({
    slots: create({
        container: {
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            height: "100%",
            width: "100%",
        },
        card: {
            minWidth: newSpacing["384"],
            minHeight: newSpacing["112"],
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            justifyContent: "space-between",
        },
        word: {
            fontFamily: "Shippori Mincho",
            fontSize: fontSize["5xl"],
        },
    }),
    variants: {},
    defaultVariants: {},
})
