//! The language selector in the top bar: Spanish is the fallback language in WordNext builds,
//! and the bar's selector changes the interface language at once.

use egui::accesskit::Role;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_ui_egui::PdfCraftApp;

fn harness(language: &'static str) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", language).unwrap();
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn the_top_bar_selector_switches_the_interface_language() {
    let mut h = harness("en");
    // The selector shows the language in use.
    h.get_by(|n| n.role() == Role::ComboBox && n.value().is_some_and(|v| v == "English")).click();
    h.run_steps(3);
    h.get_by_label("Español").scroll_to_me();
    h.run_steps(3);
    h.get_by_label("Español").click();
    h.run_steps(4);
    assert_eq!(h.state().language, "es");
    // The bar is now drawn in Spanish.
    assert!(h.query_by_label("Abrir").is_some(), "the Open button reads «Abrir» in Spanish");
}

#[test]
fn auto_is_still_an_option() {
    let mut h = harness("es");
    h.get_by(|n| n.role() == Role::ComboBox && n.value().is_some_and(|v| v == "Español")).click();
    h.run_steps(3);
    h.get_by_label("Automático").click();
    h.run_steps(4);
    assert_eq!(h.state().language, "auto");
}
