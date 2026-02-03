import {
    Sidebar as _Sidebar,
    Button,
    DelegateButton,
    Link,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarTrigger,
    SidebarRail,
} from "@sribich/fude"
import { BookA, GraduationCap, Library, Settings } from "lucide-react"

export const Sidebar = () => {
    return (
        <_Sidebar variant="inline" collapsible="full">
            <SidebarHeader>
                <SidebarTrigger>
                    <Button>...</Button>
                </SidebarTrigger>
            </SidebarHeader>
            <SidebarGroup label="Study">
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton icon={<GraduationCap />} href="/study">
                            Study
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarGroup>
            <SidebarContent>
                <SidebarGroup label="Library">
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton icon={<Library />} href="/library">
                                Library
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                            <SidebarMenuButton
                                icon={<BookA />}
                                href="/dictionary"
                                style={{ display: "flex", alignItems: "center" }}
                            >
                                <span style={{ flexGrow: 1 }}>Dictionary</span>
                            </SidebarMenuButton>
                            <span>
                                <DelegateButton size="xs">
                                    <Link href="/dictionary/settings">
                                        <Settings size="16" />
                                    </Link>
                                </DelegateButton>
                            </span>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroup>
                <SidebarGroup label="WIP">
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton href="/immerse">Immerse</SidebarMenuButton>
                        </SidebarMenuItem>
                    </SidebarMenu>
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton href="/anki">Anki Companion</SidebarMenuButton>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroup>
            </SidebarContent>
            <SidebarFooter>
                <DelegateButton fullWidth variant="light">
                    <Link href="/settings">
                        <Settings />
                        Settings
                    </Link>
                </DelegateButton>
            </SidebarFooter>
            <SidebarRail />
        </_Sidebar>
    )
}
