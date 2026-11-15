//! Project links shown on the login screen. Empty = not configured yet: the
//! button is shown disabled and nothing is fetched. The README header carries
//! the same Discord and website links: change them there too.

/// Deeply Deeply Viewer community repository.
pub const GITHUB: &str = "https://github.com/Diablo85666/Deeply-Deeply-Viewer";
/// Issue template chooser, opened in the browser by "Report a bug" (login
/// screen, top bar, Help menu) and by the crash window.
pub const NEW_ISSUE: &str = "https://github.com/Diablo85666/Deeply-Deeply-Viewer/issues/new";
/// The project's wiki, generated from the guides of `docs/`
/// (`scripts/build-wiki.ps1`): Help › Deeply Viewer wiki.
pub const WIKI: &str = "https://github.com/Diablo85666/Deeply-Deeply-Viewer?tab=readme-ov-file";
/// Community Discord invite.
pub const DISCORD: &str = "https://discord.gg/VgkrhRZka";
/// Project website.
pub const WEBSITE: &str = "https://github.com/Diablo85666/Deeply-Deeply-Viewer?tab=readme-ov-file";
/// The privacy policy on the website (Preferences › Privacy › Your data).
pub const PRIVACY_POLICY: &str = "https://github.com/Diablo85666/Deeply-Deeply-Viewer?tab=readme-ov-file";
/// News feed: this folder of `main`, hand-edited Markdown files, one
/// `<language>.md` per UI language (`news/` at the repository root). The
/// viewer keeps what it read in its cache as a JSON array of
/// `{"date", "title", "summary", "url"}`.
pub const NEWS_FEED: &str = "https://raw.githubusercontent.com/Diablo85666/Deeply-Deeply-Viewer/main/news/";
/// Files of the source repository as GitHub serves them unrendered; a branch
/// name and a path follow.
pub const RAW_FILES: &str = "https://raw.githubusercontent.com/Diablo85666/Deeply-Deeply-Viewer/";
