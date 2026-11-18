# Offline chat translation

Deeply Viewer does not send chat to LibreTranslate or any other hosted
translation service. The translation menu uses the bundled `ct2rs` backend
when the local model is present and reports an explicit unavailable state
otherwise.

The configured model directory defaults to:

```text
models/nllb-200-600M-int8
```

The directory can be changed in Preferences > Chat. The runner is CTranslate2
through `ct2rs` 0.11.2 and is configured for CPU execution. Translation work
is performed off the UI thread, and the result cache avoids repeating recent
requests. There is no network fallback.

The model directory must contain the CTranslate2-converted NLLB-200 distilled
600M int8 files, including `model.bin`, `config.json`, and the tokenizer and
vocabulary files required by the selected `ct2rs` tokenizer feature. Put the
directory beside `deeply-viewer.exe` in the portable package:

```text
Deeply-Viewer-v0.9.11-windows-x64/
  deeply-viewer.exe
  models/nllb-200-600M-int8/
```
