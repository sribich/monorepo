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

use crate::startup::Feature;

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
        features: Vec<Box<dyn Feature>>,
        modules: Vec<Box<dyn Module<State = AppState>>>,
        context: HttpServerContext,
    ) -> Self {
        // let channel = broadcast::channel(64);

        let state = AppState {
            injector: context.injector,
            //            db: context.state.db,
            //            dictionary: context.state.dictionary,
            //            library: context.state.library,
            //            study: context.state.study,
            //            pronunciation: context.state.pronunciation,
            //            channel: channel.0,
            //            /* db: Arc::new(context.db),
            //             * settings: Arc::new(context.settings),
            //             * recording: (0, false), */
        };

        let router = Self::router(&features, &modules, Arc::new(state.clone()));

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
        features: &[Box<dyn Feature>],
        modules: &[Box<dyn Module<State = AppState>>],
        state: Arc<AppState>,
    ) -> Router<AppState> {
        let context = RpcContext::<AppState>::new();
        let router = context.router();

        let mut rpc_router = context.router();

        for feature in features {
            rpc_router = rpc_router.merge(feature.routes(
                context.router(),
                context.procedure(),
                state.clone(),
            ));
        }

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
            export_path.push("../../../../project/immersly/bin/desktop-ui/src/generated/rpc-client");

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
use std::{env::current_dir, sync::Arc};

use axum::{Json, extract::State, response::IntoResponse, routing::get, serve};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use railgun::{
    rpc::{
        Empty, RpcContext,
        axum::{TypedMessage, TypedWebSocket, TypedWebSocketUpgrade},
        export::clients::typescript::TypescriptClient,
        router::Router,
    },
    typegen::{Typegen, export::config::ExportConfig},
};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, select, sync::broadcast};
use tracing::debug;

use crate::{
    context::{
        dictionary::{DictionaryPorts, infra::handler::get_dictionary_router},
        immerse::infra::provides::handler::get_immerse_router,
        library::{LibraryDomain, get_library_router},
        study::{StudyDomain, get_study_router},
    },
    module::pronunciation::{PronunciationDomain, get_pronunciation_router},
};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Sqlite>,
    pub dictionary: Arc<DictionaryPorts>,
    pub library: Arc<LibraryDomain>,
    pub study: Arc<StudyDomain>,
    pub pronunciation: Arc<PronunciationDomain>,
    // db: Arc<PrismaClient>,
    // settings: Arc<Settings>,
    // recording: (usize, bool),
    channel: broadcast::Sender<ServerMessages>,
}

impl core::fmt::Debug for AppState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}

pub struct HttpServerContext {
    // pub db: PrismaClient,
    // pub settings: Settings,

    // startup_notifier: Option<Sender<u16>>
    pub state: HttpStateContext,
}

pub struct HttpStateContext {
    pub db: Arc<Sqlite>,
    pub dictionary: Arc<DictionaryPorts>,
    pub library: Arc<LibraryDomain>,
    pub study: Arc<StudyDomain>,
    pub pronunciation: Arc<PronunciationDomain>,
}

impl HttpServer {
    fn rpc_router(state: Arc<AppState>) -> Router<AppState> {
        let context = RpcContext::<AppState>::new();
        let _procedure = context.procedure();

        let router = context.router();

        let router = router.child(
            "/rpc",
            context
                .router()
                .merge(get_study_router(
                    context.router(),
                    context.procedure(),
                    Arc::clone(&state),
                ))
                .merge(get_dictionary_router(context.router(), context.procedure()))
                .merge(get_immerse_router(context.router(), context.procedure()))
                .merge(get_library_router(
                    context.router(),
                    context.procedure(),
                    Arc::clone(&state),
                ))
                .merge(get_pronunciation_router(
                    context.router(),
                    context.procedure(),
                    Arc::clone(&state),
                ))
                .procedure("card:knownWords", _procedure.query(test)),
        );

        Self::generate_type_exports(&router);

        router
    }


}

// #[derive(Debug, Typegen, Deserialize, Serialize)]
// pub struct Empty {}

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

/*
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;



    let mut export_path = current_dir().unwrap();
    export_path.push("generated/client.ts");

    let export_config = ExportConfig::new(export_path);
    router.generate_client::<TypescriptClient>(export_config)?;

    println!("{}", current_dir().unwrap().to_str().unwrap());

    // let server = TcpListener::bind("127.0.0.1:4444").await.unwrap();
    // axum::serve(server, router.to_axum_router()).await.unwrap();

    Ok(())
}



#[derive(Type)]
struct Foo {
    name: String
}

*/

#[derive(Serialize, Typegen, Debug, Clone)]
pub struct KnownWord {
    word: String,
    reading: String,
}

#[derive(Serialize, Typegen, Debug, Clone)]
pub struct KnownWords {
    words: Vec<KnownWord>,
}

async fn test() -> Json<KnownWords> {
    use std::iter;

    use chrono::DateTime;
    use fsrs::{FSRS, FSRSItem, FSRSReview};
    use itertools::Itertools;
    use srs_bridge_anki::client::{
        AnkiClient,
        actions::{
            card::{CardsInfoRequest, FindCardsRequest},
            deck::{DeckNamesAndIdsRequest, DeckNamesRequest},
            statistic::card_reviews::{CardReviewsRequest, CardReviewsResponse},
        },
    };

    let client = AnkiClient::default();

    let deck_info = client.request(DeckNamesAndIdsRequest {}).await.unwrap();
    let card_info = client
        .request(FindCardsRequest {
            query: "deck:\"Refold JP1K v2\"".into(),
        })
        .await
        .unwrap();
    let card_data = client
        .request(CardsInfoRequest { cards: card_info })
        .await
        .unwrap();

    let words = card_data
        .into_iter()
        .map(|card| {
            let word = card.fields.get("Word").unwrap().value.clone();

            let reading = get_reading(
                card.fields
                    .get("Word With Reading")
                    .unwrap_or_else(|| card.fields.get("WordReading").unwrap())
                    .value
                    .clone(),
            );

            KnownWord { word, reading }
        })
        .collect::<Vec<_>>();

    Json(KnownWords { words })
}

/// Is a given `char` betwen あ and ゟ?
///
/// Strictly compliant with the [Unicode definition of hiragana], including
/// marks and a digraph.
///
/// ```
/// assert!(kanji::is_hiragana('あ'));
/// assert!(kanji::is_hiragana('ゟ'));
/// assert!(!kanji::is_hiragana('a'));
/// ```
///
/// [Unicode definition of hiragana]: https://www.unicode.org/charts/PDF/U3040.pdf
pub fn is_hiragana(c: char) -> bool {
    c >= '\u{3041}' && c <= '\u{309f}'
}

/// Is a given `char` between ゠ and ヿ?
///
/// Strictly compliant with the [Unicode definition of katakana], including
/// punctuation, marks and a digraph.
///
/// ```
/// assert!(kanji::is_katakana('ン'));
/// assert!(kanji::is_katakana('ヿ'));
/// assert!(!kanji::is_katakana('a'));
/// ```
///
/// [Unicode definition of katakana]: https://www.unicode.org/charts/PDF/U30A0.pdf
pub fn is_katakana(c: char) -> bool {
    c >= '\u{30a0}' && c <= '\u{30ff}'
}

/// Kanji appear in the Unicode range 4e00 to 9ffc.
/// The final Japanese Kanji is 9fef (鿯).
///
/// For a chart of the full official range, see [this pdf] from the Unicode
/// organization.
///
/// A number of Level Pre-One Kanji appear in the [CJK Compatibility
/// Ideographs][compat] list, so there is an extra check here for those.
///
/// ```
/// assert!(kanji::is_kanji('澄')); // Obviously a legal Kanji.
/// assert!(!kanji::is_kanji('a')); // Obviously not.
/// ```
///
/// [compat]: https://www.unicode.org/charts/PDF/UF900.pdf
/// [this pdf]: https://www.unicode.org/charts/PDF/U4E00.pdf
pub fn is_kanji(c: char) -> bool {
    (c >= '\u{4e00}' && c <= '\u{9ffc}') // Standard set.
        || (c >= '\u{f900}' && c <= '\u{faff}') // CJK Compatibility Ideographs.
        || (c >= '\u{3400}' && c <= '\u{4dbf}') // Extension A
        || (c >= '\u{20000}' && c <= '\u{2a6dd}') // Extension B
        || (c >= '\u{2a700}' && c <= '\u{2b734}') // Extension C
        || (c >= '\u{2b740}' && c <= '\u{2b81d}') // Extension D
        || (c >= '\u{2b820}' && c <= '\u{2cea1}') // Extension E
        || (c >= '\u{2ceb0}' && c <= '\u{2ebe0}') // Extension F
        || (c >= '\u{30000}' && c <= '\u{3134a}') // Extension G
}

fn get_reading(s: String) -> String {
    let mut reading = "".to_owned();

    for c in s.chars() {
        if is_hiragana(c) || is_katakana(c) {
            reading.push(c);
        }
    }

    reading
}
*/
