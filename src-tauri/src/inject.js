// EduX kompyuter ilovasi: veb ilova (server sahifasi) ichiga qo'shiladigan ko'prik.
// Yuklab olish (blob/data), chop etish, bildirishnomalar, tashqi havolalar va yangilanish banneri.
(function () {
    if (window.__eduxDesktop) return;
    window.__eduxDesktop = true;
    const cfg = window.__EDUX_DESKTOP__ || {};

    function invoke(cmd, args, options) {
        const internals = window.__TAURI_INTERNALS__;
        if (!internals) return Promise.reject(new Error('ipc unavailable'));
        return internals.invoke(cmd, args, options);
    }

    // Yangilanish banneri (Rust tomondan chaqiriladi; mahalliy sahifada ham ishlaydi).
    window.__eduxUpdate = function (info) {
        const show = function () {
            if (document.getElementById('edux-update-banner')) return;
            const bar = document.createElement('div');
            bar.id = 'edux-update-banner';
            bar.setAttribute('role', 'status');
            bar.style.cssText = 'position:fixed;right:16px;bottom:16px;z-index:2147483647;max-width:360px;'
                + 'background:#1F63F5;color:#fff;border-radius:12px;padding:14px 16px;box-shadow:0 8px 30px rgba(0,0,0,.25);'
                + 'font:14px/1.4 system-ui,-apple-system,"Segoe UI",Roboto,sans-serif';
            const title = document.createElement('div');
            title.style.cssText = 'font-weight:600;margin-bottom:4px';
            title.textContent = info.title + ' — ' + info.version;
            bar.appendChild(title);
            if (info.notes) {
                const notes = document.createElement('div');
                notes.style.cssText = 'opacity:.9;margin-bottom:8px;white-space:pre-line;max-height:120px;overflow:auto';
                notes.textContent = info.notes;
                bar.appendChild(notes);
            }
            const row = document.createElement('div');
            row.style.cssText = 'display:flex;gap:8px;justify-content:flex-end;margin-top:6px';
            const later = document.createElement('button');
            later.textContent = info.later;
            later.style.cssText = 'background:transparent;color:#fff;border:1px solid rgba(255,255,255,.6);border-radius:8px;padding:6px 12px;cursor:pointer;font:inherit';
            later.onclick = function () { bar.remove(); };
            const dl = document.createElement('button');
            dl.textContent = info.download;
            dl.style.cssText = 'background:#fff;color:#1F63F5;border:0;border-radius:8px;padding:6px 12px;cursor:pointer;font:inherit;font-weight:600';
            dl.onclick = function () { invoke('open_external', { url: info.url }).catch(function () {}); bar.remove(); };
            row.appendChild(later);
            row.appendChild(dl);
            bar.appendChild(row);
            document.body.appendChild(bar);
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
