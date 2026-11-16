use eframe::egui::{self, Color32, RichText};

use crate::{ActiveApp, ShowcaseApp};

#[derive(Clone)]
pub struct ShopProduct {
    pub sku: String,
    pub name: String,
    pub category: String,
    pub price_cents: i64,
    pub stock: i64,
}

#[derive(Clone)]
pub struct ShopOrder {
    pub id: String,
    pub app: &'static str,
    pub customer: String,
    pub item: String,
    pub total_cents: i64,
    pub status: String,
}

#[derive(Clone)]
pub struct RentalUnit {
    pub sku: String,
    pub name: String,
    pub daily_cents: i64,
    pub deposit_cents: i64,
    pub booked_days: i64,
    pub status: String,
}

#[derive(Clone)]
pub struct TicketEvent {
    pub code: String,
    pub title: String,
    pub date: String,
    pub price_cents: i64,
    pub capacity: i64,
    pub sold: i64,
}

#[derive(Clone)]
pub struct SubPlan {
    pub id: String,
    pub customer: String,
    pub plan: String,
    pub mrr_cents: i64,
    pub status: String,
}

impl ShowcaseApp {
    pub fn render_shop_app(&mut self, ui: &mut egui::Ui, app: ActiveApp) {
        match app {
            ActiveApp::WebShop | ActiveApp::Shop | ActiveApp::AmazonShop => {
                self.render_retail_shop(ui, app);
            }
            ActiveApp::Wholesale => self.render_wholesale(ui),
            ActiveApp::DigitalShop => self.render_digital_shop(ui),
            ActiveApp::SubShop => self.render_sub_shop(ui),
            ActiveApp::RentalShop => self.render_rental_shop(ui),
            ActiveApp::TicketShop => self.render_ticket_shop(ui),
            _ => {}
        }
    }

    fn render_retail_shop(&mut self, ui: &mut egui::Ui, app: ActiveApp) {
        let title = match app {
            ActiveApp::WebShop => "WebShopCraft — D2C Retail Storefront",
            ActiveApp::Shop => "ShopCraft — Handmade & Artisan Storefront",
            _ => "AmazonShopCraft — Multi-Vendor Marketplace",
        };
        ui.heading(title);
        ui.separator();

        match self.app_view {
            0 => {
                ui.columns(3, |cols| {
                    cols[0].group(|ui| {
                        ui.label("Catalog SKUs");
                        ui.heading(format!("{}", self.shop_products.len()));
                    });
                    cols[1].group(|ui| {
                        ui.label("Orders");
                        let count = self.shop_orders.iter().filter(|o| o.app == app.short_id()).count();
                        ui.heading(format!("{count}"));
                    });
                    cols[2].group(|ui| {
                        ui.label("Storefront Port");
                        ui.heading(if app == ActiveApp::WebShop { ":11260" } else if app == ActiveApp::Shop { ":11261" } else { ":11262" });
                    });
                });
                ui.add_space(10.0);
                ui.strong("Quick Order Placement");
                ui.horizontal(|ui| {
                    ui.label("Customer:");
                    ui.text_edit_singleline(&mut self.shop_customer_input);
                    if ui.button("Place Demo Order").clicked() {
                        if let Some(prod) = self.shop_products.first_mut() {
                            if prod.stock > 0 {
                                prod.stock -= 1;
                                let id = format!("ORD-{}", 100 + self.shop_orders.len());
                                let cust = if self.shop_customer_input.trim().is_empty() {
                                    "Walk-in Customer".to_string()
                                } else {
                                    self.shop_customer_input.trim().to_string()
                                };
                                self.shop_orders.push(ShopOrder {
                                    id: id.clone(),
                                    app: app.short_id(),
                                    customer: cust,
                                    item: prod.name.clone(),
                                    total_cents: prod.price_cents,
                                    status: "Paid".into(),
                                });
                                self.bus_log.push(format!("{} -> order.paid ({id})", app.short_id()));
                                self.toast = Some(format!("Created {id}"));
                            }
                        }
                    }
                });
            }
            1 => {
                ui.strong("Storefront Product Catalog");
                ui.add_space(4.0);
                let mut buy_idx: Option<usize> = None;
                for (i, p) in self.shop_products.iter().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&p.sku).monospace().strong());
                            ui.label(&p.name);
                            ui.label(RichText::new(format!("[{}]", p.category)).color(Color32::GRAY));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Order 1x").clicked() && p.stock > 0 {
                                    buy_idx = Some(i);
                                }
                                ui.label(format!("Stock: {}", p.stock));
                                ui.strong(format!("€{:.2}", p.price_cents as f64 / 100.0));
                            });
                        });
                    });
                }
                if let Some(idx) = buy_idx {
                    if let Some(p) = self.shop_products.get_mut(idx) {
                        p.stock -= 1;
                        let id = format!("ORD-{}", 100 + self.shop_orders.len());
                        self.shop_orders.push(ShopOrder {
                            id: id.clone(),
                            app: app.short_id(),
                            customer: self.shop_customer_input.clone(),
                            item: p.name.clone(),
                            total_cents: p.price_cents,
                            status: "Paid".into(),
                        });
                        self.bus_log.push(format!("{} -> order.paid ({id})", app.short_id()));
                        self.toast = Some(format!("Ordered {} ({id})", p.sku));
                    }
                }
            }
            _ => {
                ui.strong("Orders & Fulfillment");
                ui.add_space(4.0);
                let mut ship_idx: Option<usize> = None;
                for (i, o) in self.shop_orders.iter().enumerate() {
                    if o.app != app.short_id() && app != ActiveApp::AmazonShop {
                        continue;
                    }
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&o.id).monospace());
                        ui.label(&o.customer);
                        ui.label(&o.item);
                        ui.label(format!("€{:.2}", o.total_cents as f64 / 100.0));
                        ui.label(RichText::new(&o.status).color(if o.status == "Shipped" {
                            Color32::from_rgb(52, 211, 153)
                        } else {
                            Color32::from_rgb(251, 191, 36)
                        }));
                        if o.status == "Paid" && ui.small_button("Ship").clicked() {
                            ship_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = ship_idx {
                    if let Some(o) = self.shop_orders.get_mut(idx) {
                        o.status = "Shipped".into();
                        let id = o.id.clone();
                        self.bus_log.push(format!("{} -> order.shipped ({id})", app.short_id()));
                        self.toast = Some(format!("{id} marked Shipped"));
                    }
                }
            }
        }
    }

    fn render_wholesale(&mut self, ui: &mut egui::Ui) {
        ui.heading("WholesaleCraft — B2B Tiered Pricing, MOQ & Net-30 RFQs");
        ui.separator();
        match self.app_view {
            0 => {
                ui.label("Active B2B Buyer: Alpine Retail GmbH (VAT: ATU12345678 • Terms: Net-30)");
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label("RFQ Quantity:");
                    ui.add(egui::DragValue::new(&mut self.wholesale_qty).range(10..=500));
                    let unit_eur = if self.wholesale_qty >= 100 { 14.50 } else if self.wholesale_qty >= 50 { 16.00 } else { 18.50 };
                    ui.strong(format!("Tier Price: €{unit_eur:.2}/unit • Total: €{:.2}", unit_eur * self.wholesale_qty as f64));
                    if ui.button("Submit Net-30 RFQ").clicked() {
                        let id = format!("RFQ-{}", 200 + self.shop_orders.len());
                        self.shop_orders.push(ShopOrder {
                            id: id.clone(),
                            app: "wholesalecraft",
                            customer: "Alpine Retail GmbH (Net-30)".into(),
                            item: format!("{}x B2B Bulk Carton", self.wholesale_qty),
                            total_cents: (unit_eur * self.wholesale_qty as f64 * 100.0) as i64,
                            status: "Quoted".into(),
                        });
                        self.bus_log.push(format!("wholesalecraft -> rfq.submitted ({id})"));
                        self.toast = Some(format!("Submitted {id}"));
                    }
                });
            }
            _ => {
                let mut approve_idx: Option<usize> = None;
                for (i, o) in self.shop_orders.iter().enumerate() {
                    if o.app != "wholesalecraft" {
                        continue;
                    }
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&o.id).monospace());
                        ui.label(&o.customer);
                        ui.label(&o.item);
                        ui.label(format!("€{:.2}", o.total_cents as f64 / 100.0));
                        ui.label(&o.status);
                        if o.status == "Quoted" && ui.small_button("Approve Net-30").clicked() {
                            approve_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = approve_idx {
                    if let Some(o) = self.shop_orders.get_mut(idx) {
                        o.status = "Approved (Net-30)".into();
                        let id = o.id.clone();
                        self.bus_log.push(format!("wholesalecraft -> rfq.approved ({id} -> billcraft)"));
                        self.toast = Some(format!("Approved {id}"));
                    }
                }
            }
        }
    }

    fn render_digital_shop(&mut self, ui: &mut egui::Ui) {
        ui.heading("DigitalShopCraft — Downloads, License Keys & Courses");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Buyer Email:");
            ui.text_edit_singleline(&mut self.digital_buyer_email);
            if ui.button("Issue License Key").clicked() {
                let n = self.digital_keys.len() + 1;
                let key = format!("LIC-RUST-2026-{n:04}");
                self.digital_keys.push((key.clone(), self.digital_buyer_email.clone(), "Active".into()));
                self.bus_log.push(format!("digitalshopcraft -> license.issued ({key})"));
                self.toast = Some(format!("Issued {key}"));
            }
        });
        ui.add_space(8.0);
        let mut revoke_idx: Option<usize> = None;
        for (i, (key, email, status)) in self.digital_keys.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(RichText::new(key).monospace().strong());
                ui.label(email);
                ui.label(RichText::new(status).color(if status == "Active" {
                    Color32::from_rgb(52, 211, 153)
                } else {
                    Color32::GRAY
                }));
                if status == "Active" && ui.small_button("Revoke").clicked() {
                    revoke_idx = Some(i);
                }
            });
        }
        if let Some(idx) = revoke_idx {
            if let Some(item) = self.digital_keys.get_mut(idx) {
                item.2 = "Revoked".into();
                self.bus_log.push(format!("digitalshopcraft -> license.revoked ({})", item.0));
            }
        }
    }

    fn render_sub_shop(&mut self, ui: &mut egui::Ui) {
        ui.heading("SubShopCraft — Subscription Boxes & SEPA pain.008");
        ui.separator();
        let mrr: i64 = self.sub_plans.iter().filter(|s| s.status == "Active").map(|s| s.mrr_cents).sum();
        ui.label(RichText::new(format!("Active MRR: €{:.2}", mrr as f64 / 100.0)).strong().color(Color32::from_rgb(52, 211, 153)));
        ui.add_space(6.0);
        let mut toggle_idx: Option<usize> = None;
        for (i, s) in self.sub_plans.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(RichText::new(&s.id).monospace());
                ui.label(&s.customer);
                ui.label(&s.plan);
                ui.label(format!("€{:.2}/mo", s.mrr_cents as f64 / 100.0));
                ui.label(&s.status);
                let btn = if s.status == "Active" { "Pause" } else { "Resume" };
                if ui.small_button(btn).clicked() {
                    toggle_idx = Some(i);
                }
            });
        }
        if let Some(idx) = toggle_idx {
            if let Some(s) = self.sub_plans.get_mut(idx) {
                s.status = if s.status == "Active" { "Paused".into() } else { "Active".into() };
                self.bus_log.push(format!("subshopcraft -> subscription.{} ({})", s.status.to_lowercase(), s.id));
            }
        }
    }

    fn render_rental_shop(&mut self, ui: &mut egui::Ui) {
        ui.heading("RentalShopCraft — Equipment, Bike & Studio Gear Rental");
        ui.separator();
        let mut action: Option<(usize, &'static str)> = None;
        for (i, r) in self.rental_units.iter().enumerate() {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&r.sku).monospace().strong());
                    ui.label(&r.name);
                    ui.label(format!("€{:.2}/day • Deposit €{:.2}", r.daily_cents as f64 / 100.0, r.deposit_cents as f64 / 100.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        match r.status.as_str() {
                            "Available" if ui.button("Book & Checkout (3d)").clicked() => {
                                action = Some((i, "checkout"));
                            }
                            "Checked Out" if ui.button("Check In (Refund Deposit)").clicked() => {
                                action = Some((i, "checkin"));
                            }
                            _ => {}
                        }
                        ui.label(RichText::new(&r.status).color(if r.status == "Available" {
                            Color32::from_rgb(52, 211, 153)
                        } else {
                            Color32::from_rgb(251, 191, 36)
                        }));
                    });
                });
            });
        }
        if let Some((idx, act)) = action {
            if let Some(r) = self.rental_units.get_mut(idx) {
                if act == "checkout" {
                    r.status = "Checked Out".into();
                    r.booked_days = 3;
                    self.bus_log.push(format!("rentalshopcraft -> rental.checked_out ({})", r.sku));
                    self.toast = Some(format!("Checked out {} (deposit held)", r.sku));
                } else {
                    r.status = "Available".into();
                    r.booked_days = 0;
                    self.bus_log.push(format!("rentalshopcraft -> rental.settled ({})", r.sku));
                    self.toast = Some(format!("Checked in {} (deposit refunded)", r.sku));
                }
            }
        }
    }

    fn render_ticket_shop(&mut self, ui: &mut egui::Ui) {
        ui.heading("TicketShopCraft — Event Ticketing & Box Office");
        ui.separator();
        let mut act: Option<(usize, i64)> = None;
        for (i, ev) in self.ticket_events.iter().enumerate() {
            let rem = (ev.capacity - ev.sold).max(0);
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&ev.code).monospace().strong());
                    ui.label(&ev.title);
                    ui.label(RichText::new(&ev.date).color(Color32::GRAY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if rem > 0 && ui.button("Issue 1 Ticket").clicked() {
                            act = Some((i, 1));
                        }
                        ui.label(format!("{rem}/{} left", ev.capacity));
                        ui.strong(format!("€{:.2}", ev.price_cents as f64 / 100.0));
                    });
                });
            });
        }
        if let Some((idx, qty)) = act {
            if let Some(ev) = self.ticket_events.get_mut(idx) {
                if ev.sold + qty <= ev.capacity {
                    ev.sold += qty;
                    self.bus_log.push(format!("ticketshopcraft -> ticket.issued ({}, {qty}x)", ev.code));
                    self.toast = Some(format!("Issued {qty} ticket for {}", ev.code));
                }
            }
        }
    }
}
