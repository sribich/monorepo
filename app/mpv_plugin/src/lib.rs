#![feature(let_chains)]
#![allow(unsafe_code, reason = "We need no_mangle for mpv to see our plugin")]

mod actor;
mod context;
mod error;
mod task;

use actor::Actor;
use context::MpvContext;
use error::PluginError;
use mpv_client::Event;
use mpv_client::Handle;
use mpv_client::mpv_handle;
use task::add_subtitle::AddSubtitle;
use task::clear_subtitles::ClearSubtitles;
use tokio::sync::broadcast;
use tracing::error;

#[unsafe(no_mangle)]
extern "C" fn mpv_open_cplugin(handle: *mut mpv_handle) -> std::os::raw::c_int {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        if let Err(error) = run(handle) {
            error!("{error}");

            return 1;
        }

        0_i32
    })
}

fn run(handle: *mut mpv_handle) -> Result<(), PluginError> {
    let client = Handle::from_ptr(handle);

    let owned_client = client.create_client("prelearning").unwrap();

    let context = MpvContext::new(owned_client);

    let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

    let actor = Actor::new();

    client.observe_property::<String>(1, "sub-text")?;
    client.observe_property::<String>(2, "filename")?;

    loop {
        match client.wait_event(0.0) {
            Event::Shutdown => {
                shutdown_tx.send(());

                return Ok(());
            }
            Event::PropertyChange(id, property) => {
                let (extracted_id, extracted_name, extracted_data) = {
                    let name = property.name().to_owned();
                    let data: Option<String> = property.data();
                    (id, name, data)
                };

                let context = context.clone();
                let actor = actor.clone();

                match (extracted_id, extracted_name.as_str()) {
                    (1, "sub-text") => {
                        tokio::spawn(async move {
                            let task = AddSubtitle::new(context, extracted_data);
                            actor.send(task).await;
                        });
                    }
                    (2, "filename") => {
                        tokio::spawn(async move {
                            actor.send(ClearSubtitles::new(context.clone())).await;
                        });
                    }
                    _ => {
                        todo!("Unhandled property change: {}", extracted_name);
                    }
                }
            }
            Event::ClientMessage(_)
            | Event::CommandReply(_, _)
            | Event::FileLoaded
            | Event::Hook(_, _)
            | Event::LogMessage(_)
            | Event::None
            | Event::PlaybackRestart
            | Event::QueueOverflow
            | Event::Seek
            | Event::GetPropertyReply(_, _, _)
            | Event::SetPropertyReply(_, _)
            | Event::StartFile(_)
            | Event::EndFile(_)
            | Event::AudioReconfig
            | Event::VideoReconfig => {}
        }
    }
}
