import { create, props as stylexProps } from "@stylexjs/stylex"

const { style } = create({
    style: {
        padding: 0,
        margin: 0,
        fontSize: "48px",
        letterSpacing: "-1px",
        fontWeight: 800,
        fontFamily: "figtree",
    },
})

//==============================================================================
// ComponentName
//==============================================================================
export namespace ComponentName {
    export interface Props {
        children: string
        className: string
    }
}

export const ComponentName = (props: ComponentName.Props) => {
    return (
        <h1 {...stylexProps(style)}>{props.children}</h1>
    )
}
