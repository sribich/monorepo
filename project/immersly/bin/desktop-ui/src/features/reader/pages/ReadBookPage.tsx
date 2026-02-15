import { Button, makeStyles, SidebarTrigger, useStyles } from "@sribich/fude"
import { create } from "@stylexjs/stylex"
import { Reader } from "../components/Reader"
import { useBook } from "../hooks/useBook"

//==============================================================================
// ReadBookPage
//==============================================================================
export namespace ReadBookPage {
    export interface Props {
        bookId: string
    }
}

export const ReadBookPage = (props: ReadBookPage.Props) => {
    const { entries, timestamp, bookAudioId, isLoading, error } = useBook(props.bookId)

    const { styles } = useStyles(readBookPageStyles, {})

    // TODO: isLoading, error
    if (!entries.length) {
        return null
    }

    return (
        <div {...styles.wrapper()}>
                <div {...styles.container()}>
                    <div {...styles.header()}>
                        <SidebarTrigger>
                            <Button>A</Button>
                        </SidebarTrigger>
                    </div>
                    <div {...styles.content()}>
                        <Reader
                        bookId={props.bookId}
                        bookAudioId={bookAudioId}
                        entries={entries}
                        timestamp={timestamp}
                    />
                    </div>
                </div>
        </div>
    )
}

const readBookPageStyles = makeStyles({
    slots: create({
        wrapper: {
            height: "100%",
            width: "100%",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
        },
        container: {
            height: "100%",
            width: "100%",
            display: "flex",
            flexDirection: "column",
            maxHeight: "1000px",
            maxWidth: "800px"
        },
        header: {
            height: "48px",
        },
        content: {
            flexGrow: 1,
        },
    }),
    variants: {},
    defaultVariants: {},
})
