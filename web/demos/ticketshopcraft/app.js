let state = {
  events: [],
  orders: [],
};

const fmtMoney = (cents) => `€${(Number(cents || 0) / 100).toFixed(2)}`;

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

  state.events = data.events || [];
  state.orders = data.orders || [];

  document.getElementById("store-name").textContent = data.store_name || "NEUBAU BOX OFFICE";
  document.getElementById("store-tagline").textContent =
    data.tagline || "Local-First Event Ticketing, Workshops & Seat Reservations";
  document.getElementById("generated-at").textContent = data.generated_at
    ? `Updated ${data.generated_at}`
    : "";

  const remaining = state.events.reduce(
    (acc, ev) => acc + Math.max(0, (ev.capacity || 0) - (ev.sold_count || 0)),
    0
  );
  document.getElementById("stat-events").textContent = state.events.length;
  document.getElementById("stat-remaining").textContent = remaining;
  document.getElementById("stat-orders").textContent = state.orders.length;

  renderEvents();
  renderSelect();
  renderOrders();
  updateEstimate();
}

function renderEvents() {
  const grid = document.getElementById("events-grid");
  grid.innerHTML = "";
  state.events
    .filter((ev) => ev.active)
    .forEach((ev) => {
      const rem = Math.max(0, ev.capacity - ev.sold_count);
      const card = document.createElement("article");
      card.className = "card event-card";
      card.innerHTML = `
        <div>
          <span class="tier-pill">${ev.tier_name}</span>
          <h3>${ev.title}</h3>
          <div class="meta">${ev.event_date} · ${ev.venue}</div>
        </div>
        <div>
          <div class="event-pricing">
            <div class="price">${fmtMoney(ev.price_cents)}</div>
            <div class="seats">${rem} / ${ev.capacity} left</div>
          </div>
        </div>
        <button type="button" class="btn-secondary" ${rem === 0 ? "disabled" : ""}>
          ${rem === 0 ? "Sold Out" : "Select Tier"}
        </button>
      `;
      card.querySelector("button").onclick = () => {
        document.getElementById("code-select").value = ev.code;
        updateEstimate();
        document.getElementById("attendee-name").focus();
      };
      grid.appendChild(card);
    });
}

function renderSelect() {
  const sel = document.getElementById("code-select");
  const prev = sel.value;
  sel.innerHTML = "";
  state.events
    .filter((e) => e.active)
    .forEach((ev) => {
      const opt = document.createElement("option");
      opt.value = ev.code;
      opt.textContent = `${ev.title} — ${ev.tier_name} (${fmtMoney(ev.price_cents)})`;
      sel.appendChild(opt);
    });
  if (prev && state.events.some((e) => e.code === prev)) {
    sel.value = prev;
  }
}

function renderOrders() {
  const list = document.getElementById("orders-list");
  list.innerHTML = "";
  if (state.orders.length === 0) {
    list.innerHTML = `<div class="order-item">No tickets issued yet.</div>`;
    return;
  }
  state.orders.slice(0, 12).forEach((o) => {
    const el = document.createElement("div");
    el.className = "order-item";
    el.innerHTML = `
      <div class="order-top">
        <span>${o.ticket_code} · ${o.event_title}</span>
        <span class="badge ${o.status}">${o.status}</span>
      </div>
      <div>${o.attendee_name} · ${o.quantity}x ${o.tier_name} · ${fmtMoney(o.total_cents)}</div>
    `;
    list.appendChild(el);
  });
}

function updateEstimate() {
  const code = document.getElementById("code-select").value;
  const qty = Math.max(1, Number(document.getElementById("quantity").value || 1));
  const ev = state.events.find((e) => e.code === code);
  document.getElementById("est-qty").textContent = qty;
  if (ev) {
    document.getElementById("est-total").textContent = fmtMoney(ev.price_cents * qty);
  }
}

["code-select", "quantity"].forEach((id) => {
  document.getElementById(id).addEventListener("input", updateEstimate);
});

document.getElementById("ticket-form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const msg = document.getElementById("form-msg");
  msg.textContent = "Issuing ticket...";
  msg.className = "form-msg";

  const payload = {
    code: document.getElementById("code-select").value,
    quantity: Number(document.getElementById("quantity").value || 1),
    attendee_name: document.getElementById("attendee-name").value.trim(),
    attendee_email: document.getElementById("attendee-email").value.trim(),
  };

  try {
    const res = await fetch("/api/tickets", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
    const body = await res.json();
    if (!res.ok) throw new Error(body.error || `HTTP ${res.status}`);
    msg.textContent = `Issued ${body.ticket_code} (${body.quantity}x, ${fmtMoney(body.total_cents)})`;
    msg.className = "form-msg ok";
    e.target.reset();
    document.getElementById("quantity").value = "2";
    await loadCatalog();
  } catch (err) {
    msg.textContent = `Error: ${err.message}`;
    msg.className = "form-msg err";
  }
});

loadCatalog();
