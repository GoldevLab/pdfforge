(() => {
  let files = [];
  let abortCtl = null;
  let toolKey = "";

  const rootEl = () => document.getElementById("pdf-app");

  const syncTool = () => {
    const el = rootEl();
    const key = el ? `${el.dataset.tool || ""}|${el.dataset.endpoint || ""}` : "";
    if (key !== toolKey) {
      files = [];
      abortCtl?.abort();
      abortCtl = null;
      toolKey = key;
    }
    return el;
  };

  const csrfHeaders = () => {
    const headers = {};
    try {
      const tok = JSON.parse(document.getElementById("resuma-state")?.textContent || "{}").csrf_token;
      if (tok) headers["x-resuma-csrf"] = tok;
    } catch (_) {}
    return headers;
  };

  const setError = (t) => {
    const err = rootEl()?.querySelector("[data-error]");
    if (!err) return;
    err.textContent = t || "";
    err.hidden = !t;
  };

  const human = (n) => {
    if (n < 1024) return n + " B";
    if (n < 1024 * 1024) return Math.round(n / 1024) + " KB";
    return (n / (1024 * 1024)).toFixed(2) + " MB";
  };

  const isPdf = (f) => {
    const t = (f.type || "").toLowerCase();
    const name = f.name || "";
    return t === "application/pdf" || t === "application/x-pdf" || /\.pdf$/i.test(name);
  };
  const isImage = (f) => {
    const t = (f.type || "").toLowerCase();
    const name = f.name || "";
    return t === "image/jpeg" || t === "image/jpg" || t === "image/pjpeg" || t === "image/png"
      || /\.(jpe?g|png)$/i.test(name);
  };
  const manyFiles = (tool) => tool === "merge" || tool === "from-jpg";

  const busy = (on) => {
    const el = rootEl();
    const form = el?.querySelector("[data-form]");
    const submit = el?.querySelector("[data-submit]");
    form?.setAttribute("aria-busy", on ? "true" : "false");
    if (submit) {
      submit.disabled = on;
      submit.textContent = on ? (el.dataset.busy || "Working…") : (el.dataset.idle || "Run");
    }
  };

  const pick = (list) => {
    const el = syncTool();
    if (!el) return;
    const tool = el.dataset.tool || "compress";
    const acceptImages = tool === "from-jpg";
    const next = Array.from(list || []).filter(acceptImages ? isImage : isPdf);
    if (!next.length) {
      setError(acceptImages ? "Choose a JPG or PNG." : "Choose a PDF (application/pdf).");
      return;
    }
    if (next.some((f) => f.size > 20 * 1024 * 1024)) {
      setError("A file is over 20 MB.");
      return;
    }
    files = manyFiles(tool) ? next.slice(0, acceptImages ? 12 : 8) : next.slice(0, 1);
    setError("");
    const result = el.querySelector("[data-result]");
    if (result) result.hidden = true;
    const nameEl = el.querySelector("[data-filename]");
    if (nameEl) nameEl.textContent = files.map((f) => f.name).join(", ");
    const listEl = el.querySelector("[data-file-list]");
    if (listEl) {
      listEl.hidden = files.length < 2;
      listEl.replaceChildren(
        ...files.map((f) => {
          const li = document.createElement("li");
          li.textContent = f.name + " · " + human(f.size);
          return li;
        })
      );
    }
    el.querySelector("[data-drop]")?.classList.add("has-file");
  };

  const dropOf = (t) => {
    const el = t instanceof Element ? t.closest("[data-drop]") : null;
    const root = rootEl();
    return el && root?.contains(el) ? el : null;
  };

  document.addEventListener("dragenter", (e) => {
    const drop = dropOf(e.target);
    if (!drop) return;
    e.preventDefault();
    drop.classList.add("is-over");
  });
  document.addEventListener("dragover", (e) => {
    const drop = dropOf(e.target);
    if (!drop) return;
    e.preventDefault();
    drop.classList.add("is-over");
  });
  document.addEventListener("dragleave", (e) => {
    const drop = dropOf(e.target);
    if (!drop) return;
    if (!drop.contains(e.relatedTarget)) drop.classList.remove("is-over");
  });
  document.addEventListener("drop", (e) => {
    const drop = dropOf(e.target);
    if (!drop) return;
    e.preventDefault();
    drop.classList.remove("is-over");
    pick(e.dataTransfer?.files);
  });
  document.addEventListener("change", (e) => {
    const input = e.target;
    if (!(input instanceof HTMLInputElement) || !input.hasAttribute("data-file")) return;
    if (!rootEl()?.contains(input)) return;
    pick(input.files);
  });
  document.addEventListener("click", (e) => {
    const a = e.target instanceof Element ? e.target.closest("[data-download]") : null;
    if (!a || !rootEl()?.contains(a)) return;
    if (!a.getAttribute("href")) e.preventDefault();
  });
  document.addEventListener("submit", async (e) => {
    const form = e.target;
    if (!(form instanceof HTMLFormElement) || !form.hasAttribute("data-form")) return;
    const el = syncTool();
    if (!el || !el.contains(form)) return;
    e.preventDefault();
    const tool = el.dataset.tool || "compress";
    const endpoint = el.dataset.endpoint || "/api/compress";
    const result = el.querySelector("[data-result]");
    const download = result?.querySelector("[data-download]");
    const preview = result?.querySelector("[data-preview]");
    const textEl = result?.querySelector("[data-text]");
    if (!files.length) {
      setError(el.dataset.empty || "Drop or choose a PDF first.");
      return;
    }
    if (tool === "merge" && files.length < 2) {
      setError("Drop at least two PDFs.");
      return;
    }
    if (tool === "from-jpg" && !files.length) {
      setError("Drop JPG or PNG files first.");
      return;
    }
    setError("");
    if (result) result.hidden = true;
    abortCtl?.abort();
    abortCtl = new AbortController();
    const mine = abortCtl;
    busy(true);
    try {
      const body = new FormData();
      if (tool === "merge" || tool === "from-jpg") {
        files.forEach((f) => body.append("file", f));
      } else {
        body.append("file", files[0]);
      }
      const range = el.querySelector("[data-range]")?.value || "";
      const mode = el.querySelector("[data-mode]")?.value || "";
      const dpi = el.querySelector("[data-dpi]")?.value || "";
      if (range) body.append("range", range);
      if (mode) body.append("mode", mode);
      if (dpi) body.append("dpi", dpi);
      const r = await fetch(endpoint, {
        method: "POST",
        headers: csrfHeaders(),
        body,
        credentials: "same-origin",
        signal: mine.signal,
      });
      const data = await r.json().catch(() => ({}));
      if (!r.ok || !data.ok) {
        throw new Error(
          data.error
            || (r.status === 413 ? "File is too large." : "")
            || (r.status === 408 || r.status === 504 ? "That PDF took too long. Try fewer pages." : "")
            || (r.status === 429 ? "Too many requests." : "")
            || "Request failed."
        );
      }
      if (typeof window.appEvent === "function") {
        window.appEvent("job_complete", { tool: tool });
      }
      if (result) result.hidden = false;
      const nameNode = result?.querySelector("[data-out-name]");
      const stats = result?.querySelector("[data-stats]");
      if (nameNode) nameNode.textContent = data.filename || "";
      if (stats) {
        stats.textContent = human(data.original_bytes) + " → " + human(data.result_bytes)
          + (data.pages ? " · " + data.pages + " page PDF" : "")
          + (data.extra ? " · " + data.extra : "");
      }
      if (textEl) {
        if (data.text) {
          textEl.hidden = false;
          textEl.textContent = data.text;
        } else {
          textEl.hidden = true;
          textEl.textContent = "";
        }
      }
      if (preview) {
        if ((data.mime || "").startsWith("image/jpeg") && data.url) {
          preview.hidden = false;
          preview.src = data.url;
        } else {
          preview.hidden = true;
          preview.removeAttribute("src");
        }
      }
      if (download && data.url) {
        download.href = data.url + (data.url.includes("?") ? "&" : "?") + "dl=1";
        download.setAttribute("download", data.filename || "download");
      }
    } catch (ex) {
      if (ex.name === "AbortError") return;
      setError(ex.message || "Request failed.");
    } finally {
      if (abortCtl === mine) busy(false);
    }
  });
})();

(() => {
  let stopHero = null;
  const mountPressDust = () => {
    if (typeof stopHero === "function") {
      stopHero();
      stopHero = null;
    }
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) return;
    const root = document.querySelector("[data-hero-particles]");
    if (!root) return;
    const canvas = document.createElement("canvas");
    canvas.className = "hero-particles-canvas";
    canvas.setAttribute("aria-hidden", "true");
    root.replaceChildren(canvas);
    const ctx = canvas.getContext("2d", { alpha: true });
    if (!ctx) return;

    const COUNT = window.matchMedia("(max-width: 640px)").matches ? 42 : 72;
    const motes = Array.from({ length: COUNT }, () => {
      const roll = Math.random();
      return {
        kind: roll < 0.42 ? "sheet" : roll < 0.72 ? "fiber" : "ink",
        x: Math.random(),
        y: Math.random(),
        z: 0.28 + Math.random() * 0.72,
        rot: Math.random() * Math.PI,
        spin: (Math.random() - 0.5) * 0.35,
        vx: (Math.random() - 0.5) * 0.016,
        vy: 0.01 + Math.random() * 0.026,
      };
    });

    let w = 0;
    let h = 0;
    let pointerX = 0;
    let pointerY = 0;
    let raf = 0;
    let last = 0;
    let accent = "#3b82f6";
    let ink = "#93c5fd";

    const readColors = () => {
      const cs = getComputedStyle(document.documentElement);
      accent = (cs.getPropertyValue("--accent") || "#3b82f6").trim() || "#3b82f6";
      ink = (cs.getPropertyValue("--primary") || "#93c5fd").trim() || "#93c5fd";
    };
    const host = root.parentElement || root;
    const resize = () => {
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      w = host.clientWidth;
      h = host.clientHeight;
      if (!w || !h) return;
      canvas.width = Math.floor(w * dpr);
      canvas.height = Math.floor(h * dpr);
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    };
    const onMove = (e) => {
      const r = host.getBoundingClientRect();
      if (!r.width || !r.height) return;
      pointerX = (e.clientX - r.left) / r.width - 0.5;
      pointerY = (e.clientY - r.top) / r.height - 0.5;
    };
    const tick = (t) => {
      if (!root.isConnected) {
        if (typeof stopHero === "function") stopHero();
        return;
      }
      if (document.hidden) {
        raf = 0;
        last = 0;
        return;
      }
      const dt = Math.min(0.032, last ? (t - last) / 1000 : 0.016);
      last = t;
      ctx.clearRect(0, 0, w, h);

      for (const d of motes) {
        d.x += d.vx * dt * 18 * (0.45 + d.z);
        d.y += d.vy * dt * 18 * (0.55 + d.z * 0.5);
        d.rot += d.spin * dt;

        const dx = d.x - (pointerX + 0.5);
        const dy = d.y - (pointerY + 0.5);
        const dist2 = dx * dx + dy * dy;
        if (dist2 < 0.045 && dist2 > 0.0002) {
          const f = (0.045 - dist2) * 0.09;
          d.x += dx * f;
          d.y += dy * f;
        }

        if (d.y > 1.08) {
          d.y = -0.06;
          d.x = Math.random();
        } else if (d.y < -0.08) {
          d.y = 1.05;
        }
        if (d.x > 1.08) d.x = -0.04;
        else if (d.x < -0.08) d.x = 1.04;

        const x = d.x * w;
        const y = d.y * h;
        ctx.globalAlpha = 0.28 + d.z * 0.5;
        ctx.strokeStyle = accent;
        ctx.fillStyle = d.kind === "ink" ? ink : accent;

        if (d.kind === "sheet") {
          const pw = 10 + d.z * 16;
          const ph = pw * 1.38;
          ctx.save();
          ctx.translate(x, y);
          ctx.rotate(d.rot);
          ctx.lineWidth = 0.9;
          ctx.globalAlpha *= 0.7;
          ctx.strokeRect(-pw / 2 + 2.2, -ph / 2 - 2.4, pw, ph);
          ctx.globalAlpha = 0.28 + d.z * 0.5;
          ctx.strokeRect(-pw / 2, -ph / 2, pw, ph);
          ctx.globalAlpha *= 0.35;
          ctx.fillRect(-pw / 2, -ph / 2, pw, ph);
          ctx.globalAlpha = 0.4 + d.z * 0.35;
          ctx.fillStyle = accent;
          ctx.fillRect(-pw / 2, -ph / 2, pw, Math.max(1.6, ph * 0.14));
          ctx.restore();
        } else if (d.kind === "fiber") {
          const len = 8 + d.z * 14;
          ctx.save();
          ctx.translate(x, y);
          ctx.rotate(d.rot);
          ctx.lineWidth = 0.7 + d.z * 0.5;
          ctx.beginPath();
          ctx.moveTo(-len / 2, 0);
          ctx.lineTo(len / 2, 0);
          ctx.stroke();
          ctx.restore();
        } else {
          ctx.beginPath();
          ctx.arc(x, y, 0.9 + d.z * 1.8, 0, Math.PI * 2);
          ctx.fill();
        }
      }

      raf = requestAnimationFrame(tick);
    };
    const onVis = () => {
      if (!document.hidden && !raf && root.isConnected) raf = requestAnimationFrame(tick);
    };
    const themeWatch = new MutationObserver(readColors);
    themeWatch.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["data-theme"],
    });
    readColors();
    resize();
    const sizeWatch = typeof ResizeObserver === "function" ? new ResizeObserver(resize) : null;
    sizeWatch?.observe(host);
    window.addEventListener("resize", resize, { passive: true });
    window.addEventListener("pointermove", onMove, { passive: true });
    document.addEventListener("visibilitychange", onVis);
    raf = requestAnimationFrame(tick);
    stopHero = () => {
      cancelAnimationFrame(raf);
      raf = 0;
      last = 0;
      themeWatch.disconnect();
      sizeWatch?.disconnect();
      window.removeEventListener("resize", resize);
      window.removeEventListener("pointermove", onMove);
      document.removeEventListener("visibilitychange", onVis);
      canvas.remove();
      stopHero = null;
    };
  };
  const tryHero = () => mountPressDust();
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", tryHero, { once: true });
  } else {
    tryHero();
  }
  document.addEventListener("resuma:navigate", () => requestAnimationFrame(tryHero));
})();

(() => {
  document.addEventListener("resuma:navigate", () => {
    document.documentElement.classList.remove("is-navigating");
    const heading = document.querySelector("[data-r-vt] h1");
    if (!heading) return;
    heading.setAttribute("tabindex", "-1");
    heading.focus({ preventScroll: true });
  });
  document.addEventListener("click", (e) => {
    const el = e.target instanceof Element ? e.target.closest("a[data-r-nav]") : null;
    if (el && el.getAttribute("href")) document.documentElement.classList.add("is-navigating");
  }, true);
  if (!window.__pdfVtGuard) {
    window.__pdfVtGuard = true;
    window.addEventListener("unhandledrejection", (e) => {
      const err = e.reason;
      const name = err && err.name;
      const msg = (err && (err.message || String(err))) || "";
      if (name === "AbortError" && /Transition was skipped|ViewTransition/i.test(msg)) {
        e.preventDefault();
      }
    });
  }
})();
