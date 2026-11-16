const FALLBACK_CATALOG = {
  shop_name: "ATELIER NORD",
  tagline: "Small-batch stoneware, spun aluminum lighting, and Austrian virgin wool -- crafted for quiet spaces.",
  currency: "EUR",
  categories: ["Ceramics", "Lighting", "Textiles", "Workspace"],
  shipping_zones: [
    { zone_code: "AT", label: "Austria (Post.at)", base_rate_cents: 590, free_over_cents: 9000, eta_days: "1-2 business days" },
    { zone_code: "DE", label: "Germany (DHL)", base_rate_cents: 790, free_over_cents: 11000, eta_days: "2-3 business days" },
    { zone_code: "EU", label: "European Union", base_rate_cents: 1290, free_over_cents: 15000, eta_days: "3-5 business days" }
  ],
  products: [
    {
      sku: "CER-VASE-01",
      slug: "fluted-stoneware-vase",
      title: "Fluted Stoneware Vase",
      subtitle: "Wheel-thrown matte sandstone",
      description: "Hand-thrown in small batches using mineral-rich clay and finished in a tactile raw sandstone glaze. Watertight interior for fresh botanicals.",
      category: "Ceramics",
      price_cents: 6800,
      compare_at_cents: 8200,
      vat_rate_bps: 2000,
      stock_qty: 18,
      badge: "Bestseller",
      accent_hex: "#C2593F",
      material: "High-fire Stoneware",
      dimensions: "24 x 11 cm",
      featured: true,
      active: true
    },
    {
      sku: "LGT-LAMP-02",
      slug: "brushed-brass-atelier-lamp",
      title: "Atelier Desk Lamp",
      subtitle: "Warm-dim 2700K spun aluminum",
      description: "Precision-spun shade with a weighted cast-iron base and braided linen cord. Glare-free task lighting for studios and reading corners.",
      category: "Lighting",
      price_cents: 18900,
      compare_at_cents: null,
      vat_rate_bps: 2000,
      stock_qty: 7,
      badge: "Low Stock",
      accent_hex: "#B47B38",
      material: "Spun Aluminum & Brass",
      dimensions: "41 x 18 cm",
      featured: true,
      active: true
    },
    {
      sku: "TXT-THRW-03",
      slug: "alpine-merino-throw",
      title: "Alpine Merino Throw",
      subtitle: "Unbleached herringbone weave",
      description: "Loomed from 100% Austrian and South Tyrolean virgin wool. Naturally thermoregulating, soft-brushed, with hand-knotted fringe.",
      category: "Textiles",
      price_cents: 12400,
      compare_at_cents: 14500,
      vat_rate_bps: 2000,
      stock_qty: 14,
      badge: "New Season",
      accent_hex: "#4B6B5A",
      material: "100% Virgin Merino",
      dimensions: "140 x 200 cm",
      featured: true,
      active: true
    },
    {
      sku: "WRK-TRAY-04",
      slug: "smoked-oak-valet-tray",
      title: "Smoked Oak Valet Tray",
      subtitle: "CNC-milled solid European oak",
      description: "Carved from a single block of kiln-dried European oak, fumed for deep grain contrast and sealed with organic beeswax.",
      category: "Workspace",
      price_cents: 4900,
      compare_at_cents: null,
      vat_rate_bps: 2000,
      stock_qty: 32,
      badge: null,
      accent_hex: "#5C4B3B",
      material: "Fumed European Oak",
      dimensions: "26 x 14 x 2.2 cm",
      featured: false,
      active: true
    },
    {
      sku: "CER-MUG-05",
      slug: "espresso-tumbler-pair",
      title: "Espresso Tumbler Pair",
      subtitle: "Set of 2 speckled basalt cups",
      description: "Thick-walled handleless tumblers engineered to retain heat while staying comfortable in the palm. Dishwasher safe.",
      category: "Ceramics",
      price_cents: 3600,
      compare_at_cents: null,
      vat_rate_bps: 2000,
      stock_qty: 25,
      badge: "Set of 2",
      accent_hex: "#2E3440",
      material: "Basalt Porcelain",
      dimensions: "90 ml each",
      featured: false,
      active: true
    },
    {
      sku: "WRK-NOTE-06",
      slug: "linen-bound-studio-notebook",
      title: "Linen Studio Notebook",
      subtitle: "192 pages, 120gsm Munken paper",
      description: "Lay-flat Smyth-sewn binding wrapped in Belgian bookcloth. Subtle 4mm dot grid suited for fountain pens and sketching.",
      category: "Workspace",
      price_cents: 2400,
      compare_at_cents: null,
      vat_rate_bps: 1000,
      stock_qty: 44,
      badge: null,
      accent_hex: "#3B5978",
      material: "Belgian Linen & Munken",
      dimensions: "A5 (148 x 210 mm)",
      featured: false,
      active: true
    }
  ]
};

const PROMOS = {
  WELCOME10: { kind: "PERCENT", value: 10, minOrderCents: 3000 },
  ATELIER15: { kind: "FIXED_CENTS", value: 1500, minOrderCents: 10000 }
};

const state = {
  catalog: FALLBACK_CATALOG,
  activeCategory: "All",
  search: "",
  sort: "featured",
  cart: loadCart(),
  zoneCode: "AT",
  promoCode: null
};

function loadCart() {
  try {
    const raw = localStorage.getItem("webshopcraft_cart_v1");
    return raw ? JSON.parse(raw) : [];
  } catch {
    return [];
  }
}

function saveCart() {
  try {
    localStorage.setItem("webshopcraft_cart_v1", JSON.stringify(state.cart));
  } catch {
    // ignore storage errors
  }
}

function formatCents(cents) {
  const neg = cents < 0;
  const abs = Math.abs(cents);
  const euros = Math.floor(abs / 100);
  const rem = String(abs % 100).padStart(2, "0");
  return `${neg ? "-" : ""}\u20AC${euros},${rem}`;
}

function productSvg(product) {
  const c = product.accent_hex || "#C2593F";
  if (product.sku.includes("LAMP")) {
    return `<svg viewBox="0 0 180 180" aria-hidden="true">
      <circle cx="90" cy="90" r="68" fill="${c}18"/>
      <path d="M52 74c0-20 17-34 38-34s38 14 38 34H52Z" fill="${c}"/>
      <rect x="86" y="74" width="8" height="62" rx="4" fill="#3F3F46"/>
      <rect x="58" y="136" width="64" height="10" rx="5" fill="#27272A"/>
      <ellipse cx="90" cy="76" rx="22" ry="4" fill="#FDE68A" opacity="0.75"/>
    </svg>`;
  }
  if (product.sku.includes("THRW")) {
    return `<svg viewBox="0 0 180 180" aria-hidden="true">
      <circle cx="90" cy="90" r="68" fill="${c}18"/>
      <rect x="44" y="48" width="92" height="84" rx="12" fill="${c}"/>
      <path d="M44 72h92M44 96h92M44 120h92" stroke="rgba(255,255,255,0.28)" stroke-width="3" stroke-dasharray="6 6"/>
      <path d="M52 132v12M66 132v12M80 132v12M94 132v12M108 132v12M122 132v12" stroke="${c}" stroke-width="3" stroke-linecap="round"/>
    </svg>`;
  }
  if (product.sku.includes("TRAY")) {
    return `<svg viewBox="0 0 180 180" aria-hidden="true">
      <circle cx="90" cy="90" r="68" fill="${c}18"/>
      <rect x="34" y="66" width="112" height="48" rx="14" fill="${c}"/>
      <rect x="44" y="74" width="92" height="32" rx="9" fill="rgba(255,255,255,0.16)"/>
      <circle cx="64" cy="90" r="7" fill="rgba(255,255,255,0.35)"/>
    </svg>`;
  }
  if (product.sku.includes("MUG")) {
    return `<svg viewBox="0 0 180 180" aria-hidden="true">
      <circle cx="90" cy="90" r="68" fill="${c}18"/>
      <path d="M44 68h40l-4 54c0 6-6 10-16 10s-16-4-16-10l-4-54Z" fill="${c}"/>
      <path d="M96 68h40l-4 54c0 6-6 10-16 10s-16-4-16-10l-4-54Z" fill="${c}" opacity="0.85"/>
      <ellipse cx="64" cy="68" rx="20" ry="4" fill="rgba(255,255,255,0.28)"/>
      <ellipse cx="116" cy="68" rx="20" ry="4" fill="rgba(255,255,255,0.28)"/>
    </svg>`;
  }
  if (product.sku.includes("NOTE")) {
    return `<svg viewBox="0 0 180 180" aria-hidden="true">
      <circle cx="90" cy="90" r="68" fill="${c}18"/>
      <rect x="52" y="38" width="76" height="104" rx="8" fill="${c}"/>
      <rect x="52" y="38" width="14" height="104" rx="4" fill="rgba(0,0,0,0.22)"/>
      <rect x="78" y="62" width="36" height="20" rx="3" fill="rgba(255,255,255,0.82)"/>
    </svg>`;
  }
  return `<svg viewBox="0 0 180 180" aria-hidden="true">
    <circle cx="90" cy="90" r="68" fill="${c}18"/>
    <path d="M68 42h44l-5 20c11 14 17 32 15 54-2 18-11 30-32 30s-30-12-32-30c-2-22 4-40 15-54l-5-20Z" fill="${c}"/>
    <path d="M79 46v96M90 46v98M101 46v96" stroke="rgba(255,255,255,0.28)" stroke-width="2" stroke-linecap="round"/>
  </svg>`;
}

function productMedia(product) {
  if (product.image_url) {
    return `<img src="${product.image_url}" alt="${product.title}" loading="lazy" onerror="this.style.display='none'; this.nextElementSibling.style.display='block';" /><span style="display:none;width:100%;height:100%;align-items:center;justify-content:center;">${productSvg(product)}</span>`;
  }
  return productSvg(product);
}

function showToast(msg) {
  const el = document.getElementById("toast");
  el.textContent = msg;
  el.hidden = false;
  clearTimeout(showToast._t);
  showToast._t = setTimeout(() => {
    el.hidden = true;
  }, 2400);
}

function renderCategories() {
  const cats = ["All", ...(state.catalog.categories || [])];
  const headerNav = document.getElementById("headerCategories");
  const filterPills = document.getElementById("filterPills");

  headerNav.innerHTML = cats
    .map(
      (cat) =>
        `<button type="button" class="nav-pill ${state.activeCategory === cat ? "active" : ""}" data-cat="${cat}">${cat}</button>`
    )
    .join("");

  filterPills.innerHTML = cats
    .map(
      (cat) =>
        `<button type="button" class="filter-pill ${state.activeCategory === cat ? "active" : ""}" data-cat="${cat}">${cat}</button>`
    )
    .join("");

  document.querySelectorAll("[data-cat]").forEach((btn) => {
    btn.addEventListener("click", () => {
      state.activeCategory = btn.getAttribute("data-cat") || "All";
      renderCategories();
      renderProducts();
      if (btn.classList.contains("nav-pill")) {
        document.getElementById("catalog").scrollIntoView({ behavior: "smooth" });
      }
    });
  });
}

function getFilteredProducts() {
  const q = state.search.trim().toLowerCase();
  let list = (state.catalog.products || []).filter((p) => {
    const catMatch = state.activeCategory === "All" || p.category === state.activeCategory;
    const textMatch =
      !q ||
      p.title.toLowerCase().includes(q) ||
      p.subtitle.toLowerCase().includes(q) ||
      p.material.toLowerCase().includes(q) ||
      p.sku.toLowerCase().includes(q);
    return catMatch && textMatch;
  });

  if (state.sort === "price-asc") {
    list = [...list].sort((a, b) => a.price_cents - b.price_cents);
  } else if (state.sort === "price-desc") {
    list = [...list].sort((a, b) => b.price_cents - a.price_cents);
  } else if (state.sort === "title") {
    list = [...list].sort((a, b) => a.title.localeCompare(b.title));
  } else {
    list = [...list].sort((a, b) => Number(b.featured) - Number(a.featured));
  }
  return list;
}

function renderProducts() {
  const grid = document.getElementById("productGrid");
  const countEl = document.getElementById("resultCount");
  const list = getFilteredProducts();
  countEl.textContent = `${list.length} object${list.length === 1 ? "" : "s"}`;

  if (list.length === 0) {
    grid.innerHTML = `<div class="cart-empty" style="grid-column: 1 / -1;">No objects match your filter. Try clearing the search or choosing "All".</div>`;
    return;
  }

  grid.innerHTML = list
    .map((p) => {
      const soldOut = p.stock_qty <= 0;
      const badgeHtml = p.badge ? `<span class="product-badge">${p.badge}</span>` : "";
      const wasHtml = p.compare_at_cents
        ? `<span class="price-was">${formatCents(p.compare_at_cents)}</span>`
        : "";
      return `
      <article class="product-card" data-sku="${p.sku}">
        <div class="product-thumb" data-quick="${p.sku}">
          ${badgeHtml}
          ${productMedia(p)}
        </div>
        <div class="product-info">
          <span class="product-cat">${p.category} &middot; ${p.material}</span>
          <h3 class="product-title" data-quick="${p.sku}">${p.title}</h3>
          <p class="product-subtitle">${p.subtitle}</p>
          <div class="product-bottom">
            <div class="price-group">
              <span class="price-now">${formatCents(p.price_cents)}</span>
              ${wasHtml}
            </div>
            <button class="add-btn" type="button" data-add="${p.sku}" ${soldOut ? "disabled" : ""}>
              ${soldOut ? "Sold Out" : "Add to Bag"}
            </button>
          </div>
        </div>
      </article>`;
    })
    .join("");

  grid.querySelectorAll("[data-add]").forEach((btn) => {
    btn.addEventListener("click", () => {
      addToCart(btn.getAttribute("data-add"), 1);
    });
  });

  grid.querySelectorAll("[data-quick]").forEach((el) => {
    el.addEventListener("click", () => {
      openProductModal(el.getAttribute("data-quick"));
    });
  });
}

function findProduct(sku) {
  return (state.catalog.products || []).find((p) => p.sku === sku);
}

function openProductModal(sku) {
  const p = findProduct(sku);
  if (!p) return;
  const backdrop = document.getElementById("productModalBackdrop");
  const body = document.getElementById("productModalBody");
  const wasHtml = p.compare_at_cents
    ? `<span class="price-was">${formatCents(p.compare_at_cents)}</span>`
    : "";

  body.innerHTML = `
    <div class="modal-art" style="overflow:hidden;">${productMedia(p)}</div>
    <div>
      <span class="product-cat">${p.category} &middot; ${p.sku}</span>
      <h2 id="modalTitle" style="margin:0.35rem 0 0.4rem;font-family:var(--font-serif);font-size:2rem;font-weight:400;">${p.title}</h2>
      <div class="price-group" style="margin-bottom:0.9rem;">
        <span class="price-now" style="font-size:1.15rem;">${formatCents(p.price_cents)}</span>
        ${wasHtml}
      </div>
      <p style="margin:0;color:var(--muted);font-size:0.92rem;">${p.description}</p>
      <dl class="spec-list">
        <div><dt>Material</dt><dd>${p.material}</dd></div>
        <div><dt>Dimensions</dt><dd>${p.dimensions}</dd></div>
        <div><dt>VAT Included</dt><dd>${(p.vat_rate_bps / 100).toFixed(0)}%</dd></div>
        <div><dt>Availability</dt><dd>${p.stock_qty} in studio stock</dd></div>
      </dl>
      <button type="button" class="btn btn-primary btn-full" id="modalAddBtn" ${p.stock_qty <= 0 ? "disabled" : ""}>
        ${p.stock_qty <= 0 ? "Sold Out" : "Add to Bag"}
      </button>
    </div>
  `;

  document.getElementById("modalAddBtn").addEventListener("click", () => {
    addToCart(p.sku, 1);
    backdrop.hidden = true;
    openCart();
  });

  backdrop.hidden = false;
}

function addToCart(sku, qty) {
  const p = findProduct(sku);
  if (!p) return;
  const existing = state.cart.find((item) => item.sku === sku);
  const nextQty = (existing ? existing.qty : 0) + qty;
  if (nextQty > p.stock_qty) {
    showToast(`Only ${p.stock_qty} available for ${p.title}`);
    return;
  }
  if (existing) {
    existing.qty = nextQty;
  } else {
    state.cart.push({ sku, qty });
  }
  saveCart();
  renderCart();
  showToast(`Added ${p.title} to bag`);
}

function updateCartQty(sku, delta) {
  const idx = state.cart.findIndex((i) => i.sku === sku);
  if (idx === -1) return;
  const p = findProduct(sku);
  const next = state.cart[idx].qty + delta;
  if (next <= 0) {
    state.cart.splice(idx, 1);
  } else if (p && next > p.stock_qty) {
    showToast(`Studio stock limit (${p.stock_qty}) reached`);
    return;
  } else {
    state.cart[idx].qty = next;
  }
  saveCart();
  renderCart();
}

function computeTotals() {
  let subtotal = 0;
  let vat = 0;
  for (const entry of state.cart) {
    const p = findProduct(entry.sku);
    if (!p) continue;
    const line = p.price_cents * entry.qty;
    subtotal += line;
    vat += Math.round((line * p.vat_rate_bps) / (10000 + p.vat_rate_bps));
  }

  let discount = 0;
  if (state.promoCode && PROMOS[state.promoCode]) {
    const promo = PROMOS[state.promoCode];
    if (subtotal >= promo.minOrderCents) {
      discount =
        promo.kind === "PERCENT"
          ? Math.round((subtotal * promo.value) / 100)
          : Math.min(promo.value, subtotal);
    }
  }

  const afterDiscount = Math.max(0, subtotal - discount);
  const zone =
    (state.catalog.shipping_zones || []).find((z) => z.zone_code === state.zoneCode) ||
    state.catalog.shipping_zones[0];
  const shipping =
    state.cart.length === 0 || afterDiscount >= zone.free_over_cents ? 0 : zone.base_rate_cents;

  return {
    subtotal,
    discount,
    afterDiscount,
    shipping,
    vat,
    total: afterDiscount + shipping,
    zone
  };
}

function renderCart() {
  const totalCount = state.cart.reduce((s, i) => s + i.qty, 0);
  document.getElementById("cartCount").textContent = String(totalCount);

  const itemsEl = document.getElementById("cartItems");
  const footerEl = document.getElementById("cartFooter");

  if (state.cart.length === 0) {
    itemsEl.innerHTML = `<div class="cart-empty">Your bag is currently empty.<br/>Explore the studio collection to begin.</div>`;
    footerEl.style.display = "none";
  } else {
    footerEl.style.display = "block";
    itemsEl.innerHTML = state.cart
      .map((entry) => {
        const p = findProduct(entry.sku);
        if (!p) return "";
        return `
        <div class="cart-item">
          <div class="cart-item-thumb">${productSvg(p)}</div>
          <div>
            <strong style="font-size:0.9rem;">${p.title}</strong>
            <div style="font-size:0.78rem;color:var(--muted);">${formatCents(p.price_cents)} each</div>
            <div class="qty-controls">
              <button class="qty-btn" type="button" data-dec="${p.sku}">-</button>
              <span style="font-size:0.84rem;min-width:18px;text-align:center;">${entry.qty}</span>
              <button class="qty-btn" type="button" data-inc="${p.sku}">+</button>
            </div>
          </div>
          <div style="font-family:var(--font-mono);font-size:0.86rem;font-weight:600;">
            ${formatCents(p.price_cents * entry.qty)}
          </div>
        </div>`;
      })
      .join("");

    itemsEl.querySelectorAll("[data-dec]").forEach((b) =>
      b.addEventListener("click", () => updateCartQty(b.getAttribute("data-dec"), -1))
    );
    itemsEl.querySelectorAll("[data-inc]").forEach((b) =>
      b.addEventListener("click", () => updateCartQty(b.getAttribute("data-inc"), 1))
    );
  }

  const totals = computeTotals();
  document.getElementById("summarySubtotal").textContent = formatCents(totals.subtotal);
  document.getElementById("summaryShipping").textContent =
    totals.shipping === 0 && state.cart.length > 0 ? "Free" : formatCents(totals.shipping);
  document.getElementById("summaryVat").textContent = formatCents(totals.vat);
  document.getElementById("summaryTotal").textContent = formatCents(totals.total);

  const discountRow = document.getElementById("discountRow");
  if (totals.discount > 0) {
    discountRow.hidden = false;
    document.getElementById("summaryDiscount").textContent = formatCents(-totals.discount);
  } else {
    discountRow.hidden = true;
  }

  const remaining = Math.max(0, totals.zone.free_over_cents - totals.afterDiscount);
  const pct = Math.min(100, Math.round((totals.afterDiscount / totals.zone.free_over_cents) * 100));
  document.getElementById("shippingMeterFill").style.width = `${pct}%`;
  document.getElementById("shippingMeterText").textContent =
    remaining === 0 && state.cart.length > 0
      ? `Unlocked free ${totals.zone.label} shipping (${totals.zone.eta_days})`
      : `Add ${formatCents(remaining)} for free ${totals.zone.label} shipping`;
}

function openCart() {
  document.getElementById("cartBackdrop").hidden = false;
  document.getElementById("cartDrawer").classList.add("open");
  document.getElementById("cartDrawer").setAttribute("aria-hidden", "false");
}

function closeCart() {
  document.getElementById("cartBackdrop").hidden = true;
  document.getElementById("cartDrawer").classList.remove("open");
  document.getElementById("cartDrawer").setAttribute("aria-hidden", "true");
}

function renderShippingZones() {
  const select = document.getElementById("zoneSelect");
  select.innerHTML = (state.catalog.shipping_zones || [])
    .map(
      (z) =>
        `<option value="${z.zone_code}">${z.label} (${formatCents(z.base_rate_cents)})</option>`
    )
    .join("");
  select.value = state.zoneCode;
}

async function hydrateCatalog() {
  try {
    const res = await fetch("catalog.json", { cache: "no-store" });
    if (res.ok) {
      const data = await res.json();
      if (data && Array.isArray(data.products) && data.products.length > 0) {
        state.catalog = data;
      }
    }
  } catch {
    // file:// preview uses FALLBACK_CATALOG seamlessly
  }
  document.getElementById("shopName").textContent = state.catalog.shop_name || "ATELIER NORD";
  document.getElementById("shopTagline").textContent =
    state.catalog.tagline || FALLBACK_CATALOG.tagline;
  renderShippingZones();
  renderCategories();
  renderProducts();
  renderCart();
}

document.addEventListener("DOMContentLoaded", () => {
  hydrateCatalog();

  document.getElementById("searchInput").addEventListener("input", (e) => {
    state.search = e.target.value;
    renderProducts();
  });

  document.addEventListener("keydown", (e) => {
    if (e.key === "/" && document.activeElement.tagName !== "INPUT" && document.activeElement.tagName !== "TEXTAREA") {
      e.preventDefault();
      document.getElementById("searchInput").focus();
    }
    if (e.key === "Escape") {
      document.getElementById("productModalBackdrop").hidden = true;
      document.getElementById("checkoutModalBackdrop").hidden = true;
      closeCart();
    }
  });

  document.getElementById("sortSelect").addEventListener("change", (e) => {
    state.sort = e.target.value;
    renderProducts();
  });

  document.getElementById("themeToggle").addEventListener("click", () => {
    const root = document.documentElement;
    root.setAttribute("data-theme", root.getAttribute("data-theme") === "dark" ? "light" : "dark");
  });

  document.getElementById("cartButton").addEventListener("click", openCart);
  document.getElementById("closeCart").addEventListener("click", closeCart);
  document.getElementById("cartBackdrop").addEventListener("click", closeCart);

  document.getElementById("closeProductModal").addEventListener("click", () => {
    document.getElementById("productModalBackdrop").hidden = true;
  });

  document.getElementById("heroQuickFeatured").addEventListener("click", () => {
    const first = (state.catalog.products || [])[0];
    if (first) openProductModal(first.sku);
  });

  document.getElementById("zoneSelect").addEventListener("change", (e) => {
    state.zoneCode = e.target.value;
    renderCart();
  });

  document.getElementById("promoForm").addEventListener("submit", (e) => {
    e.preventDefault();
    const raw = document.getElementById("promoInput").value.trim().toUpperCase();
    const feedback = document.getElementById("promoFeedback");
    const totals = computeTotals();
    if (!raw) {
      state.promoCode = null;
      feedback.textContent = "";
      renderCart();
      return;
    }
    const promo = PROMOS[raw];
    if (!promo) {
      feedback.className = "promo-feedback error";
      feedback.textContent = `Unknown code "${raw}". Try WELCOME10 or ATELIER15.`;
      return;
    }
    if (totals.subtotal < promo.minOrderCents) {
      feedback.className = "promo-feedback error";
      feedback.textContent = `Code ${raw} requires at least ${formatCents(promo.minOrderCents)}.`;
      return;
    }
    state.promoCode = raw;
    feedback.className = "promo-feedback";
    feedback.textContent = `Applied ${raw}`;
    renderCart();
  });

  document.getElementById("checkoutButton").addEventListener("click", () => {
    const totals = computeTotals();
    document.getElementById("checkoutTotalDisplay").textContent = formatCents(totals.total);
    document.getElementById("checkoutForm").hidden = false;
    document.getElementById("orderConfirmation").hidden = true;
    closeCart();
    document.getElementById("checkoutModalBackdrop").hidden = false;
  });

  document.getElementById("closeCheckoutModal").addEventListener("click", () => {
    document.getElementById("checkoutModalBackdrop").hidden = true;
  });

  document.getElementById("checkoutForm").addEventListener("submit", async (e) => {
    e.preventDefault();
    const name = document.getElementById("custName").value.trim();
    const email = document.getElementById("custEmail").value.trim();
    const address = document.getElementById("custAddress").value.trim();
    const itemFlags = state.cart.map((i) => `--item "${i.sku}:${i.qty}"`).join(" ");
    const promoFlag = state.promoCode ? ` --promo "${state.promoCode}"` : "";
    const cmd = `webshopcraft order-create --name "${name}" --email "${email}" --address "${address}" --zone "${state.zoneCode}"${promoFlag} ${itemFlags}`;

    let outputText = cmd;
    try {
      const res = await fetch("/api/orders", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          customer_name: name,
          customer_email: email,
          shipping_address: address,
          zone_code: state.zoneCode,
          promo_code: state.promoCode,
          items: state.cart.map((i) => ({ sku: i.sku, qty: i.qty }))
        })
      });
      if (res.ok) {
        const order = await res.json();
        outputText = `Order ${order.order_no} created (${order.status}) -- Total ${formatCents(order.total_cents)}\nBus event channel.stock_reserved emitted to stockcraft.`;
        hydrateCatalog();
      }
    } catch {
      // Static/file:// mode falls back to CLI command preview
    }

    document.getElementById("orderCommandOutput").textContent = outputText;
    document.getElementById("checkoutForm").hidden = true;
    document.getElementById("orderConfirmation").hidden = false;
  });

  document.getElementById("copyOrderCmd").addEventListener("click", async () => {
    const text = document.getElementById("orderCommandOutput").textContent;
    try {
      await navigator.clipboard.writeText(text);
      showToast("CLI order command copied");
    } catch {
      showToast("Select and copy the command above");
    }
  });

  document.getElementById("doneOrderBtn").addEventListener("click", () => {
    state.cart = [];
    state.promoCode = null;
    saveCart();
    renderCart();
    document.getElementById("checkoutModalBackdrop").hidden = true;
    showToast("Bag cleared -- ready for next order");
  });
});
