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
