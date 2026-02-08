import { create, props as stylexProps } from "@stylexjs/stylex"
import { useMemo } from "react"

const { style } = create({
    style: {
        margin: 0,
        padding: 0,
        fontFamily: "figtree",
        fontSize: "32px",
        fontWeight: 600,
        letterSpacing: "-.23px",
    },
})

//==============================================================================
// SectionName
//==============================================================================
export namespace SectionName {
    export interface Props {
        children: string
        className: string
    }
}

export const SectionName = (props: SectionName.Props) => {
    const id = useMemo(() => {
        return props.children.toLowerCase().replace(/[’']/g, "").split(" ").join("-")
    }, [props.children])

    return (
        <h2 id={id} {...stylexProps(style)}>
            {props.children}
        </h2>
    )
}
