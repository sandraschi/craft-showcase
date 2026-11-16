const state = { plans: [], subscriptions: [] };
const eur = (cents) => `€${(cents / 100).toFixed(2)}`;

function renderPlans() {
  const grid = document.getElementById("plans-grid");
  const sel = document.getElementById("plan-select");
  grid.innerHTML = state.plans
    .map(
      (p) => `
      <article class="card">
        <div>
          <span class="pill">${p.category} · ${p.cadence_label}</span>
          <h3>${p.name}</h3>
          <p class="muted mono">${p.sku} · Save ${p.subscriber_discount_pct}%</p>
        </div>
        <div>
          <span class="price mono">${eur(p.net_price_cents)}</span>
          <span class="strike mono">${eur(p.base_price_cents)}</span>
        </div>
      </article>`
    )
    .join("");
  sel.innerHTML = state.plans
    .map((p) => `<option value="${p.sku}">${p.name} (${eur(p.net_price_cents)} / ${p.cadence_label})</option>`)
    .join("");
}

function renderSubs() {
  const el = document.getElementById("subs-list");
  if (!state.subscriptions.length) {
    el.innerHTML = `<div class="sub-card"><span class="muted">No active subscriptions yet. Activate one on the right.</span></div>`;
    return;
  }
  el.innerHTML = state.subscriptions
    .map(
      (s) => `
      <div class="sub-card">
        <div>
          <div><strong class="mono">${s.sub_number}</strong> · ${s.customer_name} · <span class="pill">${s.status}</span></div>
          <div class="muted">${s.plan_name} · Next bill: <span class="mono">${s.next_billing_date}</span> · Mandate <span class="mono">${s.mandate_ref}</span></div>
        </div>
        <div class="sub-actions">
          <button class="btn-sec" data-act="skip" data-sub="${s.sub_number}">Skip Next</button>
          <button class="btn-sec" data-act="${s.status === "Active" ? "pause" : "resume"}" data-sub="${s.sub_number}">${s.status === "Active" ? "Pause" : "Resume"}</button>
        </div>
      </div>`
    )
    .join("");

  el.querySelectorAll("button[data-act]").forEach((btn) => {
    btn.addEventListener("click", async () => {
      const sub_number = btn.dataset.sub;
      const act = btn.dataset.act;
      const url = act === "skip" ? "/api/skip" : "/api/status";
      const body = act === "skip"
        ? { sub_number }
        : { sub_number, status: act === "pause" ? "Paused" : "Active" };
      const res = await fetch(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      });
      if (res.ok) await loadData();
    });
  });
}

async function loadData() {
  let data;
  try {
    const res = await fetch("/api/catalog");
    if (!res.ok) throw new Error("api");
    data = await res.json();
  } catch {
    const res = await fetch("./catalog.json");
    data = await res.json();
  }
  document.getElementById("store-name").textContent = data.store_name;
  document.getElementById("store-tagline").textContent = data.tagline;
  state.plans = data.plans || [];
  state.subscriptions = data.subscriptions || [];
  renderPlans();
  renderSubs();
}

document.getElementById("subscribe-btn").addEventListener("click", async () => {
  const customer_name = document.getElementById("cust-name").value.trim() || "Subscriber";
  const customer_email = document.getElementById("cust-email").value.trim();
  const iban = document.getElementById("cust-iban").value.trim();
  const plan_sku = document.getElementById("plan-select").value;
  const msg = document.getElementById("msg-box");
  try {
    const res = await fetch("/api/subscribe", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ customer_name, customer_email, iban, plan_sku }),
    });
    const body = await res.json();
    if (!res.ok) throw new Error(body.error || "Failed");
    msg.className = "msg";
    msg.textContent = `Activated ${body.sub_number} (Mandate ${body.mandate_ref})`;
    await loadData();
  } catch (err) {
    msg.className = "msg";
    msg.textContent = err.message;
  }
});

loadData();
