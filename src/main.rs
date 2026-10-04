// Navigateur Rust : une fenêtre GTK 3 avec une barre d'adresse, autour du moteur WebKitGTK
// (le même moteur que GNOME Web/Epiphany) — pas un moteur de rendu réécrit en Rust : ça, c'est
// le travail de plusieurs années d'une grande équipe (voir Servo, chez Mozilla). WebKitGTK, lui,
// est déjà présent (ou installable en un paquet) sur la plupart des bureaux Linux, dont CachyOS.

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box as BoiteGtk, Button, Entry, IconSize, Orientation};
use webkit2gtk::{LoadEvent, WebView, WebViewExt};

const ACCUEIL: &str = "https://duckduckgo.com/";

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("fr.manboyx.navigateur-rust")
        .build();
    application.connect_activate(construire_fenetre);
    application.run()
}

fn construire_fenetre(app: &Application) {
    let fenetre = ApplicationWindow::builder()
        .application(app)
        .title("Navigateur Rust")
        .default_width(1100)
        .default_height(720)
        .build();

    let barre = BoiteGtk::new(Orientation::Horizontal, 6);
    barre.set_margin_top(6);
    barre.set_margin_bottom(6);
    barre.set_margin_start(6);
    barre.set_margin_end(6);

    let precedent = Button::from_icon_name(Some("go-previous-symbolic"), IconSize::Button);
    let suivant = Button::from_icon_name(Some("go-next-symbolic"), IconSize::Button);
    let actualiser = Button::from_icon_name(Some("view-refresh-symbolic"), IconSize::Button);
    let accueil = Button::from_icon_name(Some("go-home-symbolic"), IconSize::Button);
    let adresse = Entry::new();
    adresse.set_hexpand(true);
    adresse.set_placeholder_text(Some("Adresse, ou une recherche…"));

    barre.pack_start(&precedent, false, false, 0);
    barre.pack_start(&suivant, false, false, 0);
    barre.pack_start(&actualiser, false, false, 0);
    barre.pack_start(&accueil, false, false, 0);
    barre.pack_start(&adresse, true, true, 0);

    let contenu = BoiteGtk::new(Orientation::Vertical, 0);
    let vue = WebView::new();
    contenu.pack_start(&barre, false, false, 0);
    contenu.pack_start(&vue, true, true, 0);
    fenetre.add(&contenu);

    vue.load_uri(ACCUEIL);

    adresse.connect_activate({
        let vue = vue.clone();
        move |champ| vue.load_uri(&normaliser(&champ.text()))
    });
    precedent.connect_clicked({
        let vue = vue.clone();
        move |_| vue.go_back()
    });
    suivant.connect_clicked({
        let vue = vue.clone();
        move |_| vue.go_forward()
    });
    actualiser.connect_clicked({
        let vue = vue.clone();
        move |_| vue.reload()
    });
    accueil.connect_clicked({
        let vue = vue.clone();
        move |_| vue.load_uri(ACCUEIL)
    });

    // La barre d'adresse suit la page une fois la navigation confirmée (pas avant : sinon une
    // redirection intermédiaire s'y afficherait au lieu de l'adresse finale).
    vue.connect_load_changed({
        let adresse = adresse.clone();
        move |vue, evenement| {
            if evenement == LoadEvent::Committed {
                if let Some(uri) = vue.uri() {
                    adresse.set_text(&uri);
                }
            }
        }
    });
    vue.connect_title_notify({
        let fenetre = fenetre.clone();
        move |vue| {
            let titre = vue.title().map(|t| t.to_string()).unwrap_or_default();
            fenetre.set_title(&if titre.is_empty() {
                "Navigateur Rust".to_string()
            } else {
                format!("{titre} — Navigateur Rust")
            });
        }
    });

    fenetre.show_all();
}

// Ce qui est tapé dans la barre d'adresse : une URL telle quelle, une adresse simple (un point,
// pas d'espace) complétée en https://, ou sinon une recherche DuckDuckGo.
fn normaliser(saisie: &str) -> String {
    let s = saisie.trim();
    if s.is_empty() {
        return "about:blank".to_string();
    }
    if s.starts_with("http://") || s.starts_with("https://") || s.starts_with("file://") {
        return s.to_string();
    }
    if !s.contains(' ') && s.contains('.') {
        return format!("https://{s}");
    }
    format!("https://duckduckgo.com/?q={}", encoder_url(s))
}

fn encoder_url(s: &str) -> String {
    let mut sortie = String::new();
    for o in s.bytes() {
        match o {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => sortie.push(o as char),
            b' ' => sortie.push('+'),
            _ => sortie.push_str(&format!("%{o:02X}")),
        }
    }
    sortie
}
