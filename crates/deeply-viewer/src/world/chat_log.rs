//! Conversation transcripts on disk, like Firestorm's (LLLogChat,
//! LLIMModel::logToFile; indra/newview/lllogchat.cpp, llimview.cpp): every
//! line of an instant message, group chat or conference (and of the nearby
//! chat when asked) is appended to a text file, and the end of that file is
//! shown again, greyed, when the conversation opens.
//!
//! The files of an account on a grid live in their own folder
//! (`store::folder`): nothing is read or written before the login tells
//! which account this is, and after a logout nothing reaches the folder of
//! the account that left. The demo has its own grid name and its own
//! settings folder, so it never touches a real account.
//!
//! - `format`: the text of a line and its reading back;
//! - `names`: the file of each conversation;
//! - `store`: the folder, the thread that writes and reads;
//! - `transcript`: a whole file for the History window, by pages;
//! - `import`: legacy-viewer or former Deeply transcripts merged into the
//!   account's folder, adding only.

pub mod format;
pub mod import;
pub mod names;
pub mod store;
pub mod transcript;

pub use format::Entry;
pub use store::{Identity, Loaded};

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// What the user chose (Preferences › Chat).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Instant messages, group chats and conferences (Firestorm's
    /// KeepConversationLogTranscripts = 2).
    pub log_im: bool,
    /// The nearby chat (LogNearbyChat). Off by default.
    pub log_nearby: bool,
    /// Show the end of the transcript when a conversation opens
    /// (LogShowHistory).
    pub show_history: bool,
    /// Date in the file names (LogFileNamewithDate).
    pub dated_names: bool,
    /// Import the Legacy Viewer's transcripts at login. On by default.
    pub import_at_login: bool,
    /// Write to Firestorm's per-account transcript folder so both viewers share logs.
    pub share_firestorm_logs: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_im: true,
            log_nearby: false,
            show_history: true,
            dated_names: false,
            import_at_login: true,
            share_firestorm_logs: false,
        }
    }
}

/// Name of the grid in the transcripts' path. The demo is its own grid: its
/// files can never be those of an account, whatever the folder.
pub fn grid_name(choice: &crate::settings::GridChoice, custom_login_uri: &str, demo: bool) -> String {
    use crate::settings::GridChoice;
    if demo {
        return "demo".to_owned();
    }
    match choice {
        GridChoice::SecondLife => "agni".to_owned(),
        GridChoice::SecondLifeBeta => "aditi".to_owned(),
        GridChoice::Custom => {
            let uri: String = custom_login_uri
                .trim()
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
                .collect();
            // the prefix keeps another grid from naming itself like one above
            format!("custom_{}", uri.trim_matches('_'))
        }
    }
}

struct Open {
    identity: Identity,
    dir: PathBuf,
    legacy_account: Option<import::Account>,
}

/// The transcripts of the account logged in.
#[derive(Default)]
pub struct ChatLog {
    pub cfg: Config,
    /// Grid of the next login (given by the app before it logs in); empty
    /// when this run keeps no transcripts.
    grid: String,
    /// Folder holding one folder per account, when it is not the one of the
    /// settings: chosen in the preferences (InstantMessageLogPath), tests.
    accounts: Option<PathBuf>,
    /// A move to another folder asked for, until its answer comes, with the
    /// folder to go back to if it fails.
    moving: Option<(crossbeam_channel::Receiver<std::io::Result<usize>>, Option<PathBuf>)>,
    open: Option<Open>,
    /// Counts the logins and logouts: what was read for another one is
    /// thrown away.
    epoch: u64,
    worker: Option<store::Worker>,
    /// An erasing asked for, until its answer comes.
    erasing: Option<crossbeam_channel::Receiver<std::io::Result<usize>>>,
    /// The Legacy Viewers' folders, when they are not the system's (tests,
    /// demo).
    legacy_apps: Option<Vec<PathBuf>>,
    #[cfg(test)]
    firestorm_app: Option<PathBuf>,
    /// An import under way.
    importing: Option<import::Running>,
}

impl ChatLog {
    /// Transcripts under another folder than the settings' (tests).
    #[cfg(test)]
    pub fn in_folder(accounts: PathBuf) -> ChatLog {
        ChatLog {
            accounts: Some(accounts),
            ..Default::default()
        }
    }

    #[cfg(test)]
    fn set_firestorm_app(&mut self, app: PathBuf) {
        self.firestorm_app = Some(app);
    }

    /// The grid the next login is made on.
    pub fn set_grid(&mut self, grid: String) {
        if self.open.is_none() {
            self.grid = grid;
        }
    }

    /// The account is known: its folder becomes the one written and read.
    #[cfg(test)]
    pub fn login(&mut self, agent: Uuid) {
        self.login_as(agent, None);
    }

    /// Log in with the account name needed for Firestorm's transcript folder.
    pub fn login_with_name(&mut self, agent: Uuid, first: String, last: String) {
        let account = import::Account {
            first,
            last,
            grid: self.grid.clone(),
        };
        self.login_as(agent, Some(account));
    }

    fn login_as(&mut self, agent: Uuid, legacy_account: Option<import::Account>) {
        self.epoch += 1;
        self.open = None;
        if agent.is_nil() || self.grid.is_empty() {
            log::info!("conversation transcripts: off for this session (no account or no grid)");
            return;
        }
        let identity = Identity {
            grid: self.grid.clone(),
            agent,
        };
        let shared_dir = self.shared_account_folder(legacy_account.as_ref());
        let dir = shared_dir
            .clone()
            .unwrap_or_else(|| self.deeply_account_folder(&identity, self.accounts.as_deref()));
        if self.cfg.share_firestorm_logs && legacy_account.is_some() && shared_dir.is_none() {
            log::warn!("conversation transcripts: Firestorm sharing is unavailable for this account or grid; using Deeply's own folder");
        }
        self.open = Some(Open {
            identity,
            dir,
            legacy_account,
        });
    }

    fn shared_account_folder(&self, account: Option<&import::Account>) -> Option<PathBuf> {
        if !self.cfg.share_firestorm_logs {
            return None;
        }
        let account = account?;
        #[cfg(test)]
        let app = self.firestorm_app.clone().or_else(crate::legacy_profile::firestorm_app_folder)?;
        #[cfg(not(test))]
        let app = crate::legacy_profile::firestorm_app_folder()?;
        import::shared_log_folder(&app, account)
    }

    fn deeply_account_folder(&self, identity: &Identity, root: Option<&Path>) -> PathBuf {
        let accounts = root.map(Path::to_path_buf).unwrap_or_else(crate::settings::accounts_dir);
        store::folder(&accounts, identity)
    }

    fn account_folder(&self, identity: &Identity, account: Option<&import::Account>, root: Option<&Path>) -> PathBuf {
        self.shared_account_folder(account)
            .unwrap_or_else(|| self.deeply_account_folder(identity, root))
    }

    /// Logged out: nothing more is written or read, and what was being read
    /// for that account is dropped. The lines already queued keep the path
    /// they were queued with.
    pub fn logout(&mut self) {
        self.epoch += 1;
        self.open = None;
        if let Some(running) = self.importing.take() {
            running.cancel();
        }
    }

    pub fn identity(&self) -> Option<&Identity> {
        self.open.as_ref().map(|o| &o.identity)
    }

    /// Folder of the account logged in.
    pub fn dir(&self) -> Option<&Path> {
        self.open.as_ref().map(|o| o.dir.as_path())
    }

    fn today(&self) -> Option<(i64, i64, i64)> {
        self.cfg.dated_names.then(names::local_date)
    }

    /// File a conversation is written to now.
    pub fn file_of(&self, stem: &str) -> Option<PathBuf> {
        names::file_path(self.dir()?, stem, self.today())
    }

    /// File a conversation's history is read from: the one written now, or
    /// with dated names the latest that exists. Looks at the disk: not for
    /// every frame.
    pub fn history_file_of(&self, stem: &str) -> Option<PathBuf> {
        names::existing_file(self.dir()?, stem, self.today())
    }

    fn worker(&mut self) -> Option<&store::Worker> {
        if self.worker.is_none() {
            self.worker = store::Worker::start();
        }
        self.worker.as_ref()
    }

    /// Append a line to a conversation's file (LLLogChat::saveHistory). An
    /// empty `from` is a system line. Nothing happens while logged out.
    pub fn append(&mut self, stem: &str, time: SystemTime, from: &str, text: &str) {
        let Some(path) = self.file_of(stem) else {
            return;
        };
        let from = if from.trim().is_empty() { format::SYSTEM_FROM } else { from };
        let line = format::format_line(Some(&format::stamp(time, true)), from, text);
        let mut text = if store::EOL == "\n" { line } else { line.replace('\n', store::EOL) };
        text.push_str(store::EOL);
        if let Some(w) = self.worker() {
            w.append(path, text);
        }
    }

    /// Ask for the end of a conversation's file (LLIMSession::loadHistory);
    /// it comes back through `take_loaded`. False when nothing was asked
    /// (logged out, or the history is not shown).
    pub fn request_history(&mut self, conversation: Uuid, stem: &str) -> bool {
        if !self.cfg.show_history {
            return false;
        }
        let Some(dir) = self.dir().map(Path::to_path_buf) else {
            return false;
        };
        let (today, epoch) = (self.today(), self.epoch);
        match self.worker() {
            Some(w) => {
                w.tail(dir, stem.to_owned(), today, conversation, epoch);
                true
            }
            None => false,
        }
    }

    /// Histories read since the last call, for the account logged in now.
    pub fn take_loaded(&mut self) -> Vec<Loaded> {
        let epoch = self.epoch;
        match &self.worker {
            Some(w) => w.loaded().filter(|l| l.epoch == epoch).collect(),
            None => Vec::new(),
        }
    }

    /// Erase the transcripts of the account logged in (on this grid), after
    /// the lines already queued. False when there is no account. The answer
    /// comes through `take_erased`.
    /// An import, a move or an erasing is under way: another one would be
    /// refused.
    pub fn busy(&self) -> bool {
        self.importing.is_some() || self.moving.is_some() || self.erasing.is_some()
    }

    pub fn erase(&mut self) -> bool {
        if self.importing.is_some() {
            return false;
        }
        let Some(dir) = self.dir().map(Path::to_path_buf) else {
            return false;
        };
        self.erasing = self.worker().map(|w| w.erase(dir));
        self.erasing.is_some()
    }

    /// The folder chosen in the preferences (None: the default one), for the
    /// next login.
    pub fn set_folder(&mut self, accounts: Option<PathBuf>) {
        if self.moving.is_none() {
            self.accounts = accounts;
        }
    }

    /// Another folder for the transcripts while logged in: the account's
    /// files follow, after the lines already queued, and the next lines go
    /// to the new folder (moveTranscriptsAndLog). False when nothing moves.
    /// The answer comes through `take_relocated`.
    pub fn relocate(&mut self, accounts: Option<PathBuf>) -> bool {
        if self.moving.is_some() || self.importing.is_some() {
            return false;
        }
        let Some(open) = &self.open else {
            self.accounts = accounts;
            return false;
        };
        let to = self.account_folder(&open.identity, open.legacy_account.as_ref(), accounts.as_deref());
        if to == open.dir {
            self.accounts = accounts;
            return false;
        }
        let from = open.dir.clone();
        let Some(done) = self.worker().map(|w| w.relocate(from, to.clone())) else {
            return false;
        };
        let previous = std::mem::replace(&mut self.accounts, accounts);
        if let Some(open) = &mut self.open {
            open.dir = to;
        }
        self.moving = Some((done, previous));
        true
    }

    /// The answer of the last move, once: the number of files moved. On a
    /// failure the transcripts stay in (and go back to) the previous folder.
    pub fn take_relocated(&mut self) -> Option<std::io::Result<usize>> {
        let done = self.moving.as_ref()?.0.try_recv().ok()?;
        let (_, previous) = self.moving.take()?;
        if done.is_err() {
            self.accounts = previous;
            let dir = self
                .open
                .as_ref()
                .map(|open| self.account_folder(&open.identity, open.legacy_account.as_ref(), self.accounts.as_deref()));
            if let Some(open) = &mut self.open {
                if let Some(dir) = dir {
                    open.dir = dir;
                }
            }
        }
        Some(done)
    }

    /// The Legacy Viewers' folders to look in.
    pub fn legacy_apps(&self) -> Vec<PathBuf> {
        self.legacy_apps.clone().unwrap_or_else(crate::legacy_profile::app_folders)
    }

    /// Look in other folders than the system's (tests, demo).
    pub fn set_legacy_apps(&mut self, apps: Vec<PathBuf>) {
        self.legacy_apps = Some(apps);
    }

    /// Import from these folders, every file read again (the import window).
    pub fn import(&mut self, folders: Vec<PathBuf>) -> bool {
        self.start_import(import::Sources::Folders(folders), import::Kinds::ALL, false)
    }

    /// Import the selected legacy transcripts and, when sharing is enabled,
    /// merge this account's former Deeply folder into Firestorm's folder.
    /// Files are only added to the destination; sources are never moved or
    /// removed.
    pub fn import_found(&mut self, account: import::Account) -> bool {
        let kinds = import::Kinds {
            im: self.cfg.log_im,
            nearby: self.cfg.log_nearby,
        };
        if !account.grid_known() || !kinds.any() {
            return false;
        }
        let Some(open) = &self.open else {
            return false;
        };
        let dir = open.dir.clone();
        let folders: Vec<PathBuf> = if self.cfg.share_firestorm_logs {
            let former = self.deeply_account_folder(&open.identity, self.accounts.as_deref());
            (former != dir).then_some(former).into_iter().collect()
        } else {
            Vec::new()
        };
        let import_legacy = self.cfg.import_at_login;
        if folders.is_empty() && !import_legacy {
            return false;
        }
        let sources = match (folders.is_empty(), import_legacy) {
            (true, true) => import::Sources::Find {
                apps: self.legacy_apps(),
                account,
            },
            (false, true) => import::Sources::FoldersAndFind {
                folders,
                apps: self.legacy_apps(),
                account,
            },
            (false, false) => import::Sources::Folders(folders),
            (true, false) => return false,
        };
        self.start_import(sources, kinds, true)
    }

    fn start_import(&mut self, sources: import::Sources, kinds: import::Kinds, skip_unchanged: bool) -> bool {
        if self.importing.is_some() || self.moving.is_some() || self.erasing.is_some() {
            return false;
        }
        let Some(dir) = self.dir().map(Path::to_path_buf) else {
            return false;
        };
        let request = import::Request {
            sources,
            kinds,
            // beside the account's folder, not in it: never a transcript
            state: dir.with_extension("legacy-import.json"),
            dated: self.cfg.dated_names,
            skip_unchanged,
            dir,
        };
        self.importing = self.worker().and_then(|w| import::start(w.merge_port(), request));
        self.importing.is_some()
    }

    /// Files done and files to do of the import under way.
    pub fn import_progress(&self) -> Option<(usize, usize)> {
        use std::sync::atomic::Ordering;
        let running = self.importing.as_ref()?;
        Some((running.done.load(Ordering::Relaxed), running.total.load(Ordering::Relaxed)))
    }

    /// The report of the last import, once.
    pub fn take_imported(&mut self) -> Option<import::Report> {
        let report = self.importing.as_ref()?.report.try_recv().ok()?;
        self.importing = None;
        Some(report)
    }

    /// The answer of the last erasing, once: the number of files removed.
    pub fn take_erased(&mut self) -> Option<std::io::Result<usize>> {
        let done = self.erasing.as_ref()?.try_recv().ok()?;
        self.erasing = None;
        Some(done)
    }

    /// Wait for the queued lines to be on disk (exit, tests).
    pub fn flush(&self, timeout: Duration) -> bool {
        self.worker.as_ref().is_none_or(|w| w.flush(timeout))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WAIT: Duration = Duration::from_secs(10);

    fn temp() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("deeply-chatlog-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn now() -> SystemTime {
        format::parse_stamp("2026/10/12 14:05").unwrap()
    }

    fn texts(path: &Path) -> Vec<String> {
        store::read_tail(path).unwrap().into_iter().map(|e| e.text).collect()
    }

    fn wait_loaded(log: &mut ChatLog) -> Vec<Loaded> {
        assert!(log.flush(WAIT));
        log.take_loaded()
    }

    #[test]
    fn two_accounts_never_share_a_file() {
        let root = temp();
        let alice = Uuid::from_u128(0xA11CE);
        let bob = Uuid::from_u128(0xB0B);
        let mut log = ChatLog::in_folder(root.clone());
        log.set_grid("agni".into());
        // both talk to a "Jane Doe"; the writes alternate, with logins and
        // logouts in between and without waiting for the disk
        for round in 0..20 {
            log.login(alice);
            log.append("Jane Doe", now(), "Jane Doe", &format!("to alice {round}"));
            log.append(names::NEARBY, now(), "Jane Doe", &format!("near alice {round}"));
            log.logout();
            log.login(bob);
            log.append("Jane Doe", now(), "Jane Doe", &format!("to bob {round}"));
            log.append(names::NEARBY, now(), "Jane Doe", &format!("near bob {round}"));
            log.logout();
        }
        assert!(log.flush(WAIT));
        let dir = |agent| {
            store::folder(
                &root,
                &Identity {
                    grid: "agni".into(),
                    agent,
                },
            )
        };
        assert_ne!(dir(alice), dir(bob));
        for (agent, who) in [(alice, "alice"), (bob, "bob")] {
            let im = texts(&dir(agent).join("Jane Doe.txt"));
            assert_eq!(im, (0..20).map(|r| format!("to {who} {r}")).collect::<Vec<_>>());
            let near = texts(&dir(agent).join("chat.txt"));
            assert_eq!(near, (0..20).map(|r| format!("near {who} {r}")).collect::<Vec<_>>());
            // exactly its two files, nothing else
            assert_eq!(std::fs::read_dir(dir(agent)).unwrap().count(), 2);
        }
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn the_same_account_id_on_two_grids_is_two_folders() {
        let root = temp();
        let agent = Uuid::from_u128(7);
        let mut log = ChatLog::in_folder(root.clone());
        for (grid, text) in [("agni", "on the main grid"), ("aditi", "on the beta grid"), ("demo", "in the demo")] {
            log.set_grid(grid.into());
            log.login(agent);
            assert_eq!(log.identity().unwrap().grid, grid);
            log.append("Jane Doe", now(), "Jane Doe", text);
            log.logout();
        }
        assert!(log.flush(WAIT));
        for (grid, text) in [("agni", "on the main grid"), ("aditi", "on the beta grid"), ("demo", "in the demo")] {
            let dir = store::folder(&root, &Identity { grid: grid.into(), agent });
            assert_eq!(texts(&dir.join("Jane Doe.txt")), [text]);
        }
        // the grid cannot change under an open session
        log.set_grid("agni".into());
        log.login(agent);
        log.set_grid("aditi".into());
        assert_eq!(log.identity().unwrap().grid, "agni");
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn nothing_is_written_or_read_before_the_login_or_after_the_logout() {
        let root = temp();
        let agent = Uuid::from_u128(9);
        let mut log = ChatLog::in_folder(root.clone());
        log.set_grid("agni".into());
        log.append("Jane Doe", now(), "Jane Doe", "too early");
        assert!(!log.request_history(agent, "Jane Doe"));
        assert_eq!(log.dir(), None);
        assert!(!log.erase());
        // a login without a grid, or without an account, opens nothing
        let mut blind = ChatLog::in_folder(root.clone());
        blind.login(agent);
        blind.append("Jane Doe", now(), "Jane Doe", "no grid");
        log.login(Uuid::nil());
        log.append("Jane Doe", now(), "Jane Doe", "no account");
        log.login(agent);
        log.append("Jane Doe", now(), "Jane Doe", "in time");
        log.logout();
        log.append("Jane Doe", now(), "Jane Doe", "too late");
        assert!(log.flush(WAIT) && blind.flush(WAIT));
        let dir = store::folder(
            &root,
            &Identity {
                grid: "agni".into(),
                agent,
            },
        );
        assert_eq!(texts(&dir.join("Jane Doe.txt")), ["in time"]);
        // and that is the only file under the root
        fn files(dir: &Path, out: &mut Vec<PathBuf>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                if e.path().is_dir() {
                    files(&e.path(), out);
                } else {
                    out.push(e.path());
                }
            }
        }
        let mut all = Vec::new();
        files(&root, &mut all);
        assert_eq!(all, [dir.join("Jane Doe.txt")]);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_history_read_for_one_account_is_dropped_after_a_switch() {
        let root = temp();
        let alice = Uuid::from_u128(0xA11CE);
        let bob = Uuid::from_u128(0xB0B);
        let peer = Uuid::from_u128(0x1A4E);
        let mut log = ChatLog::in_folder(root.clone());
        log.set_grid("agni".into());
        log.login(alice);
        log.append("Jane Doe", now(), "Jane Doe", "alice's secret");
        assert!(log.request_history(peer, "Jane Doe"));
        // the answer arrives after alice left and bob came
        log.logout();
        log.login(bob);
        assert!(wait_loaded(&mut log).is_empty());
        // bob has no such file: his history of the same peer is empty
        assert!(log.request_history(peer, "Jane Doe"));
        let loaded = wait_loaded(&mut log);
        assert_eq!(loaded.len(), 1);
        assert!(loaded[0].entries.is_empty());
        // alice again: hers
        log.logout();
        log.login(alice);
        assert!(log.request_history(peer, "Jane Doe"));
        let loaded = wait_loaded(&mut log);
        assert_eq!(loaded[0].conversation, peer);
        assert_eq!(loaded[0].entries[0].text, "alice's secret");
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn the_history_is_read_before_the_line_that_opened_the_conversation() {
        let root = temp();
        let agent = Uuid::from_u128(3);
        let peer = Uuid::from_u128(4);
        let mut log = ChatLog::in_folder(root.clone());
        log.set_grid("agni".into());
        log.login(agent);
        log.append("Jane Doe", now(), "Jane Doe", "yesterday");
        assert!(log.flush(WAIT));
        // a new message opens the conversation: history asked, then logged
        assert!(log.request_history(peer, "Jane Doe"));
        log.append("Jane Doe", now(), "Jane Doe", "today");
        let loaded = wait_loaded(&mut log);
        assert_eq!(loaded[0].entries.len(), 1);
        assert_eq!(loaded[0].entries[0].text, "yesterday");
        // not shown when the setting is off
        log.cfg.show_history = false;
        assert!(!log.request_history(peer, "Jane Doe"));
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn lines_are_firestorms_and_multi_line_messages_come_back() {
        let root = temp();
        let mut log = ChatLog::in_folder(root.clone());
        log.set_grid("agni".into());
        log.login(Uuid::from_u128(5));
        log.append("Jane Doe", now(), "Jane Doe (jane.doe)", "one\ntwo");
        log.append("Jane Doe", now(), "", "Jane Doe is online.");
        assert!(log.flush(WAIT));
        let path = log.file_of("Jane Doe").unwrap();
        let raw = String::from_utf8(std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(
            raw,
            format!(
                "[2026/10/12 14:05]  Jane Doe (jane.doe): one{eol} two{eol}[2026/10/12 14:05]  Second Life: Jane Doe is online.{eol}",
                eol = store::EOL
            )
        );
        let entries = store::read_tail(&path).unwrap();
        assert_eq!(entries[0].text, "one\ntwo");
        assert!(entries[1].is_system());
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn shared_logs_use_firestorms_account_folders_and_custom_grids_stay_separate() {
        let root = temp();
        let app = root.join("Firestorm_x64");
        std::fs::create_dir_all(&app).unwrap();
        let mut log = ChatLog::in_folder(root.clone());
        log.set_firestorm_app(app.clone());
        log.cfg.share_firestorm_logs = true;

        log.set_grid("agni".into());
        log.login_with_name(Uuid::from_u128(42), "Jane".into(), "Doe".into());
        let main = app.join("jane_doe");
        assert_eq!(log.dir(), Some(main.as_path()));
        log.append("Friend Name", now(), "Jane Doe", "shared on the main grid");
        log.logout();

        log.set_grid("aditi".into());
        log.login_with_name(Uuid::from_u128(42), "Jane".into(), "Doe".into());
        let beta = app.join("jane_doe.second_life_beta");
        assert_eq!(log.dir(), Some(beta.as_path()));
        log.append("Friend Name", now(), "Jane Doe", "shared on the beta grid");
        log.logout();

        log.set_grid("custom_grid".into());
        log.login_with_name(Uuid::from_u128(42), "Jane".into(), "Doe".into());
        let custom = store::folder(
            &root,
            &Identity {
                grid: "custom_grid".into(),
                agent: Uuid::from_u128(42),
            },
        );
        assert_eq!(log.dir(), Some(custom.as_path()));
        log.append("Friend Name", now(), "Jane Doe", "kept on Deeply's custom-grid folder");
        assert!(log.flush(WAIT));

        assert_eq!(texts(&main.join("Friend Name.txt")), ["shared on the main grid"]);
        assert_eq!(texts(&beta.join("Friend Name.txt")), ["shared on the beta grid"]);
        assert_eq!(texts(&custom.join("Friend Name.txt")), ["kept on Deeply's custom-grid folder"]);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn enabling_shared_logs_merges_old_private_and_group_history_without_moving_sources() {
        let root = temp();
        let app = root.join("Firestorm_x64");
        std::fs::create_dir_all(&app).unwrap();
        let agent = Uuid::from_u128(42);
        let old_dir = store::folder(
            &root,
            &Identity {
                grid: "agni".into(),
                agent,
            },
        );
        std::fs::create_dir_all(&old_dir).unwrap();
        std::fs::write(
            old_dir.join("Friend Name.txt"),
            "[2026/10/12 14:05]  Jane Doe: old private message\n",
        )
        .unwrap();
        std::fs::write(
            old_dir.join("Builders (group).txt"),
            "[2026/10/12 14:05]  Jane Doe: old group message\n",
        )
        .unwrap();

        let shared = app.join("jane_doe");
        std::fs::create_dir_all(&shared).unwrap();
        std::fs::write(shared.join("Friend Name.txt"), "[2026/10/12 14:06]  Jane Doe: already shared\n").unwrap();
        let mut log = ChatLog::in_folder(root.clone());
        log.set_firestorm_app(app);
        log.cfg.share_firestorm_logs = true;
        log.cfg.import_at_login = false;
        log.set_grid("agni".into());
        log.login_with_name(agent, "Jane".into(), "Doe".into());

        assert_eq!(log.dir(), Some(shared.as_path()));
        assert!(log.import_found(import::Account {
            first: "Jane".into(),
            last: "Doe".into(),
            grid: "agni".into(),
        }));
        let report = (0..=500)
            .find_map(|_| match log.take_imported() {
                Some(report) => Some(report),
                None => {
                    std::thread::sleep(Duration::from_millis(10));
                    None
                }
            })
            .expect("the shared-history merge should finish");
        assert_eq!(report.added, 2);
        assert!(log.flush(WAIT));
        assert_eq!(texts(&shared.join("Friend Name.txt")), ["old private message", "already shared"]);
        assert_eq!(texts(&shared.join("Builders (group).txt")), ["old group message"]);
        assert!(old_dir.join("Friend Name.txt").is_file());
        assert!(old_dir.join("Builders (group).txt").is_file());
        log.logout();
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn erasing_is_for_the_account_logged_in_only() {
        let root = temp();
        let alice = Uuid::from_u128(0xA11CE);
        let bob = Uuid::from_u128(0xB0B);
        let mut log = ChatLog::in_folder(root.clone());
        log.set_grid("agni".into());
        for agent in [alice, bob] {
            log.login(agent);
            log.append("Jane Doe", now(), "Jane Doe", "hello");
            log.append(names::NEARBY, now(), "Jane Doe", "hello");
        }
        // bob is logged in
        assert!(log.erase());
        assert!(log.flush(WAIT));
        assert_eq!(log.take_erased().unwrap().unwrap(), 2);
        assert!(log.take_erased().is_none());
        let dir = |agent| {
            store::folder(
                &root,
                &Identity {
                    grid: "agni".into(),
                    agent,
                },
            )
        };
        assert!(!dir(bob).exists());
        assert_eq!(texts(&dir(alice).join("Jane Doe.txt")), ["hello"]);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn grids_have_distinct_names() {
        use crate::settings::GridChoice;
        assert_eq!(grid_name(&GridChoice::SecondLife, "", false), "agni");
        assert_eq!(grid_name(&GridChoice::SecondLifeBeta, "", false), "aditi");
        assert_eq!(
            grid_name(&GridChoice::Custom, "http://127.0.0.1:9000/", false),
            "custom_http___127_0_0_1_9000"
        );
        // a custom grid cannot pass for the main one, nor for the demo
        assert_eq!(grid_name(&GridChoice::Custom, "agni", false), "custom_agni");
        assert_eq!(grid_name(&GridChoice::Custom, "demo", false), "custom_demo");
        for choice in [GridChoice::SecondLife, GridChoice::SecondLifeBeta, GridChoice::Custom] {
            assert_eq!(grid_name(&choice, "http://example.invalid/", true), "demo");
        }
    }
}
