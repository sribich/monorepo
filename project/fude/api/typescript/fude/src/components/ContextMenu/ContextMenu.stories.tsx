import { Button } from "../Button/Button"
import { Menu, MenuItem } from "../Menu/Menu"
import { Popover } from "../Popover/Popover"
import { ContextMenu } from "./ContextMenu"

export default {
    args: {},
    argTypes: {},
}

export const Overview = (props) => (
    <ContextMenu {...props}>
        <Button>Test</Button>
        <Popover>
            <Menu>
                <MenuItem>A</MenuItem>
                <MenuItem>B</MenuItem>
                <MenuItem>C</MenuItem>
            </Menu>
        </Popover>
    </ContextMenu>
)
