import { Pressable, PressResponder } from "@react-aria/interactions"
import type { FocusableElement } from "@react-types/shared"
import { parentMarker, sidebarMenuItemMarker } from "@sribich/fude-theme/markers.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { newSpacing, spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"
import { ChevronRight } from "lucide-react"
import { createContext, type ReactNode, use, useMemo } from "react"
import type { AriaLinkOptions, PressProps } from "react-aria"
import { useLink } from "react-aria"

import { createNewGenericContext } from "../../hooks/context"
import { useIsMobileScreen } from "../../hooks/useIsMobile"
import { useIsMounted } from "../../hooks/useIsMounted"
import { useObjectRef } from "../../hooks/useObjectRef"
import { usePersistentToggle } from "../../hooks/usePersistentToggle"
import { useToggle } from "../../hooks/useToggle"
import { transitionStyles } from "../../theme/atomics/transition"
import { makeStyles, useStyles, type VariantProps } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import { DelegateButton } from "../Button/Button"
import { RouterProviderContext } from "../RouterProvider"
import { SidebarStyles, sidebarStyles } from "./Sidebar.stylex"

//==============================================================================
// SidebarProvider
//==============================================================================
const SidebarState = createContext<SidebarProvider.State>(undefined as never)

export namespace SidebarProvider {
    export interface State {
        isMobile: boolean
        isOpen: boolean
        toggleOpen: () => void
    }

    export interface Props {
        children: ReactNode
        // Whether the sidebar should be opened by default.
        //
        // @default true
        defaultOpen?: boolean
    }
}

export const SidebarProvider = (props: SidebarProvider.Props) => {
    const isMobile = useIsMobileScreen()

    const [isMobileOpen, toggleMobileOpen] = useToggle()
    const [isOpen, toggleOpen] = usePersistentToggle("sidebar:open", props.defaultOpen ?? true)

    const _isOpen = isMobile ? isMobileOpen : isOpen
    const _toggleOpen = isMobile ? toggleMobileOpen : toggleOpen

    const contextValue = useMemo<SidebarProvider.State>(
        () => ({
            isMobile,
            isOpen: _isOpen,
            toggleOpen: _toggleOpen,
        }),
        [_isOpen, _toggleOpen],
    )

    const { styles } = useStyles(sidebarProviderStyles, props)

    return (
        <SidebarState value={contextValue}>
            <div {...styles.container()}>{props.children}</div>
        </SidebarState>
    )
}

const sidebarProviderStyles = makeStyles({
    slots: create({
        container: {
            display: "flex",
            height: "100%",
            width: "100%",
            "--sidebar-width": newSpacing["224"],
            "--sidebar-width-icon": newSpacing["48"],
        },
    }),
    variants: {},
    defaultVariants: {},
})

//==============================================================================
// SidebarTrigger
//==============================================================================
export namespace SidebarTrigger {
    export interface Props {
        children: ReactNode
    }
}

export const SidebarTrigger = (props: SidebarTrigger.Props) => {
    const state = use(SidebarState)

    return <DelegateButton onPress={state.toggleOpen}>{props.children}</DelegateButton>
}

/*
const SidebarTrigger = React.forwardRef<
  React.ElementRef<typeof Button>,
  React.ComponentProps<typeof Button>
>(({ className, onClick, ...props }, ref) => {
  const { toggleSidebar } = useSidebar()

  return (
    <Button
      ref={ref}
      data-sidebar="trigger"
      variant="ghost"
      size="icon"
      className={cn("h-7 w-7", className)}
      onClick={(event) => {
        onClick?.(event)
        toggleSidebar()
      }}
      {...props}
    >
      <PanelLeft />
      <span className="sr-only">Toggle Sidebar</span>
    </Button>
  )
})
*/

//==============================================================================
// Sidebar
//==============================================================================
export namespace Sidebar {
    export interface Props extends VariantProps<typeof sidebarStyles> {
        children: ReactNode
    }
}

export const Sidebar = (props: Sidebar.Props) => {
    const isMobile = useIsMobileScreen()

    const state = use(SidebarState)

    const styles = useStyles(sidebarStyles, {
        ...props,
        collapsed: !state.isOpen,
    })

    return (
        <SidebarStyles value={styles}>
            <div {...mergeProps(styles.styles.sidebar(), parentMarker)}>
                <div {...styles.styles.sizeKeeper()} />
                <div {...styles.styles.sidebarBody(transitionStyles.movement)}>
                    <div {...styles.styles.sidebarContent()}>{props.children}</div>
                </div>
            </div>
        </SidebarStyles>
    )
}

//==============================================================================
// SidebarHeader
//==============================================================================
export namespace SidebarHeader {
    export interface Props {
        children: ReactNode
    }
}

export const SidebarHeader = (props: SidebarHeader.Props) => {
    const { styles } = use(SidebarStyles)

    return <div {...styles.header()}>{props.children}</div>
}

//==============================================================================
// SidebarFooter
//==============================================================================
export namespace SidebarFooter {
    export interface Props {
        children: ReactNode
    }
}

export const SidebarFooter = (props: SidebarFooter.Props) => {
    const { styles } = use(SidebarStyles)

    return <div {...styles.footer()}>{props.children}</div>
}

//==============================================================================
// SidebarContent
//==============================================================================
export namespace SidebarContent {
    export interface Props {
        children: ReactNode
    }
}

export const SidebarContent = (props: SidebarContent.Props) => {
    const { styles } = use(SidebarStyles)

    return <div {...styles.content()}>{props.children}</div>
}

//==============================================================================
// SidebarGroup
//==============================================================================
export namespace SidebarGroup {
    export interface Props {
        children: ReactNode
        label: ReactNode
        action?: ReactNode
    }
}

export const SidebarGroup = (props: SidebarGroup.Props) => {
    const { styles } = use(SidebarStyles)

    return (
        <div {...styles.group()}>
            <div {...styles.groupLabel(transitionStyles.movement)}>{props.label}</div>
            <div {...styles.groupContent()}>{props.children}</div>
        </div>
    )
}

//==============================================================================
// SidebarMenu
//==============================================================================
export namespace SidebarMenu {
    export interface Props {
        children: ReactNode
    }
}

export const SidebarMenu = (props: SidebarMenu.Props) => {
    const { styles } = use(SidebarStyles)

    return <div {...styles.menu()}>{props.children}</div>
}

//==============================================================================
// SidebarMenuItem
//==============================================================================
const SidebarMenuItemState = createNewGenericContext<SidebarMenuItem.State>()

export namespace SidebarMenuItem {
    export interface Props {
        children: ReactNode
    }

    export interface State {
        open: boolean
    }
}

export const SidebarMenuItem = (props: SidebarMenuItem.Props) => {
    const { styles } = use(SidebarStyles)

    const [open, toggleOpen] = useToggle()

    return (
        <SidebarMenuItemState value={{ open }}>
            <PressResponder onPress={toggleOpen}>
                <div {...styles.menuItem(sidebarMenuItemMarker)}>{props.children}</div>
            </PressResponder>
        </SidebarMenuItemState>
    )
}

//==============================================================================
// SidebarMenuButton
//==============================================================================
export namespace SidebarMenuButton {
    export interface Props extends PressProps, Omit<AriaLinkOptions, "elementType"> {
        icon?: ReactNode
        children: ReactNode
    }
}

export const SidebarMenuButton = (props: SidebarMenuButton.Props) => {
    const state = use(SidebarState)
    const { styles } = SidebarStyles.use()

    const ref = useObjectRef<FocusableElement>()

    const { linkProps, isPressed } = useLink({ elementType: "a", ...props }, ref)

    const Tag = props.href ? "a" : "button"

    const useLocation = use(RouterProviderContext)
    const pathname = useLocation().pathname

    return (
        <Tag
            {...mergeProps(
                styles.menuButton(pathname === props.href && styles.menuButton.pathSelected),
                linkProps,
                transitionStyles.movement,
            )}
        >
            {props.icon}
            {props.children}
        </Tag>
    )
}

//==============================================================================
// SidebarMenuTrigger
//==============================================================================
export namespace SidebarMenuTrigger {
    export interface Props {
        icon?: ReactNode
        children: ReactNode
    }
}

export const SidebarMenuTrigger = (props: SidebarMenuTrigger.Props) => {
    const state = SidebarMenuItemState.useContext()
    const { styles } = SidebarStyles.use()

    return (
        <Pressable>
            <button {...mergeProps(styles.menuTrigger())}>
                <span {...styles.menuTriggerIcon()}>{props.icon}</span>
                <div {...styles.menuTriggerContent()}>{props.children}</div>
                <ChevronRight
                    {...styles.menuTriggerChevron(state.open && styles.menuTriggerChevron.menuOpen)}
                />
            </button>
        </Pressable>
    )
}

//==============================================================================
// SidebarMenuContent
//==============================================================================
export namespace SidebarMenuContent {
    export interface Props {
        children: ReactNode
    }
}

export const SidebarMenuContent = (props: SidebarMenuContent.Props) => {
    const { styles } = SidebarStyles.use()

    const isMounted = useIsMounted()
    const menuState = SidebarMenuItemState.useContext()

    const { styles: styles2 } = useStyles(sidebarMenuContentStyles, {
        ...props,
        hidden: !menuState.open,
        mounted: isMounted,
    })

    return <div {...styles2.container()}>{props.children}</div>
}

const sidebarMenuContentStyles = makeStyles({
    slots: create({
        container: {
            marginLeft: spacing["3"],
            display: "flex",
            flexDirection: "column",
            gap: spacing["1"],
            borderLeftColor: colors.borderUi,
            borderLeftWidth: "1px",
            borderLeftStyle: "solid",
            // ...include({ ...padding["x-2"], ...padding["y-0.5"] }),
            // "group-data-[collapsible=icon]:hidden",

            // Prestate
            opacity: 0,
            transform: "rotateX(-90deg)",
            transformOrigin: "top center",
            maxHeight: "100vh",
        },
    }),
    conditions: {
        mounted: {
            true: create({
                container: {
                    transition: "opacity 200ms ease, transform 200ms ease, max-height 200ms ease",
                },
            }),
        },
        hidden: {
            true: create({
                container: {
                    pointerEvents: "none",
                    maxHeight: 0,
                },
            }),
            false: create({
                container: {
                    opacity: 1,
                    transform: "none",
                },
            }),
        },
    },
    variants: {},
    defaultVariants: {},
})

// <Sidebar>
//     <SidebarHeader>
//     </SidebarHeader>
//     <SidebarContent>
//     </SidebarContent>
//     <SidebarFooter>
//     </SidebarFooter>
// </Sidebar>

//
//
//
export namespace SidebarRail {}

export const SidebarRail = () => {
    const { styles } = use(SidebarStyles)
    const { toggleOpen } = use(SidebarState)

    return <button {...styles.rail(transitionStyles.movement)} onClick={toggleOpen} />
}
