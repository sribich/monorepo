import "react-aria"

/**
 * These modifications are to get around the default react-aria types
 * not supporting `exactOptionalPropertyTypes`
 */
declare module "react-aria" {
    export interface HiddenSelectProps {
        isDisabled?: boolean | undefined
    }
}
