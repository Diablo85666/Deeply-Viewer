<p align="center">
  <img src="assets/branding/deeply-deeply-viewer.jpg" alt="Deeply Deeply Viewer logo" width="220">
</p>

<h1 align="center">Deeply Deeply Viewer</h1>

<p align="center">
  <strong>0.9.10.3 Stable</strong><br>
  Community Windows build prepared by MrDiablo85
</p>

<p align="center">
  <a href="https://github.com/Diablo85666/Deeply-Deeply-Viewer">GitHub</a> ·
  <a href="https://discord.gg/VgkrhRZka">Discord</a> ·
  <a href="https://github.com/Diablo85666">Author profile</a>
</p>

Deeply Deeply Viewer is an independent community build. It is not affiliated
with or endorsed by Linden Lab or any other viewer project.

## Italiano

### Download e aggiornamenti

Scarica la versione stabile dalla [pagina ufficiale delle
release](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases/latest).

- [Download Deeply Deeply Viewer](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases/latest)
- [Download per Linux](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases/latest)
- [Tutte le release](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases)

Il pacchetto Windows attuale è:

`Deeply-Deeply-Viewer-0.9.10.3-windows-x64.zip`

Il pacchetto è portable: estrai tutti i file e avvia
`Deeply-Deeply-Viewer.exe`. Verifica sempre il checksum pubblicato nella
release prima dell'uso.

### Miglioramenti verificabili dalla cronologia disponibile

Il checkout pubblico corrente non contiene tag separati verificabili per le
versioni 0.9.6 e 0.9.7. Per evitare attribuzioni non documentate, questa
sezione riporta le funzionalità verificabili nel codice e nelle release
community disponibili da 0.9.10.1 a 0.9.10.3.

#### Identità e localizzazione

- Interfaccia italiana con override `DEEPLY_UI_LANGUAGE=it`.
- Identificatori stabili per pulsanti del mouse e scorciatoie tradotte.
- Informazioni su build, autore, licenze, privacy, report e prestazioni.
- Nome eseguibile `Deeply-Deeply-Viewer.exe`.
- Autore della release: MrDiablo85.

#### Collegamenti e aggiornamenti community

- Repository operativo: <https://github.com/Diablo85666/Deeply-Deeply-Viewer>
- Discord community: <https://discord.gg/VgkrhRZka>
- Profilo autore: <https://github.com/Diablo85666>
- Feed aggiornamenti collegato al repository personale.
- Verifica del formato degli asset e del manifest `deeply-install.json`.
- Pacchetti Windows verificabili con SHA-256.

#### Browser interno

- Browser integrato con barra degli indirizzi.
- Navigazione indietro, avanti, arresto e ricaricamento.
- Indicatore di caricamento e stato HTTPS.
- Possibilità di aprire una pagina nel browser di sistema.
- Comportamenti `SystemForAll`, `BuiltInForSecondLife` e `BuiltInForAll`.
- Supporto a HTTP, HTTPS, SLURL e app link compatibili.
- Rifiuto di schemi non consentiti come `javascript:` e `data:`.

#### Controlli per avatar

- Rifiuto persistente delle offerte di teleport per singolo avatar.
- Rifiuto persistente delle conference chat per singolo avatar.
- Rifiuto conference applicato prima dell'apertura della conversazione.
- Controlli separati per blocco totale, voce, teleport e conference chat.
- Comandi disponibili nei menu contestuali degli avatar, amici e mittenti.

#### Grafica adattiva e recupero texture

- Monitoraggio di memoria del processo, tempi frame e stalli GPU.
- Riduzione progressiva `Deluxe -> High -> Medium -> Low -> Potato`.
- Campioni consecutivi e cooldown per evitare cambiamenti impulsivi.
- Profili Custom non modificati automaticamente.
- Ricostruzione degli slot texture GPU dopo una riduzione sotto pressione.
- Ricaricamento delle texture bake dell'avatar locale.
- Recupero da texture nere su volto, mento, occhi e mondo senza riavvio.
- Safe graphics di emergenza mantenuto separato.

#### Log e report

- Condivisione opzionale dei transcript compatibili con Firestorm.
- Supporto a conversazioni private e di gruppo compatibili.
- Cartelle separate per grid personalizzate.
- Salvataggio locale dei report.
- Bozze GitHub e invio dopo autorizzazione esplicita.
- Copia del report e revisione nel modulo ufficiale Linden Lab.
- Nessun invio automatico a endpoint Linden Lab non documentati.
- Log, diagnostica, posizione e screenshot restano opzionali.

### Privacy, licenza e attribuzioni

Il pacchetto distribuito non contiene i log personali dell'utente, transcript,
credenziali, token o password. I dati generati durante l'utilizzo restano
nella cartella dati locale del viewer e non vengono aggiunti automaticamente
alle release.

Deeply Deeply Viewer è distribuito secondo i termini GNU GPLv3 applicabili al
codice sorgente. Consultare [LICENSE](LICENSE) e [NOTICE.md](NOTICE.md).

## English

### Download and updates

Download the stable build from the [official
releases page](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases/latest).

- [Download Deeply Deeply Viewer](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases/latest)
- [Download for Linux](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases/latest)
- [All releases](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases)

The current Windows package is:

`Deeply-Deeply-Viewer-0.9.10.3-windows-x64.zip`

The package is portable: extract all files and start
`Deeply-Deeply-Viewer.exe`. Always verify the checksum published with the
release before using the package.

### Improvements verifiable from the available history

The current public checkout does not contain separate verifiable tags for
versions 0.9.6 and 0.9.7. To avoid undocumented claims, this section lists
features verifiable in the code and community releases available from
0.9.10.1 through 0.9.10.3.

#### Identity and localization

- Italian interface with the `DEEPLY_UI_LANGUAGE=it` override.
- Stable identifiers for translated mouse buttons and shortcuts.
- Build, author, license, privacy, report, and performance information.
- Executable name `Deeply-Deeply-Viewer.exe`.
- Release author: MrDiablo85.

#### Community links and updates

- Operational repository: <https://github.com/Diablo85666/Deeply-Deeply-Viewer>
- Community Discord: <https://discord.gg/VgkrhRZka>
- Author profile: <https://github.com/Diablo85666>
- Update feed connected to the personal repository.
- Asset naming and `deeply-install.json` manifest validation.
- Windows packages verifiable with SHA-256.

#### Built-in browser

- Integrated browser with an address bar.
- Back, forward, stop, and reload navigation.
- Loading progress and HTTPS status.
- Option to open a page in the system browser.
- `SystemForAll`, `BuiltInForSecondLife`, and `BuiltInForAll` behaviors.
- Support for compatible HTTP, HTTPS, SLURL, and app links.
- Refusal of unsupported schemes such as `javascript:` and `data:`.

#### Avatar controls

- Persistent per-avatar rejection of teleport offers.
- Persistent per-avatar rejection of conference chats.
- Conference rejection is applied before the conversation opens.
- Separate controls for total block, voice, teleport, and conference chat.
- Commands available in avatar, friend, and chat-sender context menus.

#### Adaptive graphics and texture recovery

- Monitoring of process memory, frame times, and GPU stalls.
- Progressive reduction `Deluxe -> High -> Medium -> Low -> Potato`.
- Consecutive samples and cooldown to avoid impulsive changes.
- Custom profiles are not changed automatically.
- GPU texture slots are rebuilt after a pressure reduction.
- Local avatar bake textures are refreshed.
- Black face, chin, eye, and world textures can recover without a restart.
- Emergency safe graphics remains separate.

#### Logs and reports

- Optional transcript sharing with compatible Firestorm folders.
- Support for compatible private and group conversations.
- Separate folders for custom grids.
- Local report saving.
- GitHub drafts and submission after explicit authorization.
- Clipboard copy and review in the official Linden Lab form.
- No automatic submission to undocumented Linden Lab endpoints.
- Logs, diagnostics, position, and screenshots remain optional.

### Privacy, license, and attribution

The distributed package does not contain the user's personal logs, transcripts,
credentials, tokens, or passwords. Data generated during use remains in the
viewer's local data directory and is not automatically added to releases.

Deeply Deeply Viewer is distributed under the GNU GPLv3 terms applicable to
the source code. See [LICENSE](LICENSE) and [NOTICE.md](NOTICE.md).

## Community

- GitHub: <https://github.com/Diablo85666/Deeply-Deeply-Viewer>
- Discord: <https://discord.gg/VgkrhRZka>
- Author: <https://github.com/Diablo85666>
