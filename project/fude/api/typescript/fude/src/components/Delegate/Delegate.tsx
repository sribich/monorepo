import {
    Children,
    type HTMLAttributes,
    type ReactElement,
    type ReactNode,
    type Ref,
    type RefObject,
    cloneElement,
    isValidElement,
} from "react"
import { mergeRefs } from "../../utils/refs"
import { mergeProps } from "../../utils/mergeProps"

export interface DelegateProps extends Omit<HTMLAttributes<HTMLElement>, "children"> {
    ref?: RefObject<HTMLElement>
    children?: ReactNode | null | undefined
}

export const Delegate = ({ children, ...props }: DelegateProps) => {
    if (Children.count(children) > 1) {
        throw new Error(`Only a single child may be passed to a delegated 'asChild' component`)
    }

    if (!isValidElement(children)) {
        throw new Error(
            `Passing a child of type ${typeof children} is not valid when using a delegated 'asChild' component.`,
        )
    }

    const element = children as ReactElement<{ ref?: Ref<HTMLElement> }>

    const composedRefs = mergeRefs(props.ref, element?.props?.ref)

    return cloneElement(element, {
        ...mergeProps(props, element.props),
        ref: composedRefs,
    })
}
