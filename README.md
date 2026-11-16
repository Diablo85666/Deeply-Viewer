<p align="center">
  <img src="assets/branding/deeply-deeply-viewer.jpg" alt="Deeply Deeply Viewer logo" width="220">
</p>

<h1 align="center">Deeply Deeply Viewer</h1>

<p align="center">
  <strong>0.9.10.4 Stable</strong><br>
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

È in preparazione una build Linux community di Deeply Deeply Viewer con
pacchetti `.deb` per distribuzioni Debian e Ubuntu.

Il pacchetto Windows attuale è:

`Deeply-Deeply-Viewer-0.9.10.4-windows-x64.zip`

Il pacchetto è portable: estrai tutti i file e avvia
`Deeply-Deeply-Viewer.exe`. Verifica sempre il checksum pubblicato nella
release prima dell'uso.

### Miglioramenti cumulativi dalla serie 0.9.6 alla 0.9.10.4

Il checkout pubblico corrente non contiene tag separati verificabili per le
versioni 0.9.6 e 0.9.7. La lista seguente è quindi una panoramica cumulativa
delle funzionalità community presenti e verificabili nel ramo pubblico,
senza attribuire singole modifiche a tag che non sono disponibili.

#### Percorso delle release

- **0.9.6–0.9.7**: base community, localizzazione italiana, identità
  Deeply Deeply Viewer e consolidamento delle funzioni di compatibilità.
- **0.9.10.1–0.9.10.2**: aggiornamenti community, feed personale,
  pacchetto portable, browser interno, report, condivisione log e controlli
  per avatar.
- **0.9.10.3**: rifiuto delle conference chat per avatar e recupero delle
  texture dopo la riduzione grafica adattiva.
- **0.9.10.4**: branding community integrato nell'eseguibile, schermata
  iniziale, login, icona finestra e notifiche; pacchetto firmato con
  Minisign.

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

#### Bug risolti e stabilità

- Risolto il problema per cui il rifiuto delle conference chat era disponibile
  solo in alcuni menu: il comando è ora esposto nei menu degli avatar, amici
  e mittenti della chat.
- Risolto il problema per cui una conference chat poteva aprirsi prima che
  la preferenza di rifiuto venisse applicata.
- Risolto il problema delle texture nere persistenti dopo pressione hardware,
  rebake o riduzione della qualità grafica.
- Risolto il problema per cui gli slot texture GPU corrotti restavano in uso
  fino al riavvio del viewer.
- Risolto il problema per cui le texture bake dell'avatar locale e alcune
  texture del mondo non venivano ricaricate dopo il recupero.
- Ridotti i cambi di qualità causati da picchi isolati tramite campioni
  consecutivi e cooldown.
- Impedita la modifica automatica dei profili grafici Custom.
- Separato il recupero grafico ordinario dalla modalità safe graphics di
  emergenza.
- Reso verificabile il pacchetto con manifest, nome asset, SHA-256 e firma
  Minisign, evitando che l'updater scelga pacchetti upstream.
- Centralizzato il branding community in login, splash, icona finestra e
  notifiche per evitare nomi e loghi incoerenti.

#### Log e report

- Condivisione bilaterale opzionale dei transcript con Firestorm.
- Importazione dei transcript Firestorm già presenti, con selezione delle
  cartelle trovate o scelta manuale di una cartella.
- Quando la condivisione è attiva e la cartella dell'account viene trovata,
  Deeply legge e scrive nella stessa cartella locale compatibile con
  Firestorm: le conversazioni registrate da Deeply possono quindi essere
  consultate da Firestorm e quelle registrate da Firestorm possono essere
  importate e visualizzate da Deeply.
- La condivisione è locale e bilaterale sui file: non sincronizza i dati via
  Internet e non trasferisce transcript tra computer.
- Supporto a conversazioni private e di gruppo compatibili.
- Cartelle separate per grid personalizzate.
- Salvataggio locale dei report.
- Bozze GitHub e invio dopo autorizzazione esplicita.
- Copia del report e revisione nel modulo ufficiale Linden Lab.
- Nessun invio automatico a endpoint Linden Lab non documentati.
- Log, diagnostica, posizione e screenshot restano opzionali.

#### Verifica della release 0.9.10.4

- SHA-256 del pacchetto Windows:
  `6AE73A70D7D5EE419705DB744C214E26BFC230D543C73D185FB68C335789838A`
- Firma: `Deeply-Deeply-Viewer-0.9.10.4-windows-x64.zip.minisig`
- Chiave pubblica Minisign:
  `RWQtAyq9dTabBnUU+7nX6IESxLp36DK5GK8BDcI6Mj5jveHqB7qvMJoh`
- La chiave privata Minisign non è inclusa nel repository o nel pacchetto.

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

A community Linux build of Deeply Deeply Viewer is planned, with `.deb`
packages for Debian and Ubuntu distributions.

The current Windows package is:

`Deeply-Deeply-Viewer-0.9.10.4-windows-x64.zip`

The package is portable: extract all files and start
`Deeply-Deeply-Viewer.exe`. Always verify the checksum published with the
release before using the package. The release also includes a Minisign
signature for the package.

### Cumulative improvements from 0.9.6 through 0.9.10.4

The current public checkout does not contain separate verifiable tags for
versions 0.9.6 and 0.9.7. The following is therefore a cumulative overview
of community features present and verifiable in the public branch, without
assigning individual changes to unavailable tags.

#### Release path

- **0.9.6–0.9.7**: community foundation, Italian localization, Deeply
  Deeply Viewer identity, and compatibility work.
- **0.9.10.1–0.9.10.2**: community updates, personal update feed, portable
  package, built-in browser, reports, log sharing, and per-avatar controls.
- **0.9.10.3**: per-avatar conference-chat rejection and texture recovery
  after adaptive graphics reduction.
- **0.9.10.4**: community branding integrated into the executable, startup
  screen, login, window icon, and notifications; Minisign-signed package.

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

#### Fixed bugs and stability improvements

- Fixed conference-chat rejection being exposed in only some menus: the
  command is now available from avatar, friend, and chat-sender menus.
- Fixed conference sessions opening before the per-avatar rejection
  preference was applied.
- Fixed persistent black textures after hardware pressure, rebaking, or
  graphics-quality reduction.
- Fixed corrupted GPU texture slots remaining active until the viewer
  restarted.
- Fixed local avatar bake textures and some world textures not being
  reloaded after recovery.
- Reduced quality changes caused by isolated spikes through consecutive
  samples and a cooldown.
- Prevented Custom graphics profiles from being changed automatically.
- Kept ordinary graphics recovery separate from emergency safe graphics.
- Made packages verifiable through the manifest, asset name, SHA-256, and
  Minisign signature so the updater does not select upstream packages.
- Centralized community branding across login, splash, window icon, and
  notifications to prevent inconsistent names and logos.

#### Logs and reports

- Optional bilateral transcript sharing with Firestorm.
- Import of existing Firestorm transcripts, with discovered folders or a
  manually selected folder.
- When sharing is enabled and the account folder is found, Deeply reads and
  writes the same local Firestorm-compatible folder: conversations recorded
  by Deeply can be viewed by Firestorm, and conversations recorded by
  Firestorm can be imported and viewed by Deeply.
- This is local, bilateral file sharing. It does not synchronize data over
  the Internet or transfer transcripts between computers.
- Support for compatible private and group conversations.
- Separate folders for custom grids.
- Local report saving.
- GitHub drafts and submission after explicit authorization.
- Clipboard copy and review in the official Linden Lab form.
- No automatic submission to undocumented Linden Lab endpoints.
- Logs, diagnostics, position, and screenshots remain optional.

#### 0.9.10.4 release verification

- Windows package SHA-256:
  `6AE73A70D7D5EE419705DB744C214E26BFC230D543C73D185FB68C335789838A`
- Signature: `Deeply-Deeply-Viewer-0.9.10.4-windows-x64.zip.minisig`
- Minisign public key:
  `RWQtAyq9dTabBnUU+7nX6IESxLp36DK5GK8BDcI6Mj5jveHqB7qvMJoh`
- The Minisign private key is not included in the repository or package.

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
