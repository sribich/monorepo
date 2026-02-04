import { measurePerformance } from "reassure"

import { Menu } from "../Menu"

test("Simple test", async () => {
    await measurePerformance(<Menu />)
})
