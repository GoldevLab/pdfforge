(() => {
  const boot = () => {
    const root = document.getElementById("pdf-app");
    if (!root || root.dataset.ready === "1") return;
    root.dataset.ready = "1";
    const tool = root.dataset.tool || "compress";
    const endpoint = root.dataset.endpoint || "/api/compress";
    const idleLabel = root.dataset.idle || "Run";
    const busyText = root.dataset.busy || "Working…";
    const emptyMsg = root.dataset.empty || "Drop or choose a PDF first.";
    const form = root.querySelector("[data-form]");
    const drop = root.querySelector("[data-drop]");
    const input = root.querySelector("[data-file]");
    const nameEl = root.querySelector("[data-filename]");
    const listEl = root.querySelector("[data-file-list]");
    const err = root.querySelector("[data-error]");
    const result = root.querySelector("[data-result]");
    const submit = root.querySelector("[data-submit]");
    const download = result?.querySelector("[data-download]");
    const preview = result?.querySelector("[data-preview]");
    const textEl = result?.querySelector("[data-text]");
    let files = [];

    const csrfHeaders = () => {
      const headers = {};
      try {
        const tok = JSON.parse(document.getElementById("resuma-state")?.textContent || "{}").csrf_token;
        if (tok) headers["x-resuma-csrf"] = tok;
      } catch (_) {}
      return headers;
    };
    const setError = (t) => {
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
      return t === "application/pdf" || /\.pdf$/i.test(name);
    };
    const pick = (list) => {
      const next = Array.from(list || []).filter(isPdf);
      if (!next.length) {
        setError("Choose a PDF (application/pdf).");
        return;
      }
      files = tool === "merge" ? next.slice(0, 8) : next.slice(0, 1);
      setError("");
      if (nameEl) {
        nameEl.textContent = files.map((f) => f.name).join(", ");
      }
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
      drop?.classList.add("has-file");
    };
    const busy = (on) => {
      form?.setAttribute("aria-busy", on ? "true" : "false");
      if (submit) {
        submit.disabled = on;
        submit.textContent = on ? busyText : idleLabel;
      }
    };

    ["dragenter", "dragover"].forEach((ev) => drop?.addEventListener(ev, (e) => {
      e.preventDefault();
      drop.classList.add("is-over");
    }));
    drop?.addEventListener("dragleave", (e) => {
      if (!drop.contains(e.relatedTarget)) drop.classList.remove("is-over");
    });
    drop?.addEventListener("drop", (e) => {
      e.preventDefault();
      drop.classList.remove("is-over");
      pick(e.dataTransfer?.files);
    });
    input?.addEventListener("change", () => pick(input.files));

    form?.addEventListener("submit", async (e) => {
      e.preventDefault();
      if (!files.length) {
        setError(emptyMsg);
        return;
      }
      if (tool === "merge" && files.length < 2) {
        setError("Drop at least two PDFs.");
        return;
      }
      setError("");
      if (result) result.hidden = true;
      busy(true);
      try {
        const body = new FormData();
        if (tool === "merge") {
          files.forEach((f) => body.append("file", f));
        } else {
          body.append("file", files[0]);
        }
        const range = root.querySelector("[data-range]")?.value || "";
        const mode = root.querySelector("[data-mode]")?.value || "";
        const dpi = root.querySelector("[data-dpi]")?.value || "";
        if (range) body.append("range", range);
        if (mode) body.append("mode", mode);
        if (dpi) body.append("dpi", dpi);
        const r = await fetch(endpoint, { method: "POST", headers: csrfHeaders(), body });
        const data = await r.json().catch(() => ({}));
        if (!r.ok || !data.ok) {
          throw new Error(data.error || (r.status === 413 ? "File is too large." : r.status === 429 ? "Too many requests." : "Request failed."));
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
          if (data.mime === "image/jpeg" && data.url) {
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
        setError(ex.message || "Request failed.");
      } finally {
        busy(false);
      }
    });
  };

  const tryBoot = () => boot();
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", tryBoot, { once: true });
  } else {
    tryBoot();
  }
  document.addEventListener("resuma:navigate", () => {
    const root = document.getElementById("pdf-app");
    if (root) delete root.dataset.ready;
    tryBoot();
  });
})();
