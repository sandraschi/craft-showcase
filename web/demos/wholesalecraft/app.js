const state = {
  accounts: [],
  products: [],
  orders: [],
  selectedAccount: null,
  cart: {},
};

const eur = (cents) => `€${(cents / 100).toFixed(2)}`;

function calcLine(product, qty, account) {
  let basePct = 0;
  if (account?.tier === "Gold") basePct = product.tier_gold_pct;
  else if (account?.tier === "Silver") basePct = product.tier_silver_pct;
  const volBonus = qty >= product.moq * 3;
  const discPct = Math.min(45, basePct + (volBonus ? 5 : 0));
  const unitCents = Math.round((product.list_price_cents * (100 - discPct)) / 100);
  return { discPct, volBonus, unitCents, lineTotal: unitCents * qty };
}

function renderAccountStrip() {
  const acc = state.selectedAccount;
  const el = document.getElementById("account-strip");
  if (!acc) {
    el.innerHTML = "";
    return;
  }
  const remaining = Math.max(0, acc.credit_limit_cents - acc.balance_cents);
  const rc = acc.country !== "AT" && acc.vat_verified;
  el.innerHTML = `
    <div class="stat-card"><div class="label">Company</div><div class="val">${acc.company_name} (${acc.country})</div></div>
    <div class="stat-card"><div class="label">Tier &amp; Terms</div><div class="val">${acc.tier} · ${acc.payment_terms}</div></div>
    <div class="stat-card"><div class="label">VAT Mode</div><div class="val">${rc ? `Reverse Charge 0% (${acc.vat_uid})` : `20% AT VAT (${acc.vat_uid})`}</div></div>
    <div class="stat-card"><div class="label">Remaining Credit</div><div class="val">${eur(remaining)} / ${eur(acc.credit_limit_cents)}</div></div>
  `;
}

function renderCatalog() {
  const q = document.getElementById("search-input").value.trim().toLowerCase();
  const cat = document.getElementById("category-select").value;
  const acc = state.selectedAccount;
  const tbody = document.getElementById("catalog-body");

  const filtered = state.products.filter((p) => {
    if (cat && p.category !== cat) return false;
    if (q && !p.sku.toLowerCase().includes(q) && !p.name.toLowerCase().includes(q)) return false;
    return true;
  });

  tbody.innerHTML = filtered
    .map((p) => {
      const qty = state.cart[p.sku] || 0;
      const calc = calcLine(p, Math.max(qty, p.moq), acc);
      const lineTotal = qty > 0 ? eur(calc.unitCents * qty) : "—";
      const bonusTag = qty > 0 && calc.volBonus ? `<span class="badge badge-ok">+5% vol</span>` : "";
      return `
        <tr>
          <td class="mono">${p.sku}</td>
          <td><strong>${p.name}</strong><div class="muted">${p.category}</div></td>
          <td class="mono">${p.moq} / pack ${p.pack_size}</td>
          <td class="mono">${eur(p.list_price_cents)}</td>
          <td class="mono">${eur(calc.unitCents)} <span class="badge">-${calc.discPct}%</span> ${bonusTag}</td>
          <td class="mono">${p.stock}</td>
          <td>
            <div class="qty-ctrl">
              <button class="qty-btn" data-sku="${p.sku}" data-dir="-1">-</button>
              <span class="qty-val mono">${qty}</span>
              <button class="qty-btn" data-sku="${p.sku}" data-dir="1">+</button>
            </div>
          </td>
          <td class="mono">${lineTotal}</td>
        </tr>
      `;
    })
    .join("");

  tbody.querySelectorAll(".qty-btn").forEach((btn) => {
    btn.addEventListener("click", () => {
      const sku = btn.dataset.sku;
      const dir = Number(btn.dataset.dir);
      const prod = state.products.find((x) => x.sku === sku);
      if (!prod) return;
      const cur = state.cart[sku] || 0;
      let next = cur;
      if (dir > 0) {
        next = cur === 0 ? prod.moq : Math.min(prod.stock, cur + prod.pack_size);
      } else {
        next = cur <= prod.moq ? 0 : cur - prod.pack_size;
      }
      if (next <= 0) delete state.cart[sku];
      else state.cart[sku] = next;
      renderCatalog();
      renderSummary();
    });
  });
}

function renderSummary() {
  const acc = state.selectedAccount;
  const linesEl = document.getElementById("order-lines");
  const totalsEl = document.getElementById("totals-box");
  const submitBtn = document.getElementById("submit-po-btn");

  const entries = Object.entries(state.cart).filter(([, qty]) => qty > 0);
  if (!entries.length || !acc) {
    linesEl.innerHTML = `<p class="muted">Select quantities at or above MOQ to build a Purchase Order.</p>`;
    totalsEl.innerHTML = "";
    submitBtn.disabled = true;
    return;
  }

  let subtotal = 0;
  linesEl.innerHTML = entries
    .map(([sku, qty]) => {
      const p = state.products.find((x) => x.sku === sku);
      if (!p) return "";
      const c = calcLine(p, qty, acc);
      subtotal += c.lineTotal;
      return `<div class="order-line"><span><span class="mono">${sku}</span> × ${qty}</span><strong class="mono">${eur(c.lineTotal)}</strong></div>`;
    })
    .join("");

  const rc = acc.country !== "AT" && acc.vat_verified;
  const vat = rc ? 0 : Math.round(subtotal * 0.2);
  const total = subtotal + vat;
  const remaining = Math.max(0, acc.credit_limit_cents - acc.balance_cents);
  const creditOk = acc.payment_terms === "Prepaid" || total <= remaining;

  totalsEl.innerHTML = `
    <div class="row"><span>Subtotal (Net)</span><span class="mono">${eur(subtotal)}</span></div>
    <div class="row"><span>${rc ? "EU Reverse Charge (0%)" : "Austrian VAT (20%)"}</span><span class="mono">${eur(vat)}</span></div>
    <div class="row grand"><span>Total (${acc.payment_terms})</span><span class="mono">${eur(total)}</span></div>
  `;

  if (!creditOk) {
    showBanner(`Exceeds available credit (${eur(remaining)} remaining).`, "err");
    submitBtn.disabled = true;
  } else {
    hideBanner();
    submitBtn.disabled = false;
  }
}

function showBanner(msg, kind) {
  const b = document.getElementById("status-banner");
  b.textContent = msg;
  b.className = `banner ${kind}`;
}
function hideBanner() {
  document.getElementById("status-banner").className = "banner hidden";
}

function renderOrders() {
  const el = document.getElementById("orders-list");
  if (!state.orders.length) {
    el.innerHTML = `<p class="muted">No Purchase Orders submitted yet.</p>`;
    return;
  }
  el.innerHTML = state.orders
    .slice(0, 6)
    .map(
      (o) => `
      <div class="order-card">
        <div><strong class="mono">${o.order_number}</strong> · ${o.account_code} · <span class="badge">${o.status}</span></div>
        <div class="muted">${o.po_reference} · ${eur(o.total_cents)} · Due ${o.due_date}</div>
      </div>`
    )
    .join("");
}

async function loadData() {
  let data;
  try {
    const res = await fetch("/api/catalog");
    if (!res.ok) throw new Error("api unavailable");
    data = await res.json();
  } catch {
    const res = await fetch("./catalog.json");
    data = await res.json();
  }
  document.getElementById("portal-name").textContent = data.portal_name;
  document.getElementById("portal-tagline").textContent = data.tagline;
  state.accounts = data.accounts || [];
  state.products = data.products || [];

  const accSelect = document.getElementById("account-select");
  accSelect.innerHTML = state.accounts
    .map((a) => `<option value="${a.account_code}">${a.account_code} — ${a.company_name} (${a.tier})</option>`)
    .join("");
  state.selectedAccount = state.accounts[0] || null;

  const cats = [...new Set(state.products.map((p) => p.category))];
  const catSelect = document.getElementById("category-select");
  catSelect.innerHTML = `<option value="">All Categories</option>` + cats.map((c) => `<option value="${c}">${c}</option>`).join("");

  try {
    const ordRes = await fetch("/api/orders");
    if (ordRes.ok) state.orders = await ordRes.json();
  } catch {}

  renderAccountStrip();
  renderCatalog();
  renderSummary();
  renderOrders();
}

document.getElementById("account-select").addEventListener("change", (e) => {
  state.selectedAccount = state.accounts.find((a) => a.account_code === e.target.value) || null;
  renderAccountStrip();
  renderCatalog();
  renderSummary();
});

document.getElementById("search-input").addEventListener("input", renderCatalog);
document.getElementById("category-select").addEventListener("change", renderCatalog);

document.getElementById("csv-apply-btn").addEventListener("click", () => {
  const lines = document.getElementById("csv-input").value.split(/\r?\n/);
  for (const raw of lines) {
    const [skuRaw, qtyRaw] = raw.split(/[,;\t]/);
    if (!skuRaw || !qtyRaw) continue;
    const sku = skuRaw.trim();
    const qty = parseInt(qtyRaw.trim(), 10);
    const prod = state.products.find((p) => p.sku === sku);
    if (prod && qty >= prod.moq && qty % prod.pack_size === 0 && qty <= prod.stock) {
      state.cart[sku] = qty;
    }
  }
  renderCatalog();
  renderSummary();
});

document.getElementById("submit-po-btn").addEventListener("click", async () => {
  const acc = state.selectedAccount;
  if (!acc) return;
  const items = Object.entries(state.cart).map(([sku, qty]) => ({ sku, qty }));
  const po_reference = document.getElementById("po-ref-input").value.trim();
  try {
    const res = await fetch("/api/orders", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ account_code: acc.account_code, po_reference, items }),
    });
    const body = await res.json();
    if (!res.ok) throw new Error(body.error || "Order failed");
    state.cart = {};
    showBanner(`Purchase Order ${body.order_number} submitted (${eur(body.total_cents)}).`, "ok");
    await loadData();
  } catch (err) {
    showBanner(err.message, "err");
  }
});

loadData();
