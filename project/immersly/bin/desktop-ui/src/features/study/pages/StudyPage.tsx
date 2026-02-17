import { makeStyles, useStyles } from "@sribich/fude"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"
import { useLayoutEffect, useState } from "react"

import { reviewCard } from "../../../generated/rpc-client/scheduler_reviewCard"
import { CardBack } from "../components/CardBack"
import { CardFront } from "../components/CardFront"
import { NoMoreReviewsPage } from "./NoMoreReviewsPage"

//==============================================================================
// StudyPage
//==============================================================================
export const StudyPage = () => {
    const [shown, setShown] = useState(false)

    const { data, isLoading } = reviewCard(["review_card"], {})

    const { styles } = useStyles(studyPageStyles, {})

    useLayoutEffect(() => {
        setShown(false)
    }, [data])

    if (isLoading) {
        return null
    }

    if (!data || !data.review) {
        return <NoMoreReviewsPage />
    }

    return (
        <div {...styles.wrapper()}>
            {shown ? (
                <CardBack review={data.review} />
            ) : (
                <CardFront review={data.review} reveal={() => setShown(true)} />
            )}
        </div>
    )
}

export const studyPageStyles = makeStyles({
    slots: create({
        wrapper: {
            height: "100%",
            width: "100%",
            padding: newSpacing["8"],
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
