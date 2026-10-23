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

    // Yangilanish xabari (Rust tomondan chaqiriladi; ilovaning o'z sahifasida ham ishlaydi).
    // Ranglar veb tizimning 1.35 tokenlaridan (--surface, --accent, ...): asosiy rang va tungi rejim o'zi mos keladi;
    // tokenlar bo'lmasa (eski server) — Klassik/Moviy qiymatlari. Shadow DOM — sahifa uslublari aralashmaydi.
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
                + 'background:var(--surface,#fff);color:var(--text-1,#131c36);border:1px solid var(--border,#e2e8f2);'
                + 'border-radius:var(--r-lg,14px);box-shadow:var(--shadow-3,0 2px 6px rgb(16 24 40/.06),0 20px 40px -12px rgb(16 24 40/.22));'
                + 'padding:16px 16px 16px 20px;font:14px/1.5 var(--font-sans,"Onest Variable",system-ui,-apple-system,"Segoe UI",Roboto,sans-serif);'
                + 'animation:in var(--dur-3,320ms) var(--ease,cubic-bezier(.2,.7,.2,1))}'
                + '.box:before{content:"";position:absolute;inset:0 auto 0 0;width:4px;background:var(--accent,#1f63f5)}'
                + '@keyframes in{from{opacity:0;transform:translateY(12px)}}'
                + '@media (prefers-reduced-motion:reduce){.box{animation:none}}'
                + '.head{display:flex;gap:12px;align-items:flex-start}'
                + '.tile{flex:none;width:40px;height:40px;border-radius:var(--r-md,10px);display:grid;place-items:center;'
                + 'background:var(--accent-soft,#eef4ff);color:var(--accent-strong,#1650d4)}'
                + '.txt{flex:1;min-width:0}'
                + '.title{font-weight:700;font-size:15px;line-height:1.3;margin:1px 0 2px}'
                + '.sub{color:var(--text-2,#4f5a73);font-size:13px}'
                + '.x{flex:none;margin:-6px -6px 0 0;width:32px;height:32px;border:0;border-radius:var(--r-sm,8px);background:transparent;'
                + 'color:var(--text-3,#586379);cursor:pointer;display:grid;place-items:center}'
                + '.x:hover{background:var(--surface-2,#f8f9fc);color:var(--text-1,#131c36)}'
                + '.notes{margin:12px 0 0;padding:8px 12px;border-radius:var(--r-sm,8px);background:var(--surface-2,#f8f9fc);'
                + 'color:var(--text-2,#4f5a73);font-size:13px;white-space:pre-line;max-height:96px;overflow:auto}'
                + '.row{display:flex;gap:8px;justify-content:flex-end;margin-top:14px}'
                + 'button.b{font:inherit;font-weight:600;font-size:14px;min-height:40px;padding:0 16px;border-radius:var(--r-md,10px);cursor:pointer;'
                + 'border:1px solid transparent;display:inline-flex;align-items:center;gap:6px}'
                + '.later{background:var(--surface,#fff);color:var(--text-1,#131c36);border-color:var(--border-strong,#cbd3e1)!important}'
                + '.later:hover{background:var(--surface-2,#f8f9fc)}'
                + '.dl{background:var(--accent,#1f63f5);color:var(--text-on-accent,#fff)}'
                + '.dl:hover{background:var(--accent-hover,#1650d4)}'
                + 'button:focus-visible{outline:2px solid var(--color-indigo-500,#3478ff);outline-offset:2px}';
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

    // Internet uzilsa — sahifa tepasida kichik ogohlantirish (veb tokenlari: --warning-*), qaytsa — yo'qoladi.
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
            + '.bar{box-sizing:border-box;display:flex;gap:10px;align-items:center;padding:10px 14px;border-radius:var(--r-md,10px);'
            + 'background:var(--warning-bg,#fffbeb);color:var(--warning-fg,#b45309);border:1px solid var(--warning-line,#fde68a);'
            + 'box-shadow:var(--shadow-3,0 8px 24px -8px rgb(16 24 40/.25));font:600 13.5px/1.4 var(--font-sans,system-ui,-apple-system,"Segoe UI",Roboto,sans-serif)}';
        const bar = document.createElement('div');
        bar.className = 'bar';
        bar.setAttribute('role', 'status');
        bar.innerHTML = '<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" '
            + 'aria-hidden="true"><path d="M2 2l20 20"/><path d="M8.5 16.5a5 5 0 0 1 7 0"/><path d="M5 12.86a10 10 0 0 1 5.17-2.69"/>'
            + '<path d="M19 12.86a10 10 0 0 0-2.01-1.48"/><path d="M12 20h.01"/></svg><span></span>';
        bar.querySelector('span').textContent = cfg.offline || 'Internet aloqasi yo\'q';
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

    // Murojaat yuborish (POST /api/v1/public/support, server 1.38): veb sahifa kontekstga app=web yozadi — kompyuter
    // ilovasida platforma (windows/macos/linux) va ilova versiyasi qo'yiladi. JSON va FormData (rasm bilan) ikkalasi ham.
    const SUPPORT_PATH = /^\/api\/v1\/public\/support\/?$/;
    function isSupportPost(method, url) {
        try {
            const u = new URL(String(url), location.href);
            return String(method || 'GET').toUpperCase() === 'POST' && u.origin === location.origin && SUPPORT_PATH.test(u.pathname);
        } catch (e) {
            return false;
        }
    }
    function desktopContext() {
        const ctx = { app: cfg.platform || 'desktop', app_version: cfg.version || '' };
        Object.keys(ctx).forEach(function (k) { if (!ctx[k]) delete ctx[k]; });
        return ctx;
    }
    function withContext(body) {
        try {
            const ctx = desktopContext();
            if (typeof FormData !== 'undefined' && body instanceof FormData) {
                Object.keys(ctx).forEach(function (k) { body.set('context[' + k + ']', ctx[k]); });
                return body;
            }
            if (typeof body === 'string' && body.charAt(0) === '{') {
                const data = JSON.parse(body);
                data.context = Object.assign({}, data.context && typeof data.context === 'object' ? data.context : {}, ctx);
                return JSON.stringify(data);
            }
        } catch (e) { /* o'zgarmagan holda yuboriladi */ }
        return body;
    }
    window.__eduxSupportBody = withContext;
    if (window.XMLHttpRequest) {
        const xhrOpen = XMLHttpRequest.prototype.open;
        const xhrSend = XMLHttpRequest.prototype.send;
        XMLHttpRequest.prototype.open = function (method, url) {
            this.__eduxSupport = isSupportPost(method, url);
            return xhrOpen.apply(this, arguments);
        };
        XMLHttpRequest.prototype.send = function (body) {
            return xhrSend.call(this, this.__eduxSupport ? withContext(body) : body);
        };
    }
    if (window.fetch) {
        const nativeFetch = window.fetch;
        window.fetch = function (input, init) {
            const url = typeof input === 'string' || input instanceof URL ? input : input && input.url;
            const method = (init && init.method) || (input && typeof input === 'object' && input.method) || 'GET';
            if (init && init.body !== undefined && isSupportPost(method, url)) {
                return nativeFetch.call(this, input, Object.assign({}, init, { body: withContext(init.body) }));
            }
            return nativeFetch.apply(this, arguments);
        };
    }

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
