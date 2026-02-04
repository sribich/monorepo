/**
 * Field delcarations
 *
 * A field declaration defines the structure of a field and its associated
 * metadata.
 *
 * It is used to generate the field definition, which is the actual object
 * that is used to represent a field in the persisted schema.
 */
import { type Type, type } from "arktype"
import type { Compute } from "ts-toolbelt/out/Any/Compute"

import type { PropertyKind } from "./property-kind"

/**
 * TODO: Docs
 */
export interface FilterItem {
    kind: string
    data?: unknown
}

export type YamlSerializable =
    | string
    | number
    | boolean
    | null
    | YamlSerializable[]
    | { [name: string]: YamlSerializable }

type Morph<TConfig> = (...args: any[]) => (incoming: TConfig) => void

export interface Filter<TProperty, TFilter, TValue> {
    default: TFilter["data"]
    fn: (property: TProperty, filter: TFilter, value: TValue) => boolean
}

type InferType<T> = Type<T> extends Type<infer P> ? Type<T>["infer"] : T

export interface PropertyDefinition<
    TKind extends PropertyKind,
    TConfig,
    TConfigMorphs extends Record<string, Morph<TConfig>>,
    TField,
    TFieldMorphs extends Record<string, Morph<TField>>,
    TFilter extends { kind: string; data?: unknown },
    TDefaultFilter extends TFilter["kind"],
> {
    kind: TKind
    name: string
    config: {
        type: Type<TConfig>
        default: TConfig
        morphs: TConfigMorphs
        schema: Type<{
            name: string
            kind: TKind
            uuid: string
            config: TConfig
        }>
    }
    field: {
        type: Type<TField>
        default?: InferType<TField>
        morphs: TFieldMorphs
        schema: Type<{
            name: string
            kind: TKind
            uuid: string
            value: TField
        }>
    }
    filter: {
        type: type.instantiate<TFilter /* & { propKind: TKind }*/>
        default?: {
            kind: TDefaultFilter
            data: Extract<TFilter, { kind: TDefaultFilter }>["data"]
        }
        filters: {
            [K in TFilter["kind"]]: Filter<
                PropertyDefinition<
                    TKind,
                    TConfig,
                    TConfigMorphs,
                    TField,
                    TFieldMorphs,
                    TFilter,
                    TDefaultFilter
                >,
                InferType<Extract<TFilter, { kind: K }>>,
                InferType<TField>
            >
        }
    }
}

export const makeProperty = <const TKind extends PropertyKind>(kind: TKind) => {
    return <
        const TConfig,
        const TConfigMorphs extends Record<string, Morph<TConfig>>,
        const TField,
        const TFieldMorphs extends Record<string, Morph<TField>>,
        const TFilter extends FilterItem,
        const TDefaultFilter extends TFilter["kind"],
    >(definition: {
        // kind: TKind
        name: string
        config: {
            type: Type<TConfig>
            default: TConfig
            morphs: TConfigMorphs
        }
        field: {
            type: Type<TField>
            default: InferType<TField>
            morphs: TFieldMorphs
        }
        filter: {
            type: type.Any<TFilter>
            default?: {
                kind: TDefaultFilter
                data: InferType<Extract<TFilter, { kind: TDefaultFilter }>["data"]>
            }
            filters: {
                [K in TFilter["kind"]]: Filter<
                    PropertyDefinition<
                        TKind,
                        TConfig,
                        TConfigMorphs,
                        TField,
                        TFieldMorphs,
                        TFilter,
                        TDefaultFilter
                    >,
                    InferType<Extract<TFilter, { kind: K }>>,
                    InferType<TField>
                >
            }
        }
    }): Compute<
        PropertyDefinition<
            TKind,
            TConfig,
            TConfigMorphs,
            TField,
            TFieldMorphs,
            TFilter,
            TDefaultFilter
        >,
        "flat"
    > => {
        return {
            ...definition,
            kind,
            config: {
                ...definition.config,
                schema: type({
                    name: "string",
                    kind: `'${kind.toString()}'`, // enumVariant(kind) as Infer<TKind>,
                    uuid: "string",
                    config: definition.config.type,
                }) as never,
            },
            field: {
                ...definition.field,
                schema: type({
                    name: "string",
                    kind: `'${kind.toString()}'`,
                    uuid: "string",
                    value: definition.field.type,
                }) as never,
            },
            filter: {
                ...definition.filter,
            },
        }
    }
}

/*
const createBox = <t extends string>(of: type.Any<t>) =>
    type({
        box: of,
    })

createBox(type("number"))

// Type<{ box: string }>
const BoxType = createBox(type("string"))
*/
