let state = {
  items: [],
  bookings: [],
  category: "All",
};

const fmtMoney = (cents) => `€${(Number(cents || 0) / 100).toFixed(2)}`;

function isoToday(offsetDays = 0) {
  const d = new Date();
  d.setDate(d.getDate() + offsetDays);
  return d.toISOString().slice(0, 10);
}

function daysBetween(startStr, endStr) {
  if (!startStr || !endStr) return 1;
  const s = new Date(startStr + "T00:00:00");
  const e = new Date(endStr + "T00:00:00");
  const diff = Math.round((e - s) / 86400000) + 1;
  return diff > 0 ? diff : 1;
}

async function loadCatalog() {
  let data = null;
  try {
    const res = await fetch("/api/catalog");
    if (res.ok) data = await res.json();
  } catch (_) {}
  if (!data) {
    const res = await fetch("catalog.json");
    if (res.ok) data = await res.json();
  }
  if (!data) return;

  state.items = data.items || [];
  state.bookings = data.bookings || [];

  document.getElementById("store-name").textContent = data.store_name || "ALPINE GEAR RENTAL";
  document.getElementById("store-tagline").textContent =
    data.tagline || "Local-First Equipment, Bike & Studio Gear Rental";
  document.getElementById("generated-at").textContent = data.generated_at
    ? `Updated ${data.generated_at}`
    : "";

  const totalUnits = state.items.reduce((acc, it) => acc + (it.units_total || 0), 0);
  document.getElementById("stat-skus").textContent = state.items.length;
  document.getElementById("stat-units").textContent = totalUnits;
  document.getElementById("stat-bookings").textContent = state.bookings.length;

  renderFilters();
  renderGrid();
  renderSkuSelect();
  renderBookings();
  updateEstimate();
}

function renderFilters() {
  const cats = ["All", ...new Set(state.items.map((i) => i.category))];
  const wrap = document.getElementById("category-filters");
  wrap.innerHTML = "";
  cats.forEach((cat) => {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "filter-btn" + (state.category === cat ? " active" : "");
    btn.textContent = cat;
    btn.onclick = () => {
      state.category = cat;
      renderFilters();
      renderGrid();
    };
    wrap.appendChild(btn);
  });
}

function renderGrid() {
  const grid = document.getElementById("gear-grid");
  grid.innerHTML = "";
  const visible = state.items.filter(
    (it) => it.active && (state.category === "All" || it.category === state.category)
  );
  visible.forEach((it) => {
    const card = document.createElement("article");
    card.className = "card gear-card";
    card.innerHTML = `
      <div>
        <span class="gear-cat">${it.category}</span>
        <h3>${it.name}</h3>
        <div class="gear-sku">${it.sku}</div>
      </div>
      <div>
        <div class="gear-pricing">
          <div class="rate">${fmtMoney(it.daily_rate_cents)} <span>/ day</span></div>
          <div class="deposit">Deposit ${fmtMoney(it.deposit_cents)}</div>
        </div>
        <div class="units-pill">${it.units_total} unit(s) in fleet</div>
      </div>
      <button type="button" class="btn-secondary">Reserve Unit</button>
    `;
    card.querySelector("button").onclick = () => {
      document.getElementById("sku-select").value = it.sku;
      updateEstimate();
      document.getElementById("customer-name").focus();
    };
    grid.appendChild(card);
  });
}

function renderSkuSelect() {
  const sel = document.getElementById("sku-select");
  const prev = sel.value;
  sel.innerHTML = "";
  state.items
    .filter((i) => i.active)
    .forEach((it) => {
      const opt = document.createElement("option");
      opt.value = it.sku;
      opt.textContent = `${it.name} (${fmtMoney(it.daily_rate_cents)}/day)`;
      sel.appendChild(opt);
    });
  if (prev && state.items.some((i) => i.sku === prev)) {
    sel.value = prev;
  }
}

function renderBookings() {
  const list = document.getElementById("bookings-list");
  list.innerHTML = "";
  if (state.bookings.length === 0) {
    list.innerHTML = `<div class="booking-item">No reservations yet.</div>`;
    return;
  }
  state.bookings.slice(0, 12).forEach((b) => {
    const el = document.createElement("div");
    el.className = "booking-item";
    el.innerHTML = `
      <div class="booking-top">
        <span>${b.booking_number} · ${b.item_name}</span>
        <span class="badge ${b.status}">${b.status}</span>
      </div>
      <div>${b.customer_name} · ${b.start_date} to ${b.end_date} (${b.days}d)</div>
      <div>Fee ${fmtMoney(b.rental_fee_cents)} · Deposit ${fmtMoney(b.deposit_cents)}</div>
    `;
    list.appendChild(el);
  });
}

function updateEstimate() {
  const sku = document.getElementById("sku-select").value;
  const start = document.getElementById("start-date").value;
  const end = document.getElementById("end-date").value;
  const item = state.items.find((i) => i.sku === sku);
  const days = daysBetween(start, end);
  document.getElementById("est-days").textContent = `${days} day${days === 1 ? "" : "s"}`;
  if (item) {
    document.getElementById("est-fee").textContent = fmtMoney(item.daily_rate_cents * days);
    document.getElementById("est-deposit").textContent = fmtMoney(item.deposit_cents);
  }
}

document.getElementById("start-date").value = isoToday(1);
document.getElementById("end-date").value = isoToday(3);

["sku-select", "start-date", "end-date"].forEach((id) => {
  document.getElementById(id).addEventListener("change", updateEstimate);
});

document.getElementById("booking-form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const msg = document.getElementById("form-msg");
  msg.textContent = "Submitting reservation...";
  msg.className = "form-msg";

  const payload = {
    sku: document.getElementById("sku-select").value,
    start_date: document.getElementById("start-date").value,
    end_date: document.getElementById("end-date").value,
    customer_name: document.getElementById("customer-name").value.trim(),
    customer_email: document.getElementById("customer-email").value.trim(),
  };

  try {
    const res = await fetch("/api/book", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
    const body = await res.json();
    if (!res.ok) {
      throw new Error(body.error || `HTTP ${res.status}`);
    }
    msg.textContent = `Reserved ${body.booking_number} (${body.days} days, ${fmtMoney(
      body.rental_fee_cents
    )})`;
    msg.className = "form-msg ok";
    e.target.reset();
    document.getElementById("start-date").value = payload.start_date;
    document.getElementById("end-date").value = payload.end_date;
    await loadCatalog();
  } catch (err) {
    msg.textContent = `Error: ${err.message}`;
    msg.className = "form-msg err";
  }
});

loadCatalog();
