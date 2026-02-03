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

    useLayoutEffect(() => {
        setShown(false)
    }, [data])

    if (isLoading) {
        return null
    }

    if (!data || !data.review) {
        return <NoMoreReviewsPage />
    }

    return shown ? (
        <CardBack review={data.review} />
    ) : (
        <CardFront review={data.review} reveal={() => setShown(true)} />
    )
}
