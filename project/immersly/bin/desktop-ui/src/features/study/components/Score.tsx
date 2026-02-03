import type { ReactNode } from "react"
import {
    reviewCard,
    type Card,
    type NextState,
} from "../../../generated/rpc-client/scheduler_reviewCard"
import { Button, makeStyles, useStyles } from "@sribich/fude"
import { create } from "@stylexjs/stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { answerCard } from "../../../generated/rpc-client/scheduler_answerCard"
import { queryClient } from "../../../query-client"

//==============================================================================
// Score
//==============================================================================
export namespace Score {
    export interface Props {
        children: ReactNode
        card: Card
        state: NextState
    }
}

export const Score = (props: Score.Props) => {
    const { mutateAsync } = answerCard([], {
        onSuccess: () => {
            queryClient.invalidateQueries({
                queryKey: ["review_card"],
            })
        },
    })

    const { styles } = useStyles(scoreStyles, {})

    const onPress = () => {
        mutateAsync({
            cardId: props.card.id,
            nextState: props.state,
        })
    }

    /*
    let time = Math.round(interval * 24 * 60);
        let unit = "m";

        if (time > 180) {
            time = Math.round(time / 60);
            unit = "h";

            if (time > 48) {
                time = Math.round(time / 24);
                unit = "d";
            }
        }
    */

    return (
        <div {...styles.rank()}>
            <span {...styles.interval()}>
                {"interval" in props.state
                    ? `${props.state.interval}min`
                    : `${props.state.interval_days}d`}
            </span>
            <Button {...styles.button()} size="sm" onPress={onPress}>
                {props.children}
            </Button>
        </div>
    )
}

const scoreStyles = makeStyles({
    slots: create({
        scoreList: {
            display: "flex",
            flexDirection: "row",
            justifyContent: "center",
            gap: "8px",
        },
        rank: {
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
        },
        interval: {
            fontSize: fontSize.sm,
            color: "#5d5d5d",
        },
        button: {
            minWidth: newSpacing["64"],
            justifyContent: "center",
        },
    }),
    variants: {},
    defaultVariants: {},
})
