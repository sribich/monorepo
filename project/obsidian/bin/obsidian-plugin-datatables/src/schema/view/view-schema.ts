import type { JoinUnion } from "@sribich/ts-utils"
import { scope } from "arktype"

import { extract } from "../../ui/util/array"
import { views } from "./view"

export const { viewSchema } = scope({
    ...extract(views, "kind", "schema"),
    viewSchema: Object.values(views)
        .map((it) => it.kind)
        .join(" | ") as JoinUnion<(typeof views)[number]["kind"], " | ">,
}).export()

export type ViewSchema = (typeof viewSchema)["infer"]
