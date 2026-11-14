//! Preferences › Deeply: what Deeply does differently from the Legacy
//! Viewer, and for each redrawn element the way back to the Legacy look.
//! The entry of the sidebar carries the deeply gradient, the one exception
//! to the theme here (docs/BRANDING.md); the settings keep the rows of the
//! other pages.

use super::super::cx::{Cx, F, field};
use crate::build::MoveHologram;
use crate::settings::{ChatStyle, DestinationsStyle, NotificationsStyle, PhotoWindowStyle, Settings, WorldMapStyle};
use crate::theme::Palette;
use crate::world::lookat::{Aim, Style};
use egui::epaint::{Mesh, Vertex, WHITE_UV};
use egui::{Color32, FontId, Pos2, Rect, Sense, Shape, Vec2};

/// The two looks a redrawn element can take.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Look {
    Deeply,
    Legacy,
}

/// Every choice of the page set to one look (the colored sky ambient
/// included), and Deeply's water with it;
/// the avatar plates' own settings and the filing of received items are
/// left as they are, and so is a gaze on the point under the cursor: the
/// Deeply look does not choose to show others what the cursor is over.
pub fn set_all(s: &mut Settings, look: Look) {
    let deeply = look == Look::Deeply;
    s.chat_style = if deeply { ChatStyle::Deeply } else { ChatStyle::Legacy };
    s.classic_name_tags = !deeply;
    s.lookat.style = if deeply { Style::Deeply } else { Style::Legacy };
    if !deeply {
        s.lookat.aim = Aim::Direction;
    }
    s.notifications_style = if deeply {
        NotificationsStyle::Deeply
    } else {
        NotificationsStyle::Legacy
    };
    s.world_map_style = if deeply { WorldMapStyle::Deeply } else { WorldMapStyle::Legacy };
    s.classic_bars = !deeply;
    s.classic_people = !deeply;
    s.classic_profiles = !deeply;
    s.photo_window = if deeply {
        PhotoWindowStyle::Deeply
    } else {
        PhotoWindowStyle::Legacy
    };
    s.classic_about_land = !deeply;
    s.classic_search = !deeply;
    s.classic_region_estate = !deeply;
    s.classic_inventory = !deeply;
    s.classic_outfit_gallery = !deeply;
    s.build.legacy_window = !deeply;
    s.classic_places = !deeply;
    s.destinations_style = if deeply {
        DestinationsStyle::Deeply
    } else {
        DestinationsStyle::Legacy
    };
    s.classic_pay_window = !deeply;
    s.legacy_money_tracker = !deeply;
    s.classic_controls = !deeply;
    s.classic_restart_alert = !deeply;
    s.classic_chat_lines = !deeply;
    s.classic_rez_effect = !deeply;
    s.classic_hover_tips = !deeply;
    s.predict_own_motion = deeply;
    s.water_effects.enabled = deeply;
    s.synchronize_animations = deeply;
    s.colored_sky_ambient = deeply;
}

/// The deeply gradient at `t` (0 at its start, 1 at its end): the brand's
/// green, turquoise, indigo, violet and rose, taken from the palette.
fn deeply_color(p: &Palette, t: f32) -> Color32 {
    let stops = [p.success, p.teal, p.indigo_light, p.violet_light, p.rose];
    let x = t.clamp(0.0, 1.0) * (stops.len() - 1) as f32;
    let i = (x.floor() as usize).min(stops.len() - 2);
    stops[i].lerp_to_gamma(stops[i + 1], x - i as f32)
}

fn vertex(pos: Pos2, color: Color32) -> Vertex {
    Vertex { pos, uv: WHITE_UV, color }
}

/// A strip of the deeply gradient, from its top to its bottom.
fn deeply_strip(p: &Palette, r: Rect) -> Shape {
    const STEPS: usize = 16;
    let mut m = Mesh::default();
    for i in 0..=STEPS {
        let t = i as f32 / STEPS as f32;
        let c = deeply_color(p, t);
        let y = r.top() + r.height() * t;
        m.vertices.push(vertex(Pos2::new(r.left(), y), c));
        m.vertices.push(vertex(Pos2::new(r.right(), y), c));
        if i > 0 {
            let k = (i as u32) * 2;
            m.add_triangle(k - 2, k - 1, k);
            m.add_triangle(k - 1, k + 1, k);
        }
    }
    Shape::mesh(m)
}

/// The entry of the page in the sidebar: the neighbours' entry, with the
/// deeply gradient in its edge mark and its name.
pub fn nav_entry(ui: &mut egui::Ui, p: &Palette, label: &str, icon: &str, selected: bool) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 34.0), Sense::click());
    let radius = ui.visuals().widgets.inactive.corner_radius;
    let face = if selected {
        p.violet.gamma_multiply(0.24)
    } else if response.hovered() {
        p.raised
    } else {
        p.raised.gamma_multiply(0.3)
    };
    ui.painter().rect_filled(rect, radius, face);
    ui.painter().add(deeply_strip(
        p,
        Rect::from_min_max(rect.left_top(), Pos2::new(rect.left() + 3.0, rect.bottom())),
    ));
    if let Some(texture) = crate::ui::icons::global(icon) {
        ui.painter().image(
            texture.id(),
            Rect::from_center_size(rect.left_center() + Vec2::new(17.0, 0.0), Vec2::splat(17.0)),
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            p.teal,
        );
    }
    // the name in the gradient, one letter after the other
    let font = FontId::new(12.0, crate::ui::fonts::semibold(ui.ctx()));
    let count = label.chars().count().max(2) as f32 - 1.0;
    let mut job = egui::text::LayoutJob::default();
    for (i, ch) in label.chars().enumerate() {
        let color = deeply_color(p, i as f32 / count);
        job.append(&ch.to_string(), 0.0, egui::TextFormat::simple(font.clone(), color));
    }
    let galley = ui.painter().layout_job(job);
    ui.painter()
        .galley(rect.left_center() + Vec2::new(34.0, -galley.size().y / 2.0), galley, p.ink);
    response.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, selected, label));
    response.on_hover_text(label)
}

/// A choice between Deeply's look (first, in the accent) and the Legacy
/// one, over a "classic" switch.
fn look(cx: &mut Cx, name: &str, help: &str, deeply: &str, legacy: &str, f: &F<bool>) {
    cx.wide().choice(name, help, f, &[(false, deeply, true), (true, legacy, false)]);
}

/// An experimental choice: its warning joins its help.
fn warned(help: &str, warning: &str) -> String {
    format!("{help}\n{warning}")
}

pub fn page(cx: &mut Cx, tab: usize) {
    use deeply_i18n::tr;
    let deeply = tr!("ui-options-deeply-about-land-deeply");
    let classic = tr!("ui-options-deeply-about-land-classic");
    match tab {
        0 => cx.group(tr!("ui-options-deeply-redrawn-windows"), |cx| {
            cx.wide().choice(
                tr!("ui-options-deeply-conversations"),
                tr!("ui-options-deeply-conversations-help"),
                &field!(chat_style),
                &[
                    (ChatStyle::Deeply, tr!("ui-options-deeply-workspace"), true),
                    (ChatStyle::Compact, tr!("ui-options-deeply-compact"), true),
                    (ChatStyle::Bubbles, tr!("ui-options-deeply-bubbles"), true),
                    (ChatStyle::Legacy, tr!("ui-options-deeply-lines"), false),
                ],
            );
            look(
                cx,
                tr!("ui-options-deeply-chat-lines"),
                tr!("ui-options-deeply-chat-lines-help"),
                tr!("ui-options-deeply-chat-lines-cards"),
                tr!("ui-options-deeply-chat-lines-classic"),
                &field!(classic_chat_lines),
            );
            cx.wide().choice(
                tr!("ui-options-deeply-world-map"),
                tr!("ui-options-deeply-world-map-help"),
                &field!(world_map_style),
                &[
                    (WorldMapStyle::Deeply, tr!("ui-options-deeply-world-map-full"), true),
                    (WorldMapStyle::Legacy, tr!("ui-options-deeply-world-map-side"), false),
                ],
            );
            look(
                cx,
                tr!("ui-options-deeply-places"),
                tr!("ui-options-deeply-places-help"),
                tr!("ui-options-deeply-places-deeply"),
                tr!("ui-options-deeply-places-classic"),
                &field!(classic_places),
            );
            cx.wide().choice(
                tr!("ui-options-deeply-destinations"),
                tr!("ui-options-deeply-destinations-help"),
                &field!(destinations_style),
                &[
                    (DestinationsStyle::Deeply, tr!("ui-options-deeply-destinations-deeply"), true),
                    (DestinationsStyle::Legacy, tr!("ui-options-deeply-destinations-classic"), false),
                ],
            );
            look(
                cx,
                tr!("ui-options-deeply-people"),
                tr!("ui-options-deeply-people-help"),
                tr!("ui-options-deeply-people-rows"),
                tr!("ui-options-deeply-people-table"),
                &field!(classic_people),
            );
            look(
                cx,
                tr!("ui-options-deeply-profiles"),
                tr!("ui-options-deeply-profiles-help"),
                tr!("ui-options-deeply-profiles-deeply"),
                tr!("ui-options-deeply-profiles-classic"),
                &field!(classic_profiles),
            );
            let classic_profiles = cx.s.classic_profiles;
            cx.subs(classic_profiles, |cx| {
                cx.toggle(
                    tr!("ui-options-deeply-profiles-rights"),
                    tr!("ui-options-deeply-profiles-rights-help"),
                    &field!(profile_rights),
                );
            });
            cx.wide().choice(
                tr!("ui-options-deeply-photo-window"),
                tr!("ui-options-deeply-photo-window-help"),
                &field!(photo_window),
                &[
                    (PhotoWindowStyle::Deeply, tr!("ui-options-deeply-photo-viewfinder"), true),
                    (PhotoWindowStyle::Panel, tr!("ui-options-deeply-photo-panel"), true),
                    (PhotoWindowStyle::Legacy, tr!("ui-options-deeply-photo-classic"), false),
                ],
            );
            let viewfinder = cx.s.photo_window == PhotoWindowStyle::Deeply;
            cx.subs(!viewfinder, |cx| {
                use crate::snapshot::viewfinder::OnScreen;
                let labels = [
                    (tr!("ui-options-deeply-vf-controls-always"), true),
                    (tr!("ui-options-deeply-vf-controls-hover"), true),
                    (tr!("ui-options-deeply-vf-controls-hidden"), false),
                ];
                let choices: Vec<(OnScreen, &str, bool)> = OnScreen::ALL.iter().zip(labels).map(|(o, (l, a))| (*o, l, a)).collect();
                cx.wide().choice(
                    tr!("ui-options-deeply-vf-controls"),
                    tr!("ui-options-deeply-vf-controls-help"),
                    &field!(snapshot.viewfinder.controls),
                    &choices,
                );
            });
            look(
                cx,
                tr!("ui-options-deeply-about-land"),
                tr!("ui-options-deeply-about-land-help"),
                deeply,
                classic,
                &field!(classic_about_land),
            );
            look(
                cx,
                tr!("search-title"),
                tr!("ui-options-deeply-search-help"),
                deeply,
                tr!("ui-options-deeply-search-classic"),
                &field!(classic_search),
            );
            look(
                cx,
                tr!("region-title"),
                tr!("ui-options-deeply-region-estate-help"),
                deeply,
                classic,
                &field!(classic_region_estate),
            );
            look(
                cx,
                tr!("common-inventory"),
                tr!("ui-options-deeply-inventory-help"),
                deeply,
                classic,
                &field!(classic_inventory),
            );
            look(
                cx,
                tr!("ui-options-deeply-outfit-gallery"),
                tr!("ui-options-deeply-outfit-gallery-help"),
                tr!("ui-options-deeply-outfit-gallery-deeply"),
                tr!("ui-options-deeply-outfit-gallery-classic"),
                &field!(classic_outfit_gallery),
            );
            look(
                cx,
                tr!("ui-options-deeply-build"),
                tr!("ui-options-deeply-build-help"),
                deeply,
                classic,
                &field!(build.legacy_window),
            );
            look(
                cx,
                tr!("ui-options-deeply-pay-window"),
                tr!("ui-options-deeply-pay-window-help"),
                tr!("ui-options-deeply-pay-deeply"),
                tr!("ui-options-deeply-pay-classic"),
                &field!(classic_pay_window),
            );
            look(
                cx,
                tr!("money-tracker-title"),
                tr!("ui-options-deeply-money-tracker-help"),
                deeply,
                classic,
                &field!(legacy_money_tracker),
            );
            look(
                cx,
                tr!("ui-options-deeply-controls"),
                tr!("ui-options-deeply-controls-help"),
                tr!("ui-options-deeply-controls-deeply"),
                tr!("ui-options-deeply-controls-classic"),
                &field!(classic_controls),
            );
        }),
        1 => {
            cx.group(tr!("ui-options-deeply-in-the-world"), |cx| {
                look(
                    cx,
                    tr!("ui-options-deeply-name-tags"),
                    tr!("ui-options-deeply-name-tags-help"),
                    tr!("ui-options-deeply-plates"),
                    tr!("ui-options-deeply-classic"),
                    &field!(classic_name_tags),
                );
                cx.wide().choice(
                    tr!("ui-options-deeply-look-at"),
                    tr!("ui-options-deeply-look-at-help"),
                    &field!(lookat.style),
                    &[
                        (Style::Deeply, tr!("ui-options-deeply-halo"), true),
                        (Style::Legacy, tr!("ui-options-deeply-crosshair"), false),
                    ],
                );
                look(
                    cx,
                    tr!("ui-options-deeply-rez-effect"),
                    tr!("ui-options-deeply-rez-effect-help"),
                    tr!("ui-options-deeply-rez-effect-deeply"),
                    tr!("ui-options-deeply-rez-effect-classic"),
                    &field!(classic_rez_effect),
                );
                look(
                    cx,
                    tr!("ui-options-deeply-hover-tips"),
                    tr!("ui-options-deeply-hover-tips-help"),
                    deeply,
                    tr!("ui-options-deeply-hover-tips-legacy"),
                    &field!(classic_hover_tips),
                );
                // also in Graphics › Sky and environment, the same setting
                cx.wide().experimental().choice(
                    tr!("ui-options-colored-sky-ambient"),
                    tr!("ui-options-colored-sky-ambient-help"),
                    &field!(colored_sky_ambient),
                    &[(true, deeply, true), (false, tr!("ui-options-deeply-classic"), false)],
                );
            });
            cx.group(tr!("common-interface"), |cx| {
                cx.wide().choice(
                    tr!("ui-options-deeply-notifications"),
                    tr!("ui-options-deeply-notifications-help"),
                    &field!(notifications_style),
                    &[
                        (NotificationsStyle::Deeply, tr!("ui-options-deeply-animated-bell"), true),
                        (NotificationsStyle::Legacy, tr!("ui-options-deeply-toasts"), false),
                    ],
                );
                look(
                    cx,
                    tr!("ui-options-deeply-bars"),
                    tr!("ui-options-deeply-bars-help"),
                    tr!("ui-options-deeply-bars-islands"),
                    tr!("ui-options-deeply-bars-classic"),
                    &field!(classic_bars),
                );
                look(
                    cx,
                    tr!("ui-options-deeply-restart-alert"),
                    tr!("ui-options-deeply-restart-alert-help"),
                    tr!("ui-options-deeply-restart-card"),
                    tr!("ui-options-deeply-restart-classic"),
                    &field!(classic_restart_alert),
                );
            });
        }
        _ => {
            cx.group(tr!("ui-options-deeply-building"), |cx| {
                cx.wide().choice(
                    tr!("ui-options-deeply-move-hologram"),
                    tr!("ui-options-deeply-move-hologram-help"),
                    &field!(build.move_hologram),
                    &[
                        (MoveHologram::Off, tr!("ui-options-deeply-move-hologram-off"), false),
                        (MoveHologram::Moving, tr!("ui-options-deeply-move-hologram-moving"), true),
                        (MoveHologram::Start, tr!("ui-options-deeply-move-hologram-start"), true),
                    ],
                );
            });
            cx.group(tr!("ui-options-deeply-bridge"), |cx| {
                cx.toggle(
                    tr!("ui-options-deeply-use-bridge"),
                    tr!("ui-options-deeply-use-bridge-help"),
                    &field!(use_bridge),
                );
                use crate::world::portals::Sources;
                cx.combo(
                    tr!("ui-options-deeply-portal-sources"),
                    tr!("ui-options-deeply-portal-sources-help"),
                    &field!(portal_sources),
                    &[
                        (Sources::Friends, tr!("ui-options-deeply-portal-sources-friends")),
                        (Sources::FriendsAndGroups, tr!("ui-options-deeply-portal-sources-groups")),
                        (Sources::Everyone, tr!("ui-options-deeply-portal-sources-everyone")),
                        (Sources::Nobody, tr!("ui-options-deeply-portal-sources-nobody")),
                    ],
                );
            });
            cx.group(tr!("ui-options-deeply-gaze"), |cx| {
                cx.wide().choice(
                    tr!("ui-options-deeply-aim"),
                    tr!("ui-options-deeply-aim-help"),
                    &field!(lookat.aim),
                    &[
                        (Aim::Point, tr!("ui-options-deeply-aim-point"), true),
                        (Aim::Direction, tr!("ui-options-deeply-aim-direction"), false),
                    ],
                );
            });
            // the Legacy Viewer's until the human has tried it on the grid,
            // marked experimental meanwhile
            cx.group(tr!("ui-options-deeply-movement"), |cx| {
                let help = warned(
                    tr!("ui-options-deeply-own-motion-help"),
                    tr!("ui-options-deeply-own-motion-experimental"),
                );
                cx.wide().experimental().choice(
                    tr!("ui-options-deeply-own-motion"),
                    &help,
                    &field!(predict_own_motion),
                    &[
                        (true, tr!("ui-options-deeply-own-motion-instant"), true),
                        (false, tr!("ui-options-deeply-own-motion-server"), false),
                    ],
                );
            });
            cx.group(tr!("ui-options-deeply-animations"), |cx| {
                let help = warned(
                    tr!("ui-options-deeply-animation-clock-help"),
                    tr!("ui-options-deeply-animation-clock-experimental"),
                );
                cx.wide().experimental().choice(
                    tr!("ui-options-deeply-animation-clock"),
                    &help,
                    &field!(synchronize_animations),
                    &[
                        (true, tr!("ui-options-deeply-animation-clock-synchronized"), true),
                        (false, tr!("ui-options-deeply-animation-clock-legacy"), false),
                    ],
                );
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn received_filing_stays_opt_in_when_the_looks_change() {
        let mut settings = Settings::default();
        set_all(&mut settings, Look::Deeply);
        assert!(!settings.sort_received_items);
        settings.sort_received_items = true;
        set_all(&mut settings, Look::Legacy);
        assert!(settings.sort_received_items);
    }

    #[test]
    fn the_two_buttons_set_the_colored_sky_ambient_too() {
        let mut s = Settings::default();
        assert!(!s.colored_sky_ambient, "off by default");
        set_all(&mut s, Look::Deeply);
        assert!(s.colored_sky_ambient);
        set_all(&mut s, Look::Legacy);
        assert!(!s.colored_sky_ambient);
    }

    #[test]
    fn the_deeply_button_keeps_the_gaze_choice() {
        let mut s = Settings::default();
        set_all(&mut s, Look::Deeply);
        assert_eq!(s.lookat.aim, Aim::Direction, "never turned to the point by the button");
        s.lookat.aim = Aim::Point;
        set_all(&mut s, Look::Deeply);
        assert_eq!(s.lookat.aim, Aim::Point, "a chosen point stays");
        set_all(&mut s, Look::Legacy);
        assert_eq!(s.lookat.aim, Aim::Direction);
    }

    #[test]
    fn the_two_buttons_set_every_look_and_leave_the_plates_alone() {
        let mut s = Settings {
            plates_friends_only_full: true,
            ..Settings::default()
        };
        set_all(&mut s, Look::Legacy);
        assert_eq!(s.chat_style, ChatStyle::Legacy);
        assert!(s.classic_name_tags);
        assert_eq!(s.lookat.style, Style::Legacy);
        assert_eq!(s.lookat.aim, Aim::Direction);
        assert_eq!(s.notifications_style, NotificationsStyle::Legacy);
        assert_eq!(s.world_map_style, WorldMapStyle::Legacy);
        assert!(s.classic_bars, "the full-width bars come back");
        assert!(s.classic_people, "People › Near you shows the radar table");
        assert!(s.classic_profiles);
        assert_eq!(s.photo_window, PhotoWindowStyle::Legacy);
        assert!(s.classic_about_land, "About Land opens the Legacy Viewer's nine tabs");
        assert!(s.classic_search, "Search opens the Legacy Viewer's tabs");
        assert!(s.classic_region_estate);
        assert!(s.classic_inventory, "the Legacy Viewer's single inventory tree");
        assert!(s.classic_outfit_gallery, "the Legacy Viewer's small outfit tiles");
        assert!(s.build.legacy_window, "the Legacy Viewer's build tools and tabs");
        assert!(s.classic_places, "Places opens the Legacy Viewer's three tabs");
        assert_eq!(s.destinations_style, DestinationsStyle::Legacy, "the guide's web page");
        assert!(s.classic_pay_window);
        assert!(s.legacy_money_tracker, "the Legacy Viewer's Time / Name / Amount list");
        assert!(s.classic_controls, "the Legacy Viewer's two control windows");
        assert!(s.classic_restart_alert, "the Legacy Viewer's yellow restart box");
        assert!(s.classic_chat_lines, "the Legacy Viewer's one-line chat lines");
        assert!(s.classic_rez_effect, "grey objects while they load");
        assert!(s.classic_hover_tips, "the Legacy Viewer's text tooltips");
        assert!(!s.predict_own_motion, "the avatar waits for the simulator");
        assert!(!s.synchronize_animations);
        assert!(!s.water_effects.enabled);
        assert!(s.plates_friends_only_full, "a plate setting is not a look");
        set_all(&mut s, Look::Deeply);
        let fresh = Settings::default();
        assert_eq!(s.chat_style, fresh.chat_style);
        assert_eq!(s.classic_name_tags, fresh.classic_name_tags);
        assert_eq!(s.lookat.style, fresh.lookat.style);
        assert_eq!(s.lookat.aim, fresh.lookat.aim);
        assert_eq!(s.notifications_style, NotificationsStyle::Deeply);
        assert_eq!(fresh.notifications_style, NotificationsStyle::Deeply);
        assert_eq!(s.world_map_style, fresh.world_map_style);
        assert!(!s.classic_bars && !fresh.classic_bars, "the islands are the default");
        assert!(
            !s.classic_people && !fresh.classic_people,
            "the rows grouped by range are the default"
        );
        assert_eq!(s.classic_profiles, fresh.classic_profiles);
        assert_eq!(s.photo_window, PhotoWindowStyle::Deeply);
        assert_eq!(fresh.photo_window, PhotoWindowStyle::Deeply);
        assert_eq!(s.classic_about_land, fresh.classic_about_land);
        assert!(!fresh.classic_about_land, "the Deeply About Land is the default");
        assert!(!s.classic_search && !fresh.classic_search, "the Deeply Search is the default");
        assert!(!s.classic_region_estate && !fresh.classic_region_estate);
        assert!(
            !s.classic_inventory && !fresh.classic_inventory,
            "the Deeply inventory is the default"
        );
        assert!(!s.classic_places && !fresh.classic_places, "Deeply's Places is the default");
        assert_eq!(s.destinations_style, DestinationsStyle::Deeply);
        assert_eq!(
            fresh.destinations_style,
            DestinationsStyle::Deeply,
            "Deeply's Destinations is the default"
        );
        assert!(
            !s.classic_outfit_gallery && !fresh.classic_outfit_gallery,
            "the showcase is the default"
        );
        assert!(!s.build.legacy_window && !fresh.build.legacy_window);
        assert!(!s.classic_pay_window && !fresh.classic_pay_window);
        assert!(!s.legacy_money_tracker && !fresh.legacy_money_tracker);
        assert!(!s.classic_controls && !fresh.classic_controls);
        assert!(!s.classic_restart_alert && !fresh.classic_restart_alert);
        assert!(!s.classic_chat_lines && !fresh.classic_chat_lines, "the cards are the default");
        assert!(!s.classic_rez_effect && !fresh.classic_rez_effect, "the rez effect is the default");
        assert!(!s.classic_hover_tips && !fresh.classic_hover_tips, "Deeply's tips are the default");
        assert!(s.predict_own_motion, "Deeply's movement");
        assert!(!fresh.predict_own_motion, "the Legacy movement until tried on the grid");
        assert!(s.synchronize_animations, "the Deeply bulk choice enables receipt clocks");
        assert!(s.water_effects.enabled, "the explicit Deeply bulk choice enables water effects");
        assert!(!fresh.water_effects.enabled, "enhanced water is disabled by default");
        assert!(!fresh.synchronize_animations, "Legacy animation timing is the default");
        assert!(s.plates_friends_only_full);
    }

    /// The notifications' style is saved; a file from before it was added
    /// gets the animated bell.
    #[test]
    fn the_notifications_style_is_saved() {
        let s = Settings {
            notifications_style: NotificationsStyle::Legacy,
            ..Settings::default()
        };
        let json = serde_json::to_string(&s).expect("settings serialize");
        let back: Settings = serde_json::from_str(&json).expect("settings read back");
        assert_eq!(back.notifications_style, NotificationsStyle::Legacy);
        let old: Settings = serde_json::from_str("{}").expect("empty settings");
        assert_eq!(old.notifications_style, NotificationsStyle::Deeply);
    }

    #[test]
    fn the_deeply_gradient_runs_from_green_to_rose() {
        let p = crate::theme::Theme::default().palette();
        assert_eq!(deeply_color(&p, 0.0), p.success);
        assert_eq!(deeply_color(&p, 1.0), p.rose);
        assert_eq!(deeply_color(&p, 0.5), p.indigo_light);
    }
}
