/**
 * We need to think about the type of elements we have to start organizing things.
 * Because this is an opinionated ui kit, there will be different "categories" of
 * elements that we need to deal with that shouldn't all sit under the same umbrella.
 *
 * Components
 *   - Layout
 *   - Display
 *   - Input
 *   - Feedback
 *   - Typography
 *   - Overlay
 *   - Navigation
 *
 *   - Media & Icons
 */

// TODO: Asset
// import "./reset.css"

export { Popover, PopoverContext, VisuallyHidden } from "react-aria-components"
export { GridList, GridListItem, useDragAndDrop, isFileDropItem } from "react-aria-components"

///
///
///
export { RouterProvider } from "./components/RouterProvider"

export * from "./components/Box/Box"
export * from "./components/Calendar/Calendar"
export * from "./components/Checkbox/Checkbox"
export * from "./components/Flex/Flex"

export * from "./components/Button/Button"
export * from "./components/Card/Card"
export * from "./components/Image/Image"

export * from "./components/Dialog/Dialog"
export * from "./components/Divider/Divider"
export * from "./components/Form/Form"
// export * from "./components/GridList/GridList"
export * from "./components/Heading/Heading"
export * from "./components/Input/Input"
export * from "./components/Link/Link"
export * from "./components/Menu/Menu"
export * from "./components/Slider/Slider"
// export * from "./components/Switch/Switch"
export * from "./components/Modal/Modal"
export * from "./components/Tabs/Tabs"
export * from "./components/TextField/TextField"
export * from "./components/Chip/Chip"
export * from "./components/ColorPicker/ColorPicker"
export * from "./components/ContextMenu/ContextMenu"

export * from "./components/DatePicker/DatePicker"
// export * from "./components/Header/primitive/Header"
// export * from "./components/InlineMenu/InlineMenu"
export * from "./components/ListBox/ListBox"
export * from "./components/Select/Select"
export * from "./components/Sidebar/Sidebar"
export * from "./components/Table/Table"
export * from "./components/Toolbar/Toolbar"
export * from "./components/Typography/Typography"

///
///
///
export * from "./hooks/context"
export * from "./hooks/useComposedRefs"
// export * from "./hooks/useContextMenu"
// export * from "./hooks/useDragAndDrop"
export * from "./hooks/useObjectRef"
export * from "./hooks/useRenderProps"

///
///
///
// export * from "./theme/index"
// export * from "./theme/themes"
export * from "./theme/props"

// ///
// ///
// ///
// export * from "./utils/collection/Document"
// export * from "./utils/collection/hooks"
export * from "./utils/context"
export * from "./utils/mergeProps"
// export * from "./utils/props"
// export * from "./utils/refs"
