const state = {
  vatRates: [],
  products: [],
  cart: [],
  selectedCountry: "AT",
};

const eur = (cents) => `€${(cents / 100).toFixed(2)}`;

function currentVat() {
  return state.vatRates.find((v) => v.country_code === state.selectedCountry) || {
    country_code: "AT",
    country_name: "Austria",
    vat_pct: 20,
    oss_eligible: true,
  };
}

function renderCatalog() {
  const vat = currentVat();
  const grid = document.getElementById("catalog-grid");
  grid.innerHTML = state.products
    .map((p) => {
      const gross = Math.round(p.price_net_cents * (1 + vat.vat_pct / 100));
      return `
        <article class="card">
          <div>
            <div class="card-top">
              <span class="pill">${p.kind}</span>
              <span class="mono muted">v${p.version} · ${p.file_size_mb} MB</span>
            </div>
            <h3>${p.title}</h3>
            <p class="muted">${p.issues_license ? "Includes signed license key" : "Direct DRM-free download"} · Max ${p.max_downloads} downloads</p>
          </div>
          <div class="price-row">
            <div>
              <div class="price mono">${eur(gross)}</div>
              <div class="muted">${eur(p.price_net_cents)} net + ${vat.vat_pct}% VAT</div>
            </div>
            <button class="btn-add" data-sku="${p.sku}" type="button">Add</button>
          </div>
        </article>
      `;
    })
    .join("");

  grid.querySelectorAll(".btn-add").forEach((btn) => {
    btn.addEventListener("click", () => {
      if (!state.cart.includes(btn.dataset.sku)) {
        state.cart.push(btn.dataset.sku);
        renderCart();
      }
    });
  });
}

function renderCart() {
  const vat = currentVat();
  const itemsEl = document.getElementById("cart-items");
  const totalsEl = document.getElementById("totals-box");
  const btn = document.getElementById("checkout-btn");

  if (!state.cart.length) {
    itemsEl.innerHTML = `<p class="muted">Add digital products to generate instant license keys &amp; download tokens.</p>`;
    totalsEl.innerHTML = "";
    btn.disabled = true;
    return;
  }

  let net = 0;
  itemsEl.innerHTML = state.cart
    .map((sku) => {
      const p = state.products.find((x) => x.sku === sku);
      if (!p) return "";
      net += p.price_net_cents;
      return `<div class="cart-line"><span>${p.title}</span><span><strong class="mono">${eur(p.price_net_cents)}</strong> <button class="rm-btn" data-sku="${sku}">✕</button></span></div>`;
    })
    .join("");

  itemsEl.querySelectorAll(".rm-btn").forEach((b) => {
    b.addEventListener("click", () => {
      state.cart = state.cart.filter((s) => s !== b.dataset.sku);
      renderCart();
    });
  });

  const vatCents = Math.round((net * vat.vat_pct) / 100);
  const total = net + vatCents;
  totalsEl.innerHTML = `
    <div class="row"><span>Net Subtotal</span><span class="mono">${eur(net)}</span></div>
    <div class="row"><span>${vat.country_name} VAT (${vat.vat_pct}%)</span><span class="mono">${eur(vatCents)}</span></div>
    <div class="row grand"><span>Total</span><span class="mono">${eur(total)}</span></div>
  `;
  btn.disabled = false;
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
  document.getElementById("store-name").textContent = data.store_name;
  document.getElementById("store-tagline").textContent = data.tagline;
  state.vatRates = data.vat_rates || [];
  state.products = data.products || [];

  const sel = document.getElementById("country-select");
  sel.innerHTML = state.vatRates
    .map((v) => `<option value="${v.country_code}">${v.country_code} — ${v.country_name} (${v.vat_pct}%)</option>`)
    .join("");
  sel.value = state.selectedCountry;

  renderCatalog();
  renderCart();
}

document.getElementById("country-select").addEventListener("change", (e) => {
  state.selectedCountry = e.target.value;
  renderCatalog();
  renderCart();
});

document.getElementById("checkout-btn").addEventListener("click", async () => {
  const buyer_name = document.getElementById("buyer-name").value.trim() || "Digital Buyer";
  const buyer_email = document.getElementById("buyer-email").value.trim();
  const receipt = document.getElementById("receipt-box");
  if (!buyer_email) {
    receipt.className = "receipt";
    receipt.textContent = "Please enter a delivery email.";
    return;
  }
  try {
    const res = await fetch("/api/checkout", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        buyer_name,
        buyer_email,
        country_code: state.selectedCountry,
        items: state.cart,
      }),
    });
    const order = await res.json();
    if (!res.ok) throw new Error(order.error || "Checkout failed");
    state.cart = [];
    renderCart();
    receipt.className = "receipt";
    receipt.innerHTML = `
      <strong>Order ${order.order_number} Paid (${eur(order.total_cents)})</strong>
      ${order.licenses
        .map(
          (l) => `
        <div class="lic-box">
          <div class="mono"><strong>${l.sku}</strong></div>
          <div class="mono">Key: ${l.license_key}</div>
          <div class="mono">Token: ${l.download_token} (${l.downloads_used}/${l.max_downloads})</div>
        </div>`
        )
        .join("")}
    `;
  } catch (err) {
    receipt.className = "receipt";
    receipt.textContent = err.message;
  }
});

loadData();
