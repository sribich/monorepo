import type { SchemaSettings } from "../schema/schema-settings"

export class SettingsContainer {
    public readonly schema: SchemaSettings

    constructor(settings: { schema: SchemaSettings }) {
        this.schema = settings.schema
    }
}
