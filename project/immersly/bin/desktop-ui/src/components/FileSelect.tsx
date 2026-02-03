import { Button, FormContext, Input, makeStyles, useStyles, VisuallyHidden } from "@sribich/fude"
import { create } from "@stylexjs/stylex"
import { pickFile } from "../generated/rpc-client/storage_PickFile"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { use } from "react"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"

export namespace FileSelect {
    export interface Props {
        name: string
        text: string
    }
}

export const FileSelect = (props: FileSelect.Props) => {
    const picker = pickFile(["pickFile"], {})

    const onClick = () => {
        picker.mutateAsync({})
    }

    const { styles } = useStyles(fileSelectStyles, {})

    const filePath = picker.data?.path ?? ""
    const fileName = filePath.split(/[\\/]/).pop()

    return (
        <div {...styles.wrapper()}>
            <Button onClick={onClick} size="md">
                {props.text}
            </Button>
            <span {...styles.span()}>{fileName}</span>

            <VisuallyHidden>
                <Input name={props.name} value={filePath} />
            </VisuallyHidden>
        </div>
    )
}

const fileSelectStyles = makeStyles({
    slots: create({
        wrapper: {
            width: "100%",
            display: "flex",
            alignItems: "center",
            flexDirection: "row",
            justifyContent: "start",
            gap: newSpacing["8"],
        },
        span: {
            color: colors.secondaryForeground,
            textOverflow: "ellipsis",
            overflow: "hidden",
        },
    }),
    variants: {},
    defaultVariants: {},
})
