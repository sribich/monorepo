import type { JoinUnion } from "@sribich/ts-utils"
import { scope } from "arktype"

import { extract } from "../../ui/util/array"
import { properties } from "./property"

export const { propertySchema } = scope({
    ...extract(properties, "kind", "config.schema"),
    propertySchema: Object.values(properties)
        .map((it) => it.kind)
        .join(" | ") as JoinUnion<(typeof properties)[number]["kind"], " | ">,
}).export()

export type PropertySchema = (typeof propertySchema)["infer"]
