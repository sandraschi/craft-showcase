#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use eframe::egui::{self, Color32, RichText};
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum ActiveTab {
    Helm,
    Bill,
    Spend,
    Hire,
    Task,
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

pub struct ShowcaseApp {
    tab: ActiveTab,
    // Helm State
    cash_millions: f64,
    runway_months: f64,
    briefing_generated: bool,
    // Bill State
    invoices: Vec<Invoice>,
    show_xml_preview: bool,
    xml_content: String,
    // Spend State
    trip_km: f64,
    trip_hours: f64,
    provided_lunch: bool,
    provided_dinner: bool,
    calculated_km_geld: f64,
    calculated_diaeten: f64,
    // Hire State
    candidates: Vec<Candidate>,
    cascade_alert: Option<String>,
    // Task State
    issues: Vec<TaskIssue>,
}

impl Default for ShowcaseApp {
    fn default() -> Self {
        Self {
            tab: ActiveTab::Helm,
            cash_millions: 1.42,
            runway_months: 20.8,
            briefing_generated: false,
            invoices: vec![
                Invoice { id: "INV-2026-001".into(), customer: "Alpine Tech GmbH".into(), amount_cents: 14_400_00, status: "Paid".into(), date: "2026-11-01".into() },
                Invoice { id: "INV-2026-002".into(), customer: "Wiener Software AG".into(), amount_cents: 8_500_00, status: "Issued".into(), date: "2026-11-04".into() },
                Invoice { id: "INV-2026-003".into(), customer: "Grazer Cloud KG".into(), amount_cents: 12_000_00, status: "Overdue".into(), date: "2026-10-15".into() },
            ],
            show_xml_preview: false,
            xml_content: String::new(),
            trip_km: 120.0,
            trip_hours: 8.5,
            provided_lunch: true,
            provided_dinner: false,
            calculated_km_geld: 50.40,
            calculated_diaeten: 13.20,
            candidates: vec![
                Candidate { id: "C-101".into(), name: "Florian Bauer".into(), role: "Staff SRE".into(), stage: "offer".into(), score: 5 },
                Candidate { id: "C-102".into(), name: "Johanna Gruber".into(), role: "Rust Systems Engineer".into(), stage: "interview".into(), score: 4 },
                Candidate { id: "C-103".into(), name: "Lukas Weber".into(), role: "Frontend UI Specialist".into(), stage: "screening".into(), score: 4 },
            ],
            cascade_alert: None,
            issues: vec![
                TaskIssue { id: "ENG-101".into(), title: "Audit ISO 20022 pain.008 batch format".into(), column: "In Progress".into(), assignee: "Sandra".into() },
                TaskIssue { id: "ENG-102".into(), title: "Implement Austrian §26 EStG Diäten calculator".into(), column: "Done".into(), assignee: "Sandra".into() },
                TaskIssue { id: "ENG-103".into(), title: "Refactor blind scorecard aggregation".into(), column: "Todo".into(), assignee: "Agent".into() },
                TaskIssue { id: "ENG-104".into(), title: "Compile WebAssembly eframe showcase runner".into(), column: "In Progress".into(), assignee: "Sandra".into() },
            ],
        }
    }
}

impl eframe::App for ShowcaseApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Top Navigation Bar
        ui.horizontal(|ui| {
            ui.heading(RichText::new("Sovereign Craft Suite").strong().color(Color32::from_rgb(56, 189, 248)));
            ui.label(RichText::new("— In-Browser WASM Showcase").italics().color(Color32::GRAY));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new("Pure Rust • 60 FPS • Zero Cloud").size(11.0).color(Color32::from_rgb(34, 197, 94)));
            });
        });

        ui.add_space(4.0);

        // Tab Switcher
        ui.horizontal(|ui| {
            if ui.selectable_label(self.tab == ActiveTab::Helm, "🧭 HelmCraft (Executive Cockpit)").clicked() {
                self.tab = ActiveTab::Helm;
            }
            if ui.selectable_label(self.tab == ActiveTab::Bill, "🧾 BillCraft (Invoicing & ebInterface)").clicked() {
                self.tab = ActiveTab::Bill;
            }
            if ui.selectable_label(self.tab == ActiveTab::Spend, "💳 SpendCraft (Travel §26 EStG)").clicked() {
                self.tab = ActiveTab::Spend;
            }
            if ui.selectable_label(self.tab == ActiveTab::Hire, "🎯 HireCraft (ATS & Onboarding)").clicked() {
                self.tab = ActiveTab::Hire;
            }
            if ui.selectable_label(self.tab == ActiveTab::Task, "📋 TaskCraft (Kanban Board)").clicked() {
                self.tab = ActiveTab::Task;
            }
        });

        ui.separator();

        match self.tab {
            ActiveTab::Helm => self.render_helm(ui),
            ActiveTab::Bill => self.render_bill(ui),
            ActiveTab::Spend => self.render_spend(ui),
            ActiveTab::Hire => self.render_hire(ui),
            ActiveTab::Task => self.render_task(ui),
        }
    }
}

impl ShowcaseApp {
    fn render_helm(&mut self, ui: &mut egui::Ui) {
        ui.columns(4, |cols| {
            cols[0].group(|ui| {
                ui.label(RichText::new("TREASURY CASH").size(11.0).color(Color32::GRAY));
                ui.label(RichText::new(format!("€{:.2}M", self.cash_millions)).size(20.0).strong().color(Color32::from_rgb(34, 197, 94)));
                ui.label(format!("Runway: {:.1} months", self.runway_months));
            });
            cols[1].group(|ui| {
                ui.label(RichText::new("ARR / MRR").size(11.0).color(Color32::GRAY));
                ui.label(RichText::new("€1.77M / €148k").size(18.0).strong().color(Color32::from_rgb(56, 189, 248)));
                ui.label("Monthly Burn: €68.2k");
            });
            cols[2].group(|ui| {
                ui.label(RichText::new("TALENT & ORG").size(11.0).color(Color32::GRAY));
                ui.label(RichText::new("42 FTE").size(20.0).strong().color(Color32::from_rgb(168, 85, 247)));
                ui.label("3 Open Reqs • 1 Pending Offer");
            });
            cols[3].group(|ui| {
                ui.label(RichText::new("AUSTRIAN COMPLIANCE").size(11.0).color(Color32::GRAY));
                ui.label(RichText::new("100% AZG Clean").size(18.0).strong().color(Color32::from_rgb(34, 197, 94)));
                ui.label("0 Workweek Violations");
            });
        });

        ui.add_space(8.0);

        ui.columns(3, |cols| {
            cols[0].group(|ui| {
                ui.heading(RichText::new("Treasury Health").size(13.0).strong());
                ui.separator();
                ui.label("• Cash: €1,420,500.00");
                ui.label("• Receivables: €34,200.00");
                ui.label(RichText::new("• Overdue Dunning: €12,000.00").color(Color32::from_rgb(239, 68, 68)));
                ui.add_space(6.0);
                ui.label(RichText::new("SEPA Direct Debit batch ready in BillCraft.").italics());
            });

            cols[1].group(|ui| {
                ui.heading(RichText::new("Live Fleet Radar").size(13.0).strong());
                ui.separator();
                ui.label("• 11 SQLite DBs Synchronized");
                ui.label("• Contract Expirations: 0 in 90d");
                ui.label("• Customer Health: 98% Green Band");
                ui.add_space(6.0);
                ui.label(RichText::new("Atomic event bus: 0 failed events.").italics());
            });

            cols[2].group(|ui| {
                ui.heading(RichText::new("Executive Actions").size(13.0).strong());
                ui.separator();
                ui.label("1. Execute SEPA pain.008 collection");
                ui.label("2. Finalize SRE Lead hire in HireCraft");
                ui.label("3. File quarterly AfA schedule under §7");
                ui.add_space(6.0);
                if ui.button("⚡ Generate Board Briefing").clicked() {
                    self.briefing_generated = true;
                }
            });
        });

        if self.briefing_generated {
            ui.add_space(8.0);
            ui.group(|ui| {
                ui.heading(RichText::new("Generated CEO / CFO Board Briefing").size(13.0).color(Color32::from_rgb(56, 189, 248)));
                ui.separator();
                ui.label(RichText::new("Enterprise operations remain sound with €1.42M treasury cash (runway 20.8 months at €68.2k/mo burn). MRR stands at €148k (€1.77M ARR). Headcount is 42 FTE across engineering and operations with 100% Austrian labor law (AZG) compliance.").size(12.0));
            });
        }
    }

    fn render_bill(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new("BillCraft — Invoicing & Subscriptions").size(15.0));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("📥 Export Austrian ebInterface 6.1 XML").clicked() {
                    self.show_xml_preview = true;
                    self.xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<eb:Invoice xmlns:eb="http://www.ebinterface.at/schema/6p1/"
            eb:GeneratingSystem="BillCraft Sovereign Engine v0.1.0"
            eb:DocumentType="Invoice"
            eb:InvoiceNumber="INV-2026-002">
  <eb:InvoiceRecipient>
    <eb:VATIdentificationNumber>ATU98765432</eb:VATIdentificationNumber>
    <eb:Name>Wiener Software AG</eb:Name>
  </eb:InvoiceRecipient>
  <eb:TotalGrossAmount>10200.00</eb:TotalGrossAmount>
  <eb:PaymentMethod>
    <eb:DirectDebit>
      <eb:CreditorID>AT12ZZZ00000000001</eb:CreditorID>
    </eb:DirectDebit>
  </eb:PaymentMethod>
</eb:Invoice>"#.to_string();
                }
            });
        });

        ui.separator();

        egui::Grid::new("invoice_grid").striped(true).min_col_width(120.0).show(ui, |ui| {
            ui.strong("Invoice ID");
            ui.strong("Customer");
            ui.strong("Amount");
            ui.strong("Status");
            ui.strong("Date");
            ui.end_row();

            for inv in &self.invoices {
                ui.label(&inv.id);
                ui.label(&inv.customer);
                ui.label(format!("€{:.2}", inv.amount_cents as f64 / 100.0));
                let color = match inv.status.as_str() {
                    "Paid" => Color32::from_rgb(34, 197, 94),
                    "Issued" => Color32::from_rgb(56, 189, 248),
                    _ => Color32::from_rgb(239, 68, 68),
                };
                ui.label(RichText::new(&inv.status).color(color));
                ui.label(&inv.date);
                ui.end_row();
            }
        });

        if self.show_xml_preview {
            ui.add_space(8.0);
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Austrian ebInterface 6.1 XML Output").strong());
                    if ui.button("✕ Close").clicked() {
                        self.show_xml_preview = false;
                    }
                });
                ui.separator();
                ui.add(egui::TextEdit::multiline(&mut self.xml_content).font(egui::TextStyle::Monospace).desired_rows(10).desired_width(f32::INFINITY));
            });
        }
    }

    fn render_spend(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("SpendCraft — Statutory Austrian Travel Allowance (§26 EStG)").size(15.0));
        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.heading(RichText::new("Trip Parameters").size(13.0).strong());
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Travel Distance (km):");
                    if ui.add(egui::Slider::new(&mut self.trip_km, 10.0..=500.0)).changed() {
                        self.recalc_travel();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Trip Duration (hours):");
                    if ui.add(egui::Slider::new(&mut self.trip_hours, 1.0..=24.0)).changed() {
                        self.recalc_travel();
                    }
                });
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.provided_lunch, "Company provided lunch (-€13.20)").changed() {
                        self.recalc_travel();
                    }
                });
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.provided_dinner, "Company provided dinner (-€13.20)").changed() {
                        self.recalc_travel();
                    }
                });
            });

            cols[1].group(|ui| {
                ui.heading(RichText::new("Austrian Tax Allowances (§26 EStG)").size(13.0).strong());
                ui.separator();
                ui.label(format!("• Kilometergeld (42¢/km): €{:.2}", self.calculated_km_geld));
                ui.label(format!("• Tagesdiäten (Max €26.40/day): €{:.2}", self.calculated_diaeten));
                let total = self.calculated_km_geld + self.calculated_diaeten;
                ui.add_space(4.0);
                ui.label(RichText::new(format!("Total Statutory Reimbursement: €{:.2}", total)).size(16.0).strong().color(Color32::from_rgb(34, 197, 94)));
                ui.add_space(6.0);
                ui.label("Ready for ISO 20022 SEPA Credit Transfer (pain.001) bank batch.");
            });
        });
    }

    fn recalc_travel(&mut self) {
        self.calculated_km_geld = self.trip_km * 0.42;
        let mut diaeten = if self.trip_hours <= 3.0 {
            0.0
        } else {
            let hourly = (self.trip_hours * 2.20).min(26.40);
            hourly
        };
        if self.provided_lunch {
            diaeten = (diaeten - 13.20).max(0.0);
        }
        if self.provided_dinner {
            diaeten = (diaeten - 13.20).max(0.0);
        }
        self.calculated_diaeten = diaeten;
    }

    fn render_hire(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("HireCraft — ATS & Blind Scorecard Pipeline").size(15.0));
        ui.separator();

        if let Some(ref alert) = self.cascade_alert {
            ui.group(|ui| {
                ui.label(RichText::new(alert).strong().color(Color32::from_rgb(34, 197, 94)));
            });
            ui.add_space(4.0);
        }

        ui.columns(3, |cols| {
            for (idx, cand) in self.candidates.iter_mut().enumerate() {
                cols[idx % 3].group(|ui| {
                    ui.label(RichText::new(&cand.name).strong().size(13.0));
                    ui.label(format!("Role: {}", cand.role));
                    ui.label(format!("Stage: {}", cand.stage.to_uppercase()));
                    ui.label(format!("Blind Scorecard: {} / 5 ★", cand.score));
                    ui.add_space(4.0);
                    if cand.stage == "offer" {
                        if ui.button("✅ Mark as HIRED (Trigger Fleet Cascade)").clicked() {
                            cand.stage = "hired".to_string();
                        }
                    } else if cand.stage == "hired" {
                        ui.label(RichText::new("Status: HIRED").color(Color32::from_rgb(34, 197, 94)).strong());
                    } else {
                        ui.label(RichText::new("In Evaluation").color(Color32::from_rgb(56, 189, 248)));
                    }
                });
            }
        });

        ui.add_space(8.0);
        ui.group(|ui| {
            ui.label(RichText::new("Fleet Onboarding Cascade Integration:").strong());
            ui.label("When a candidate is marked HIRED, HireCraft simultaneously emits candidate.hired across the atomic bus:");
            ui.label("1. ContractCraft drafts an Austrian ABGB employment contract with statutory 1-month probation.");
            ui.label("2. TeamCraft provisions an employee profile with Austrian AZG 38.5h weekly limit.");
            ui.label("3. AssetCraft reserves a primary hardware workstation and generates an asset QR tag.");
        });
    }

    fn render_task(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("TaskCraft — Local-First Kanban Board").size(15.0));
        ui.separator();

        ui.columns(4, |cols| {
            let col_names = ["Todo", "In Progress", "Review", "Done"];
            for (c_idx, col_name) in col_names.iter().enumerate() {
                cols[c_idx].group(|ui| {
                    ui.heading(RichText::new(*col_name).size(13.0).strong());
                    ui.separator();
                    for issue in &self.issues {
                        if issue.column == *col_name {
                            ui.group(|ui| {
                                ui.label(RichText::new(&issue.id).size(10.0).color(Color32::from_rgb(56, 189, 248)));
                                ui.label(RichText::new(&issue.title).size(11.0));
                                ui.label(RichText::new(format!("👤 {}", issue.assignee)).size(9.0).color(Color32::GRAY));
                            });
                        }
                    }
                });
            }
        });
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
