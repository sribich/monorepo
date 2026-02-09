use std::fs::File;
use std::fs::create_dir_all;
use std::io::Write;

use axum::handler::Handler;
use axum::routing::get;
use axum::routing::post;
use typegen::Generics;
use typegen::NamedType;
use typegen::cache::TypeCache;
use typegen::datatype::NamedDataType;
use typegen::export::ExportError;
use typegen::export::config::ExportConfig;

use crate::RpcHandler;
use crate::export::clients::ClientExporter;
use crate::procedure::Procedure;
use crate::procedure::Resolved;

#[derive(Debug)]
pub struct Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    cache: TypeCache,
    router: axum::Router<S>,
    procedures: Vec<Route>,
}

#[derive(Clone, Debug)]
pub struct Route {
    pub path: String,
    pub method: String,
    pub like_method: String,
    pub request: NamedDataType,
    pub response: NamedDataType,
}

impl<S> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub(crate) fn new() -> Self {
        Self {
            cache: TypeCache::default(),
            router: axum::Router::new(),
            procedures: vec![],
        }
    }

    pub fn apply<T>(mut self, f: T) -> Self
    where
        T: FnOnce(axum::Router<S>) -> axum::Router<S>,
    {
        self.router = f(self.router);
        self
    }

    /// TODO: We need to validate paths here.
    ///         - . marks children
    /// TODO: We need to check for collisions.
    /// TODO: We must take into account nesting operators
    #[must_use]
    pub fn procedure<H, T, Req, Res>(
        mut self,
        key: &'static str,
        procedure: Procedure<Resolved<H, T, S>>,
    ) -> Self
    where
        H: Handler<T, S> + RpcHandler<Req, Res, T, S>,
        T: 'static,
        Req: NamedType,
        Res: NamedType,
    {
        let handler = procedure.state.0.0;

        let key = if key.starts_with('/') {
            key.to_string()
        } else {
            format!("/{key}")
        };

        match &procedure.kind {
            crate::procedure::ProcedureKind::Query => {
                self.procedures.push(Route {
                    path: key.clone(),
                    method: "POST".to_owned(),
                    like_method: "GET".to_owned(),
                    request: Req::named_datatype(&mut self.cache, &Generics::Impl),
                    response: Res::named_datatype(&mut self.cache, &Generics::Impl),
                });
                self.router = self.router.route(key.as_str(), post(handler));
            }
            crate::procedure::ProcedureKind::Mutation => {
                self.procedures.push(Route {
                    path: key.clone(),
                    // TODO: Make this some kind of enum
                    method: "POST".to_owned(),
                    like_method: "POST".to_owned(),
                    request: Req::named_datatype(&mut self.cache, &Generics::Impl),
                    response: Res::named_datatype(&mut self.cache, &Generics::Impl),
                });
                self.router = self.router.route(key.as_str(), post(handler));
            }
            crate::procedure::ProcedureKind::Subscription => {
                panic!("Please use Router::subscription for subscriptions over Router::procedure.");
            }
        }

        self
    }

    #[must_use]
    pub fn subscription<H, T>(
        mut self,
        key: &'static str,
        procedure: Procedure<Resolved<H, T, S>>,
    ) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let handler = procedure.state.0.0;
        /*


        self.procedures
            .push((Cow::Borrowed(key), Req::datatype(), Res::datatype()));
        self.router = self.router.route(key, post(handler));
        */
        self.router = self.router.route(key, get(handler));

        self
    }

    #[must_use]
    pub fn child(mut self, key: &'static str, router: Self) -> Self {
        let normalized_key = if let Some(stripped) = key.strip_suffix('/') {
            stripped
        } else {
            key
        };

        let mut result = router
            .procedures
            .iter()
            .map(|child_route| {
                let child_key = if let Some(stripped) = child_route.path.strip_prefix('/') {
                    stripped
                } else {
                    &child_route.path
                };

                Route {
                    path: format!("{normalized_key}/{child_key}"),
                    method: child_route.method.clone(),
                    like_method: child_route.like_method.clone(),
                    request: child_route.request.clone(),
                    response: child_route.response.clone(),
                }
            })
            .collect();

        router
            .cache
            .into_iter()
            .for_each(|(k, v)| self.cache.insert(k, v));

        self.procedures.append(&mut result);

        self.router = self.router.nest(key, router.router);

        self
    }

    #[must_use]
    pub fn merge(mut self, mut router: Self) -> Self {
        router
            .cache
            .into_iter()
            .for_each(|(k, v)| self.cache.insert(k, v));

        self.procedures.append(&mut router.procedures);

        self.router = self.router.merge(router.router);

        self
    }

    pub fn to_axum_router(self) -> axum::Router<S> {
        self.router
    }

    pub fn generate_client<T>(
        &self,
        prefix: Option<impl Into<String>>,
        config: ExportConfig<T>,
    ) -> Result<(), ExportError>
    where
        T: ClientExporter,
    {
        let result = T::export_client(
            config.options,
            prefix.map(Into::into),
            self.procedures.clone(),
            &self.cache,
        )?;

        create_dir_all(config.path.clone()).unwrap();

        for (module_filename, content) in result {
            let mut file = File::create(config.path.join(module_filename)).expect("TODO");

            file.write_all(content.as_bytes()).expect("TODO");
        }

        Ok(())
    }
}

// TODO: Test child routers work
// TODO: Test merging routers throws when there are conflicts
#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
