use std::collections::{BTreeMap, HashSet};

use convert_case::{Case, Casing};
use typegen::{
    cache::{ExportIdentifier, TypeCache},
    datatype::NamedDataType,
    export::{DuplicateRouteContext, ExportError, InvariantErrorContext, TypeExporter},
};
use typegen_typescript::{TypeDefinition, process_type};

use super::ClientExporter;
use crate::router::Route;

pub struct TypescriptClient {}

#[derive(Default)]
pub struct TypescriptClientOptions {
    prefix: Option<String>,
}

impl ClientExporter for TypescriptClient {
    fn export_client(
        mut options: Self::Options,
        prefix: Option<String>,
        data: Vec<Route>,
        cache: &typegen::cache::TypeCache,
    ) -> Result<BTreeMap<String, String>, ExportError> {
        options.prefix = prefix;

        let mut visited_routes: HashSet<String> = HashSet::with_capacity(data.len());
        // let mut resolver = ExportResolver::new();

        // let mut visited_types: HashSet<TypeId> = Default::default();
        let prefix = options.prefix;

        let mut segmented_routes: BTreeMap<String, Vec<Route>> = BTreeMap::new();

        for route in data {
            let path = route.path.clone();

            if visited_routes.contains(&path) {
                return DuplicateRouteContext { path }.fail();
            }

            visited_routes.insert(path.clone());

            let path = if let Some(prefix) = &prefix
                && path.starts_with(prefix)
            {
                path.strip_prefix(prefix).unwrap().to_string()
            } else {
                path
            };

            let path = if let Some(stripped) = path.strip_prefix('/') {
                stripped
            } else {
                &path
            };

            let parts = path.split(':').collect::<Vec<_>>();

            if parts.len() != 2 {
                return InvariantErrorContext {
                    msg: "RPC does not have 2 parts".to_owned(),
                }
                .fail();
            }

            let [namespace, action] = [parts.first().unwrap(), parts.get(1).unwrap()];

            let namespace = format!("{namespace}_{action}");

            match segmented_routes.get_mut(&namespace) {
                Some(routes) => routes.push(route),
                None => {
                    segmented_routes.insert(namespace, vec![route]);
                },
            }

            // segmented_routes.get()
        }

        let modules = segmented_routes
            .into_iter()
            .map(|(namespace, routes)| {
                // visited_types.insert(request.id().clone());
                // visited_types.insert(response.id().clone());

                let mut result = "// This is a generated file. DO NOT EDIT.\n\n".to_string();

                for Route {
                    path: route,
                    method,
                    like_method,
                    request,
                    response,
                } in routes
                {
                    result.push_str(&process_route(
                        cache,
                        &route,
                        prefix.as_ref(),
                        &method,
                        &like_method,
                        &request,
                        &response,
                        &namespace,
                    )?);
                }

                Ok((format!("{namespace}.ts"), result))
            })
            .collect::<Result<BTreeMap<String, String>, ExportError>>();

        modules.map(|mut modules| {
            // create shared module
            modules.insert("_shared.ts".to_owned(), String::new());

            let exports = cache.get_exports();

            for (
                _,
                ExportIdentifier {
                    id,
                    dependent_modules,
                    content,
                    ..
                },
            ) in exports
            {
                let module_names = dependent_modules.iter().collect::<Vec<_>>();

                if dependent_modules.len() > 1 {
                    modules.insert(
                        "_shared.ts".to_owned(),
                        format!(
                            "{}\n{}",
                            modules.get("_shared.ts").unwrap(),
                            content.clone().unwrap()
                        ),
                    );

                    for module in module_names {
                        let module = format!("{module}.ts");
                        let existing = modules.get(&module).unwrap();

                        modules.insert(
                            module.clone(),
                            format!(
                                "import type {{ {} }} from \"{}\" \n{}",
                                &id.name, "./_shared", existing
                            ),
                        );
                    }
                } else {
                    for module in module_names {
                        let module = format!("{module}.ts");
                        let existing = modules.get(&module).unwrap();

                        modules.insert(
                            module.clone(),
                            format!("{}\n{}", existing, content.clone().unwrap()),
                        );
                    }
                }
            }

            // Add react-query
            for module in modules.iter().map(|it| it.0.clone()).collect::<Vec<_>>() {
                let value = modules.get(&module).unwrap();

                modules.insert(
                    module.clone(),
                    format!(
                        "{}\n\n{}\n{}",
                        "import { useQuery, useMutation, type UseQueryResult, type UseMutationOptions, type UseMutationResult, type UseQueryOptions } from \"@tanstack/react-query\"",
                        "
type _Success<T> = Exclude<T, { error: any }>
type _Err<T> = Extract<T, { error: any }>
                        ",
                        value
                    ),
                );
            }

            modules
        })
    }
}

impl TypeExporter for TypescriptClient {
    type Data = Vec<Route>;
    type Options = TypescriptClientOptions;

    fn export(
        _options: Self::Options,
        _data: Self::Data,
        _cache: &typegen::cache::TypeCache,
    ) -> Result<String, ExportError> {
        Ok(String::new())
    }
}

fn get_name_from_route(route: &str, prefix: Option<&String>) -> String {
    let mut route = route.to_owned();

    if let Some(prefix) = prefix
        && route.starts_with(prefix)
    {
        route = route.strip_prefix(prefix.as_str()).unwrap().to_string();
    }

    route
        .split(':')
        .next_back()
        .map(|it| it.to_case(Case::Snake))
        .unwrap()
        .to_case(Case::Camel)
}

fn process_route(
    cache: &TypeCache,
    route: &String,
    prefix: Option<&String>,
    method: &String,
    like_method: &str,
    request: &NamedDataType,
    response: &NamedDataType,
    module: &str,
) -> Result<String, ExportError> {
    // let cache = &cache.clone();

    let mut items = vec![];

    let func = get_name_from_route(route, prefix);

    let TypeDefinition {
        name: request_ty_name,
        refs: request_typedefs,
    } = process_type(cache, request.datatype(), module)?;
    let TypeDefinition {
        name: response_ty_name,
        refs: response_typedefs,
    } = process_type(cache, response.datatype(), module)?;

    if let Some(mut typedefs) = request_typedefs {
        items.append(&mut typedefs);
    }

    if let Some(mut typedefs) = response_typedefs {
        items.append(&mut typedefs);
    }

    /*
        items.push(format!(
            "
export async function {func}(data: {request_ty_name}): Promise<{response_ty_name}> {{
    const response = await fetch(
                '{route}',
                {{
                    method: '{method}',
                    headers: {{
                        'Accept': 'application/json',
                        'Content-Type': 'application/json'
                    }},
                    body: JSON.stringify(data),
                }},
            )

            const json = await response.json()

            if (!response.ok) {{
                throw new Error(json)
            }}

            return json
}}
"
        ));
    */

    let [query_kind, query_key, query_fn, query_result] = match like_method {
        "GET" => ["useQuery", "queryKey", "queryFn", "UseQueryResult"],
        "POST" => [
            "useMutation",
            "mutationKey",
            "mutationFn",
            "UseMutationResult",
        ],
        _ => {
            return InvariantErrorContext {
                msg: format!("Unknown method {like_method}"),
            }
            .fail();
        },
    };

    items.push(match like_method {
        "GET" => format!("
export function {func}(
    key: string[],
    data: {request_ty_name},
    options?: Partial<
                 UseQueryOptions<
                     {request_ty_name},
                     _Err<{response_ty_name}>,
                     _Success<{response_ty_name}>,
                     string[]
                 >
             >,
): {query_result}<_Success<{response_ty_name}>, _Err<{response_ty_name}>> {{
    return {query_kind}<
        {request_ty_name},
        _Err<{response_ty_name}>,
        _Success<{response_ty_name}>,
        string[]
    >({{
        {query_key}: key,
        {query_fn}: async () => {{
            const response = await fetch(
                '{route}',
                {{
                    method: '{method}',
                    headers: {{
                        'Accept': 'application/json',
                        'Content-Type': 'application/json'
                    }},
                    body: JSON.stringify(data),
                }},
            )

            const json = await response.json()

            if (!response.ok) {{
                throw new Error(json)
            }}

            return json
        }},
        ...(options || {{}}),
    }})
}}
"),
        "POST" => format!("
export function {func}(key: string[], options?: UseMutationOptions<
    _Success<{response_ty_name}>,
    _Err<{response_ty_name}>,
    {request_ty_name}
>): {query_result}<_Success<{response_ty_name}>, _Err<{response_ty_name}>, {request_ty_name}> {{

    return {query_kind}<_Success<{response_ty_name}>, _Err<{response_ty_name}>, {request_ty_name}>({{
        {query_key}: key,
        {query_fn}: async (data: {request_ty_name}) => {{
            const response = await fetch(
                '{route}',
                {{
                    method: '{method}',
                    headers: {{
                        'Accept': 'application/json',
                        'Content-Type': 'application/json'
                    }},
                    body: JSON.stringify(data),
                }},
            )

            const json = await response.json()

            if (!response.ok) {{
                throw new Error(json)
            }}

            return json
        }},
        ...(options ?? ({{}})),
    }})
}}
"),
        _ => {
            return InvariantErrorContext {
                msg: format!("Unknown method {like_method}"),
            }
            .fail();
        }
    });

    items.push(
        "

"
        .to_owned(),
    );

    Ok(items.join(""))
}

/*
        /*
        cache
            .cache
            .iter()
            .map(|(k, v)| {
                /*
                if visited_types.contains(k) {
                    return Ok(());
                }
                 */

                let v = match v {
                    CachedType::InProgress => Err(ExportError::MissingType(k.name.to_owned())),
                    CachedType::Resolved(dt) => Ok(dt),
                }?;

                visited_types.insert(k.clone());

                let (name, defs) = process_type(&mut resolver, v.datatype())?;

                if let Some(defs) = defs {
                    result.push_str(&defs.join("\n"));
                }

                Ok(())

            .collect::<Result<Vec<()>, ExportError>>()?;
        */
*/

/*


/*
    let (req_ty, res_ty) = match (request, response) {
        (DataType::Struct(req), DataType::Struct(res)) => (&req.name, &res.name),
        _ => panic!("oh no"),
    };
*/
*/
