use crate::actor::Task;
use crate::context::MpvContext;
use crate::context::Subtitle;

pub struct AddSubtitle {
    context: MpvContext,
    text: Option<String>,
}

impl AddSubtitle {
    pub fn new(context: MpvContext, text: Option<String>) -> Self {
        Self { context, text }
    }
}

impl Task for AddSubtitle {
    async fn execute(&self) {
        if let Some(data) = self.text.clone()
            && let Ok(start_time) = self.context.get_float_property("sub-start").await
            && let Ok(end_time) = self.context.get_float_property("sub-end").await
        {
            let sub_delay = self
                .context
                .get_float_property("sub-delay")
                .await
                .unwrap_or(0.0_f64);
            let audio_delay = self
                .context
                .get_float_property("audio-delay")
                .await
                .unwrap_or(0.0_f64);

            let text = clean_text(data);

            if text.is_empty() {
                return;
            }

            let subtitle = Subtitle {
                ts_0: start_time + sub_delay - audio_delay,
                ts_1: end_time + sub_delay - audio_delay,
                text,
            };

            self.context.add_subtitle(subtitle.clone()).await;

            self.context
                .set_clipboard(subtitle.text.replace(['\n', '\r'], ""))
                .await;
        }
    }
}

fn clean_text<S: AsRef<str>>(input: S) -> String {
    let char_items = [
        '\u{a0}', '\u{180e}', '\u{1680}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}',
        '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200a}', '\u{200b}', '\u{202f}',
        '\u{205f}', '\u{3000}', '\u{feff}', '\u{202a}',
    ];

    input.as_ref().replace("%s", "").replace(char_items, "")
}
