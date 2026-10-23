//! The native menu bar, described by the page (krizaka-ui ADR-002: « desktop is the web in a window »).
//!
//! The page owns the words (its messages, in the person's language) and the shortcuts (the same map `useShortcuts`
//! answers, turned into accelerators with `acceleratorOf` of `@krizaka/ui`). It sends a [`MenuSpec`]; the kit checks it
//! here, then builds the macOS menu: the app menu (about, hide, quit), the Edit menu (without it ⌘C and ⌘V do nothing
//! in a text field), the product's menus, the Window menu. A click on a product item is sent back to the page by its id.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// @fixed: a menu bar a person can read, not a setting
pub const MAX_MENUS: usize = 6;
// @fixed: a menu a person can read, not a setting
pub const MAX_ITEMS: usize = 24;
// @fixed: the width of a menu label, not a setting
pub const MAX_LABEL: usize = 60;

/// The labels of the menus macOS expects, in the page's language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardLabels {
    pub about: String,
    pub hide: String,
    pub hide_others: String,
    pub show_all: String,
    pub quit: String,
    pub edit: String,
    pub undo: String,
    pub redo: String,
    pub cut: String,
    pub copy: String,
    pub paste: String,
    pub select_all: String,
    pub window: String,
    pub minimize: String,
    pub zoom: String,
    pub close: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuItemSpec {
    /// Sent back to the page when the item is chosen: lowercase letters, digits, `.`, `-` (`library.add-folder`).
    pub id: String,
    pub label: String,
    /// A Tauri accelerator (`CmdOrCtrl+K`), from `acceleratorOf` of `@krizaka/ui`.
    #[serde(default)]
    pub accelerator: Option<String>,
    #[serde(default = "enabled")]
    pub enabled: bool,
    /// A separator before this item.
    #[serde(default)]
    pub separated: bool,
}

fn enabled() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuSpec {
    pub labels: StandardLabels,
    /// The product's own menus, between Edit and Window (File, View, Go…).
    pub menus: Vec<MenuGroupSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuGroupSpec {
    pub title: String,
    pub items: Vec<MenuItemSpec>,
}

/// The shortcuts macOS gives to the standard menus (quit, hide, minimise, close, the Edit menu): never a product's.
pub const RESERVED: &[&str] = &[
    "CmdOrCtrl+Q",
    "CmdOrCtrl+H",
    "Alt+CmdOrCtrl+H",
    "CmdOrCtrl+M",
    "CmdOrCtrl+W",
    "CmdOrCtrl+Z",
    "Shift+CmdOrCtrl+Z",
    "CmdOrCtrl+X",
    "CmdOrCtrl+C",
    "CmdOrCtrl+V",
    "CmdOrCtrl+A",
];

const MODIFIERS: &[&str] = &["CmdOrCtrl", "Cmd", "Ctrl", "Alt", "Shift", "Super"];
const NAMED_KEYS: &[&str] = &[
    "Escape",
    "Enter",
    "Space",
    "Up",
    "Down",
    "Left",
    "Right",
    "Delete",
    "Backspace",
    "Tab",
    "Plus",
];

/// A label is words a person reads: not empty, not too long, no control character.
fn check_label(label: &str, what: &str) -> Result<(), String> {
    let n = label.chars().count();
    if label.trim().is_empty() || n > MAX_LABEL || label.chars().any(char::is_control) {
        return Err(format!("{what}: a label of 1 to {MAX_LABEL} characters"));
    }
    Ok(())
}

/// An accelerator as `acceleratorOf` writes it: modifiers, then one key (`CmdOrCtrl+Shift+K`, `CmdOrCtrl+,`, `CmdOrCtrl+1`).
pub fn check_accelerator(accelerator: &str) -> Result<(), String> {
    let parts: Vec<&str> = accelerator.split('+').collect();
    let (key, modifiers) = parts.split_last().ok_or("an empty accelerator")?;
    let mut seen = HashSet::new();
    for m in modifiers {
        if !MODIFIERS.contains(m) || !seen.insert(*m) {
            return Err(format!("not a modifier: {m} in {accelerator}"));
        }
    }
    let single = key.chars().count() == 1
        && key
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || ",./;'[]-=`".contains(c));
    let function = key.len() >= 2
        && key.starts_with('F')
        && key[1..].parse::<u8>().is_ok_and(|n| (1..=12).contains(&n));
    if !(single || function || NAMED_KEYS.contains(key)) {
        return Err(format!("not a key: {key} in {accelerator}"));
    }
    Ok(())
}

fn check_id(id: &str) -> Result<(), String> {
    let ok = !id.is_empty()
        && id.len() <= 64
        && id.starts_with(|c: char| c.is_ascii_lowercase())
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-');
    if ok {
        Ok(())
    } else {
        Err(format!("not a menu id: {id}"))
    }
}

impl MenuSpec {
    /// Everything the page sent, checked before a native menu is built from it.
    pub fn validate(&self) -> Result<(), String> {
        let l = &self.labels;
        for (label, what) in [
            (&l.about, "about"),
            (&l.hide, "hide"),
            (&l.hide_others, "hideOthers"),
            (&l.show_all, "showAll"),
            (&l.quit, "quit"),
            (&l.edit, "edit"),
            (&l.undo, "undo"),
            (&l.redo, "redo"),
            (&l.cut, "cut"),
            (&l.copy, "copy"),
            (&l.paste, "paste"),
            (&l.select_all, "selectAll"),
            (&l.window, "window"),
            (&l.minimize, "minimize"),
            (&l.zoom, "zoom"),
            (&l.close, "close"),
        ] {
            check_label(label, what)?;
        }
        if self.menus.len() > MAX_MENUS {
            return Err(format!("at most {MAX_MENUS} menus"));
        }
        let mut ids = HashSet::new();
        let mut accelerators = HashSet::new();
        for menu in &self.menus {
            check_label(&menu.title, "menu title")?;
            if menu.items.is_empty() || menu.items.len() > MAX_ITEMS {
                return Err(format!("{}: 1 to {MAX_ITEMS} items", menu.title));
            }
            for item in &menu.items {
                check_id(&item.id)?;
                check_label(&item.label, &item.id)?;
                if !ids.insert(item.id.as_str()) {
                    return Err(format!("the id {} is used twice", item.id));
                }
                if let Some(a) = &item.accelerator {
                    check_accelerator(a)?;
                    if RESERVED.contains(&a.as_str()) {
                        return Err(format!("{a} belongs to a standard menu"));
                    }
                    if !accelerators.insert(a.as_str()) {
                        return Err(format!("the shortcut {a} is used twice"));
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels() -> StandardLabels {
        let w = |s: &str| s.to_string();
        StandardLabels {
            about: w("À propos de Yomizaka"),
            hide: w("Masquer Yomizaka"),
            hide_others: w("Masquer les autres"),
            show_all: w("Tout afficher"),
            quit: w("Quitter Yomizaka"),
            edit: w("Édition"),
            undo: w("Annuler"),
            redo: w("Rétablir"),
            cut: w("Couper"),
            copy: w("Copier"),
            paste: w("Coller"),
            select_all: w("Tout sélectionner"),
            window: w("Fenêtre"),
            minimize: w("Réduire"),
            zoom: w("Zoom"),
            close: w("Fermer"),
        }
    }

    fn item(id: &str, accelerator: Option<&str>) -> MenuItemSpec {
        MenuItemSpec {
            id: id.into(),
            label: "Ajouter un dossier…".into(),
            accelerator: accelerator.map(Into::into),
            enabled: true,
            separated: false,
        }
    }

    fn spec(items: Vec<MenuItemSpec>) -> MenuSpec {
        MenuSpec {
            labels: labels(),
            menus: vec![MenuGroupSpec {
                title: "Fichier".into(),
                items,
            }],
        }
    }

    #[test]
    fn a_menu_as_the_page_sends_it_is_accepted() {
        let s = spec(vec![
            item("library.add-folder", Some("CmdOrCtrl+O")),
            item("check.new", Some("CmdOrCtrl+N")),
            item("go.reports", Some("CmdOrCtrl+2")),
            item("app.settings", Some("CmdOrCtrl+,")),
            item("app.search", Some("Shift+CmdOrCtrl+K")),
            item("view.help", None),
        ]);
        assert_eq!(s.validate(), Ok(()));
        let from_page: MenuSpec = serde_json::from_value(serde_json::json!({
            "labels": serde_json::to_value(labels()).unwrap(),
            "menus": [{ "title": "Fichier", "items": [{ "id": "check.new", "label": "Nouvelle vérification", "accelerator": "CmdOrCtrl+N" }] }]
        }))
        .unwrap();
        assert!(from_page.menus[0].items[0].enabled);
        assert_eq!(from_page.validate(), Ok(()));
    }

    #[test]
    fn accelerators_are_the_ones_accelerator_of_writes() {
        for ok in [
            "CmdOrCtrl+K",
            "Shift+CmdOrCtrl+/",
            "Alt+Up",
            "CmdOrCtrl+Enter",
            "F5",
            "CmdOrCtrl+Plus",
        ] {
            assert_eq!(check_accelerator(ok), Ok(()), "{ok}");
        }
        for bad in [
            "",
            "CmdOrCtrl+",
            "Hyper+K",
            "CmdOrCtrl+CmdOrCtrl+K",
            "CmdOrCtrl+k",
            "CmdOrCtrl+KK",
            "F13",
            "CmdOrCtrl+é",
        ] {
            assert!(check_accelerator(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn refuses_bad_ids_twice_used_ids_or_shortcuts_and_unreadable_labels() {
        assert!(spec(vec![item("Check.New", None)]).validate().is_err());
        assert!(spec(vec![item("check new", None)]).validate().is_err());
        assert!(spec(vec![item("a", None), item("a", None)])
            .validate()
            .is_err());
        assert!(spec(vec![
            item("a", Some("CmdOrCtrl+N")),
            item("b", Some("CmdOrCtrl+N"))
        ])
        .validate()
        .is_err());
        assert!(spec(vec![]).validate().is_err());
        for standard in ["CmdOrCtrl+Q", "CmdOrCtrl+C", "Shift+CmdOrCtrl+Z"] {
            assert!(
                spec(vec![item("a", Some(standard))]).validate().is_err(),
                "{standard}"
            );
        }
        let mut long = item("a", None);
        long.label = "x".repeat(MAX_LABEL + 1);
        assert!(spec(vec![long]).validate().is_err());
        let mut blank = spec(vec![item("a", None)]);
        blank.labels.copy = " ".into();
        assert!(blank.validate().is_err());
        let mut many = spec(vec![item("a", None)]);
        many.menus = (0..=MAX_MENUS)
            .map(|i| MenuGroupSpec {
                title: format!("M{i}"),
                items: vec![item(&format!("m{i}"), None)],
            })
            .collect();
        assert!(many.validate().is_err());
    }
}
