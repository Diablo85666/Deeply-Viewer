use crate::settings::TranslationLanguage;
use egui::Context;
use std::sync::mpsc::{self, Receiver};

#[derive(Default)]
pub(crate) struct State {
    endpoint: String,
    target: TranslationLanguage,
    automatic: bool,
    request: Option<Receiver<Result<String, String>>>,
    source: Option<String>,
    result: Option<Result<String, String>>,
}

impl State {
    pub(crate) fn configure(&mut self, endpoint: &str, target: TranslationLanguage, automatic: bool) {
        self.endpoint = endpoint.to_owned();
        self.target = target;
        // TODO: hook this setting into live incoming-chat dispatch. Keeping
        // it persisted and explicitly labelled inactive avoids translating
        // history or the user's own messages by accident.
        self.automatic = automatic;
    }

    pub(super) fn menu(ui: &mut egui::Ui, p: &crate::theme::Palette, text: &str, state: &mut State) {
        ui.menu_button(aurora_i18n::tr!("ui-chat-translate"), |ui| {
            let target = state.target();
            if ui
                .button(format!("{} ({})", aurora_i18n::tr!("ui-chat-translate"), target.as_str()))
                .clicked()
            {
                state.request(text, target);
                ui.close();
            }
            crate::ui::menu::separator(ui, p);
            for language in [
                TranslationLanguage::It,
                TranslationLanguage::En,
                TranslationLanguage::Es,
                TranslationLanguage::De,
                TranslationLanguage::Fr,
            ] {
                if ui.button(language.as_str()).clicked() {
                    state.request(text, language);
                    ui.close();
                }
            }
        });
    }

    fn target(&self) -> TranslationLanguage {
        self.target
    }

    pub(super) fn request(&mut self, text: &str, target: TranslationLanguage) {
        if self.request.is_some() || text.trim().is_empty() {
            return;
        }

        let endpoint = self.endpoint.clone();
        let text = text.to_owned();
        let language = target.as_str().to_owned();
        let (tx, rx) = mpsc::channel();
        self.source = Some(text.clone());
        self.result = None;
        self.request = Some(rx);
        std::thread::spawn(move || {
            let result = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| e.to_string())
                .and_then(|runtime| {
                    runtime.block_on(async move {
                        let client = reqwest::Client::new();
                        let response = client
                            .post(endpoint)
                            .header(reqwest::header::CONTENT_TYPE, "application/json")
                            .body(
                                serde_json::json!({
                                    "q": text,
                                    "source": "auto",
                                    "target": language,
                                    "format": "text",
                                })
                                .to_string(),
                            )
                            .send()
                            .await
                            .map_err(|e| e.to_string())?;
                        let status = response.status();
                        let body = response.text().await.map_err(|e| e.to_string())?;
                        if !status.is_success() {
                            return Err(format!("HTTP {status}: {body}"));
                        }
                        serde_json::from_str::<serde_json::Value>(&body)
                            .ok()
                            .and_then(|v| v.get("translatedText").and_then(|v| v.as_str()).map(str::to_owned))
                            .ok_or_else(|| "LibreTranslate returned no translation".to_owned())
                    })
                });
            let _ = tx.send(result);
        });
    }

    pub(super) fn show(&mut self, ctx: &Context) {
        if let Some(rx) = &self.request
            && let Ok(result) = rx.try_recv()
        {
            self.result = Some(result);
            self.request = None;
        }
        if self.request.is_some() {
            ctx.request_repaint();
        }
        let Some(source) = self.source.as_deref() else { return };
        let mut open = true;
        egui::Window::new(aurora_i18n::tr!("ui-chat-translation-title"))
            .open(&mut open)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label(aurora_i18n::tr!("ui-chat-translation-source"));
                ui.label(source);
                ui.separator();
                if self.request.is_some() {
                    ui.spinner();
                    ui.label(aurora_i18n::tr!("ui-chat-translation-loading"));
                } else {
                    match self.result.as_ref() {
                        Some(Ok(text)) => {
                            ui.label(aurora_i18n::tr!("ui-chat-translation-result"));
                            ui.label(text);
                        }
                        Some(Err(error)) => {
                            ui.colored_label(
                                ui.visuals().error_fg_color,
                                format!("{}: {error}", aurora_i18n::tr!("ui-chat-translation-error")),
                            );
                        }
                        None => {}
                    }
                }
            });
        if !open {
            self.source = None;
            self.result = None;
        }
    }
}

pub(super) fn menu(ui: &mut egui::Ui, p: &crate::theme::Palette, text: &str, state: &mut State) {
    State::menu(ui, p, text, state);
}
