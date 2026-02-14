use std::env::current_dir;
use std::sync::Arc;

use axum::serve;
use features::shared::infra::http::AppState;
use railgun::di::Module;
use railgun::rpc::RpcContext;
use railgun::rpc::export::clients::typescript::TypescriptClient;
use railgun::rpc::router::Router;
use railgun::typegen::export::config::ExportConfig;
use railgun_di::Injector;
use tokio::net::TcpListener;
use tracing::debug;

pub struct HttpServerContext {
    pub port: u16,
    pub injector: Arc<Injector>,
}

pub struct HttpServer {
    router: axum::Router,
    server: tokio::net::TcpListener,
}

impl HttpServer {
    pub async fn from_features(
        modules: Vec<Box<dyn Module<State = AppState>>>,
        context: HttpServerContext,
    ) -> Self {
        // let channel = broadcast::channel(64);

        let state = AppState {
            injector: context.injector,
        };

        let router = Self::router(&modules, Arc::new(state.clone()));

        let cors = tower_http::cors::CorsLayer::new()
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::HEAD,
            ])
            .allow_origin(tower_http::cors::Any);

        let axum_router = router
            .to_axum_router()
            // .layer(OtelInResponseLayer::default())
            // .layer(OtelAxumLayer::default())
            // .route("/ws", get(ws_handler))
            .with_state(state)
            .layer(tower_http::cors::CorsLayer::very_permissive());

        // .with_graceful_shutdown(shutdown_signal()) ,_ oneshot?

        // if let Some(notifier) = startup_notifier {
        //     let _ = notifier.send(port);
        // }

        // FIXME: We need to change this to another interface if we ever want
        //        to allow exposing the server to the internet for self-hosted
        //        reviewing.
        // FIXME: We should probably return a Result here.
        let server = TcpListener::bind(format!("127.0.0.1:{}", context.port))
            .await
            .unwrap();

        debug!("Server starting on 127.0.0.1:{}", context.port);

        Self {
            router: axum_router,
            server,
        }
    }

    pub async fn run(self) {
        serve(self.server, self.router).await.unwrap();
    }

    fn router(
        modules: &[Box<dyn Module<State = AppState>>],
        state: Arc<AppState>,
    ) -> Router<AppState> {
        let context = RpcContext::<AppState>::new();
        let router = context.router();

        let mut rpc_router = context.router();

        for module in modules {
            if let Some(router) = module.as_routes() {
                rpc_router = rpc_router.merge(router.routes(
                    context.router(),
                    context.procedure(),
                    state.clone(),
                ));
            }
        }

        let router = router.child("/rpc", rpc_router);

        // .procedure("card:knownWords", _procedure.query(test)),

        Self::generate_type_exports(&router);

        router
    }

    fn generate_type_exports(router: &Router<AppState>) {
        #[cfg(debug_assertions)]
        {
            let mut export_path = current_dir().unwrap();
            println!("{:#?}", export_path);
            // TODO(sr): This needs to be configurable.
            export_path
                .push("../../../../project/immersly/bin/desktop-ui/src/generated/rpc-client");

            let export_config =
                ExportConfig::<TypescriptClient>::new(export_path.clone(), Default::default());

            router
                .generate_client::<TypescriptClient>(Some("/rpc"), export_config)
                .unwrap();
        }

        /*

                   {


                       let mut file = File::create(export_path.join("websocket.ts")).expect("TODO");
                       file.write_all(items.join("\n").as_bytes()).expect("TODO");

                       // ServerMessages::typ
                       let server_dt = ServerMessages::datatype(&mut cache, &Generics::Impl);
                       let TypeDefinition {
                           name: server_ty_name,
                           refs: server_typedefs,
                       } = process_type(&cache, &server_dt).unwrap();

                       let client_dt = ClientMessages::datatype(&mut cache, &Generics::Impl);
                       let TypeDefinition {
                           name: client_ty_name,
                           refs: client_typedefs,
                       } = process_type(&cache, &client_dt).unwrap();

                       let mut items = vec![];

                       if let Some(mut typedefs) = server_typedefs {
                           items.append(&mut typedefs);
                       }

                       if let Some(mut typedefs) = client_typedefs {
                           items.append(&mut typedefs);
                       }

        let mut cache = TypeCache::default();

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
                */
    }
}

/*
#[derive(Clone, Debug, Typegen, Serialize)]
#[serde(tag = "kind", content = "content")]
enum ServerMessages {
    Subtitle(Vec</* Segment */ ()>),
}

#[derive(Clone, Debug, Typegen, Deserialize)]
enum ClientMessages {
    Empty {},
}

async fn ws_handler(
    ws: TypedWebSocketUpgrade<ServerMessages, ClientMessages>,
    State(AppState { channel, .. }): State<AppState>,
    // user_agent: Option<TypedHeader<axum_extra::headers::UserAgent>>,
    // ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("We're in ws handler!");
    // let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
    //     user_agent.to_string()
    // } else {
    //     String::from("Unknown browser")
    // };
    // println!("`{user_agent}` at {{addr}} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.

    ws.on_upgrade(move |socket| handle_socket(socket, channel))
}

async fn handle_socket(
    mut socket: TypedWebSocket<ServerMessages, ClientMessages>,
    recv: broadcast::Sender<ServerMessages>,
) {
    let (mut sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(socket_recv(receiver));
    let mut send_task = tokio::spawn(socket_send(sender, recv));

    select! {
        recv = (&mut recv_task) => {
            send_task.abort();
        },
        send = (&mut send_task) => {
            recv_task.abort();
        },
    }
}


async fn socket_recv(mut receiver: SplitStream<TypedWebSocket<ServerMessages, ClientMessages>>) {
    while let Some(Ok(msg)) = receiver.next().await {}
}

async fn socket_send(
    mut sender: SplitSink<
        TypedWebSocket<ServerMessages, ClientMessages>,
        TypedMessage<ServerMessages>,
    >,
    recv: broadcast::Sender<ServerMessages>,
) {
    let mut recv = recv.subscribe();

    while let Ok(msg) = recv.recv().await {
        let serialized = serde_json::to_string(&msg).unwrap();

        sender.send(TypedMessage::Item(msg)).await;
    }
}

// #[derive(Debug, Typegen, Serialize, Deserialize)]
// struct Test {
//     name: String,
//     age: bool,
//     freqs: Vec<Option<f32>>,
// }
//
// #[derive(Typegen, Deserialize, Serialize)]
// struct OtherEmpty {}
//
// async fn route_test(Json(_): Json<Empty>) -> Json<Test> {
//     let clip = AudioClip::record().unwrap();
//
//     println!("{:?}", clip.samples.len());
//
//     let freqs = clip.play();
//
//     Json(Test {
//         name: "Bob".to_string(),
//         age: false,
//         freqs,
//     })
// }
*/
