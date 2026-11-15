# Deeply Deeply Viewer

Deeply Deeply Viewer 0.9.10.3 Stable is an independent Windows community
viewer prepared by MrDiablo85.

- GitHub: <https://github.com/Diablo85666/Deeply-Deeply-Viewer>
- Discord: <https://discord.gg/VgkrhRZka>
- Author profile: <https://github.com/Diablo85666>
- License and attribution: [NOTICE.md](NOTICE.md)

This repository is the home of the Deeply Deeply Viewer build, its updates,
release packages, documentation, and community support. It is not affiliated
with or endorsed by Linden Lab or any other viewer project.

## Download and updates

Use the official [Deeply Deeply Viewer releases page](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases)
to download the current Windows build and any future Linux packages.

- [Download Deeply Deeply Viewer](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases/latest)
- [Download for Linux](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases/latest)
- [All releases and updates](https://github.com/Diablo85666/Deeply-Deeply-Viewer/releases)

The viewer checks the Deeply Deeply Viewer release feed for newer packages.
An update becomes available after a new release and its assets have been
published to this repository. Updates are never downloaded from an unrelated
repository.

## Deeply Deeply Viewer improvements

### Italian interface and localization

- Italian interface support with the `DEEPLY_UI_LANGUAGE=it` override.
- Stable internal identifiers for translated mouse buttons and shortcuts.
- Localized About, license, privacy, report, and performance messages.

### Firestorm transcript sharing

- Optional shared chat transcripts for compatible account folders.
- Bidirectional access to supported private and group conversations.
- Separate folders for custom grids.
- No transcript changes when sharing is disabled.

### Avatar controls

- Persistent rejection of teleport offers from a selected avatar.
- Persistent rejection of text conference invitations from a selected avatar.
- Separate controls for total block, voice mute, teleport rejection, and
  conference rejection.
- The conference rejection is applied before the conversation is opened.

### Adaptive graphics protection

When sustained pressure is detected, the viewer reduces one graphics level at
a time:

`Deluxe -> High -> Medium -> Low -> Potato`

The monitor considers process memory, frame time, and GPU stalls. It uses
consecutive samples and a cooldown to avoid rapid changes. Custom graphics
profiles are not changed automatically. Emergency safe graphics remains
available for severe graphics-device failures.

### Privacy-aware reports

Reports can be:

- saved locally;
- opened as a GitHub browser draft;
- submitted to the Deeply Deeply Viewer GitHub issue tracker after explicit
  GitHub authorization;
- copied to the clipboard and reviewed in the official Linden Lab support
  form before manual submission.

No Linden Lab data is sent automatically through undocumented endpoints.
Position, avatar information, logs, diagnostics, and screenshots remain
optional and are reviewed by the user before online publication.

## Community

For support, release announcements, testing, and feedback, use:

- [Deeply Deeply Viewer GitHub repository](https://github.com/Diablo85666/Deeply-Deeply-Viewer)
- [Discord community](https://discord.gg/VgkrhRZka)
- [README and project information](https://github.com/Diablo85666/Deeply-Deeply-Viewer?tab=readme-ov-file)

## Windows package

The distributed executable is:

`Deeply-Deeply-Viewer.exe`

The portable ZIP package can be extracted without replacing another viewer
installation. Check the release notes and SHA-256 checksum published with
each package before installing.

## License

Deeply Deeply Viewer is distributed under the GNU GPLv3 terms applicable to
the source code. See [LICENSE](LICENSE) and [NOTICE.md](NOTICE.md) for the
complete license and attribution information.
