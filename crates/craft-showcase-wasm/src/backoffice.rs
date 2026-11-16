use eframe::egui::{self, Color32, RichText};

use crate::{ActiveApp, Invoice, LedgerRow, ShowcaseApp, TaskIssue};

impl ShowcaseApp {
    pub fn render_backoffice_app(&mut self, ui: &mut egui::Ui, app: ActiveApp) {
        match app {
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
            ActiveApp::Stock => self.render_stock(ui),
            _ => {}
        }
    }

    fn render_helm(&mut self, ui: &mut egui::Ui) {
        ui.heading("HelmCraft — Executive Telemetry Cockpit");
        ui.separator();
        ui.columns(3, |cols| {
            cols[0].group(|ui| {
                ui.label("Cash Reserve");
                ui.heading(
                    RichText::new(format!("€{:.2}M", self.cash_millions))
                        .color(Color32::from_rgb(52, 211, 153)),
                );
            });
            cols[1].group(|ui| {
                ui.label("Runway");
                ui.heading(
                    RichText::new(format!("{:.1} mo", self.runway_months))
                        .color(Color32::from_rgb(56, 189, 248)),
                );
            });
            cols[2].group(|ui| {
                ui.label("Bus Events");
                ui.heading(
                    RichText::new(format!("{}", self.bus_log.len()))
                        .color(Color32::from_rgb(167, 139, 250)),
                );
            });
        });
        ui.add_space(10.0);
        if self.app_view == 1 {
            ui.strong("Cross-App Ledger & Suite Health");
            ui.label(format!("Invoices: {} • Ledger rows: {} • Open tickets: {}", self.invoices.len(), self.ledger.len(), self.tickets.iter().filter(|t| t.status == "Open").count()));
            ui.label(format!("Shop orders: {} • Rentals: {} • Ticket events: {}", self.shop_orders.len(), self.rental_units.len(), self.ticket_events.len()));
        } else {
            ui.horizontal(|ui| {
                if ui.button("Simulate +€150k Invoice Paid").clicked() {
                    self.cash_millions += 0.15;
                    self.runway_months += 1.4;
                    self.bus_log
                        .push("billcraft -> invoice.paid (+€150,000)".into());
                    self.toast = Some("Cash & runway updated".into());
                }
                if ui.button("Generate Board Briefing").clicked() {
                    self.briefing_generated = true;
                }
            });
            if self.briefing_generated {
                ui.add_space(6.0);
                ui.group(|ui| {
                    ui.strong("Executive Briefing (Generated Locally)");
                    ui.label(format!(
                        "Cash: €{:.2}M • Runway: {:.1} months • All SQLite ledgers healthy.",
                        self.cash_millions, self.runway_months
                    ));
                });
            }
        }
    }

    fn render_bill(&mut self, ui: &mut egui::Ui) {
        ui.heading("BillCraft — Austrian ebInterface 6.1 & SEPA");
        ui.separator();
        match self.app_view {
            0 => {
                let total_cents: i64 = self.invoices.iter().map(|i| i.amount_cents).sum();
                let paid_cents: i64 = self.invoices.iter().filter(|i| i.status == "Paid").map(|i| i.amount_cents).sum();
                ui.columns(3, |cols| {
                    cols[0].group(|ui| {
                        ui.label("Invoices");
                        ui.heading(format!("{}", self.invoices.len()));
                    });
                    cols[1].group(|ui| {
                        ui.label("Total Billed");
                        ui.heading(format!("€{:.2}", total_cents as f64 / 100.0));
                    });
                    cols[2].group(|ui| {
                        ui.label("Paid");
                        ui.heading(RichText::new(format!("€{:.2}", paid_cents as f64 / 100.0)).color(Color32::from_rgb(52, 211, 153)));
                    });
                });
                ui.add_space(10.0);
                ui.strong("Create New Invoice");
                ui.horizontal(|ui| {
                    ui.label("Customer:");
                    ui.text_edit_singleline(&mut self.new_invoice_customer);
                    ui.label("EUR:");
                    ui.add(egui::DragValue::new(&mut self.new_invoice_eur).range(50.0..=100_000.0));
                    if ui.button("Issue Invoice").clicked() && !self.new_invoice_customer.trim().is_empty() {
                        let id = format!("INV-2026-{:03}", self.invoices.len() + 1);
                        self.invoices.push(Invoice {
                            id: id.clone(),
                            customer: self.new_invoice_customer.trim().to_string(),
                            amount_cents: (self.new_invoice_eur * 100.0) as i64,
                            status: "Issued".into(),
                            date: "2026-11-16".into(),
                        });
                        self.selected_invoice = self.invoices.len() - 1;
                        self.bus_log.push(format!("billcraft -> invoice.issued ({id})"));
                        self.toast = Some(format!("Issued {id}"));
                    }
                });
            }
            1 => {
                for (i, inv) in self.invoices.iter().enumerate() {
                    let selected = self.selected_invoice == i;
                    if ui
                        .selectable_label(
                            selected,
                            format!(
                                "{} | {} | €{:.2} | {} | {}",
                                inv.id,
                                inv.customer,
                                inv.amount_cents as f64 / 100.0,
                                inv.date,
                                inv.status
                            ),
                        )
                        .clicked()
                    {
                        self.selected_invoice = i;
                    }
                }
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button("Mark Selected Paid").clicked() {
                        if let Some(inv) = self.invoices.get_mut(self.selected_invoice) {
                            inv.status = "Paid".into();
                            self.bus_log
                                .push(format!("billcraft -> invoice.paid ({})", inv.id));
                            self.toast = Some(format!("{} marked Paid", inv.id));
                        }
                    }
                    if ui.button("Export SEPA pain.008").clicked() {
                        self.bus_log.push("billcraft -> sepa.pain008_exported".into());
                        self.toast = Some("SEPA pain.008 XML generated".into());
                    }
                });
            }
            _ => {
                if let Some(inv) = self.invoices.get(self.selected_invoice) {
                    ui.strong(format!("ebInterface 6.1 XML Preview — {}", inv.id));
                    ui.group(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "<eb:Invoice GeneratingSystem=\"BillCraft\">\n  <eb:InvoiceNumber>{}</eb:InvoiceNumber>\n  <eb:InvoiceDate>{}</eb:InvoiceDate>\n  <eb:TotalGrossAmount>{:.2}</eb:TotalGrossAmount>\n</eb:Invoice>",
                                inv.id,
                                inv.date,
                                inv.amount_cents as f64 / 100.0
                            ))
                            .monospace()
                            .size(11.0),
                        );
                    });
                }
            }
        }
    }

    fn render_buch(&mut self, ui: &mut egui::Ui) {
        ui.heading("BuchCraft — Austrian Double-Entry Bookkeeping");
        ui.separator();

        match self.app_view {
            0 => {
                let mut storno_idx = None;
                for (idx, row) in self.ledger.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let txt = format!(
                            "{} | {} | {} | €{:.2} ({})",
                            row.date,
                            row.account,
                            row.text,
                            row.net_cents as f64 / 100.0,
                            row.vat_key
                        );
                        if row.stornoed {
                            ui.label(
                                RichText::new(format!("[STORNO] {txt}"))
                                    .strikethrough()
                                    .color(Color32::GRAY),
                            );
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
                        self.bus_log
                            .push(format!("buchcraft -> ledger.storno ({id})"));
                        self.toast = Some(format!("Stornoed {id}"));
                    }
                }
            }
            1 => {
                let rev: i64 = self
                    .ledger
                    .iter()
                    .filter(|r| !r.stornoed && r.vat_key == "U20")
                    .map(|r| r.net_cents)
                    .sum();
                let pre: i64 = self
                    .ledger
                    .iter()
                    .filter(|r| !r.stornoed && r.vat_key == "V20")
                    .map(|r| r.net_cents)
                    .sum();
                ui.label(format!(
                    "KZ 000 / 022 (20% Umsatzsteuer-Basis): €{:.2}",
                    rev as f64 / 100.0
                ));
                ui.label(format!(
                    "KZ 060 (Vorsteuer 20%): €{:.2}",
                    (pre as f64 * 0.20) / 100.0
                ));
                ui.add_space(6.0);
                if ui
                    .button(if self.month_closed {
                        "Month 2026-11 Closed"
                    } else {
                        "Close Month 2026-11"
                    })
                    .clicked()
                {
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
        let base_diaeten = if self.trip_hours >= 11.0 {
            26.40
        } else if self.trip_hours > 3.0 {
            (self.trip_hours.floor() / 12.0 * 26.40).min(26.40)
        } else {
            0.0
        };
        let ded = (if self.provided_lunch { 13.20 } else { 0.0 })
            + (if self.provided_dinner { 13.20 } else { 0.0 });
        let net_diaeten = (base_diaeten - ded).max(0.0);

        ui.add_space(6.0);
        ui.group(|ui| {
            ui.label(format!("Kilometergeld (€0.42/km): €{km_geld:.2}"));
            ui.label(format!("Tagesdiäten (net): €{net_diaeten:.2}"));
            ui.strong(format!(
                "Total Reimbursement: €{:.2}",
                km_geld + net_diaeten
            ));
        });
        if ui.button("Export SEPA pain.001 Batch").clicked() {
            self.bus_log.push(format!(
                "spendcraft -> reimbursement.batched (€{:.2})",
                km_geld + net_diaeten
            ));
            self.toast = Some("SEPA pain.001 batch queued".into());
        }
    }

    fn render_hire(&mut self, ui: &mut egui::Ui) {
        ui.heading("HireCraft — Blind ATS & Onboarding Cascade");
        ui.separator();
        let mut hired_name: Option<String> = None;
        for c in &mut self.candidates {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{} ({}) — {} | Score: {}/5 | Stage: {}",
                    c.name, c.id, c.role, c.score, c.stage
                ));
                if c.stage != "hired" && ui.small_button("Hire & Trigger Cascade").clicked() {
                    c.stage = "hired".into();
                    hired_name = Some(c.name.clone());
                }
            });
        }
        if let Some(name) = hired_name {
            self.bus_log.push(format!(
                "hirecraft -> candidate.hired ({name}) -> ContractCraft + TeamCraft + AssetCraft"
            ));
            self.toast = Some(format!("Cascade triggered for {name}"));
        }
    }

    fn render_task(&mut self, ui: &mut egui::Ui) {
        ui.heading("TaskCraft — Local-First Kanban Board");
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.new_task_title)
                    .hint_text("New task title..."),
            );
            if ui.button("Add to Todo").clicked() && !self.new_task_title.trim().is_empty() {
                let next_id = format!("ENG-{}", 101 + self.issues.len());
                self.issues.push(TaskIssue {
                    id: next_id.clone(),
                    title: self.new_task_title.trim().to_string(),
                    column: "Todo".into(),
                    assignee: "Sandra".into(),
                });
                self.new_task_title.clear();
                self.bus_log.push(format!("taskcraft -> issue.created ({next_id})"));
                self.toast = Some(format!("Created {next_id}"));
            }
        });
        ui.separator();

        if self.app_view == 1 {
            for issue in &self.issues {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&issue.id).monospace().strong());
                    ui.label(&issue.title);
                    ui.label(RichText::new(format!("[{}]", issue.column)).color(Color32::GRAY));
                });
            }
            return;
        }

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
                                ui.label(
                                    RichText::new(&issue.id)
                                        .size(10.0)
                                        .color(Color32::from_rgb(56, 189, 248)),
                                );
                                ui.label(RichText::new(&issue.title).size(11.0));
                                ui.horizontal(|ui| {
                                    if c_idx > 0 && ui.small_button("<-").clicked() {
                                        move_req = Some((idx, cols_names[c_idx - 1]));
                                    }
                                    if c_idx + 1 < cols_names.len() && ui.small_button("->").clicked()
                                    {
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
                self.bus_log
                    .push(format!("taskcraft -> issue.moved ({} -> {target})", issue.id));
            }
        }
    }

    fn render_desk(&mut self, ui: &mut egui::Ui) {
        ui.heading("DeskCraft — Maildir Support & SLA Clocks");
        ui.separator();
        for (i, t) in self.tickets.iter().enumerate() {
            if ui
                .selectable_label(
                    self.selected_ticket == i,
                    format!(
                        "{} | {} | {} | SLA {}m | {}",
                        t.id, t.customer, t.subject, t.sla_mins, t.status
                    ),
                )
                .clicked()
            {
                self.selected_ticket = i;
            }
        }
        ui.add_space(6.0);
        if let Some(t) = self.tickets.get_mut(self.selected_ticket) {
            ui.horizontal(|ui| {
                if ui.button("Escalate to TaskCraft").clicked() {
                    let id = t.id.clone();
                    let subj = t.subject.clone();
                    self.issues.push(TaskIssue {
                        id: format!("DESK-{id}"),
                        title: subj,
                        column: "Todo".into(),
                        assignee: "Sandra".into(),
                    });
                    self.bus_log
                        .push(format!("deskcraft -> ticket.escalated ({id} -> taskcraft)"));
                    self.toast = Some(format!("Escalated {id} to TaskCraft"));
                }
                if ui.button("Mark Resolved").clicked() {
                    t.status = "Resolved".into();
                    self.bus_log
                        .push(format!("deskcraft -> ticket.resolved ({})", t.id));
                }
            });
        }
    }

    fn render_contract(&mut self, ui: &mut egui::Ui) {
        ui.heading("ContractCraft — Ed25519 Sealing & Notice Periods");
        ui.separator();
        for c in &mut self.contracts {
            ui.horizontal(|ui| {
                let seal_txt = if c.sealed { "Ed25519 Sealed" } else { "Draft" };
                ui.label(format!(
                    "{} | {} ({}) | Notice: {}d | {}",
                    c.id, c.title, c.counterparty, c.notice_days, seal_txt
                ));
                if !c.sealed && ui.small_button("Seal & Sign").clicked() {
                    c.sealed = true;
                    self.bus_log
                        .push(format!("contractcraft -> contract.signed ({})", c.id));
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
        let status = if ok {
            "AZG Compliant (<= 10h/day, <= 50h/week)"
        } else {
            "AZG Limit Exceeded (>10h/day or >50h/week)"
        };
        ui.label(RichText::new(status).color(if ok {
            Color32::from_rgb(52, 211, 153)
        } else {
            Color32::from_rgb(244, 63, 94)
        }));
    }

    fn render_asset(&mut self, ui: &mut egui::Ui) {
        ui.heading("AssetCraft — IT Registry & §7 EStG AfA");
        ui.separator();
        for a in &self.assets {
            let annual_afa = a.cost_eur / a.afa_years as f64;
            ui.label(format!(
                "{} | {} | Assigned: {} | €{:.0} / {}y = €{:.2}/yr",
                a.tag, a.name, a.assignee, a.cost_eur, a.afa_years, annual_afa
            ));
        }
        ui.add_space(6.0);
        if ui.button("Emit Monthly AfA to BuchCraft").clicked() {
            self.bus_log
                .push("assetcraft -> asset.depreciation_due -> buchcraft".into());
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
                self.bus_log.push(format!(
                    "formcraft -> signal.negative (NPS {}, routed to GainCraft)",
                    self.nps_score
                ));
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
            if !q.is_empty()
                && !doc.title.to_lowercase().contains(&q)
                && !doc.body.to_lowercase().contains(&q)
            {
                continue;
            }
            if ui
                .selectable_label(
                    self.selected_wiki == i,
                    format!("{} ({})", doc.title, doc.kind),
                )
                .clicked()
            {
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

    fn render_stock(&mut self, ui: &mut egui::Ui) {
        ui.heading("StockCraft — Local-First Inventory & Warehouse");
        ui.separator();
        let total_cents: i64 = self
            .stock_items
            .iter()
            .map(|s| s.qty * s.unit_cost_cents)
            .sum();
        let low_count = self
            .stock_items
            .iter()
            .filter(|s| s.qty <= s.min_stock)
            .count();
        ui.horizontal(|ui| {
            ui.label(format!(
                "SKUs: {}  •  Valuation: €{:.2}  •  Low stock: {}",
                self.stock_items.len(),
                total_cents as f64 / 100.0,
                low_count
            ));
            if ui.button("Drain Bus (billcraft / modcraft)").clicked() {
                self.bus_log
                    .push("stockcraft <- invoice.issued (pick list created)".into());
                self.toast = Some("Drained 1 pending pick list from bus".into());
            }
        });
        ui.add_space(8.0);
        for row in &mut self.stock_items {
            if self.app_view == 1 && row.qty > row.min_stock {
                continue;
            }
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.strong(&row.sku);
                    ui.label(&row.name);
                    ui.label(format!("Bin {}", row.location));
                    let qty_col = if row.qty <= row.min_stock {
                        Color32::from_rgb(251, 191, 36)
                    } else {
                        Color32::from_rgb(52, 211, 153)
                    };
                    ui.colored_label(qty_col, format!("Qty: {} (min {})", row.qty, row.min_stock));
                    ui.label(format!("€{:.2}/ea", row.unit_cost_cents as f64 / 100.0));
                    if ui.small_button("+1 Receive").clicked() {
                        row.qty += 1;
                        self.bus_log
                            .push(format!("stockcraft -> goods.received ({} +1)", row.sku));
                    }
                    if ui.small_button("-1 Issue").clicked() && row.qty > 0 {
                        row.qty -= 1;
                        self.bus_log.push(format!(
                            "stockcraft -> stock.cogs_booked ({} -> buchcraft)",
                            row.sku
                        ));
                        if row.qty <= row.min_stock {
                            self.bus_log
                                .push(format!("stockcraft -> stock.low ({} -> taskcraft)", row.sku));
                        }
                    }
                });
            });
        }
    }
}
