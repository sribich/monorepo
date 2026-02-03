import { useStyleProps, type StyleProps } from "@sribich/fude"

//==============================================================================
// Ruby
//==============================================================================
export namespace Ruby {
    export interface Props extends StyleProps {
        children: string
    }
}

export const Ruby = (props: Ruby.Props) => {
    const styleProps = useStyleProps(props, {})

    const rubyFragments = []

    let text = props.children

    while (true) {
        const startRubyPos = text.indexOf("[")

        if (startRubyPos === -1) {
            if (rubyFragments.length === 0) {
                return text
            }

            rubyFragments.push(text)
            break
        }

        rubyFragments.push(text.substring(0, startRubyPos))
        text = text.substring(startRubyPos + 1)

        const endRubyPos = text.indexOf("]")

        if (endRubyPos === -1) {
            rubyFragments.push(`[${text}`)
            break
        }

        const rtText = text.substring(0, endRubyPos)
        text = text.substring(endRubyPos + 1)

        if (rtText === "") {
            rubyFragments.push(<rt />)
            continue
        }

        rubyFragments.push(<rp>(</rp>)
        rubyFragments.push(<rt>{rtText}</rt>)
        rubyFragments.push(<rp>)</rp>)
    }

    return <ruby {...styleProps}>{rubyFragments}</ruby>
}
