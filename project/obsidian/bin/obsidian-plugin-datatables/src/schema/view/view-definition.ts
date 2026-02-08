import { type Infer, type Type, type } from "arktype"

import { enumVariant } from "../../util/arktype"
import type { ViewKind } from "./view.kind"
import { type ViewScope, viewScope } from "./view.scope"

export type View<T> =
    T extends ViewDefinition<infer TKind, infer TOptions, infer TConfig>
        ? {
              kind: TKind
              name: string
              options: TOptions
              config: TConfig
          }
        : never

export interface ViewDefinition<TKind extends ViewKind, TLayout, TConfig> {
    kind: TKind
    name: string
    layout: {
        default: TLayout
        type: Type<TLayout>
    }
    scope: {
        default: ViewScope
        type: typeof viewScope
    }
    config: {
        default: TConfig
        type: Type<TConfig>
    }
    schema: Type<{
        name: string
        kind: TKind
        uuid: string
        layout: TLayout
        scope: ViewScope
        config: TConfig
    }>
}

export const makeView = <const TKind extends ViewKind>(kind: TKind) => {
    return <const TLayout, const TConfig>(definition: {
        name: string
        layout: {
            default: TLayout
            type: Type<TLayout>
        }
        config: {
            default: TConfig
            type: Type<TConfig>
        }
    }): ViewDefinition<TKind, TLayout, TConfig> => {
        return {
            kind: kind,
            name: definition.name,
            layout: {
                ...definition.layout,
            },
            scope: {
                default: {
                    properties: [],
                    filters: [],
                },
                type: viewScope,
            },
            config: {
                ...definition.config,
            },
            schema: type({
                name: "string",
                kind: enumVariant(kind) as Infer<TKind>,
                uuid: "string",
                layout: definition.layout.type,
                scope: viewScope as Infer<ViewScope>,
                config: definition.config.type,
            }),
        }
    }
}
