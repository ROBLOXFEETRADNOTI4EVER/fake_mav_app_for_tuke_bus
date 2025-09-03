use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TUKE_PNG: Asset = asset!("/assets/tuke.png");
const BARCODE_PNG: Asset = asset!("/assets/barcode.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        div { class: "ticket-container",
            BusTicket {}
        }
    }
}

#[component]
pub fn BusTicket() -> Element {
    rsx! {
        div { class: "bus-ticket",
            
            // Info box positioned at the top of barcode
            div { class: "info-box", onclick: |_| println!("Info box clicked!"),
                div { class: "info-arrow", "←" }
                div { class: "info-text", "Jegyinformáció" }
                div { class: "info-dots", "⋯" }
            }
            
            div { class: "ticket-header",
                img { src: BARCODE_PNG, class: "barcode-image", alt: "Barcode" }
            }
            
            div { class: "ticket-body",
                
                div { class: "name-section",
                    p { class: "name-text", "Szabo BALAZS (2008.04.30.)" }
                }
                
                div { class: "id-section",
                    p { class: "id-text", "Ig. szám 024787SE" }
                }

                div { class: "title-section-special",
                    div { class: "title-inner-blue",
                        p { class: "title-text-blue", "Pécs havi tanuló bérlet           ‎ ‎     / ‎ ‎ ‎    monthly" }
                        p { class: "title-text-blue", "pass for students Pécs" }
                    }
                }
                
                div { class: "monthly-section-no-border",
                    p { class: "monthly-text", "Havi bérlet" }
                }
                
              div { class: "validity-section-no-border",
    div { class: "validity-line",
        span { class: "validity-label", "Érvényes: " }
        span { class: "validity-date-inline date-value", "2025.09.01." }
    }
    div { class: "validity-dates-centered",
        p { class: "date-value", "2025.11.05." }
    }
}

                
                div { class: "terms-section",
                    p { "Érvényes a feltűntetett hónap első napján 0 órától az azt követő hónap 2.-én 12 óráig. A bérlet csak érvényes diákigazolvánnyal együtt használható. A bérletet (ha szükséges Tüke kártyát is) ellenőrzéskor fel kell mutatni és az ellenőrzést végző személy kérésére át kell adni. A bérlet az érvényesség megkezdését követően nem visszaváltható. Further details: tukebusz.hu Please show, and hand over your pass if requested by the inspector. For more information about this ticket or pass please visit Tuke Busz's website: www.tukebusz.hu" }
                }
                
                div { class : "price_thingy_continaer",
                div { class: "price-container",
                    div { class: "price-box",
                        p { class: "price-label", "*1 db" }
                        p { class: "price-amount", "3900 Ft" }
                    }
                    p { class: "tax-text-right", "Az ár 21,26% áfát tartalmaz." }
                }
            }
                br {}
                
                div { class: "civ-container",
                    p { class: "civ-text", "CIV 5594003Q800235142" }
                    p { class: "timestamp-text", "2025.09.01. 11:28" }
                }
                    
                div { class: "logo-container",
                    img { src: TUKE_PNG, class: "tuke-logo-big", alt: "Tüke" }
                }
                
                div { class: "reference-container",
                    p { class: "reference-text-right", "250101/TSZ" }
                }
            }
        }
    }
}
