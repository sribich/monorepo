export const isValidPropertyKind = (kind: unknown): kind is PropertyKind => {
    return (propertyKindIterator as readonly unknown[]).includes(kind)
}

/**
 * ! This file exists to resolve a circular dependency between the
 * ! various property files.
 *
 * When adding a field remember to update the following places as well:
 *
 *   1. Add the field validators to `fieldValidator` in `src/modules/datatables/schema/field/field.ts`
 *   2. Add the field to `FieldDefinition` in `src/modules/datatables/schema/field/field.ts`
 *   3. Add the field to `fieldDefinitions` in `src/modules/datatables/schema/field/field.ts`
 */
export const propertyKindIterator = [
    "text",
    "numbers", // number
    "select",
    // multiselect
    // status
    "date",
    // attachment
    "checkbox",
    // url
    // email
    // phone
    // formula
    "reference",
    "backreference",
    // Title is a special property that always exists in the dataview, referencing the
    // entity that is being displayed.
    "title",
] as const

export type PropertyKind = (typeof propertyKindIterator)[number]
