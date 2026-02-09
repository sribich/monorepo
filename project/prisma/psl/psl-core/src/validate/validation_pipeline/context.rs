use diagnostics::DatamodelError;
use diagnostics::DatamodelWarning;
use diagnostics::Diagnostics;
use enumflags2::BitFlags;
use parser_database::ExtensionTypes;

use crate::Datasource;
use crate::PreviewFeature;
use crate::builtin_connectors::has_capability;
use crate::datamodel_connector::Connector;
use crate::datamodel_connector::ConnectorCapability;
use crate::datamodel_connector::RelationMode;

/// The validation context. The lifetime parameter is _not_ the AST lifetime, but the subtype of
/// all relevant lifetimes. No data escapes for validations, so the context only need to be valid
/// for the duration of validations.
pub(crate) struct Context<'a> {
    pub(super) db: &'a parser_database::ParserDatabase,
    pub(super) datasource: Option<&'a Datasource>,
    pub(super) preview_features: BitFlags<PreviewFeature>,
    pub(super) connector: &'static dyn Connector,
    /// Relation mode is a pure function of the datasource, but since there are defaults,
    /// it's more consistent to resolve it once, here.
    pub(super) relation_mode: RelationMode,
    pub(super) diagnostics: &'a mut Diagnostics,
    pub(super) extension_types: &'a dyn ExtensionTypes,
}

impl Context<'_> {
    /// Pure convenience method. Forwards to Diagnostics::push_error().
    pub(super) fn push_error(&mut self, error: DatamodelError) {
        self.diagnostics.push_error(error);
    }

    /// Pure convenience method. Forwards to Diagnostics::push_warning().
    pub(super) fn push_warning(&mut self, warning: DatamodelWarning) {
        self.diagnostics.push_warning(warning);
    }

    pub(super) fn has_capability(&self, capability: ConnectorCapability) -> bool {
        has_capability(self.connector, capability)
    }
}
