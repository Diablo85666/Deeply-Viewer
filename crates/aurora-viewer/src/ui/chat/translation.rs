use crate::settings::TranslationLanguage;
use ct2rs::{Config, Device, Translator};
use egui::Context;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};

#[derive(Default)]
pub(crate) struct State {
    model_path: String,
    target: TranslationLanguage,
    automatic: bool,
    request: Option<Receiver<Result<String, String>>>,
    source: Option<String>,
    result: Option<Result<String, String>>,
    cache: HashMap<(String, String), String>,
}

impl State {
    pub(crate) fn configure(&mut self, model_path: &str, target: TranslationLanguage, automatic: bool) {
        self.model_path = model_path.to_owned();
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
                TranslationLanguage::Pt,
                TranslationLanguage::Ru,
                TranslationLanguage::Ja,
                TranslationLanguage::Zh,
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

        let model_path = self.model_path.clone();
        let text = text.to_owned();
        let language = target.flores_code().to_owned();
        if let Some(translated) = self.cache.get(&(text.clone(), language.clone())) {
            self.source = Some(text);
            self.result = Some(Ok(translated.clone()));
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.source = Some(text.clone());
        self.result = None;
        self.request = Some(rx);
        std::thread::spawn(move || {
            let result = Self::offline_translate(&model_path, &text, &language);
            let _ = tx.send(result);
        });
    }

    fn offline_translate(model_path: &str, text: &str, language: &str) -> Result<String, String> {
        let path = std::path::Path::new(model_path);
        if !path.is_dir() {
            return Err(format!(
                "Offline translation model unavailable at {model_path}; install models/nllb-200-600M-int8"
            ));
        }
        let config = Config {
            device: Device::CPU,
            ..Config::default()
        };
        let translator = Translator::new(path, &config).map_err(|error| format!("NLLB initialization failed: {error}"))?;
        let sources = vec![text.to_owned()];
        let source_prefixes = vec![vec!["eng_Latn".to_owned()]];
        let target_prefixes = vec![vec![language.to_owned()]];
        let mut results = translator
            .translate_batch_with_source_and_target_prefix(&sources, &source_prefixes, &target_prefixes, &Default::default(), None)
            .map_err(|error| format!("NLLB translation failed: {error}"))?;
        results
            .pop()
            .map(|(translated, _)| translated)
            .ok_or_else(|| "NLLB returned no translation".to_owned())
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
                            self.cache
                                .insert((source.to_owned(), self.target.flores_code().to_owned()), text.clone());
                            if self.cache.len() > 64 {
                                if let Some(key) = self.cache.keys().next().cloned() {
                                    self.cache.remove(&key);
                                }
                            }
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
