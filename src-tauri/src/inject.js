// EduX kompyuter ilovasi: veb ilova (server sahifasi) ichiga qo'shiladigan ko'prik.
// Yuklab olish (blob/data), chop etish, bildirishnomalar, tashqi havolalar va yangilanish banneri.
// `window.__EDUX_DESKTOP__` = {platform, version, offline} — veb uni o'qiydi: «Yuklab olish» tugmalari ilova ichida
// yashiriladi, murojaat kontekstiga ilova versiyasi yoziladi.
(function () {
    if (window.__eduxDesktop) return;
    window.__eduxDesktop = true;
    const cfg = window.__EDUX_DESKTOP__ || {};

    function invoke(cmd, args, options) {
        const internals = window.__TAURI_INTERNALS__;
        if (!internals) return Promise.reject(new Error('ipc unavailable'));
        return internals.invoke(cmd, args, options);
    }

    // Yangilanish xabari (Rust tomondan chaqiriladi; ilovaning o'z sahifasida ham ishlaydi).
    // Ranglar veb tizimning 1.41 «Koshin» tokenlaridan (--raised, --ink, --accent, --font-display ...): asosiy rang,
    // shablon va tungi rejim o'zi mos keladi; tokenlar bo'lmasa (eski server) — eski nomlar, keyin Koshin/Feruza qiymatlari.
    // Belgi — to'liq rangli plitka, muhr kabi bosiladi (veb toast); chap chiziq (1.40 hoshiya) yo'q.
    // Shadow DOM — sahifa uslublari aralashmaydi.
    window.__eduxUpdate = function (info) {
        const show = function () {
            if (document.getElementById('edux-update-banner')) return;
            const host = document.createElement('div');
            host.id = 'edux-update-banner';
            host.style.cssText = 'position:fixed;right:16px;bottom:calc(16px + var(--bottom-nav-h, 0px));z-index:2147483647';
            const root = host.attachShadow ? host.attachShadow({ mode: 'open' }) : host;
            const css = ''
                + ':host{all:initial}'
                + '.box{position:relative;box-sizing:border-box;width:360px;max-width:calc(100vw - 32px);overflow:hidden;'
                + 'background:var(--raised,var(--surface,#fff));color:var(--ink,var(--text-1,#111c3a));border:1px solid var(--line,var(--border,#dde5ec));'
                + 'border-radius:var(--r-lg,14px);box-shadow:var(--shadow-3,0 4px 10px rgb(17 28 58/.06),0 28px 56px -16px rgb(17 28 58/.32));'
                + 'padding:16px;font:14px/1.5 var(--font-sans,"Onest Variable","Commissioner Variable",system-ui,-apple-system,"Segoe UI",Roboto,sans-serif);'
                + 'animation:in var(--dur-3,280ms) var(--ease-spring,cubic-bezier(.34,1.5,.64,1))}'
                + '@keyframes in{from{opacity:0;transform:translateY(var(--motion-shift,8px))}}'
                + '@keyframes stamp{0%{opacity:0;transform:scale(1.5) rotate(-14deg)}55%{opacity:1;transform:scale(.94) rotate(2deg)}100%{opacity:1;transform:none}}'
                + '@media (prefers-reduced-motion:reduce){.box,.tile{animation:none}}'
                + '.head{display:flex;gap:12px;align-items:flex-start}'
                + '.tile{flex:none;width:40px;height:40px;border-radius:var(--r-md,10px);display:grid;place-items:center;'
                + 'background:var(--accent,#00808b);color:var(--text-on-accent,#fff);animation:stamp 520ms var(--ease-spring,cubic-bezier(.34,1.5,.64,1)) 120ms both}'
                + '.txt{flex:1;min-width:0}'
                + '.title{font-family:var(--font-display,"Commissioner Variable","Onest Variable",system-ui,sans-serif);font-weight:800;font-size:16px;'
                + 'letter-spacing:-.01em;line-height:1.3;margin:1px 0 2px}'
                + '.sub{color:var(--muted,var(--text-2,#49566d));font-size:13px}'
                + '.x{flex:none;margin:-6px -6px 0 0;width:32px;height:32px;border:0;border-radius:var(--r-sm,8px);background:transparent;'
                + 'color:var(--subtle,var(--text-3,#556378));cursor:pointer;display:grid;place-items:center}'
                + '.x:hover{background:var(--surface-2,#f6f9fb);color:var(--ink,var(--text-1,#111c3a))}'
                + '.notes{margin:12px 0 0;padding:8px 12px;border-radius:var(--r-sm,8px);background:var(--surface-2,#f6f9fb);'
                + 'color:var(--muted,var(--text-2,#49566d));font-size:13px;white-space:pre-line;max-height:96px;overflow:auto}'
                + '.row{display:flex;gap:8px;justify-content:flex-end;margin-top:14px}'
                + 'button.b{font:inherit;font-weight:650;font-size:14px;min-height:40px;padding:0 16px;border-radius:var(--r-md,10px);cursor:pointer;'
                + 'border:1px solid transparent;display:inline-flex;align-items:center;gap:6px;'
                + 'transition:transform var(--dur-2,200ms) var(--ease-spring,cubic-bezier(.34,1.5,.64,1)),background-color var(--dur-1,110ms)}'
                + '.later{background:var(--surface,#fff);color:var(--ink,var(--text-1,#111c3a));border-color:var(--line-strong,var(--border-strong,#c8d2dc))!important}'
                + '.later:hover{background:var(--surface-2,#f6f9fb)}'
                + '.dl{background:var(--accent,#00808b);color:var(--text-on-accent,#fff);box-shadow:inset 0 1px 0 rgb(255 255 255/.18)}'
                + '.dl:hover{background:var(--accent-hover,#006a73)}'
                + 'button:active{transform:scale(var(--press-scale,.96))}'
                + 'button:focus-visible{outline:2px solid var(--focus-ring,#00808b);outline-offset:2px}';
            const arrow = '<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" '
                + 'stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 4v11"/><path d="m7 10 5 5 5-5"/><path d="M5 20h14"/></svg>';
            const cross = '<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" '
                + 'stroke-linecap="round" aria-hidden="true"><path d="M6 6l12 12M18 6 6 18"/></svg>';
            const style = document.createElement('style');
            style.textContent = css;
            const box = document.createElement('div');
            box.className = 'box';
            box.setAttribute('role', 'status');
            box.setAttribute('aria-live', 'polite');
            box.innerHTML = '<div class="head"><div class="tile">' + arrow + '</div><div class="txt"><div class="title"></div>'
                + '<div class="sub"></div></div><button type="button" class="x">' + cross + '</button></div>'
                + '<div class="notes" hidden></div><div class="row"><button type="button" class="b later"></button>'
                + '<button type="button" class="b dl">' + arrow.replace(/20/g, '18') + '<span></span></button></div>';
            box.querySelector('.title').textContent = info.title;
            box.querySelector('.sub').textContent = info.text || ('EduX ' + info.version);
            if (info.notes) {
                const notes = box.querySelector('.notes');
                notes.hidden = false;
                notes.textContent = info.notes;
            }
            const close = function () { host.remove(); };
            const x = box.querySelector('.x');
            x.setAttribute('aria-label', info.close || info.later);
            x.title = info.close || info.later;
            x.onclick = close;
            const later = box.querySelector('.later');
            later.textContent = info.later;
            later.onclick = close;
            const dl = box.querySelector('.dl');
            dl.querySelector('span').textContent = info.download;
            dl.onclick = function () { invoke('open_external', { url: info.url }).catch(function () {}); close(); };
            root.appendChild(style);
            root.appendChild(box);
            document.body.appendChild(host);
        };
        if (document.body) show(); else document.addEventListener('DOMContentLoaded', show);
    };

    // Ilovaning o'z sahifalari (server manzili, xato) — qolgan ko'prik kerak emas.
    if (location.protocol === 'tauri:' || location.hostname === 'tauri.localhost') return;
    if (!/^https?:$/.test(location.protocol)) return;

    function isSameServer(url) {
        try {
            const u = new URL(url, location.href);
            return u.origin === location.origin;
        } catch (e) {
            return false;
        }
    }

    function openUrl(url) {
        const abs = new URL(url, location.href).href;
        if (/^https?:/.test(abs) && isSameServer(abs)) {
            return invoke('open_window', { url: abs }).catch(function () { location.href = abs; });
        }
        return invoke('open_external', { url: abs }).catch(function () {});
    }

    // Blob/data fayllarni native «Saqlash» oynasi orqali saqlash.
    async function saveUrl(href, name) {
        try {
            const res = await fetch(href);
            const buf = new Uint8Array(await res.arrayBuffer());
            await invoke('save_file', buf, { headers: { 'x-edux-filename': encodeURIComponent(name || 'fayl') } });
        } catch (e) {
            console.error('EduX: faylni saqlab bo\'lmadi', e);
        }
    }

    function fileNameFrom(a, url) {
        const attr = a.getAttribute('download');
        if (attr) return attr;
        try {
            const last = new URL(url).pathname.split('/').pop();
            if (last) return decodeURIComponent(last);
        } catch (e) { /* bo'sh */ }
        return 'fayl';
    }

    /** true — havola shu yerda qayta ishlandi (brauzer odatiy harakati to'xtatiladi). */
    function handleAnchor(a) {
        const raw = a.getAttribute('href');
        if (!raw || raw.charAt(0) === '#' || /^javascript:/i.test(raw)) return false;
        let url;
        try {
            url = new URL(raw, location.href);
        } catch (e) {
            return false;
        }
        if (url.protocol === 'blob:' || url.protocol === 'data:') {
            saveUrl(url.href, fileNameFrom(a, url.href));
            return true;
        }
        if (url.protocol === 'http:' || url.protocol === 'https:') {
            if (!isSameServer(url.href)) {
                invoke('open_external', { url: url.href }).catch(function () {});
                return true;
            }
            const target = (a.getAttribute('target') || '').toLowerCase();
            if (target && target !== '_self' && target !== '_top' && target !== '_parent' && !a.hasAttribute('download')) {
                openUrl(url.href);
                return true;
            }
            return false; // o'sha server: ilova ichida (yuklab olish bo'lsa — native dialog)
        }
        // mailto:, tel:, tg:, whatsapp: va h.k. — tizim ilovasi
        invoke('open_external', { url: url.href }).catch(function () {});
        return true;
    }

    document.addEventListener('click', function (e) {
        if (e.defaultPrevented || e.button !== 0) return;
        const a = e.target && e.target.closest ? e.target.closest('a[href]') : null;
        if (!a) return;
        if (handleAnchor(a)) e.preventDefault();
    }, true);

    // DOM ga qo'shilmagan <a> ga .click() (ba'zi kutubxonalar shunday yuklab oladi).
    const nativeClick = HTMLAnchorElement.prototype.click;
    HTMLAnchorElement.prototype.click = function () {
        if (!this.isConnected && this.getAttribute('href') && handleAnchor(this)) return;
        return nativeClick.call(this);
    };

    // window.open: tashqi manzil — brauzerda, o'sha server — yangi ilova oynasida.
    window.open = function (url, target) {
        const t = (target || '').toLowerCase();
        if (url && (t === '_self' || t === '_top' || t === '_parent')) {
            location.href = url;
            return window;
        }
        if (url && url !== 'about:blank') {
            openUrl(url);
            return null;
        }
        // Bo'sh oyna (keyin location beriladi — masalan Telegram orqali kirish).
        const popup = { closed: false, opener: null };
        const go = function (v) { if (v) openUrl(String(v)); };
        popup.location = {
            set href(v) { go(v); },
            get href() { return 'about:blank'; },
            assign: go,
            replace: go,
            toString: function () { return 'about:blank'; },
        };
        popup.close = function () { popup.closed = true; };
        popup.focus = function () {};
        popup.blur = function () {};
        popup.postMessage = function () {};
        popup.document = { write: function () {}, writeln: function () {}, open: function () {}, close: function () {}, title: '' };
        return popup;
    };

    // window.print: macOS/Linux da native chop etish oynasi (Windows WebView2 o'zi qo'llab-quvvatlaydi).
    if (cfg.platform === 'macos' || cfg.platform === 'linux') {
        window.print = function () {
            invoke('print_page').catch(function () {});
        };
    }

    // Internet uzilsa — sahifa tepasida kichik ogohlantirish (veb tokenlari: --warning-*, belgi shafran doirada), qaytsa — yo'qoladi.
    // Aloqasiz F5 / Ctrl+R sahifani WebView'ning «sahifa ochilmadi» oynasiga almashtirib yubormasin.
    let offlineHost = null;
    function showOffline() {
        if (offlineHost || !document.body) return;
        offlineHost = document.createElement('div');
        offlineHost.id = 'edux-offline-banner';
        offlineHost.style.cssText = 'position:fixed;left:50%;top:12px;transform:translateX(-50%);z-index:2147483647;max-width:calc(100vw - 32px)';
        const root = offlineHost.attachShadow ? offlineHost.attachShadow({ mode: 'open' }) : offlineHost;
        const style = document.createElement('style');
        style.textContent = ':host{all:initial}'
            + '.bar{position:relative;overflow:hidden;box-sizing:border-box;display:flex;gap:10px;align-items:center;padding:6px 16px 6px 6px;border-radius:999px;'
            + 'background:var(--warning-bg,#fffbeb);color:var(--warning-fg,#bb4d00);border:1px solid var(--warning-line,#fee685);'
            + 'box-shadow:var(--shadow-3,0 4px 10px rgb(17 28 58/.06),0 28px 56px -16px rgb(17 28 58/.32));'
            + 'font:600 13.5px/1.4 var(--font-sans,"Onest Variable","Commissioner Variable",system-ui,-apple-system,"Segoe UI",Roboto,sans-serif)}'
            + '.ic{flex:none;width:28px;height:28px;border-radius:999px;display:grid;place-items:center;'
            + 'background:var(--warning-solid,var(--k-shafran,#f2a93b));color:var(--k-shafran-ink,#2a1a00)}';
        const bar = document.createElement('div');
        bar.className = 'bar';
        bar.setAttribute('role', 'status');
        bar.innerHTML = '<span class="ic"><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" '
            + 'aria-hidden="true"><path d="M2 2l20 20"/><path d="M8.5 16.5a5 5 0 0 1 7 0"/><path d="M5 12.86a10 10 0 0 1 5.17-2.69"/>'
            + '<path d="M19 12.86a10 10 0 0 0-2.01-1.48"/><path d="M12 20h.01"/></svg></span><span></span>';
        bar.querySelector('span:last-child').textContent = cfg.offline || 'Internet aloqasi yo\'q';
        root.appendChild(style);
        root.appendChild(bar);
        document.body.appendChild(offlineHost);
    }
    function hideOffline() {
        if (offlineHost) offlineHost.remove();
        offlineHost = null;
    }
    window.addEventListener('offline', showOffline);
    window.addEventListener('online', hideOffline);
    if (navigator.onLine === false) {
        if (document.body) showOffline(); else document.addEventListener('DOMContentLoaded', showOffline);
    }
    document.addEventListener('keydown', function (e) {
        const reload = e.key === 'F5' || ((e.ctrlKey || e.metaKey) && (e.key === 'r' || e.key === 'R'));
        if (reload && navigator.onLine === false) {
            e.preventDefault();
            showOffline();
        }
    }, true);

    // Web Notification → OS bildirishnomasi.
    function EduxNotification(title, options) {
        options = options || {};
        this.title = String(title || '');
        this.body = String(options.body || '');
        this.tag = options.tag || '';
        this.data = options.data;
        this.onclick = null;
        this.onclose = null;
        this.onerror = null;
        this.onshow = null;
        const self = this;
        invoke('notify_cmd', { title: this.title, body: this.body })
            .then(function () { if (typeof self.onshow === 'function') self.onshow(); })
            .catch(function () { if (typeof self.onerror === 'function') self.onerror(); });
    }
    EduxNotification.prototype.close = function () {};
    EduxNotification.prototype.addEventListener = function (type, fn) { this['on' + type] = fn; };
    EduxNotification.prototype.removeEventListener = function () {};
    Object.defineProperty(EduxNotification, 'permission', { get: function () { return 'granted'; } });
    EduxNotification.requestPermission = function (cb) {
        if (typeof cb === 'function') cb('granted');
        return Promise.resolve('granted');
    };
    EduxNotification.maxActions = 0;
    try {
        window.Notification = EduxNotification;
    } catch (e) { /* bo'sh */ }

    // Murojaat konteksti (1.2.1): veb (server 1.38) `window.__EDUX_DESKTOP__` dan ilova va versiyani o'zi yozadi —
    // ilova so'rovlarga aralashmaydi (ilgari fetch/XHR orqali qo'shilardi — endi ikki marta yozilmasin).

    // Ilova ichida sahifaga o'tish (deep link, Yordam markazi): Vue Router bo'lsa — SPA ichida.
    window.__eduxNavigate = function (path) {
        try {
            const root = document.querySelector('#app');
            const router = root && root.__vue_app__ && root.__vue_app__.config.globalProperties.$router;
            if (router) {
                router.push(path).catch(function () { location.assign(path); });
                return;
            }
        } catch (e) { /* bo'sh */ }
        location.assign(path);
    };
})();
