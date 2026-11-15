#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use eframe::egui::{self, Color32, RichText};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveApp {
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
}

impl ActiveApp {
    const ALL: [Self; 13] = [
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
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Helm => "🧭 HelmCraft",
            Self::Bill => "🧾 BillCraft",
            Self::Buch => "📒 BuchCraft",
            Self::Spend => "💳 SpendCraft",
            Self::Hire => "🎯 HireCraft",
            Self::Task => "📋 TaskCraft",
            Self::Desk => "🎧 DeskCraft",
            Self::Contract => "📜 ContractCraft",
            Self::Gain => "📊 GainCraft",
            Self::Team => "⏱️ TeamCraft",
            Self::Asset => "💻 AssetCraft",
            Self::Form => "📝 FormCraft",
            Self::Wiki => "📚 WikiCraft",
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            Self::Helm => "Executive Telemetry & Board Cockpit",
            Self::Bill => "ebInterface 6.1 & SEPA Invoicing",
            Self::Buch => "Austrian Double-Entry Bookkeeping (EKR & U30)",
            Self::Spend => "§26 EStG Travel & Expense Claims",
            Self::Hire => "Blind ATS & Onboarding Cascade",
            Self::Task => "Local-First Kanban & Issue Bus",
            Self::Desk => "Maildir Support Desk & SLA Clocks",
            Self::Contract => "Ed25519 Contract Sealing & ABGB",
            Self::Gain => "Customer 360 & Health Triage",
            Self::Team => "AZG 38.5h Time & Attendance",
            Self::Asset => "§7 EStG AfA & Hardware Registry",
            Self::Form => "NPS Surveys & Detractor Routing",
            Self::Wiki => "FTS5 Docs, Backlinks & ADRs",
        }
    }
}

#[derive(Clone)]
struct Invoice {
    id: String,
    customer: String,
    amount_cents: i64,
    status: String,
    date: String,
}

#[derive(Clone)]
struct LedgerRow {
    id: String,
    date: String,
    account: String,
    text: String,
    net_cents: i64,
    vat_key: String,
    stornoed: bool,
}

#[derive(Clone)]
struct Candidate {
    id: String,
    name: String,
    role: String,
    stage: String,
    score: i32,
}

#[derive(Clone)]
struct TaskIssue {
    id: String,
    title: String,
    column: String,
    assignee: String,
}

#[derive(Clone)]
struct SupportTicket {
    id: String,
    subject: String,
    customer: String,
    sla_mins: i32,
    status: String,
}

#[derive(Clone)]
struct ContractDoc {
    id: String,
    title: String,
    counterparty: String,
    notice_days: i32,
    sealed: bool,
}

#[derive(Clone)]
struct AssetItem {
    tag: String,
    name: String,
    assignee: String,
    cost_eur: f64,
    afa_years: u32,
}

#[derive(Clone)]
struct WikiDoc {
    slug: String,
    title: String,
    kind: String,
    body: String,
}

pub struct ShowcaseApp {
    app: ActiveApp,
    dark_mode: bool,
    show_sidebar: bool,
    show_bus_drawer: bool,
    show_about: bool,
    nav_filter: String,
    bus_log: Vec<String>,
    toast: Option<String>,

    // Helm
    cash_millions: f64,
    runway_months: f64,
    briefing_generated: bool,

    // Bill
    invoices: Vec<Invoice>,
    selected_invoice: usize,
    show_xml_preview: bool,

    // Buch
    buch_tab: usize,
    ledger: Vec<LedgerRow>,
    bank_inbox_booked: bool,
    month_closed: bool,

    // Spend
    trip_km: f64,
    trip_hours: f64,
    provided_lunch: bool,
    provided_dinner: bool,

    // Hire
    candidates: Vec<Candidate>,

    // Task
    issues: Vec<TaskIssue>,
    new_task_title: String,

    // Desk
    tickets: Vec<SupportTicket>,
    selected_ticket: usize,

    // Contract
    contracts: Vec<ContractDoc>,

    // Gain
    gain_filter: String,

    // Team
    daily_hours: f64,
    weekly_hours: f64,

    // Asset
    assets: Vec<AssetItem>,

    // Form
    nps_score: i32,
    nps_comment: String,

    // Wiki
    wiki_query: String,
    wiki_docs: Vec<WikiDoc>,
    selected_wiki: usize,
}

impl Default for ShowcaseApp {
    fn default() -> Self {
        Self {
            app: ActiveApp::Helm,
            dark_mode: true,
            show_sidebar: true,
            show_bus_drawer: false,
            show_about: false,
            nav_filter: String::new(),
            bus_log: vec![
                "billcraft -> invoice.issued (INV-2026-002, €8,500.00)".into(),
                "assetcraft -> asset.depreciation_due (AST-014 -> buchcraft)".into(),
            ],
            toast: None,
            cash_millions: 1.42,
            runway_months: 20.8,
            briefing_generated: false,
            invoices: vec![
                Invoice { id: "INV-2026-001".into(), customer: "Alpine Tech GmbH".into(), amount_cents: 14_400_00, status: "Paid".into(), date: "2026-11-01".into() },
                Invoice { id: "INV-2026-002".into(), customer: "Wiener Software AG".into(), amount_cents: 8_500_00, status: "Issued".into(), date: "2026-11-04".into() },
                Invoice { id: "INV-2026-003".into(), customer: "Grazer Cloud KG".into(), amount_cents: 12_000_00, status: "Overdue".into(), date: "2026-10-15".into() },
            ],
            selected_invoice: 1,
            show_xml_preview: false,
            buch_tab: 0,
            ledger: vec![
                LedgerRow { id: "BK-101".into(), date: "2026-11-02".into(), account: "7380 Telefon & Internet".into(), text: "A1 Telekom Business".into(), net_cents: 89_00, vat_key: "V20".into(), stornoed: false },
                LedgerRow { id: "BK-102".into(), date: "2026-11-04".into(), account: "4000 Erlöse 20%".into(), text: "Wiener Software AG".into(), net_cents: 8_500_00, vat_key: "U20".into(), stornoed: false },
                LedgerRow { id: "BK-103".into(), date: "2026-11-08".into(), account: "7010 AfA Sachanlagen".into(), text: "Workstation Depreciation".into(), net_cents: 75_00, vat_key: "NONE".into(), stornoed: false },
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
        }
    }
}

impl eframe::App for ShowcaseApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        ctx.set_visuals(if self.dark_mode { egui::Visuals::dark() } else { egui::Visuals::light() });

        // 1. MENU BAR (File / Edit / View / Bus / Help)
        egui::Panel::top("menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Reset Demo Data").clicked() {
                        *self = Self { dark_mode: self.dark_mode, ..Self::default() };
                        self.toast = Some("Demo state reset".into());
                        ui.close();
                    }
                    if ui.button("Emit Snapshot Event").clicked() {
                        self.bus_log.push(format!("shell -> snapshot.exported ({})", self.app.label()));
                        self.toast = Some("Snapshot event logged to bus".into());
                        ui.close();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    let theme_lbl = if self.dark_mode { "Switch to Light Theme" } else { "Switch to Dark Theme" };
                    if ui.button(theme_lbl).clicked() {
                        self.dark_mode = !self.dark_mode;
                        ui.close();
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_sidebar, "Show App Sidebar");
                    ui.checkbox(&mut self.show_bus_drawer, "Show Event Bus Log");
                    ui.separator();
                    for candidate in ActiveApp::ALL {
                        if ui.selectable_label(self.app == candidate, candidate.label()).clicked() {
                            self.app = candidate;
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

                ui.menu_button("Help", |ui| {
                    if ui.button("About Local Craft Suite").clicked() {
                        self.show_about = true;
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("Pure Rust • egui 0.37 • 13 Apps").size(11.0).color(Color32::from_rgb(56, 189, 248)));
                });
            });
        });

        // 2. BOTTOM STATUS BAR
        egui::Panel::bottom("status_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(self.app.label()).strong().size(11.0));
                ui.separator();
                ui.label(RichText::new(self.app.subtitle()).size(11.0).color(Color32::GRAY));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button(format!("Bus: {} events", self.bus_log.len())).clicked() {
                        self.show_bus_drawer = !self.show_bus_drawer;
                    }
                    if let Some(msg) = &self.toast {
                        ui.label(RichText::new(format!("• {msg}")).size(11.0).color(Color32::from_rgb(52, 211, 153)));
                    }
                });
            });
        });

        // 3. OPTIONAL BUS DRAWER
        if self.show_bus_drawer {
            egui::Panel::bottom("bus_drawer").resizable(true).default_size(110.0).show(ui, |ui| {
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

        // 4. LEFT SIDEBAR NAVIGATION RAIL (ALL 13 APPS)
        if self.show_sidebar {
            egui::Panel::left("nav_rail").default_size(185.0).resizable(true).show(ui, |ui| {
                ui.add_space(4.0);
                ui.label(RichText::new("BACK-OFFICE SUITE (13)").size(10.0).strong().color(Color32::GRAY));
                ui.add(egui::TextEdit::singleline(&mut self.nav_filter).hint_text("Filter apps..."));
                ui.separator();
                let q = self.nav_filter.trim().to_lowercase();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for candidate in ActiveApp::ALL {
                        if !q.is_empty() && !candidate.label().to_lowercase().contains(&q) {
                            continue;
                        }
                        if ui.selectable_label(self.app == candidate, candidate.label()).clicked() {
                            self.app = candidate;
                        }
                    }
                });
            });
        }

        // 5. ABOUT MODAL
        if self.show_about {
            egui::Window::new("About Local Craft Suite").collapsible(false).resizable(false).show(&ctx, |ui| {
                ui.label("13 local-first Back-Office apps built in pure Rust over a shared craft-biz kernel & atomic event bus.");
                ui.label("Interactive WASM preview — click any sidebar app, menu item, or action button.");
                if ui.button("Close").clicked() {
                    self.show_about = false;
                }
            });
        }

        // 6. MAIN WORKSPACE
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.app {
                    ActiveApp::Helm => self.render_helm(ui),
                    ActiveApp::Bill => self.render_bill(ui),
                    ActiveApp::Buch => self.render_buch(ui),
                    ActiveApp::Spend => self.render_spend(ui),
                    ActiveApp::Hire => self.render_hire(ui),
                    ActiveApp::Task => self.render_task(ui),
                    ActiveApp::Desk => self.render_desk(ui),
                    ActiveApp::Contract => self.render_contract(ui),
                    ActiveApp::Gain => self.render_gain(ui),
                    ActiveApp::Team => self.render_team(ui),
                    ActiveApp::Asset => self.render_asset(ui),
                    ActiveApp::Form => self.render_form(ui),
                    ActiveApp::Wiki => self.render_wiki(ui),
                }
            });
        });
    }
}

impl ShowcaseApp {
    fn render_helm(&mut self, ui: &mut egui::Ui) {
        ui.heading("HelmCraft — Executive Telemetry Cockpit");
        ui.separator();
        ui.columns(3, |cols| {
            cols[0].group(|ui| {
                ui.label("Cash Reserve");
                ui.heading(RichText::new(format!("€{:.2}M", self.cash_millions)).color(Color32::from_rgb(52, 211, 153)));
            });
            cols[1].group(|ui| {
                ui.label("Runway");
                ui.heading(RichText::new(format!("{:.1} mo", self.runway_months)).color(Color32::from_rgb(56, 189, 248)));
            });
            cols[2].group(|ui| {
                ui.label("Bus Events");
                ui.heading(RichText::new(format!("{}", self.bus_log.len())).color(Color32::from_rgb(167, 139, 250)));
            });
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button("Simulate +€150k Invoice Paid").clicked() {
                self.cash_millions += 0.15;
                self.runway_months += 1.4;
                self.bus_log.push("billcraft -> invoice.paid (+€150,000)".into());
            }
            if ui.button("📄 Generate Board Briefing").clicked() {
                self.briefing_generated = true;
            }
        });
        if self.briefing_generated {
            ui.add_space(6.0);
            ui.group(|ui| {
                ui.strong("Executive Briefing (Generated Locally)");
                ui.label(format!("Cash: €{:.2}M • Runway: {:.1} months • All 13 SQLite ledgers healthy.", self.cash_millions, self.runway_months));
            });
        }
    }

    fn render_bill(&mut self, ui: &mut egui::Ui) {
        ui.heading("BillCraft — Austrian ebInterface 6.1 & SEPA");
        ui.separator();
        for (i, inv) in self.invoices.iter().enumerate() {
            let selected = self.selected_invoice == i;
            if ui.selectable_label(selected, format!("{} | {} | €{:.2} | {}", inv.id, inv.customer, inv.amount_cents as f64 / 100.0, inv.status)).clicked() {
                self.selected_invoice = i;
            }
        }
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if ui.button("Mark Selected Paid").clicked() {
                if let Some(inv) = self.invoices.get_mut(self.selected_invoice) {
                    inv.status = "Paid".into();
                    self.bus_log.push(format!("billcraft -> invoice.paid ({})", inv.id));
                    self.toast = Some(format!("{} marked Paid", inv.id));
                }
            }
            if ui.button("Toggle ebInterface 6.1 XML").clicked() {
                self.show_xml_preview = !self.show_xml_preview;
            }
        });
        if self.show_xml_preview {
            if let Some(inv) = self.invoices.get(self.selected_invoice) {
                ui.add_space(6.0);
                ui.group(|ui| {
                    ui.label(RichText::new(format!(
                        "<eb:Invoice GeneratingSystem=\"BillCraft\"><eb:InvoiceNumber>{}</eb:InvoiceNumber><eb:InvoiceDate>{}</eb:InvoiceDate><eb:TotalGrossAmount>{:.2}</eb:TotalGrossAmount></eb:Invoice>",
                        inv.id, inv.date, inv.amount_cents as f64 / 100.0
                    )).monospace().size(11.0));
                });
            }
        }
    }

    fn render_buch(&mut self, ui: &mut egui::Ui) {
        ui.heading("BuchCraft — Austrian Double-Entry Bookkeeping");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.buch_tab, 0, "Journal & Storno");
            ui.selectable_value(&mut self.buch_tab, 1, "U30 Kennzahlen");
            ui.selectable_value(&mut self.buch_tab, 2, "Bank CSV Inbox");
        });
        ui.separator();

        match self.buch_tab {
            0 => {
                let mut storno_idx = None;
                for (idx, row) in self.ledger.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let txt = format!("{} | {} | {} | €{:.2} ({})", row.date, row.account, row.text, row.net_cents as f64 / 100.0, row.vat_key);
                        if row.stornoed {
                            ui.label(RichText::new(format!("[STORNO] {txt}")).strikethrough().color(Color32::GRAY));
                        } else {
                            ui.label(txt);
                            if !self.month_closed && ui.small_button("Storno").clicked() {
                                storno_idx = Some(idx);
                            }
                        }
                    });
                }
                if let Some(idx) = storno_idx {
                    if let Some(row) = self.ledger.get_mut(idx) {
                        row.stornoed = true;
                        let id = row.id.clone();
                        self.bus_log.push(format!("buchcraft -> ledger.storno ({id})"));
                    }
                }
            }
            1 => {
                let rev: i64 = self.ledger.iter().filter(|r| !r.stornoed && r.vat_key == "U20").map(|r| r.net_cents).sum();
                let pre: i64 = self.ledger.iter().filter(|r| !r.stornoed && r.vat_key == "V20").map(|r| r.net_cents).sum();
                ui.label(format!("KZ 000 / 022 (20% Umsatzsteuer-Basis): €{:.2}", rev as f64 / 100.0));
                ui.label(format!("KZ 060 (Vorsteuer 20%): €{:.2}", (pre as f64 * 0.20) / 100.0));
                ui.add_space(6.0);
                if ui.button(if self.month_closed { "Month 2026-11 Closed" } else { "Close Month 2026-11" }).clicked() {
                    self.month_closed = true;
                    self.toast = Some("Month 2026-11 locked against edits".into());
                }
            }
            _ => {
                if self.bank_inbox_booked {
                    ui.label("Bank CSV inbox is empty (all rows booked).");
                } else {
                    ui.horizontal(|ui| {
                        ui.label("2026-11-10 | Hetzner Online GmbH | -€42.00");
                        if ui.button("Book to 7380 (IGE20)").clicked() {
                            self.bank_inbox_booked = true;
                            self.ledger.push(LedgerRow {
                                id: "BK-104".into(),
                                date: "2026-11-10".into(),
                                account: "7380 Cloud/Hosting".into(),
                                text: "Hetzner Online GmbH".into(),
                                net_cents: 42_00,
                                vat_key: "IGE20".into(),
                                stornoed: false,
                            });
                            self.toast = Some("Booked BK-104 from Bank CSV".into());
                        }
                    });
                }
            }
        }
    }

    fn render_spend(&mut self, ui: &mut egui::Ui) {
        ui.heading("SpendCraft — §26 EStG Travel & Expenses");
        ui.separator();
        ui.add(egui::Slider::new(&mut self.trip_km, 0.0..=800.0).text("Distance (km)"));
        ui.add(egui::Slider::new(&mut self.trip_hours, 1.0..=24.0).text("Duration (hours)"));
        ui.checkbox(&mut self.provided_lunch, "Provided Lunch (-€13.20)");
        ui.checkbox(&mut self.provided_dinner, "Provided Dinner (-€13.20)");

        let km_geld = self.trip_km * 0.42;
        let base_diaeten = if self.trip_hours >= 11.0 { 26.40 } else if self.trip_hours > 3.0 { (self.trip_hours.floor() / 12.0 * 26.40).min(26.40) } else { 0.0 };
        let ded = (if self.provided_lunch { 13.20 } else { 0.0 }) + (if self.provided_dinner { 13.20 } else { 0.0 });
        let net_diaeten = (base_diaeten - ded).max(0.0);

        ui.add_space(6.0);
        ui.group(|ui| {
            ui.label(format!("Kilometergeld (€0.42/km): €{km_geld:.2}"));
            ui.label(format!("Tagesdiäten (net): €{net_diaeten:.2}"));
            ui.strong(format!("Total Reimbursement: €{:.2}", km_geld + net_diaeten));
        });
        if ui.button("Export SEPA pain.001 Batch").clicked() {
            self.bus_log.push(format!("spendcraft -> reimbursement.batched (€{:.2})", km_geld + net_diaeten));
            self.toast = Some("SEPA pain.001 batch queued".into());
        }
    }

    fn render_hire(&mut self, ui: &mut egui::Ui) {
        ui.heading("HireCraft — Blind ATS & Onboarding Cascade");
        ui.separator();
        let mut hired_name: Option<String> = None;
        for c in &mut self.candidates {
            ui.horizontal(|ui| {
                ui.label(format!("{} ({}) — {} | Score: {}/5 | Stage: {}", c.name, c.id, c.role, c.score, c.stage));
                if c.stage != "hired" && ui.small_button("Hire & Trigger Cascade").clicked() {
                    c.stage = "hired".into();
                    hired_name = Some(c.name.clone());
                }
            });
        }
        if let Some(name) = hired_name {
            self.bus_log.push(format!("hirecraft -> candidate.hired ({name}) -> ContractCraft + TeamCraft + AssetCraft"));
            self.toast = Some(format!("Cascade triggered for {name}"));
        }
    }

    fn render_task(&mut self, ui: &mut egui::Ui) {
        ui.heading("TaskCraft — Local-First Kanban Board");
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.new_task_title).hint_text("New task title..."));
            if ui.button("Add to Todo").clicked() && !self.new_task_title.trim().is_empty() {
                let next_id = format!("ENG-{}", 101 + self.issues.len());
                self.issues.push(TaskIssue {
                    id: next_id,
                    title: self.new_task_title.trim().to_string(),
                    column: "Todo".into(),
                    assignee: "Sandra".into(),
                });
                self.new_task_title.clear();
            }
        });
        ui.separator();

        let cols_names = ["Todo", "In Progress", "Review", "Done"];
        let mut move_req: Option<(usize, &'static str)> = None;

        ui.columns(4, |cols| {
            for (c_idx, col_name) in cols_names.iter().enumerate() {
                cols[c_idx].group(|ui| {
                    ui.strong(*col_name);
                    ui.separator();
                    for (idx, issue) in self.issues.iter().enumerate() {
                        if issue.column == *col_name {
                            ui.group(|ui| {
                                ui.label(RichText::new(&issue.id).size(10.0).color(Color32::from_rgb(56, 189, 248)));
                                ui.label(RichText::new(&issue.title).size(11.0));
                                ui.horizontal(|ui| {
                                    if c_idx > 0 && ui.small_button("←").clicked() {
                                        move_req = Some((idx, cols_names[c_idx - 1]));
                                    }
                                    if c_idx + 1 < cols_names.len() && ui.small_button("→").clicked() {
                                        move_req = Some((idx, cols_names[c_idx + 1]));
                                    }
                                });
                            });
                        }
                    }
                });
            }
        });
        if let Some((idx, target)) = move_req {
            if let Some(issue) = self.issues.get_mut(idx) {
                issue.column = target.into();
            }
        }
    }

    fn render_desk(&mut self, ui: &mut egui::Ui) {
        ui.heading("DeskCraft — Maildir Support & SLA Clocks");
        ui.separator();
        for (i, t) in self.tickets.iter().enumerate() {
            if ui.selectable_label(self.selected_ticket == i, format!("{} | {} | {} | SLA {}m | {}", t.id, t.customer, t.subject, t.sla_mins, t.status)).clicked() {
                self.selected_ticket = i;
            }
        }
        ui.add_space(6.0);
        if let Some(t) = self.tickets.get_mut(self.selected_ticket) {
            ui.horizontal(|ui| {
                if ui.button("Escalate to TaskCraft").clicked() {
                    let id = t.id.clone();
                    let subj = t.subject.clone();
                    self.issues.push(TaskIssue { id: format!("DESK-{id}"), title: subj, column: "Todo".into(), assignee: "Sandra".into() });
                    self.bus_log.push(format!("deskcraft -> ticket.escalated ({id} -> taskcraft)"));
                    self.toast = Some(format!("Escalated {id} to TaskCraft"));
                }
                if ui.button("Mark Resolved").clicked() {
                    t.status = "Resolved".into();
                }
            });
        }
    }

    fn render_contract(&mut self, ui: &mut egui::Ui) {
        ui.heading("ContractCraft — Ed25519 Sealing & Kündigungsfrist");
        ui.separator();
        for c in &mut self.contracts {
            ui.horizontal(|ui| {
                let seal_txt = if c.sealed { "🔒 Ed25519 Sealed" } else { "📝 Draft" };
                ui.label(format!("{} | {} ({}) | Notice: {}d | {}", c.id, c.title, c.counterparty, c.notice_days, seal_txt));
                if !c.sealed && ui.small_button("Seal & Sign").clicked() {
                    c.sealed = true;
                    self.bus_log.push(format!("contractcraft -> contract.signed ({})", c.id));
                    self.toast = Some(format!("{} cryptographically sealed", c.id));
                }
            });
        }
    }

    fn render_gain(&mut self, ui: &mut egui::Ui) {
        ui.heading("GainCraft — Customer 360 & Health Bands");
        ui.horizontal(|ui| {
            for band in ["All", "Green", "Yellow", "Red"] {
                if ui.selectable_label(self.gain_filter == band, band).clicked() {
                    self.gain_filter = band.into();
                }
            }
        });
        ui.separator();
        let accounts = [
            ("Alpine Tech GmbH", "Green", "NPS 9 • All invoices paid"),
            ("Wiener Software AG", "Yellow", "NPS 7 • 1 open support ticket"),
            ("Grazer Cloud KG", "Red", "NPS 4 • Overdue INV-2026-003"),
        ];
        for (name, band, note) in accounts {
            if self.gain_filter == "All" || self.gain_filter == band {
                ui.label(format!("[{band}] {name} — {note}"));
            }
        }
    }

    fn render_team(&mut self, ui: &mut egui::Ui) {
        ui.heading("TeamCraft — Austrian AZG 38.5h Compliance");
        ui.separator();
        ui.add(egui::Slider::new(&mut self.daily_hours, 4.0..=12.0).text("Today's Hours"));
        ui.add(egui::Slider::new(&mut self.weekly_hours, 20.0..=55.0).text("Weekly Hours"));
        let ok = self.daily_hours <= 10.0 && self.weekly_hours <= 50.0;
        let status = if ok { "✅ AZG Compliant (<= 10h/day, <= 50h/week)" } else { "⚠ AZG Limit Exceeded (>10h/day or >50h/week)" };
        ui.label(RichText::new(status).color(if ok { Color32::from_rgb(52, 211, 153) } else { Color32::from_rgb(244, 63, 94) }));
    }

    fn render_asset(&mut self, ui: &mut egui::Ui) {
        ui.heading("AssetCraft — IT Registry & §7 EStG AfA");
        ui.separator();
        for a in &self.assets {
            let annual_afa = a.cost_eur / a.afa_years as f64;
            ui.label(format!("{} | {} | Assigned: {} | €{:.0} / {}y = €{:.2}/yr", a.tag, a.name, a.assignee, a.cost_eur, a.afa_years, annual_afa));
        }
        ui.add_space(6.0);
        if ui.button("Emit Monthly AfA to BuchCraft").clicked() {
            self.bus_log.push("assetcraft -> asset.depreciation_due -> buchcraft".into());
            self.toast = Some("AfA draft sent to BuchCraft".into());
        }
    }

    fn render_form(&mut self, ui: &mut egui::Ui) {
        ui.heading("FormCraft — NPS Intake & Detractor Routing");
        ui.separator();
        ui.add(egui::Slider::new(&mut self.nps_score, 0..=10).text("NPS Score"));
        ui.add(egui::TextEdit::singleline(&mut self.nps_comment));
        if ui.button("Submit Response").clicked() {
            if self.nps_score <= 6 {
                self.bus_log.push(format!("formcraft -> signal.negative (NPS {}, routed to GainCraft)", self.nps_score));
                self.toast = Some("Detractor routed to GainCraft!".into());
            } else {
                self.toast = Some("Promoter/Passive NPS recorded".into());
            }
        }
    }

    fn render_wiki(&mut self, ui: &mut egui::Ui) {
        ui.heading("WikiCraft — SQLite FTS5 Docs & ADRs");
        ui.add(egui::TextEdit::singleline(&mut self.wiki_query).hint_text("Search docs..."));
        ui.separator();
        let q = self.wiki_query.trim().to_lowercase();
        for (i, doc) in self.wiki_docs.iter().enumerate() {
            if !q.is_empty() && !doc.title.to_lowercase().contains(&q) && !doc.body.to_lowercase().contains(&q) {
                continue;
            }
            if ui.selectable_label(self.selected_wiki == i, format!("{} ({})", doc.title, doc.kind)).clicked() {
                self.selected_wiki = i;
            }
        }
        ui.add_space(6.0);
        if let Some(doc) = self.wiki_docs.get(self.selected_wiki) {
            ui.group(|ui| {
                ui.strong(&doc.title);
                ui.label(&doc.body);
            });
        }
    }
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("No document found"))?;
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
