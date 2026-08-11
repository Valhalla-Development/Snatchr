use crate::config::Config;
use axum::response::Html;

/*
 * HTTP handler for serving the download page.
 *
 * Returns an HTML page with JavaScript functionality to make download requests.
 * Shows different content based on whether web UI is enabled or disabled.
 */

#[axum::debug_handler]
pub async fn download_page() -> Html<&'static str> {
    let config = Config::from_env();

    if !config.enable_web_ui {
        return Html(
            r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Snatchr — API Only Mode</title>
    <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🎬</text></svg>">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@500;700&family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;600&display=swap" rel="stylesheet">
    <style>
        :root {
            --bg: #05050a;
            --border: rgba(255, 255, 255, 0.09);
            --text: #f4f4f8;
            --muted: rgba(235, 235, 245, 0.55);
            --violet: #8b5cf6;
            --fuchsia: #d946ef;
            --cyan: #22d3ee;
            --amber: #fbbf24;
        }
        * { margin: 0; padding: 0; box-sizing: border-box; }
        html { color-scheme: dark; }
        body {
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 24px;
            background: var(--bg);
            color: var(--text);
            font-family: 'Inter', system-ui, sans-serif;
            overflow-x: hidden;
        }
        .aurora {
            position: fixed; inset: 0; z-index: -2; overflow: hidden;
            filter: blur(90px) saturate(140%);
        }
        .blob { position: absolute; border-radius: 50%; opacity: 0.45; }
        .b1 { width: 44vw; height: 44vw; left: -10vw; top: -12vw; background: radial-gradient(circle, var(--violet), transparent 65%); animation: drift1 26s ease-in-out infinite alternate; }
        .b2 { width: 38vw; height: 38vw; right: -8vw; top: 20vh; background: radial-gradient(circle, var(--fuchsia), transparent 65%); animation: drift2 32s ease-in-out infinite alternate; }
        .b3 { width: 34vw; height: 34vw; left: 28vw; bottom: -14vw; background: radial-gradient(circle, var(--cyan), transparent 65%); animation: drift3 38s ease-in-out infinite alternate; }
        @keyframes drift1 { to { transform: translate(9vw, 7vh) scale(1.15); } }
        @keyframes drift2 { to { transform: translate(-7vw, -6vh) scale(0.9); } }
        @keyframes drift3 { to { transform: translate(-5vw, -8vh) scale(1.2); } }
        .grid-overlay {
            position: fixed; inset: 0; z-index: -1; pointer-events: none;
            background-image: radial-gradient(rgba(255, 255, 255, 0.05) 1px, transparent 1px);
            background-size: 34px 34px;
            mask-image: radial-gradient(ellipse 80% 70% at 50% 40%, black 30%, transparent 100%);
            -webkit-mask-image: radial-gradient(ellipse 80% 70% at 50% 40%, black 30%, transparent 100%);
        }
        .card {
            position: relative;
            width: 100%; max-width: 560px;
            padding: 40px 36px;
            border-radius: 24px;
            border: 1px solid var(--border);
            background: linear-gradient(180deg, rgba(255, 255, 255, 0.06), rgba(255, 255, 255, 0.02));
            backdrop-filter: blur(20px);
            -webkit-backdrop-filter: blur(20px);
            animation: rise 0.6s cubic-bezier(0.2, 0.8, 0.2, 1) both;
        }
        .card::before {
            content: ""; position: absolute; inset: -1px; border-radius: inherit; padding: 1px;
            background: linear-gradient(120deg, rgba(139, 92, 246, 0.55), rgba(217, 70, 239, 0.18) 40%, rgba(34, 211, 238, 0.45));
            -webkit-mask: linear-gradient(white 0 0) content-box, linear-gradient(white 0 0);
            -webkit-mask-composite: xor;
            mask-composite: exclude;
            pointer-events: none;
        }
        @keyframes rise { from { opacity: 0; transform: translateY(18px); } to { opacity: 1; transform: none; } }
        .brand { display: flex; align-items: center; gap: 14px; margin-bottom: 26px; }
        .brand-mark {
            width: 48px; height: 48px; display: grid; place-items: center; font-size: 26px;
            border-radius: 14px; border: 1px solid var(--border);
            background: linear-gradient(160deg, rgba(139, 92, 246, 0.35), rgba(34, 211, 238, 0.15));
            box-shadow: 0 0 28px rgba(139, 92, 246, 0.35);
        }
        .brand-name {
            font-family: 'Space Grotesk', sans-serif; font-weight: 700; font-size: 26px; letter-spacing: 0.06em;
            background: linear-gradient(90deg, #e9d5ff, #a5f3fc);
            -webkit-background-clip: text; background-clip: text; color: transparent;
        }
        .badge {
            display: inline-flex; align-items: center; gap: 8px;
            padding: 8px 14px; border-radius: 999px; margin-bottom: 20px;
            font-size: 13px; font-weight: 600; letter-spacing: 0.04em; text-transform: uppercase;
            color: var(--amber); border: 1px solid rgba(251, 191, 36, 0.35);
            background: rgba(251, 191, 36, 0.08);
        }
        .badge-dot { width: 8px; height: 8px; border-radius: 50%; background: var(--amber); box-shadow: 0 0 10px var(--amber); }
        p.lede { color: var(--muted); line-height: 1.6; margin-bottom: 26px; }
        .endpoints { display: grid; gap: 10px; }
        .endpoint {
            display: flex; flex-direction: column; gap: 4px;
            padding: 14px 16px; border-radius: 14px;
            border: 1px solid var(--border); background: rgba(255, 255, 255, 0.03);
            transition: border-color 0.2s ease, transform 0.2s ease;
        }
        .endpoint:hover { border-color: rgba(139, 92, 246, 0.5); transform: translateX(4px); }
        .endpoint code { font-family: 'JetBrains Mono', monospace; font-size: 14px; color: #6ee7b7; }
        .endpoint span { font-size: 13px; color: var(--muted); }
        .hint { margin-top: 24px; font-size: 12.5px; color: var(--muted); }
        .hint code { font-family: 'JetBrains Mono', monospace; background: rgba(255, 255, 255, 0.08); padding: 2px 7px; border-radius: 6px; color: #e9d5ff; }
        @media (prefers-reduced-motion: reduce) { .blob { animation: none; } }
    </style>
</head>
<body>
    <div class="aurora" aria-hidden="true"><span class="blob b1"></span><span class="blob b2"></span><span class="blob b3"></span></div>
    <div class="grid-overlay" aria-hidden="true"></div>
    <main class="card">
        <div class="brand">
            <div class="brand-mark">🎬</div>
            <div class="brand-name">SNATCHR</div>
        </div>
        <div class="badge"><span class="badge-dot"></span>API-only mode</div>
        <p class="lede">The web interface is disabled on this server, but the API is alive and kicking. Point your requests at these endpoints:</p>
        <div class="endpoints">
            <div class="endpoint"><code>POST /download</code><span>Download a video — JSON body with a url field</span></div>
            <div class="endpoint"><code>GET /health</code><span>Check server health</span></div>
            <div class="endpoint"><code>GET /files/{video_id}/{filename}</code><span>Fetch a downloaded file</span></div>
        </div>
        <div class="hint">Want the pretty version? Set <code>ENABLE_WEB_UI=true</code> and restart.</div>
    </main>
</body>
</html>
        "##,
        );
    }
    Html(
        r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Snatchr — Lightning-Fast Video Downloader</title>
    <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🎬</text></svg>">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@500;700&family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;600&display=swap" rel="stylesheet">
    <script type="module" src="https://cdn.jsdelivr.net/npm/media-chrome@4/+esm"></script>
    <style>
        /* ============================= Base ============================= */
        :root {
            --bg: #05050a;
            --surface: rgba(255, 255, 255, 0.04);
            --border: rgba(255, 255, 255, 0.09);
            --text: #f4f4f8;
            --muted: rgba(235, 235, 245, 0.55);
            --violet: #8b5cf6;
            --fuchsia: #d946ef;
            --cyan: #22d3ee;
            --success: #34d399;
            --danger: #fb7185;
            --amber: #fbbf24;
        }
        * { margin: 0; padding: 0; box-sizing: border-box; }
        html { color-scheme: dark; scroll-behavior: smooth; }
        body {
            min-height: 100vh;
            background: var(--bg);
            color: var(--text);
            font-family: 'Inter', system-ui, sans-serif;
            overflow-x: hidden;
        }
        ::selection { background: rgba(139, 92, 246, 0.45); color: white; }
        ::-webkit-scrollbar { width: 10px; }
        ::-webkit-scrollbar-track { background: transparent; }
        ::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.14); border-radius: 99px; }
        ::-webkit-scrollbar-thumb:hover { background: rgba(139, 92, 246, 0.55); }
        .hidden { display: none !important; }

        /* ======================== Background FX ========================= */
        .aurora {
            position: fixed; inset: 0; z-index: -2; overflow: hidden;
            filter: blur(90px) saturate(140%);
        }
        .blob { position: absolute; border-radius: 50%; opacity: 0.45; }
        .b1 { width: 44vw; height: 44vw; left: -10vw; top: -12vw; background: radial-gradient(circle, var(--violet), transparent 65%); animation: drift1 26s ease-in-out infinite alternate; }
        .b2 { width: 38vw; height: 38vw; right: -8vw; top: 18vh; background: radial-gradient(circle, var(--fuchsia), transparent 65%); animation: drift2 32s ease-in-out infinite alternate; }
        .b3 { width: 34vw; height: 34vw; left: 30vw; bottom: -14vw; background: radial-gradient(circle, var(--cyan), transparent 65%); animation: drift3 38s ease-in-out infinite alternate; }
        @keyframes drift1 { to { transform: translate(9vw, 7vh) scale(1.15); } }
        @keyframes drift2 { to { transform: translate(-7vw, -6vh) scale(0.9); } }
        @keyframes drift3 { to { transform: translate(-5vw, -8vh) scale(1.2); } }
        .grid-overlay {
            position: fixed; inset: 0; z-index: -1; pointer-events: none;
            background-image: radial-gradient(rgba(255, 255, 255, 0.05) 1px, transparent 1px);
            background-size: 34px 34px;
            mask-image: radial-gradient(ellipse 80% 70% at 50% 35%, black 30%, transparent 100%);
            -webkit-mask-image: radial-gradient(ellipse 80% 70% at 50% 35%, black 30%, transparent 100%);
        }

        /* =========================== Layout ============================= */
        .shell {
            width: 100%; max-width: 760px;
            margin: 0 auto;
            padding: 28px 20px 48px;
            display: flex; flex-direction: column;
            min-height: 100vh;
        }
        .topbar {
            display: flex; align-items: center; justify-content: space-between;
            animation: fadeDown 0.5s ease both;
        }
        @keyframes fadeDown { from { opacity: 0; transform: translateY(-10px); } to { opacity: 1; transform: none; } }
        .brand { display: flex; align-items: center; gap: 12px; }
        .brand-mark {
            width: 42px; height: 42px; display: grid; place-items: center; font-size: 22px;
            border-radius: 12px; border: 1px solid var(--border);
            background: linear-gradient(160deg, rgba(139, 92, 246, 0.35), rgba(34, 211, 238, 0.15));
            box-shadow: 0 0 24px rgba(139, 92, 246, 0.35);
        }
        .brand-name {
            font-family: 'Space Grotesk', sans-serif; font-weight: 700; font-size: 20px; letter-spacing: 0.08em;
            background: linear-gradient(90deg, #e9d5ff, #a5f3fc);
            -webkit-background-clip: text; background-clip: text; color: transparent;
        }
        .status-pill {
            display: inline-flex; align-items: center; gap: 8px;
            font-size: 12px; font-weight: 600; letter-spacing: 0.08em; text-transform: uppercase;
            color: var(--muted);
            border: 1px solid var(--border); border-radius: 999px; padding: 7px 14px;
            background: var(--surface);
        }
        .status-dot { width: 7px; height: 7px; border-radius: 50%; background: var(--success); box-shadow: 0 0 9px var(--success); animation: pulse 2.4s ease-in-out infinite; }
        @keyframes pulse { 50% { opacity: 0.4; } }
        .status-pill[data-state="checking"] .status-dot { background: var(--amber); box-shadow: 0 0 9px var(--amber); }
        .status-pill[data-state="offline"] .status-dot { background: var(--danger); box-shadow: 0 0 9px var(--danger); animation: none; }

        /* ============================ Hero ============================== */
        .hero { text-align: center; margin: 54px 0 34px; animation: rise 0.6s 0.08s cubic-bezier(0.2, 0.8, 0.2, 1) both; }
        @keyframes rise { from { opacity: 0; transform: translateY(18px); } to { opacity: 1; transform: none; } }
        .hero h1 {
            font-family: 'Space Grotesk', sans-serif;
            font-size: clamp(38px, 7vw, 60px);
            font-weight: 700; line-height: 1.06; letter-spacing: -0.02em;
        }
        .grad {
            background: linear-gradient(90deg, var(--violet), var(--fuchsia) 45%, var(--cyan));
            background-size: 200% 100%;
            -webkit-background-clip: text; background-clip: text; color: transparent;
            animation: shimmer 7s linear infinite;
        }
        @keyframes shimmer { to { background-position: 200% 0; } }
        .hero .sub { margin-top: 16px; color: var(--muted); font-size: 16.5px; line-height: 1.6; }

        /* ============================ Card ============================== */
        .card {
            position: relative;
            padding: 30px 28px;
            border-radius: 24px;
            border: 1px solid var(--border);
            background: linear-gradient(180deg, rgba(255, 255, 255, 0.06), rgba(255, 255, 255, 0.02));
            backdrop-filter: blur(20px);
            -webkit-backdrop-filter: blur(20px);
            animation: rise 0.6s 0.16s cubic-bezier(0.2, 0.8, 0.2, 1) both;
        }
        .card::before {
            content: ""; position: absolute; inset: -1px; border-radius: inherit; padding: 1px;
            background: linear-gradient(120deg, rgba(139, 92, 246, 0.55), rgba(217, 70, 239, 0.18) 40%, rgba(34, 211, 238, 0.45));
            -webkit-mask: linear-gradient(white 0 0) content-box, linear-gradient(white 0 0);
            -webkit-mask-composite: xor;
            mask-composite: exclude;
            pointer-events: none;
        }
        .field-label {
            display: block; margin-bottom: 10px;
            font-size: 12px; font-weight: 600; letter-spacing: 0.14em; text-transform: uppercase;
            color: var(--muted);
        }
        .input-wrap {
            position: relative; display: flex; align-items: center;
            border-radius: 16px; border: 1px solid var(--border);
            background: rgba(3, 3, 8, 0.55);
            transition: border-color 0.25s ease, box-shadow 0.25s ease;
        }
        .input-wrap:focus-within {
            border-color: rgba(139, 92, 246, 0.7);
            box-shadow: 0 0 0 4px rgba(139, 92, 246, 0.18), 0 0 32px rgba(139, 92, 246, 0.12);
        }
        .input-wrap.shake {
            animation: shake 0.45s cubic-bezier(0.36, 0.07, 0.19, 0.97);
            border-color: rgba(251, 113, 133, 0.7);
        }
        @keyframes shake {
            10%, 90% { transform: translateX(-1px); }
            20%, 80% { transform: translateX(2px); }
            30%, 50%, 70% { transform: translateX(-4px); }
            40%, 60% { transform: translateX(4px); }
        }
        .input-icon { flex: none; margin-left: 16px; width: 20px; height: 20px; color: var(--muted); }
        .input-wrap input {
            flex: 1; min-width: 0;
            padding: 17px 14px;
            background: transparent; border: none; outline: none;
            color: var(--text); font-size: 15.5px; font-family: inherit;
        }
        .input-wrap input::placeholder { color: rgba(235, 235, 245, 0.32); }
        .paste-btn {
            flex: none; margin-right: 8px;
            display: grid; place-items: center;
            width: 38px; height: 38px; border-radius: 11px;
            border: none; cursor: pointer;
            background: transparent; color: var(--muted);
            transition: background 0.2s ease, color 0.2s ease;
        }
        .paste-btn:hover { background: rgba(255, 255, 255, 0.08); color: var(--text); }
        .paste-btn svg { width: 18px; height: 18px; }

        .cta {
            position: relative; overflow: hidden;
            width: 100%; margin-top: 16px;
            padding: 17px 24px;
            border: none; border-radius: 16px; cursor: pointer;
            font-family: 'Space Grotesk', sans-serif; font-size: 17px; font-weight: 700; letter-spacing: 0.02em;
            color: white;
            background: linear-gradient(90deg, var(--violet), var(--fuchsia) 55%, var(--cyan));
            background-size: 180% 100%;
            transition: transform 0.2s ease, box-shadow 0.25s ease, background-position 0.5s ease;
            box-shadow: 0 8px 32px rgba(139, 92, 246, 0.35);
        }
        .cta:hover:not(:disabled) {
            transform: translateY(-2px);
            background-position: 100% 0;
            box-shadow: 0 12px 44px rgba(217, 70, 239, 0.4);
        }
        .cta:active:not(:disabled) { transform: translateY(0); }
        .cta:disabled { opacity: 0.55; cursor: not-allowed; }
        .cta::after {
            content: ""; position: absolute; top: 0; left: -80%;
            width: 50%; height: 100%;
            background: linear-gradient(100deg, transparent, rgba(255, 255, 255, 0.35), transparent);
            transform: skewX(-20deg);
            transition: left 0.55s ease;
        }
        .cta:hover:not(:disabled)::after { left: 130%; }
        .cta-inner { display: inline-flex; align-items: center; gap: 10px; position: relative; z-index: 1; }
        .cta-inner svg { width: 20px; height: 20px; }
        .spinner {
            width: 18px; height: 18px; border-radius: 50%;
            border: 2.5px solid rgba(255, 255, 255, 0.35); border-top-color: white;
            animation: spin 0.7s linear infinite;
        }
        @keyframes spin { to { transform: rotate(360deg); } }

        /* ========================= Platform chips ======================= */
        .platforms {
            display: flex; flex-wrap: wrap; gap: 8px; justify-content: center;
            margin-top: 22px;
        }
        .chip {
            display: inline-flex; align-items: center; gap: 7px;
            padding: 6px 13px; border-radius: 999px;
            font-size: 12.5px; font-weight: 500; color: var(--muted);
            border: 1px solid var(--border); background: rgba(255, 255, 255, 0.03);
            transition: color 0.2s ease, border-color 0.2s ease, transform 0.2s ease;
        }
        .chip:hover { color: var(--text); border-color: rgba(139, 92, 246, 0.5); transform: translateY(-1px); }
        .chip i { width: 6px; height: 6px; border-radius: 50%; display: inline-block; }

        /* ====================== Loading + Hamster ======================= */
        .loading { margin-top: 28px; display: flex; flex-direction: column; align-items: center; }
        .hamster-stage { position: relative; padding-bottom: 6px; }
        .hamster-glow {
            position: absolute; left: 50%; bottom: -6px; transform: translateX(-50%);
            width: 170px; height: 42px; border-radius: 50%;
            background: radial-gradient(ellipse, rgba(217, 70, 239, 0.4), transparent 70%);
            filter: blur(12px);
            animation: glowPulse 2s ease-in-out infinite;
        }
        @keyframes glowPulse { 50% { opacity: 0.5; transform: translateX(-50%) scale(0.92); } }
        .loader-quip {
            margin-top: 22px; min-height: 24px;
            font-family: 'JetBrains Mono', monospace; font-size: 14px; color: #d8b4fe;
            transition: opacity 0.3s ease;
        }
        .loader-quip.fade { opacity: 0; }
        .loader-note { margin-top: 8px; font-size: 13px; color: var(--muted); }
        .loader-elapsed {
            margin-top: 14px; padding: 5px 13px; border-radius: 999px;
            font-family: 'JetBrains Mono', monospace; font-size: 12.5px; color: var(--muted);
            border: 1px solid var(--border); background: rgba(255, 255, 255, 0.03);
            font-variant-numeric: tabular-nums;
        }

        /* Wheel & Hamster — the legend himself (uiverse-style, untouched physics) */
        .wheel-and-hamster {
            --dur: 1s;
            position: relative;
            width: 12em;
            height: 12em;
            font-size: 10px;
        }
        .wheel, .hamster, .hamster div, .spoke { position: absolute; }
        .wheel, .spoke { border-radius: 50%; top: 0; left: 0; width: 100%; height: 100%; }
        .wheel {
            background: radial-gradient(100% 100% at center,hsla(0,0%,60%,0) 47.8%,hsl(0,0%,60%) 48%);
            z-index: 2;
        }
        .hamster {
            animation: hamster var(--dur) ease-in-out infinite;
            top: 50%;
            left: calc(50% - 3.5em);
            width: 7em;
            height: 3.75em;
            transform: rotate(4deg) translate(-0.8em,1.85em);
            transform-origin: 50% 0;
            z-index: 1;
        }
        .hamster__head {
            animation: hamsterHead var(--dur) ease-in-out infinite;
            background: hsl(30,90%,55%);
            border-radius: 70% 30% 0 100% / 40% 25% 25% 60%;
            box-shadow: 0 -0.25em 0 hsl(30,90%,80%) inset,
                0.75em -1.55em 0 hsl(30,90%,90%) inset;
            top: 0;
            left: -2em;
            width: 2.75em;
            height: 2.5em;
            transform-origin: 100% 50%;
        }
        .hamster__ear {
            animation: hamsterEar var(--dur) ease-in-out infinite;
            background: hsl(0,90%,85%);
            border-radius: 50%;
            box-shadow: -0.25em 0 hsl(30,90%,55%) inset;
            top: -0.25em;
            right: -0.25em;
            width: 0.75em;
            height: 0.75em;
            transform-origin: 50% 75%;
        }
        .hamster__eye {
            animation: hamsterEye var(--dur) linear infinite;
            background-color: hsl(0,0%,0%);
            border-radius: 50%;
            top: 0.375em;
            left: 1.25em;
            width: 0.5em;
            height: 0.5em;
        }
        .hamster__nose {
            background: hsl(0,90%,75%);
            border-radius: 35% 65% 85% 15% / 70% 50% 50% 30%;
            top: 0.75em;
            left: 0;
            width: 0.2em;
            height: 0.25em;
        }
        .hamster__body {
            animation: hamsterBody var(--dur) ease-in-out infinite;
            background: hsl(30,90%,90%);
            border-radius: 50% 30% 50% 30% / 15% 60% 40% 40%;
            box-shadow: 0.1em 0.75em 0 hsl(30,90%,55%) inset,
                0.15em -0.5em 0 hsl(30,90%,80%) inset;
            top: 0.25em;
            left: 2em;
            width: 4.5em;
            height: 3em;
            transform-origin: 17% 50%;
            transform-style: preserve-3d;
        }
        .hamster__limb--fr, .hamster__limb--fl {
            clip-path: polygon(0 0,100% 0,70% 80%,60% 100%,0% 100%,40% 80%);
            top: 2em;
            left: 0.5em;
            width: 1em;
            height: 1.5em;
            transform-origin: 50% 0;
        }
        .hamster__limb--fr {
            animation: hamsterFRLimb var(--dur) linear infinite;
            background: linear-gradient(hsl(30,90%,80%) 80%,hsl(0,90%,75%) 80%);
            transform: rotate(15deg) translateZ(-1px);
        }
        .hamster__limb--fl {
            animation: hamsterFLLimb var(--dur) linear infinite;
            background: linear-gradient(hsl(30,90%,90%) 80%,hsl(0,90%,85%) 80%);
            transform: rotate(15deg);
        }
        .hamster__limb--br, .hamster__limb--bl {
            border-radius: 0.75em 0.75em 0 0;
            clip-path: polygon(0 0,100% 0,100% 30%,70% 90%,70% 100%,30% 100%,40% 90%,0% 30%);
            top: 1em;
            left: 2.8em;
            width: 1.5em;
            height: 2.5em;
            transform-origin: 50% 30%;
        }
        .hamster__limb--br {
            animation: hamsterBRLimb var(--dur) linear infinite;
            background: linear-gradient(hsl(30,90%,80%) 90%,hsl(0,90%,75%) 90%);
            transform: rotate(-25deg) translateZ(-1px);
        }
        .hamster__limb--bl {
            animation: hamsterBLLimb var(--dur) linear infinite;
            background: linear-gradient(hsl(30,90%,90%) 90%,hsl(0,90%,85%) 90%);
            transform: rotate(-25deg);
        }
        .hamster__tail {
            animation: hamsterTail var(--dur) linear infinite;
            background: hsl(0,90%,85%);
            border-radius: 0.25em 50% 50% 0.25em;
            box-shadow: 0 -0.2em 0 hsl(0,90%,75%) inset;
            top: 1.5em;
            right: -0.5em;
            width: 1em;
            height: 0.5em;
            transform: rotate(30deg) translateZ(-1px);
            transform-origin: 0.25em 0.25em;
        }
        .spoke {
            animation: spoke var(--dur) linear infinite;
            background: radial-gradient(100% 100% at center,hsl(0,0%,60%) 4.8%,hsla(0,0%,60%,0) 5%),
                linear-gradient(hsla(0,0%,55%,0) 46.9%,hsl(0,0%,65%) 47% 52.9%,hsla(0,0%,65%,0) 53%) 50% 50% / 99% 99% no-repeat;
        }
        @keyframes hamster {
            from, to { transform: rotate(4deg) translate(-0.8em,1.85em); }
            50% { transform: rotate(0) translate(-0.8em,1.85em); }
        }
        @keyframes hamsterHead {
            from, 25%, 50%, 75%, to { transform: rotate(0); }
            12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(8deg); }
        }
        @keyframes hamsterEye {
            from, 90%, to { transform: scaleY(1); }
            95% { transform: scaleY(0); }
        }
        @keyframes hamsterEar {
            from, 25%, 50%, 75%, to { transform: rotate(0); }
            12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(12deg); }
        }
        @keyframes hamsterBody {
            from, 25%, 50%, 75%, to { transform: rotate(0); }
            12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(-2deg); }
        }
        @keyframes hamsterFRLimb {
            from, 25%, 50%, 75%, to { transform: rotate(50deg) translateZ(-1px); }
            12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(-30deg) translateZ(-1px); }
        }
        @keyframes hamsterFLLimb {
            from, 25%, 50%, 75%, to { transform: rotate(-30deg); }
            12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(50deg); }
        }
        @keyframes hamsterBRLimb {
            from, 25%, 50%, 75%, to { transform: rotate(-60deg) translateZ(-1px); }
            12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(20deg) translateZ(-1px); }
        }
        @keyframes hamsterBLLimb {
            from, 25%, 50%, 75%, to { transform: rotate(20deg); }
            12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(-60deg); }
        }
        @keyframes hamsterTail {
            from, 25%, 50%, 75%, to { transform: rotate(30deg) translateZ(-1px); }
            12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(10deg) translateZ(-1px); }
        }
        @keyframes spoke {
            from { transform: rotate(0); }
            to { transform: rotate(-1turn); }
        }

        /* =========================== Results ============================ */
        .result { margin-top: 28px; animation: rise 0.4s cubic-bezier(0.2, 0.8, 0.2, 1) both; }
        .panel {
            border-radius: 18px; padding: 22px;
            border: 1px solid var(--border);
        }
        .panel-success { background: rgba(52, 211, 153, 0.07); border-color: rgba(52, 211, 153, 0.3); }
        .panel-error { background: rgba(251, 113, 133, 0.07); border-color: rgba(251, 113, 133, 0.3); }
        .panel-warn { background: rgba(251, 191, 36, 0.07); border-color: rgba(251, 191, 36, 0.3); }
        .panel-head { display: flex; align-items: center; gap: 13px; margin-bottom: 12px; }
        .panel-icon {
            flex: none; width: 40px; height: 40px; display: grid; place-items: center;
            border-radius: 12px;
        }
        .panel-icon svg { width: 22px; height: 22px; }
        .panel-success .panel-icon { background: rgba(52, 211, 153, 0.15); color: var(--success); }
        .panel-error .panel-icon { background: rgba(251, 113, 133, 0.15); color: var(--danger); }
        .panel-warn .panel-icon { background: rgba(251, 191, 36, 0.15); color: var(--amber); }
        .panel-title { font-family: 'Space Grotesk', sans-serif; font-size: 19px; font-weight: 700; }
        .panel-body { color: var(--muted); font-size: 14.5px; line-height: 1.6; }
        .file-name {
            margin: 14px 0 4px;
            font-family: 'JetBrains Mono', monospace; font-size: 13px; color: #d8b4fe;
            word-break: break-all;
        }
        .action-row { display: flex; flex-wrap: wrap; gap: 10px; margin: 14px 0 18px; }
        .btn {
            display: inline-flex; align-items: center; gap: 9px;
            padding: 12px 20px; border-radius: 13px; border: none; cursor: pointer;
            font-family: 'Space Grotesk', sans-serif; font-size: 15px; font-weight: 700;
            text-decoration: none; color: white;
            transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.2s ease;
        }
        .btn svg { width: 18px; height: 18px; }
        .btn-primary {
            background: linear-gradient(90deg, #10b981, #06b6d4);
            box-shadow: 0 6px 24px rgba(16, 185, 129, 0.35);
        }
        .btn-primary:hover { transform: translateY(-2px); box-shadow: 0 10px 32px rgba(16, 185, 129, 0.45); }
        .btn-ghost {
            background: rgba(255, 255, 255, 0.07); color: var(--text);
            border: 1px solid var(--border);
        }
        .btn-ghost:hover { background: rgba(255, 255, 255, 0.12); }
        media-controller {
            width: 100%; border-radius: 16px; overflow: hidden;
            --media-primary-color: #c4b5fd;
            --media-control-background: rgba(8, 8, 16, 0.55);
            --media-control-hover-background: rgba(139, 92, 246, 0.4);
        }
        media-controller video { width: 100%; display: block; }

        /* =========================== History ============================ */
        .history { margin-top: 22px; padding-top: 20px; border-top: 1px solid var(--border); }
        .history h4 {
            font-size: 11.5px; font-weight: 600; letter-spacing: 0.14em; text-transform: uppercase;
            color: var(--muted); margin-bottom: 12px;
        }
        .history-item {
            display: flex; align-items: center; gap: 12px;
            padding: 11px 14px; border-radius: 12px;
            border: 1px solid transparent;
            transition: background 0.2s ease, border-color 0.2s ease;
        }
        .history-item:hover { background: rgba(255, 255, 255, 0.04); border-color: var(--border); }
        .history-meta { flex: 1; min-width: 0; }
        .history-name { font-size: 13.5px; font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
        .history-time { font-size: 11.5px; color: var(--muted); font-family: 'JetBrains Mono', monospace; }
        .history-dl { flex: none; display: grid; place-items: center; width: 34px; height: 34px; border-radius: 10px; color: var(--muted); transition: color 0.2s ease, background 0.2s ease; }
        .history-dl:hover { color: var(--text); background: rgba(139, 92, 246, 0.25); }
        .history-dl svg { width: 17px; height: 17px; }

        /* =========================== Footer ============================= */
        .foot {
            margin-top: auto; padding-top: 44px;
            text-align: center; font-size: 12.5px; color: rgba(235, 235, 245, 0.35);
        }
        .foot span { color: rgba(235, 235, 245, 0.55); }

        @media (max-width: 520px) {
            .card { padding: 24px 18px; }
            .hero { margin: 40px 0 26px; }
        }
        @media (prefers-reduced-motion: reduce) {
            .blob, .grad, .hamster-glow, .status-dot { animation: none; }
            .wheel-and-hamster { --dur: 6s; }
        }
    </style>
</head>
<body>
    <div class="aurora" aria-hidden="true"><span class="blob b1"></span><span class="blob b2"></span><span class="blob b3"></span></div>
    <div class="grid-overlay" aria-hidden="true"></div>

    <main class="shell">
        <header class="topbar">
            <div class="brand">
                <div class="brand-mark">🎬</div>
                <div class="brand-name">SNATCHR</div>
            </div>
            <div class="status-pill" id="statusPill" data-state="checking"><span class="status-dot"></span><span id="statusText">checking</span></div>
        </header>

        <section class="hero">
            <h1>Snatch any video.<br><span class="grad">Stupidly fast.</span></h1>
            <p class="sub">Paste a link, hit the button, get the file.</p>
        </section>

        <section class="card">
            <form id="downloadForm" autocomplete="off">
                <label class="field-label" for="videoUrl">Video URL</label>
                <div class="input-wrap">
                    <svg class="input-icon" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 011.242 7.244l-4.5 4.5a4.5 4.5 0 01-6.364-6.364l1.757-1.757m13.35-.622l1.757-1.757a4.5 4.5 0 00-6.364-6.364l-4.5 4.5a4.5 4.5 0 001.242 7.244"/>
                    </svg>
                    <input type="url" id="videoUrl" name="videoUrl" placeholder="https://youtube.com/watch?v=…" required autofocus>
                    <button type="button" id="pasteBtn" class="paste-btn" title="Paste from clipboard" aria-label="Paste from clipboard">
                        <svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
                        </svg>
                    </button>
                </div>
                <button type="submit" id="downloadBtn" class="cta">
                    <span class="cta-inner">
                        <svg fill="none" stroke="currentColor" stroke-width="2.2" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"/>
                        </svg>
                        Snatch it
                    </span>
                </button>
            </form>

            <div class="platforms" aria-label="Supported platforms">
                <span class="chip"><i style="background:#ff5b5b"></i>YouTube</span>
                <span class="chip"><i style="background:#7ee7f9"></i>TikTok</span>
                <span class="chip"><i style="background:#7aa8ff"></i>Vimeo</span>
                <span class="chip"><i style="background:#b48cff"></i>Twitch</span>
                <span class="chip"><i style="background:#ff8ac4"></i>Instagram</span>
                <span class="chip"><i style="background:#e7e7ef"></i>X</span>
                <span class="chip"><i style="background:#86a2ff"></i>Facebook</span>
            </div>

            <!-- Loading state -->
            <div id="loading" class="hidden loading">
                <div class="hamster-stage">
                    <div aria-label="Orange and tan hamster running in a metal wheel" role="img" class="wheel-and-hamster">
                        <div class="wheel"></div>
                        <div class="hamster">
                            <div class="hamster__body">
                                <div class="hamster__head">
                                    <div class="hamster__ear"></div>
                                    <div class="hamster__eye"></div>
                                    <div class="hamster__nose"></div>
                                </div>
                                <div class="hamster__limb hamster__limb--fr"></div>
                                <div class="hamster__limb hamster__limb--fl"></div>
                                <div class="hamster__limb hamster__limb--br"></div>
                                <div class="hamster__limb hamster__limb--bl"></div>
                                <div class="hamster__tail"></div>
                            </div>
                        </div>
                        <div class="spoke"></div>
                    </div>
                    <div class="hamster-glow"></div>
                </div>
                <div id="loaderQuip" class="loader-quip">Feeding the hamster…</div>
                <div class="loader-note">Bigger videos take longer — the wheel only spins so fast</div>
                <div id="loaderElapsed" class="loader-elapsed">0s</div>
                <div id="loadingHistory" style="width:100%"></div>
            </div>

            <!-- Result -->
            <div id="result" class="hidden result"></div>
        </section>
    </main>

    <script>
        // URL validation: basic allow-list by hostname; backend performs canonical validation
        function isValidVideoUrl(url) {
            try {
                const u = new URL(url);
                const host = u.hostname.toLowerCase();
                const allowedHosts = [
                    'youtube.com','www.youtube.com','m.youtube.com','youtu.be',
                    'vimeo.com','www.vimeo.com',
                    'twitch.tv','www.twitch.tv','clips.twitch.tv',
                    'tiktok.com','www.tiktok.com','vm.tiktok.com',
                    'instagram.com','www.instagram.com',
                    'twitter.com','www.twitter.com','x.com','www.x.com',
                    'facebook.com','www.facebook.com','fb.watch'
                ];
                return allowedHosts.includes(host);
            } catch (_) {
                return false;
            }
        }

        // --- Icons (inline SVG snippets reused in templates) ---
        const ICON_DOWNLOAD = '<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/></svg>';
        const ICON_CHECK = '<svg fill="none" stroke="currentColor" stroke-width="2.4" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/></svg>';
        const ICON_X = '<svg fill="none" stroke="currentColor" stroke-width="2.2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/></svg>';
        const ICON_WARN = '<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"/></svg>';
        const ICON_COPY = '<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/></svg>';

        // --- Live server status in the top bar ---
        async function checkHealth() {
            const pill = document.getElementById('statusPill');
            const text = document.getElementById('statusText');
            try {
                const res = await fetch('/health', { cache: 'no-store' });
                if (!res.ok) throw new Error('unhealthy');
                pill.dataset.state = 'online';
                text.textContent = 'online';
            } catch (_) {
                pill.dataset.state = 'offline';
                text.textContent = 'offline';
            }
        }
        checkHealth();
        setInterval(checkHealth, 30000);

        // Press "/" anywhere to jump to the URL field
        document.addEventListener('keydown', function(e) {
            if (e.key === '/' && document.activeElement.tagName !== 'INPUT') {
                e.preventDefault();
                document.getElementById('videoUrl').focus();
            }
        });

        // --- Elapsed time while the hamster runs ---
        let elapsedTimer = null;
        function startElapsed() {
            const el = document.getElementById('loaderElapsed');
            const startedAt = Date.now();
            el.textContent = '0s';
            elapsedTimer = setInterval(() => {
                const total = Math.floor((Date.now() - startedAt) / 1000);
                const mins = Math.floor(total / 60);
                const secs = total % 60;
                el.textContent = mins > 0 ? mins + 'm ' + secs + 's' : secs + 's';
            }, 1000);
        }
        function stopElapsed() {
            if (elapsedTimer) { clearInterval(elapsedTimer); elapsedTimer = null; }
        }

        // --- Loader quips: the hamster deserves commentary ---
        const QUIPS = [
            'Feeding the hamster…',
            'Wheel spinning at maximum RPM…',
            'Snatching pixels from the void…',
            'Negotiating with the server hamsters…',
            'Untangling the internet tubes…',
            'Compressing hamster sweat into video…',
            'Almost there, little guy is giving it everything…'
        ];
        let quipTimer = null;
        function startQuips() {
            const el = document.getElementById('loaderQuip');
            let i = 0;
            el.textContent = QUIPS[0];
            quipTimer = setInterval(() => {
                el.classList.add('fade');
                setTimeout(() => {
                    i = (i + 1) % QUIPS.length;
                    el.textContent = QUIPS[i];
                    el.classList.remove('fade');
                }, 300);
            }, 2600);
        }
        function stopQuips() {
            if (quipTimer) { clearInterval(quipTimer); quipTimer = null; }
        }

        // --- Download history ---
        let downloadHistory = [];
        let lastDownloadedUrl = '';

        function historyHTML(items) {
            if (!items.length) return '';
            const rows = items.map(item => `
                <div class="history-item">
                    <div class="history-meta">
                        <div class="history-name">${item.fileName}</div>
                        <div class="history-time">${item.timestamp}</div>
                    </div>
                    <a class="history-dl" href="${item.fileUrl}" target="_blank" title="Download again">${ICON_DOWNLOAD}</a>
                </div>
            `).join('');
            return `<div class="history"><h4>Previous downloads</h4>${rows}</div>`;
        }

        function panelHTML(kind, icon, title, bodyHTML) {
            return `
                <div class="panel panel-${kind}">
                    <div class="panel-head">
                        <div class="panel-icon">${icon}</div>
                        <div class="panel-title">${title}</div>
                    </div>
                    <div class="panel-body">${bodyHTML}</div>
                </div>
            `;
        }

        // --- Paste button ---
        document.getElementById('pasteBtn').addEventListener('click', async function() {
            try {
                const text = await navigator.clipboard.readText();
                document.getElementById('videoUrl').value = text;
                document.getElementById('videoUrl').focus();
            } catch (err) {
                console.log('Failed to read clipboard:', err);
            }
        });

        // --- Copy link (delegated, result is re-rendered each time) ---
        document.addEventListener('click', async function(e) {
            const btn = e.target.closest('[data-copy]');
            if (!btn) return;
            try {
                await navigator.clipboard.writeText(btn.getAttribute('data-copy'));
                const original = btn.innerHTML;
                btn.innerHTML = ICON_CHECK + 'Copied!';
                setTimeout(() => { btn.innerHTML = original; }, 1600);
            } catch (err) {
                console.log('Failed to copy:', err);
            }
        });

        // --- Main form ---
        const ctaButton = document.getElementById('downloadBtn');
        const CTA_IDLE_HTML = ctaButton.innerHTML;

        function shakeInput() {
            const wrap = document.querySelector('.input-wrap');
            wrap.classList.remove('shake');
            void wrap.offsetWidth; /* restart animation */
            wrap.classList.add('shake');
            wrap.addEventListener('animationend', () => wrap.classList.remove('shake'), { once: true });
        }

        document.getElementById('downloadForm').addEventListener('submit', async function(e) {
            e.preventDefault();

            const url = document.getElementById('videoUrl').value;
            const button = ctaButton;
            const loading = document.getElementById('loading');
            const result = document.getElementById('result');

            if (!isValidVideoUrl(url)) {
                shakeInput();
                result.innerHTML = panelHTML('error', ICON_WARN, 'That link looks off',
                    'Please paste a URL from a supported platform — YouTube, TikTok, Vimeo, Twitch, Instagram, X or Facebook.');
                result.classList.remove('hidden');
                return;
            }

            if (url === lastDownloadedUrl && lastDownloadedUrl !== '') {
                shakeInput();
                result.innerHTML = panelHTML('warn', ICON_WARN, 'Nice try, smarty pants 🤦',
                    'You literally just downloaded this video. Try a different URL, or refresh the page if you really want it again.');
                result.classList.remove('hidden');
                return;
            }

            // Enter loading state
            button.disabled = true;
            button.innerHTML = '<span class="cta-inner"><span class="spinner"></span>Snatching…</span>';
            document.getElementById('loadingHistory').innerHTML = historyHTML(downloadHistory);
            loading.classList.remove('hidden');
            result.classList.add('hidden');
            startQuips();
            startElapsed();
            loading.scrollIntoView({ behavior: 'smooth', block: 'nearest' });

            try {
                // TEST MODE: Uncomment the next line to simulate success for testing
                // const testMode = true;

                let data;
                if (typeof testMode !== 'undefined' && testMode) {
                    await new Promise(resolve => setTimeout(resolve, 4000));
                    data = {
                        success: true,
                        file_url: 'https://localhost:3000/files/06a75bee-bc99-4894-b184-c497d70ca7f5/video.mp4'
                    };
                } else {
                    const response = await fetch('/download', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ url: url })
                    });
                    data = await response.json();
                }

                if (data.success) {
                    lastDownloadedUrl = url;
                    document.getElementById('videoUrl').value = '';

                    const fileName = data.file_url.split('/').pop().replaceAll('_', ' ');

                    downloadHistory.unshift({
                        fileName: fileName,
                        fileUrl: data.file_url,
                        timestamp: new Date().toLocaleTimeString()
                    });
                    if (downloadHistory.length > 5) downloadHistory.pop();

                    const successBody = `
                        Your video is ready. Stream it below or grab the file.
                        <div class="file-name">${fileName}</div>
                        <div class="action-row">
                            <a class="btn btn-primary" href="${data.file_url}" target="_blank">${ICON_DOWNLOAD}Download file</a>
                            <button type="button" class="btn btn-ghost" data-copy="${data.file_url}">${ICON_COPY}Copy link</button>
                        </div>
                        <media-controller>
                            <video slot="media" src="${data.file_url}?stream=true" playsinline>
                                Your browser does not support the video tag.
                            </video>
                            <media-control-bar>
                                <media-play-button></media-play-button>
                                <media-mute-button></media-mute-button>
                                <media-volume-range></media-volume-range>
                                <media-time-range></media-time-range>
                                <media-pip-button></media-pip-button>
                                <media-fullscreen-button></media-fullscreen-button>
                            </media-control-bar>
                        </media-controller>
                        ${historyHTML(downloadHistory.slice(1))}
                    `;
                    result.innerHTML = panelHTML('success', ICON_CHECK, 'Snatched! 🎉', successBody);
                } else {
                    result.innerHTML = panelHTML('error', ICON_X, 'Download failed',
                        (data.error || 'An unknown error occurred while processing your request.'));
                }
            } catch (error) {
                result.innerHTML = panelHTML('error', ICON_WARN, 'Connection error',
                    'Could not reach the server. Check your connection and try again.');
            } finally {
                stopQuips();
                stopElapsed();
                button.disabled = false;
                button.innerHTML = CTA_IDLE_HTML;
                loading.classList.add('hidden');
                result.classList.remove('hidden');
                result.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
            }
        });
    </script>
</body>
</html>
        "##,
    )
}
