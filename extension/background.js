import { SITE } from "./config.js";

chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.removeAll(() => {
    chrome.contextMenus.create({
      id: "pdfforge-compress",
      title: "Open PDF in PDFForge",
      contexts: ["link"],
      targetUrlPatterns: ["*://*/*.pdf", "*://*/*.pdf?*"],
    });
    chrome.contextMenus.create({
      id: "pdfforge-home",
      title: "Open PDFForge",
      contexts: ["page"],
    });
  });
});

chrome.contextMenus.onClicked.addListener((info) => {
  if (info.menuItemId === "pdfforge-compress") {
    chrome.tabs.create({ url: `${SITE}/pdf-compress` });
    return;
  }
  if (info.menuItemId === "pdfforge-home") {
    chrome.tabs.create({ url: `${SITE}/` });
  }
});
