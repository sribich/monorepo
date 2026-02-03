import { makeStyles, useStyles } from "@sribich/fude"
import type { Review } from "../../../generated/rpc-client/scheduler_reviewCard"
import { create } from "@stylexjs/stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { Score } from "./Score"

//==============================================================================
// Scores
//==============================================================================
export namespace Scores {
    export interface Props {
        review: Review
    }
}

export const Scores = (props: Scores.Props) => {
    const { card, nextStates } = props.review

    const { styles } = useStyles(scoresStyles)

    return (
        <div {...styles.scoreList()}>
            <Score card={card} state={nextStates.again}>
                Again
            </Score>
            <Score card={card} state={nextStates.hard}>
                Hard
            </Score>
            <Score card={card} state={nextStates.good}>
                Good
            </Score>
            <Score card={card} state={nextStates.easy}>
                Easy
            </Score>
        </div>
    )
}

const scoresStyles = makeStyles({
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
            minWidth: spacing["16"],
            justifyContent: "center",
        },
    }),
    variants: {},
    defaultVariants: {},
})
