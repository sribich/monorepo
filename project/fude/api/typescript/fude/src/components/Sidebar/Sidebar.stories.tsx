import type { Meta, StoryObj } from "@storybook/react-vite"
import { CatIcon, SquareTerminal, SquareTerminalIcon, Terminal } from "lucide-react"
import { stylesToArgTypes } from "./../../theme/props"
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuContent,
    SidebarMenuItem,
    SidebarMenuTrigger,
    SidebarProvider,
    SidebarRail,
    SidebarTrigger,
} from "./Sidebar"
import { sidebarStyles } from "./Sidebar.stylex"
import { Button } from "../Button/Button"

type Story = StoryObj<typeof meta>

const meta = {
    argTypes: stylesToArgTypes(sidebarStyles),
} satisfies Meta<typeof Sidebar>

export default meta

export const Default: Story = {
    render: (props) => (
        <>
            <SidebarProvider>
                <div style={{ display: "flex", height: "100%" }}>
                    <div style={{ flex: "0 1 auto" }}>
                        <Sidebar {...props}>
                            <SidebarRail />
                            <SidebarHeader>header</SidebarHeader>
                            <SidebarContent>
                                <SidebarGroup label="components">
                                    <SidebarMenu>
                                        <SidebarMenuItem>
                                            <SidebarMenuTrigger icon={<CatIcon />}>
                                                <span>trigger</span>
                                            </SidebarMenuTrigger>
                                            <SidebarMenuContent>
                                                <SidebarMenuButton>Hi :)</SidebarMenuButton>
                                            </SidebarMenuContent>
                                        </SidebarMenuItem>
                                    </SidebarMenu>
                                </SidebarGroup>
                                <SidebarGroup label="components">
                                    <SidebarMenu>
                                        <SidebarMenuItem>
                                            <SidebarMenuButton icon={<CatIcon />}>
                                                First
                                            </SidebarMenuButton>
                                        </SidebarMenuItem>
                                        <SidebarMenuItem>
                                            <SidebarMenuButton icon={<SquareTerminalIcon />}>
                                                Second
                                            </SidebarMenuButton>
                                        </SidebarMenuItem>
                                    </SidebarMenu>
                                </SidebarGroup>
                            </SidebarContent>

                            <SidebarFooter>footer</SidebarFooter>
                        </Sidebar>
                    </div>
                    <div style={{ flex: "1 0 auto", fontFamily: "Inter" }}>
                        <SidebarTrigger>
                            <Button>Toggle Sidebar</Button>
                        </SidebarTrigger>
                        Content
                        <SquareTerminalIcon size={16} />
                    </div>
                </div>
            </SidebarProvider>
            <SquareTerminalIcon size={16} />
        </>
    ),
}
