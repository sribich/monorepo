import { use, useState } from "react"

import { getTitle } from "../../../generated/rpc-client/library_GetTitle"
import { ApiHostContext } from "../../../hooks/useApiPort"
import { reprocessSync } from "../../../generated/rpc-client/library_ReprocessSync"
import { Button } from "@sribich/fude"

export namespace LibraryEditPage {
    export interface Props {
        bookId: string
    }
}

export const LibraryEditPage = (props: LibraryEditPage.Props) => {
    const { host } = use(ApiHostContext)

    // const { data, isLoading } = getTitle([props.bookId], {
    //     titleId: props.bookId,
    // })

    const { mutateAsync } = reprocessSync([props.bookId], {})

    // const editTitleMutation = editTitle()

    const mutate = () => {
        mutateAsync({ bookId: props.bookId })
    }

    return (
        <>
            <Button onPress={mutate}>
                Reprocess
            </Button>
        </>
    )

    const submit = async (data: FormData) => {
        // const file = data.get("file") as File

        await fetch(`${host}/rpc/edit_title/${props.mediaId}`, {
            method: "POST",
            body: data,
        })
    }

    const submitChunked = async (data: FormData) => {
        const chunkSize = 5 * 1024 * 1024 // 5MB per chunk
        const file = data.get("file") as File
        const totalChunks = Math.ceil(file.size / chunkSize)
        const uploadId = Date.now().toString() // unique id for this upload

        for (let i = 0; i < totalChunks; i++) {
            const start = i * chunkSize
            const end = Math.min(start + chunkSize, file.size)
            const chunk = file.slice(start, end)

            const formData = new FormData()
            formData.append("chunk", chunk)
            formData.append("index", i)
            formData.append("total", totalChunks)
            formData.append("uploadId", uploadId)
            formData.append("filename", file.name)

            await fetch(`${host}/rpc/edit_title/123`, {
                method: "POST",
                body: formData,
            })
        }

        console.log("Upload complete")
    }

    return (
        <>
            <div>Edit {data?.title}</div>
            <form action={submit}>
                <input name="file" type="file" />
                <button type="submit">Submit</button>
            </form>
            {data && data.imageId && <img src={`${host}/rpc/resource/${data.imageId}`} />}
        </>
    )
}
