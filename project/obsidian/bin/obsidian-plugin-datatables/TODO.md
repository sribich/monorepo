-   Add migrations

    -   Template migration. Remove setup code for creating template structure in createTemplate
        when one does not already exist.

-   Implement a method to make "public" fields and rows that are shown when in a "public" mode.

-   Frontend error when the schema becomes out of date. We should keep track of modifications
    and if a modification has happened that we aren't aware of, show an error.
