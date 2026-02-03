import { Button, TextField } from "@sribich/fude"
import { useState } from "react"
import { scheduleCards } from "../../../generated/rpc-client/scheduler_scheduleCards"

//==============================================================================
// NoMoreReviewsPage
//==============================================================================
export namespace NoMoreReviewsPage {
    export type Props = Record<string, never>
}

export const NoMoreReviewsPage = (props: NoMoreReviewsPage.Props) => {
    const { mutateAsync } = scheduleCards(["schedule_cards"])

    const [value, setValue] = useState(1)

    const onPress = () => {
        mutateAsync({ count: Number(value) })
    }

    return (
        <>
            <div>You have finished all of your scheduled daily reviews.</div>
            <TextField value={value} onChange={setValue} />
            <Button onPress={onPress} />
        </>
    )
}
