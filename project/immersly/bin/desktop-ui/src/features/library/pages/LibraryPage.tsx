import {
    Box,
    Button,
    Card,
    CardHeader,
    CardBody,
    CardFooter,
    Image,
    CardView,
    DelegateButton,
    DialogTrigger,
    Flex,
    Form,
    Heading,
    Link,
    Modal,
    SidebarTrigger,
    TextField,
    makeStyles,
    useStyles,
} from "@sribich/fude"
import { cardMarker } from "@sribich/fude-theme/markers.stylex"
import { create, defaultMarker, when } from "@stylexjs/stylex"
import { Settings } from "lucide-react"
import { use } from "react"
import { type ListMediaResponse, listMedia } from "../../../generated/rpc-client/library_ListMedia"
import { ApiHostContext } from "../../../hooks/useApiPort"
import { FileSelect } from "../../../components/FileSelect"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { addBook } from "../../../generated/rpc-client/library_AddBook"
import { RouterProvider } from "react-aria"
import { useRouter } from "@tanstack/react-router"

//==============================================================================
// LibraryPage
//==============================================================================
export const LibraryPage = () => {
    const { data, isError, isLoading } = listMedia(["listMedia"], {})

    if (isError) {
        return <div>Error</div>
    }

    if (isLoading) {
        return null
    }

    return (
        <>
            <Flex direction="row" style={{ width: "100%" }}>
                <h1 style={{ flexGrow: 1 }}>Library</h1>
                <NewModal />

                <DelegateButton color="primary">
                    <Link href="/library/new">Add</Link>
                </DelegateButton>
            </Flex>
            <LibraryView media={data} />
        </>
    )
}

//==============================================================================
// LibraryView
//==============================================================================
namespace LibraryView {
    export interface Props {
        media: ListMediaResponse
    }
}

const LibraryView = (props: LibraryView.Props) => {
    const { navigate } = useRouter()

    const { host } = use(ApiHostContext)

    const { styles } = useStyles(anchorStyles, {})

    const cards = props.media.books.map((book) => {
        const onPress = () => {
            navigate({ to: `/library/${book.id}/read` })
        }

        return (
            <Card key={book.id} blurFooter footerStyle="sticky" isPressable onPress={onPress}>
                <SettingsAnchor seriesId={book.id} />
                <div {...styles.card()}>
                    {book.image_id && (
                        <Image src={`${host}/rpc/resource/${book.image_id}`} {...styles.image()} />
                    )}
                    <CardFooter>
                        <p {...styles.text()}>{book.title}</p>
                    </CardFooter>
                </div>
            </Card>
        )
    })

    return <CardView footerStyle="sticky">{cards}</CardView>
}

const SettingsAnchor = (props) => {
    const { styles } = useStyles(anchorStyles, {})

    return (
        <Box slot="menuArea" {...styles.anchor()}>
            <DelegateButton size="sm" iconOnly>
                <Link href={`/library/${props.seriesId}/edit`}>
                    <Settings size={16} />
                </Link>
            </DelegateButton>
        </Box>
    )
}

const anchorStyles = makeStyles({
    slots: create({
        card: {
            height: "300px",
            width: "200px",
        },
        anchorButton: {
            background: "transparent",
            backgroundColor: "red",
        },
        text: {
            textOverflow: "ellipsis",
            textWrap: "nowrap",
            overflow: "hidden",
        },
        image: {
            height: "100%",
            width: "100%",
            objectFit: "cover",
        },
        anchor: {
            display: {
                default: "none",
                // [when.ancestor(":hover")]: "block",
                [when.ancestor(":hover", cardMarker)]: "block",
            },

            /*

            position: "absolute",
            top: 100,
            right: 0,
            width: "fit-content",
            padding: 0,
            margin: 0,
            [when.ancestor(":hover", cardMarker)]: {
                display: "block"
            }
             */
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})

const NewModal = () => {
    return (
        <DialogTrigger>
            <Button>Open Me</Button>
            <Modal>
                <InnerModal />
            </Modal>
        </DialogTrigger>
    )
}

const InnerModal = () => {
    const test = addBook([])

    const submit = (data: FormData) => {
        console.log(data)
        console.log(data.entries())

        console.log(data, data.entries())

        test.mutate({ ...Object.fromEntries(data) })
    }

    const { styles } = useStyles(modalStyles, {})

    return (
        <div {...styles.modal()}>
            <Form action={submit}>
                <TextField label="Title" name="title" />

                <FileSelect name="bookPath" text="Select Book" />
                <FileSelect name="audioPath" text="Select Audio" />

                <Button type="submit" color="primary" {...styles.submit()}>
                    Add Book
                </Button>
            </Form>
        </div>
    )
}

const modalStyles = makeStyles({
    slots: create({
        modal: {
            borderColor: colors.borderUi,
            borderWidth: borderWidth.md,
            backgroundColor: colors.background,
            padding: newSpacing["8"],
            borderRadius: borderRadius.md,
            // width: "100%",
            width: "min(90vw, 450px)",
            maxHeight: "calc(100dvh * 0.9)",
        },
        submit: {
            marginLeft: "auto",
        },
    }),
    variants: {},
    defaultVariants: {},
})
