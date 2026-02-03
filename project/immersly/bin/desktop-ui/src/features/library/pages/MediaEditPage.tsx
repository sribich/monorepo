import { use, useState } from "react"
import { getTitle } from "../../../generated/rpc-client/library_GetTitle"
import { ApiHostContext } from "../../../hooks/useApiPort"

export namespace MediaEditPage {
    export interface Props {
        mediaId: string
    }
}

export const MediaEditPage = (props: MediaEditPage.Props) => {
    const { host } = use(ApiHostContext)

    const { data, isLoading } = getTitle([props.mediaId], {
        titleId: props.mediaId,
    })

    // const editTitleMutation = editTitle()

    if (isLoading) {
        return null
    }

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
