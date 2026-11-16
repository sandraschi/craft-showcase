#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

mod backoffice;
mod shopcraft;

use eframe::egui::{self, Color32, RichText};
use wasm_bindgen::prelude::*;

use crate::shopcraft::{RentalUnit, ShopOrder, ShopProduct, SubPlan, TicketEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveApp {
    // Back-Office Suite (14)
    Helm,
    Bill,
    Buch,
    Spend,
    Hire,
    Task,
    Desk,
    Contract,
    Gain,
    Team,
    Asset,
    Form,
    Wiki,
    Stock,
    // Shop Craft Storefronts (8)
    WebShop,
    Shop,
    AmazonShop,
    Wholesale,
    DigitalShop,
    SubShop,
    RentalShop,
    TicketShop,
}

impl ActiveApp {
    pub const BACKOFFICE: [Self; 14] = [
        Self::Helm,
        Self::Bill,
        Self::Buch,
        Self::Spend,
        Self::Hire,
        Self::Task,
        Self::Desk,
        Self::Contract,
        Self::Gain,
        Self::Team,
        Self::Asset,
        Self::Form,
        Self::Wiki,
        Self::Stock,
    ];

    pub const SHOPCRAFT: [Self; 8] = [
        Self::WebShop,
        Self::Shop,
        Self::AmazonShop,
        Self::Wholesale,
        Self::DigitalShop,
        Self::SubShop,
        Self::RentalShop,
        Self::TicketShop,
    ];

    pub fn short_id(self) -> &'static str {
        match self {
            Self::Helm => "helmcraft",
            Self::Bill => "billcraft",
            Self::Buch => "buchcraft",
            Self::Spend => "spendcraft",
            Self::Hire => "hirecraft",
            Self::Task => "taskcraft",
            Self::Desk => "deskcraft",
            Self::Contract => "contractcraft",
            Self::Gain => "gaincraft",
            Self::Team => "teamcraft",
            Self::Asset => "assetcraft",
            Self::Form => "formcraft",
            Self::Wiki => "wikicraft",
            Self::Stock => "stockcraft",
            Self::WebShop => "webshopcraft",
            Self::Shop => "shopcraft",
            Self::AmazonShop => "amazonshopcraft",
            Self::Wholesale => "wholesalecraft",
            Self::DigitalShop => "digitalshopcraft",
            Self::SubShop => "subshopcraft",
            Self::RentalShop => "rentalshopcraft",
            Self::TicketShop => "ticketshopcraft",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Helm => "HelmCraft",
            Self::Bill => "BillCraft",
            Self::Buch => "BuchCraft",
            Self::Spend => "SpendCraft",
            Self::Hire => "HireCraft",
            Self::Task => "TaskCraft",
            Self::Desk => "DeskCraft",
            Self::Contract => "ContractCraft",
            Self::Gain => "GainCraft",
            Self::Team => "TeamCraft",
            Self::Asset => "AssetCraft",
            Self::Form => "FormCraft",
            Self::Wiki => "WikiCraft",
            Self::Stock => "StockCraft",
            Self::WebShop => "WebShopCraft",
            Self::Shop => "ShopCraft",
            Self::AmazonShop => "AmazonShopCraft",
            Self::Wholesale => "WholesaleCraft",
            Self::DigitalShop => "DigitalShopCraft",
            Self::SubShop => "SubShopCraft",
            Self::RentalShop => "RentalShopCraft",
            Self::TicketShop => "TicketShopCraft",
        }
    }

    pub fn subtitle(self) -> &'static str {
        match self {
            Self::Helm => "Executive Telemetry & Board Cockpit",
            Self::Bill => "ebInterface 6.1 & SEPA Invoicing",
            Self::Buch => "Austrian Double-Entry Bookkeeping (EKR & U30)",
            Self::Spend => "§26 EStG Travel & Expense Claims",
            Self::Hire => "Blind ATS & Onboarding Cascade",
            Self::Task => "Local-First Kanban & Issue Bus",
            Self::Desk => "Maildir Support Desk & SLA Clocks",
            Self::Contract => "Ed25519 Contract Sealing & Notice Periods",
            Self::Gain => "Customer 360 & Health Triage",
            Self::Team => "AZG 38.5h Time & Attendance",
            Self::Asset => "§7 EStG AfA & Hardware Registry",
            Self::Form => "NPS Surveys & Detractor Routing",
            Self::Wiki => "FTS5 Docs, Backlinks & ADRs",
            Self::Stock => "Inventory, Moving-Average Cost & Low-Stock",
            Self::WebShop => "D2C Retail Storefront & Checkout",
            Self::Shop => "Handmade & Artisan Storefront",
            Self::AmazonShop => "Multi-Vendor Marketplace",
            Self::Wholesale => "B2B Tiered Pricing, MOQ & Net-30 RFQs",
            Self::DigitalShop => "Downloads, License Keys & Courses",
            Self::SubShop => "Subscription Boxes & SEPA pain.008",
            Self::RentalShop => "Equipment, Bike & Studio Gear Rental",
            Self::TicketShop => "Event Ticketing, Workshops & Box Office",
        }
    }

    pub fn nav_items(self) -> &'static [&'static str] {
        match self {
            Self::Helm => &["Executive Radar", "Suite Health"],
            Self::Bill => &["Overview & New", "Invoices", "ebInterface XML"],
            Self::Buch => &["Journal & Storno", "U30 Kennzahlen", "Bank CSV Inbox"],
            Self::Task => &["Kanban Board", "Issue List"],
            Self::Stock => &["All Inventory", "Low-Stock Alerts"],
            Self::WebShop | Self::Shop | Self::AmazonShop => {
                &["Overview & Quick Order", "Catalog", "Orders & Fulfillment"]
            }
            Self::Wholesale => &["New Net-30 RFQ", "RFQ Queue"],
            _ => &["Workspace"],
        }
    }

    pub fn mcp_tools(self) -> &'static [&'static str] {
        match self {
            Self::Helm => &["helm_status", "helm_briefing"],
            Self::Bill => &["bill_status", "bill_list", "bill_export_sepa"],
            Self::Buch => &["buch_status", "buch_u30", "buch_journal"],
            Self::Task => &["task_status", "task_board"],
            Self::Stock => &["stock_status", "stock_low"],
            Self::WebShop | Self::Shop | Self::AmazonShop => {
                &["shop_status", "shop_catalog", "shop_orders"]
            }
            Self::Wholesale => &["wholesale_status", "wholesale_rfqs"],
            Self::DigitalShop => &["digital_status", "digital_licenses"],
            Self::SubShop => &["sub_status", "sub_mrr"],
            Self::RentalShop => &["rental_status", "rental_fleet"],
            Self::TicketShop => &["ticket_status", "ticket_events"],
            _ => &["app_status", "app_seed"],
        }
    }

    pub fn demo_url(self) -> Option<&'static str> {
        match self {
            Self::WebShop => Some("../demos/webshopcraft/index.html"),
            Self::Wholesale => Some("../demos/wholesalecraft/index.html"),
            Self::DigitalShop => Some("../demos/digitalshopcraft/index.html"),
            Self::SubShop => Some("../demos/subshopcraft/index.html"),
            Self::RentalShop => Some("../demos/rentalshopcraft/index.html"),
            Self::TicketShop => Some("../demos/ticketshopcraft/index.html"),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Invoice {
    pub id: String,
    pub customer: String,
    pub amount_cents: i64,
    pub status: String,
    pub date: String,
}

#[derive(Clone)]
pub struct LedgerRow {
    pub id: String,
    pub date: String,
    pub account: String,
    pub text: String,
    pub net_cents: i64,
    pub vat_key: String,
    pub stornoed: bool,
}

#[derive(Clone)]
pub struct Candidate {
    pub id: String,
    pub name: String,
    pub role: String,
    pub stage: String,
    pub score: i32,
}

#[derive(Clone)]
pub struct TaskIssue {
    pub id: String,
    pub title: String,
    pub column: String,
    pub assignee: String,
}

#[derive(Clone)]
pub struct SupportTicket {
    pub id: String,
    pub subject: String,
    pub customer: String,
    pub sla_mins: i32,
    pub status: String,
}

#[derive(Clone)]
pub struct ContractDoc {
    pub id: String,
    pub title: String,
    pub counterparty: String,
    pub notice_days: i32,
    pub sealed: bool,
}

#[derive(Clone)]
pub struct AssetItem {
    pub tag: String,
    pub name: String,
    pub assignee: String,
    pub cost_eur: f64,
    pub afa_years: u32,
}

#[derive(Clone)]
pub struct WikiDoc {
    pub slug: String,
    pub title: String,
    pub kind: String,
    pub body: String,
}

#[derive(Clone)]
pub struct StockRow {
    pub sku: String,
    pub name: String,
    pub location: String,
    pub qty: i64,
    pub min_stock: i64,
    pub unit_cost_cents: i64,
}

pub struct ShowcaseApp {
    pub app: ActiveApp,
    pub app_view: usize,
    pub lang_de: bool,
    pub dark_mode: bool,
    pub show_app_nav: bool,
    pub show_bus_drawer: bool,
    pub show_app_launcher: bool,
    pub show_about: bool,
    pub mcp_output_modal: Option<(String, String)>,
    pub launcher_filter: String,
    pub bus_log: Vec<String>,
    pub toast: Option<String>,

    // Helm
    pub cash_millions: f64,
    pub runway_months: f64,
    pub briefing_generated: bool,

    // Stock
    pub stock_items: Vec<StockRow>,

    // Bill
    pub invoices: Vec<Invoice>,
    pub selected_invoice: usize,
    pub new_invoice_customer: String,
    pub new_invoice_eur: f64,

    // Buch
    pub ledger: Vec<LedgerRow>,
    pub bank_inbox_booked: bool,
    pub month_closed: bool,

    // Spend
    pub trip_km: f64,
    pub trip_hours: f64,
    pub provided_lunch: bool,
    pub provided_dinner: bool,

    // Hire
    pub candidates: Vec<Candidate>,

    // Task
    pub issues: Vec<TaskIssue>,
    pub new_task_title: String,

    // Desk
    pub tickets: Vec<SupportTicket>,
    pub selected_ticket: usize,

    // Contract
    pub contracts: Vec<ContractDoc>,

    // Gain
    pub gain_filter: String,

    // Team
    pub daily_hours: f64,
    pub weekly_hours: f64,

    // Asset
    pub assets: Vec<AssetItem>,

    // Form
    pub nps_score: i32,
    pub nps_comment: String,

    // Wiki
    pub wiki_query: String,
    pub wiki_docs: Vec<WikiDoc>,
    pub selected_wiki: usize,

    // Shop Craft state
    pub shop_products: Vec<ShopProduct>,
    pub shop_orders: Vec<ShopOrder>,
    pub shop_customer_input: String,
    pub wholesale_qty: i64,
    pub digital_buyer_email: String,
    pub digital_keys: Vec<(String, String, String)>,
    pub sub_plans: Vec<SubPlan>,
    pub rental_units: Vec<RentalUnit>,
    pub ticket_events: Vec<TicketEvent>,
}

impl Default for ShowcaseApp {
    fn default() -> Self {
        Self {
            app: ActiveApp::Bill,
            app_view: 0,
            lang_de: false,
            dark_mode: true,
            show_app_nav: true,
            show_bus_drawer: false,
            show_app_launcher: false,
            show_about: false,
            mcp_output_modal: None,
            launcher_filter: String::new(),
            bus_log: vec![
                "billcraft -> invoice.issued (INV-2026-002, €8,500.00)".into(),
                "assetcraft -> asset.depreciation_due (AST-014 -> buchcraft)".into(),
            ],
            toast: None,
            cash_millions: 1.42,
            runway_months: 20.8,
            briefing_generated: false,
            stock_items: vec![
                StockRow { sku: "NVME-2TB-PRO".into(), name: "NVMe PCIe 4.0 2TB SSD".into(), location: "A-01-02".into(), qty: 5, min_stock: 6, unit_cost_cents: 13_900 },
                StockRow { sku: "RAM-DDR5-32".into(), name: "DDR5-5600 32GB ECC".into(), location: "A-01-03".into(), qty: 14, min_stock: 8, unit_cost_cents: 9_800 },
                StockRow { sku: "CAT6A-3M".into(), name: "Cat6A Patch Cable 3m".into(), location: "C-01-04".into(), qty: 18, min_stock: 20, unit_cost_cents: 650 },
                StockRow { sku: "PASTE-01".into(), name: "Solder Paste (MHD 2026-11)".into(), location: "C-02-01".into(), qty: 9, min_stock: 4, unit_cost_cents: 2_000 },
            ],
            invoices: vec![
                Invoice { id: "INV-2026-001".into(), customer: "Alpine Tech GmbH".into(), amount_cents: 1_440_000, status: "Paid".into(), date: "2026-11-01".into() },
                Invoice { id: "INV-2026-002".into(), customer: "Wiener Software AG".into(), amount_cents: 850_000, status: "Issued".into(), date: "2026-11-04".into() },
                Invoice { id: "INV-2026-003".into(), customer: "Grazer Cloud KG".into(), amount_cents: 1_200_000, status: "Overdue".into(), date: "2026-10-15".into() },
            ],
            selected_invoice: 1,
            new_invoice_customer: "Donau Handel OG".into(),
            new_invoice_eur: 2400.0,
            ledger: vec![
                LedgerRow { id: "BK-101".into(), date: "2026-11-02".into(), account: "7380 Telefon & Internet".into(), text: "A1 Telekom Business".into(), net_cents: 8_900, vat_key: "V20".into(), stornoed: false },
                LedgerRow { id: "BK-102".into(), date: "2026-11-04".into(), account: "4000 Erlöse 20%".into(), text: "Wiener Software AG".into(), net_cents: 850_000, vat_key: "U20".into(), stornoed: false },
                LedgerRow { id: "BK-103".into(), date: "2026-11-08".into(), account: "7010 AfA Sachanlagen".into(), text: "Workstation Depreciation".into(), net_cents: 7_500, vat_key: "NONE".into(), stornoed: false },
            ],
            bank_inbox_booked: false,
            month_closed: false,
            trip_km: 120.0,
            trip_hours: 8.5,
            provided_lunch: true,
            provided_dinner: false,
            candidates: vec![
                Candidate { id: "C-101".into(), name: "Florian Bauer".into(), role: "Staff SRE".into(), stage: "offer".into(), score: 5 },
                Candidate { id: "C-102".into(), name: "Johanna Gruber".into(), role: "Rust Systems Engineer".into(), stage: "interview".into(), score: 4 },
                Candidate { id: "C-103".into(), name: "Lukas Weber".into(), role: "Frontend UI Specialist".into(), stage: "screening".into(), score: 4 },
            ],
            issues: vec![
                TaskIssue { id: "ENG-101".into(), title: "Audit ISO 20022 pain.008 batch".into(), column: "In Progress".into(), assignee: "Sandra".into() },
                TaskIssue { id: "ENG-102".into(), title: "Austrian §26 EStG Diäten calc".into(), column: "Done".into(), assignee: "Sandra".into() },
                TaskIssue { id: "ENG-103".into(), title: "Blind scorecard rollup".into(), column: "Todo".into(), assignee: "Agent".into() },
                TaskIssue { id: "ENG-104".into(), title: "WASM full-shell navigation".into(), column: "Review".into(), assignee: "Sandra".into() },
            ],
            new_task_title: String::new(),
            tickets: vec![
                SupportTicket { id: "TK-401".into(), subject: "ebInterface 6.1 XSD check on invoice".into(), customer: "Alpine Tech GmbH".into(), sla_mins: 45, status: "Open".into() },
                SupportTicket { id: "TK-402".into(), subject: "George CSV header mapping".into(), customer: "Donau Handel OG".into(), sla_mins: 180, status: "Open".into() },
            ],
            selected_ticket: 0,
            contracts: vec![
                ContractDoc { id: "CTR-2026-09".into(), title: "Enterprise Support MSA".into(), counterparty: "Alpine Tech GmbH".into(), notice_days: 90, sealed: true },
                ContractDoc { id: "CTR-2026-14".into(), title: "Cloud Exit Migration SOW".into(), counterparty: "Wiener Software AG".into(), notice_days: 30, sealed: false },
            ],
            gain_filter: "All".into(),
            daily_hours: 8.0,
            weekly_hours: 38.5,
            assets: vec![
                AssetItem { tag: "AST-014".into(), name: "ThinkPad P16s Gen 3".into(), assignee: "Johanna Gruber".into(), cost_eur: 2400.0, afa_years: 3 },
                AssetItem { tag: "AST-015".into(), name: "RTX 4090 Local Node".into(), assignee: "Lab / Goliath".into(), cost_eur: 1950.0, afa_years: 3 },
            ],
            nps_score: 5,
            nps_comment: "Invoice export stalled on custom VAT ID".into(),
            wiki_query: String::new(),
            wiki_docs: vec![
                WikiDoc { slug: "adr-001".into(), title: "ADR-001: Single-File SQLite WAL per App".into(), kind: "ADR • Accepted".into(), body: "Each Back-Office app owns its own SQLite WAL database. Cross-app sync uses atomic JSON events on the craft-biz bus.".into() },
                WikiDoc { slug: "ekr-guide".into(), title: "Austrian EKR & U30 Mapping Guide".into(), kind: "Handbook".into(), body: "Book NET amounts with VAT keys (U20, U10, V20, IGE20). BuchCraft computes tax lines and U30 Kennzahlen automatically.".into() },
                WikiDoc { slug: "airgap".into(), title: "Offline & Airgapped Mode Spec".into(), kind: "Security".into(), body: "Local means on your machine. Offline works without Wi-Fi. Airgapped enforces zero outbound sockets.".into() },
            ],
            selected_wiki: 0,
            shop_products: vec![
                ShopProduct { sku: "CER-VASE-01".into(), name: "Stoneware Ceramic Vase".into(), category: "Ceramics".into(), price_cents: 6_800, stock: 12 },
                ShopProduct { sku: "LGT-LAMP-02".into(), name: "Matte Brass Desk Lamp".into(), category: "Lighting".into(), price_cents: 14_500, stock: 7 },
                ShopProduct { sku: "TXT-THRW-03".into(), name: "Merino Wool Throw Blanket".into(), category: "Textiles".into(), price_cents: 9_400, stock: 15 },
            ],
            shop_orders: vec![
                ShopOrder { id: "ORD-101".into(), app: "webshopcraft", customer: "Elena Rostova".into(), item: "Stoneware Ceramic Vase".into(), total_cents: 6_800, status: "Paid".into() },
                ShopOrder { id: "ORD-102".into(), app: "shopcraft", customer: "Lukas Meyer".into(), item: "Matte Brass Desk Lamp".into(), total_cents: 14_500, status: "Shipped".into() },
            ],
            shop_customer_input: "Clara Lindqvist".into(),
            wholesale_qty: 50,
            digital_buyer_email: "dev@alpinestudio.at".into(),
            digital_keys: vec![
                ("LIC-RUST-2026-0001".into(), "dev@alpinestudio.at".into(), "Active".into()),
            ],
            sub_plans: vec![
                SubPlan { id: "SUB-001".into(), customer: "Cafe Neubau".into(), plan: "Monthly Espresso Box (2kg)".into(), mrr_cents: 4_800, status: "Active".into() },
                SubPlan { id: "SUB-002".into(), customer: "Studio Lehm".into(), plan: "Quarterly Clay & Glaze Kit".into(), mrr_cents: 8_900, status: "Active".into() },
            ],
            rental_units: vec![
                RentalUnit { sku: "BIKE-GRAVEL-01".into(), name: "Carbon Gravel Bike (56cm)".into(), daily_cents: 4_500, deposit_cents: 25_000, booked_days: 0, status: "Available".into() },
                RentalUnit { sku: "CAM-FX3-01".into(), name: "Sony FX3 Cinema Body + 24-70mm".into(), daily_cents: 11_000, deposit_cents: 50_000, booked_days: 3, status: "Checked Out".into() },
                RentalUnit { sku: "AUDIO-ZOOM-F6".into(), name: "Zoom F6 Field Recorder + Shotgun".into(), daily_cents: 3_500, deposit_cents: 15_000, booked_days: 0, status: "Available".into() },
            ],
            ticket_events: vec![
                TicketEvent { code: "EVT-RUST-WS".into(), title: "Pure-Rust Systems Workshop".into(), date: "2026-12-05 10:00".into(), price_cents: 89_00, capacity: 24, sold: 14 },
                TicketEvent { code: "EVT-JAZZ-GA".into(), title: "Winter Courtyard Jazz Quartet".into(), date: "2026-12-11 20:00".into(), price_cents: 28_00, capacity: 120, sold: 82 },
                TicketEvent { code: "EVT-CERAMIC".into(), title: "Wheel-Throwing Pottery Evening".into(), date: "2026-12-14 18:30".into(), price_cents: 54_00, capacity: 12, sold: 9 },
            ],
        }
    }
}

impl ShowcaseApp {
    fn switch_app(&mut self, next: ActiveApp) {
        self.app = next;
        self.app_view = 0;
        self.show_app_launcher = false;
        self.toast = Some(format!("Launched {}", next.label()));
    }

    fn run_mcp_tool(&self, tool: &str) -> String {
        serde_json::to_string_pretty(&serde_json::json!({
            "server": self.app.short_id(),
            "tool": tool,
            "ok": true,
            "summary": {
                "invoices": self.invoices.len(),
                "ledger_rows": self.ledger.len(),
                "tasks": self.issues.len(),
                "stock_skus": self.stock_items.len(),
                "shop_orders": self.shop_orders.len(),
                "bus_events": self.bus_log.len()
            }
        }))
        .unwrap_or_else(|_| "{}".into())
    }
}

impl eframe::App for ShowcaseApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        ctx.set_visuals(if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

        let (m_file, m_edit, m_view, m_tools, m_help) = if self.lang_de {
            ("Datei", "Bearbeiten", "Ansicht", "Werkzeuge", "Hilfe")
        } else {
            ("File", "Edit", "View", "Tools", "Help")
        };

        // 1. PER-APP MENU BAR (matches real craft-biz-ui desktop app chrome)
        egui::Panel::top("app_menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.label(
                    RichText::new(self.app.label())
                        .strong()
                        .color(Color32::from_rgb(56, 189, 248)),
                );
                ui.separator();

                ui.menu_button(m_file, |ui| {
                    if ui.button("Switch App (App Launcher)...").clicked() {
                        self.show_app_launcher = true;
                        ui.close();
                    }
                    if ui.button("Export JSON Snapshot").clicked() {
                        let json = self.run_mcp_tool("export_snapshot");
                        self.mcp_output_modal =
                            Some((format!("{} — Snapshot", self.app.label()), json));
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Reset Demo Data").clicked() {
                        let cur = self.app;
                        let dark = self.dark_mode;
                        let de = self.lang_de;
                        *self = Self {
                            app: cur,
                            dark_mode: dark,
                            lang_de: de,
                            ..Self::default()
                        };
                        self.toast = Some("Demo state reset".into());
                        ui.close();
                    }
                });

                ui.menu_button(m_edit, |ui| {
                    let theme_lbl = if self.dark_mode {
                        "Switch to Light Theme"
                    } else {
                        "Switch to Dark Theme"
                    };
                    if ui.button(theme_lbl).clicked() {
                        self.dark_mode = !self.dark_mode;
                        ui.close();
                    }
                    let lang_lbl = if self.lang_de {
                        "Language: English (EN)"
                    } else {
                        "Sprache: Deutsch (DE)"
                    };
                    if ui.button(lang_lbl).clicked() {
                        self.lang_de = !self.lang_de;
                        ui.close();
                    }
                });

                ui.menu_button(m_view, |ui| {
                    ui.checkbox(&mut self.show_app_nav, "Show App Navigation Rail");
                    ui.checkbox(&mut self.show_bus_drawer, "Show Domino Bus Drawer");
                    ui.separator();
                    for (idx, nav_lbl) in self.app.nav_items().iter().enumerate() {
                        if ui
                            .selectable_label(self.app_view == idx, *nav_lbl)
                            .clicked()
                        {
                            self.app_view = idx;
                            ui.close();
                        }
                    }
                });

                ui.menu_button(m_tools, |ui| {
                    for tool in self.app.mcp_tools() {
                        if ui.button(*tool).clicked() {
                            let out = self.run_mcp_tool(tool);
                            self.mcp_output_modal =
                                Some((format!("MCP: {tool}"), out));
                            self.bus_log
                                .push(format!("{} -> mcp.call ({tool})", self.app.short_id()));
                            ui.close();
                        }
                    }
                });

                ui.menu_button("Bus", |ui| {
                    if ui.button("Toggle Live Event Log").clicked() {
                        self.show_bus_drawer = !self.show_bus_drawer;
                        ui.close();
                    }
                    if ui.button("Clear Bus Log").clicked() {
                        self.bus_log.clear();
                        ui.close();
                    }
                });

                ui.menu_button(m_help, |ui| {
                    if ui
                        .button(format!("About {}", self.app.label()))
                        .clicked()
                    {
                        self.show_about = true;
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Apps (22)").clicked() {
                        self.show_app_launcher = !self.show_app_launcher;
                    }
                    if let Some(url) = self.app.demo_url() {
                        if ui.button("Open Demo Webshop").clicked() {
                            ctx.open_url(egui::OpenUrl::new_tab(url));
                        }
                    }
                    if ui.small_button(if self.lang_de { "DE" } else { "EN" }).clicked() {
                        self.lang_de = !self.lang_de;
                    }
                    if ui
                        .small_button(if self.dark_mode { "Light" } else { "Dark" })
                        .clicked()
                    {
                        self.dark_mode = !self.dark_mode;
                    }
                });
            });
        });

        // 2. BOTTOM STATUS BAR
        egui::Panel::bottom("status_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(self.app.label()).strong().size(11.0));
                ui.separator();
                ui.label(
                    RichText::new(self.app.subtitle())
                        .size(11.0)
                        .color(Color32::GRAY),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .small_button(format!("Bus: {} events", self.bus_log.len()))
                        .clicked()
                    {
                        self.show_bus_drawer = !self.show_bus_drawer;
                    }
                    ui.label(
                        RichText::new(format!("MCP: {} tools", self.app.mcp_tools().len()))
                            .size(11.0)
                            .color(Color32::GRAY),
                    );
                    if let Some(msg) = &self.toast {
                        ui.label(
                            RichText::new(format!("• {msg}"))
                                .size(11.0)
                                .color(Color32::from_rgb(52, 211, 153)),
                        );
                    }
                });
            });
        });

        // 3. OPTIONAL DOMINO BUS DRAWER
        if self.show_bus_drawer {
            egui::Panel::bottom("bus_drawer")
                .resizable(true)
                .default_size(110.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.strong("Atomic Event Bus (craft-biz)");
                        if ui.small_button("Close").clicked() {
                            self.show_bus_drawer = false;
                        }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for line in self.bus_log.iter().rev() {
                            ui.label(RichText::new(format!("• {line}")).monospace().size(11.0));
                        }
                    });
                });
        }

        // 4. PER-APP LEFT NAVIGATION RAIL (THIS APP'S OWN VIEWS, NOT A GLOBAL APP LIST)
        if self.show_app_nav {
            egui::Panel::left("app_nav_rail")
                .default_size(175.0)
                .resizable(true)
                .show(ui, |ui| {
                    ui.add_space(6.0);
                    ui.label(RichText::new(self.app.label()).strong().size(14.0));
                    ui.label(
                        RichText::new(self.app.subtitle())
                            .size(10.5)
                            .color(Color32::GRAY),
                    );
                    ui.separator();
                    for (idx, nav_lbl) in self.app.nav_items().iter().enumerate() {
                        if ui
                            .selectable_label(self.app_view == idx, *nav_lbl)
                            .clicked()
                        {
                            self.app_view = idx;
                        }
                    }
                    ui.add_space(16.0);
                    ui.separator();
                    if let Some(url) = self.app.demo_url() {
                        if ui.button("Open Demo Webshop").clicked() {
                            ctx.open_url(egui::OpenUrl::new_tab(url));
                        }
                    }
                    if ui.button("Switch App (22)...").clicked() {
                        self.show_app_launcher = true;
                    }
                });
        }

        // 5. APP LAUNCHER MODAL (22 APPS: 14 BACK-OFFICE + 8 SHOP CRAFT)
        if self.show_app_launcher {
            let mut open = self.show_app_launcher;
            let mut next_app: Option<ActiveApp> = None;
            egui::Window::new("Craft Suite App Launcher (22 Apps)")
                .open(&mut open)
                .default_width(560.0)
                .resizable(true)
                .show(&ctx, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.launcher_filter)
                            .hint_text("Search Back-Office & Shop Craft apps..."),
                    );
                    ui.separator();
                    let q = self.launcher_filter.trim().to_lowercase();

                    ui.strong("Back-Office Suite (14)");
                    ui.horizontal_wrapped(|ui| {
                        for candidate in ActiveApp::BACKOFFICE {
                            if !q.is_empty()
                                && !candidate.label().to_lowercase().contains(&q)
                                && !candidate.subtitle().to_lowercase().contains(&q)
                            {
                                continue;
                            }
                            if ui
                                .selectable_label(self.app == candidate, candidate.label())
                                .clicked()
                            {
                                next_app = Some(candidate);
                            }
                        }
                    });
                    ui.add_space(8.0);
                    ui.strong("Shop Craft Storefronts (8)");
                    ui.horizontal_wrapped(|ui| {
                        for candidate in ActiveApp::SHOPCRAFT {
                            if !q.is_empty()
                                && !candidate.label().to_lowercase().contains(&q)
                                && !candidate.subtitle().to_lowercase().contains(&q)
                            {
                                continue;
                            }
                            if ui
                                .selectable_label(self.app == candidate, candidate.label())
                                .clicked()
                            {
                                next_app = Some(candidate);
                            }
                        }
                    });
                });
            self.show_app_launcher = open;
            if let Some(next) = next_app {
                self.switch_app(next);
            }
        }

        // 6. MCP TOOL OUTPUT / SNAPSHOT MODAL
        if let Some((title, body)) = self.mcp_output_modal.clone() {
            let mut open = true;
            egui::Window::new(title)
                .open(&mut open)
                .default_width(440.0)
                .show(&ctx, |ui| {
                    egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
                        ui.label(RichText::new(&body).monospace().size(11.0));
                    });
                });
            if !open {
                self.mcp_output_modal = None;
            }
        }

        // 7. ABOUT MODAL
        if self.show_about {
            let mut open = self.show_about;
            egui::Window::new(format!("About {}", self.app.label()))
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(&ctx, |ui| {
                    ui.label(format!("{} — {}", self.app.label(), self.app.subtitle()));
                    ui.label("Part of the 22-app Pure-Rust Local Craft Suite (Back-Office + Shop Craft).");
                    ui.label("Full app chrome: Menu Bar (File/Edit/View/Tools/Bus/Help), Per-App Nav, MCP Tools & Domino Bus.");
                });
            self.show_about = open;
        }

        // 8. MAIN APP WORKSPACE
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.app {
                    ActiveApp::WebShop
                    | ActiveApp::Shop
                    | ActiveApp::AmazonShop
                    | ActiveApp::Wholesale
                    | ActiveApp::DigitalShop
                    | ActiveApp::SubShop
                    | ActiveApp::RentalShop
                    | ActiveApp::TicketShop => self.render_shop_app(ui, self.app),
                    other => self.render_backoffice_app(ui, other),
                }
            });
        });
    }
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document found"))?;
    let canvas = document
        .get_element_by_id("craft_canvas")
        .ok_or_else(|| JsValue::from_str("Canvas element 'craft_canvas' not found"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Failed to cast element to HtmlCanvasElement"))?;

    let runner = eframe::WebRunner::new();
    let web_options = eframe::WebOptions::default();

    runner
        .start(
            canvas,
            web_options,
            Box::new(|_cc| Ok(Box::new(ShowcaseApp::default()))),
        )
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to start eframe runner: {e:?}")))?;

    Ok(())
}
