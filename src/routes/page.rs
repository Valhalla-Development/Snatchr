use axum::response::Html;

/*
 * HTTP handler for serving the download page.
 *
 * Returns an HTML page with JavaScript functionality to make download requests.
 */

#[axum::debug_handler]
pub async fn download_page() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Snatchr - Video Downloader</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {
            theme: {
                extend: {
                    animation: {
                        'fade-in': 'fadeIn 0.5s ease-in-out',
                        'slide-up': 'slideUp 0.3s ease-out',
                        'pulse-slow': 'pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                    }
                }
            }
        }
    </script>
    <style>
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        @keyframes slideUp {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }
        .gradient-bg {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        }
        .glass-effect {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.2);
        }

        /* Hamster Wheel Animation */
        .wheel-and-hamster {
            --dur: 1s;
            position: relative;
            width: 12em;
            height: 12em;
            font-size: 10px;
        }

        .wheel,
        .hamster,
        .hamster div,
        .spoke {
            position: absolute;
        }

        .wheel,
        .spoke {
            border-radius: 50%;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        }

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

        .hamster__limb--fr,
        .hamster__limb--fl {
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

        .hamster__limb--br,
        .hamster__limb--bl {
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

        /* Hamster Animations */
        @keyframes hamster {
            from, to {
                transform: rotate(4deg) translate(-0.8em,1.85em);
            }

            50% {
                transform: rotate(0) translate(-0.8em,1.85em);
            }
        }

        @keyframes hamsterHead {
            from, 25%, 50%, 75%, to {
                transform: rotate(0);
            }

            12.5%, 37.5%, 62.5%, 87.5% {
                transform: rotate(8deg);
            }
        }

        @keyframes hamsterEye {
            from, 90%, to {
                transform: scaleY(1);
            }

            95% {
                transform: scaleY(0);
            }
        }

        @keyframes hamsterEar {
            from, 25%, 50%, 75%, to {
                transform: rotate(0);
            }

            12.5%, 37.5%, 62.5%, 87.5% {
                transform: rotate(12deg);
            }
        }

        @keyframes hamsterBody {
            from, 25%, 50%, 75%, to {
                transform: rotate(0);
            }

            12.5%, 37.5%, 62.5%, 87.5% {
                transform: rotate(-2deg);
            }
        }

        @keyframes hamsterFRLimb {
            from, 25%, 50%, 75%, to {
                transform: rotate(50deg) translateZ(-1px);
            }

            12.5%, 37.5%, 62.5%, 87.5% {
                transform: rotate(-30deg) translateZ(-1px);
            }
        }

        @keyframes hamsterFLLimb {
            from, 25%, 50%, 75%, to {
                transform: rotate(-30deg);
            }

            12.5%, 37.5%, 62.5%, 87.5% {
                transform: rotate(50deg);
            }
        }

        @keyframes hamsterBRLimb {
            from, 25%, 50%, 75%, to {
                transform: rotate(-60deg) translateZ(-1px);
            }

            12.5%, 37.5%, 62.5%, 87.5% {
                transform: rotate(20deg) translateZ(-1px);
            }
        }

        @keyframes hamsterBLLimb {
            from, 25%, 50%, 75%, to {
                transform: rotate(20deg);
            }

            12.5%, 37.5%, 62.5%, 87.5% {
                transform: rotate(-60deg);
            }
        }

        @keyframes hamsterTail {
            from, 25%, 50%, 75%, to {
                transform: rotate(30deg) translateZ(-1px);
            }

            12.5%, 37.5%, 62.5%, 87.5% {
                transform: rotate(10deg) translateZ(-1px);
            }
        }

        @keyframes spoke {
            from {
                transform: rotate(0);
            }

            to {
                transform: rotate(-1turn);
            }
        }
    </style>
</head>
<body class="min-h-screen gradient-bg flex items-center justify-center p-4">
    <div class="w-full max-w-2xl">
        <!-- Header -->
        <div class="text-center mb-8 animate-fade-in">
            <div class="inline-flex items-center justify-center w-20 h-20 bg-white/20 rounded-full mb-4 backdrop-blur-sm">
                <span class="text-4xl">🎬</span>
            </div>
            <h1 class="text-5xl font-bold text-white mb-2">Snatchr</h1>
            <p class="text-white/80 text-lg">Lightning-Fast Video Downloader! ⚡</p>
        </div>

        <!-- Main Card -->
        <div class="glass-effect rounded-3xl p-8 shadow-2xl animate-fade-in">
            <form id="downloadForm" class="space-y-6">
                <!-- URL Input -->
                <div class="space-y-2">
                    <label for="videoUrl" class="block text-white font-semibold text-sm uppercase tracking-wide">
                        Video URL
                    </label>
                    <div class="relative">
                        <input 
                            type="url" 
                            id="videoUrl" 
                            name="videoUrl" 
                            placeholder="https://www.youtube.com/watch?v=..." 
                            required
                            class="w-full px-6 py-4 bg-white/10 border border-white/20 rounded-2xl text-white placeholder-white/50 focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent backdrop-blur-sm transition-all duration-300"
                        >
                        <button 
                            type="button" 
                            id="pasteBtn"
                            class="absolute inset-y-0 right-0 flex items-center pr-4 hover:text-white transition-colors duration-200"
                            title="Paste from clipboard"
                        >
                            <svg class="w-5 h-5 text-white/60 hover:text-white transition-all duration-200 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                            </svg>
                        </button>
                    </div>
                </div>

                <!-- Download Button -->
                <button 
                    type="submit" 
                    id="downloadBtn"
                    class="w-full bg-gradient-to-r from-purple-500 to-pink-500 hover:from-purple-600 hover:to-pink-600 text-white font-bold py-4 px-6 rounded-2xl transition-all duration-300 transform hover:scale-105 hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
                >
                    <span class="flex items-center justify-center space-x-2">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                        </svg>
                        <span>Download Video</span>
                    </span>
                </button>
            </form>

            <!-- Loading State -->
            <div id="loading" class="hidden mt-6">
                <div class="flex flex-col items-center justify-center space-y-4 text-white">
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
                    <div class="text-lg font-medium">Processing your video...</div>
                    <div class="text-center text-white/70 text-sm">
                        This may take a few minutes depending on the video size
                    </div>
                </div>
            </div>

            <!-- Result -->
            <div id="result" class="hidden mt-6 animate-slide-up"></div>
        </div>


    </div>

    <script>
        // URL validation function
        function isValidVideoUrl(url) {
            const youtubePatterns = [
                /^https?:\/\/(?:www\.)?youtube\.com\/watch\?v=[^&]+/,
                /^https?:\/\/youtu\.be\/[^&]+/
            ];
            
            return youtubePatterns.some(pattern => pattern.test(url));
        }

        // Track last downloaded URL
        let lastDownloadedUrl = '';

        // Paste button functionality
        document.getElementById('pasteBtn').addEventListener('click', async function() {
            try {
                const text = await navigator.clipboard.readText();
                document.getElementById('videoUrl').value = text;
            } catch (err) {
                console.log('Failed to read clipboard:', err);
            }
        });

        document.getElementById('downloadForm').addEventListener('submit', async function(e) {
            e.preventDefault();
            
            const url = document.getElementById('videoUrl').value;
            const button = document.getElementById('downloadBtn');
            const loading = document.getElementById('loading');
            const result = document.getElementById('result');
            
            // Validate URL
            if (!isValidVideoUrl(url)) {
                result.innerHTML = `
                    <div class="bg-red-500/20 border border-red-500/30 rounded-2xl p-6 text-white">
                        <div class="flex items-center space-x-3 mb-4">
                            <div class="w-10 h-10 bg-red-500/30 rounded-full flex items-center justify-center">
                                <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold">Invalid URL</h3>
                        </div>
                        <p class="text-white/80">Please enter a valid YouTube URL</p>
                    </div>
                `;
                result.classList.remove('hidden');
                return;
            }

            // Check if they're trying to download the same URL again
            if (url === lastDownloadedUrl && lastDownloadedUrl !== '') {
                result.innerHTML = `
                    <div class="bg-yellow-500/20 border border-yellow-500/30 rounded-2xl p-6 text-white">
                        <div class="flex items-center space-x-3 mb-4">
                            <div class="w-10 h-10 bg-yellow-500/30 rounded-full flex items-center justify-center">
                                <svg class="w-6 h-6 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold">Nice try, smarty pants! 🤦‍♂️</h3>
                        </div>
                        <p class="text-white/80">You literally just downloaded this video. Try a different URL or refresh the page if you want to download it again.</p>
                    </div>
                `;
                result.classList.remove('hidden');
                return;
            }
            
            // Reset UI
            button.disabled = true;
            button.innerHTML = `
                <span class="flex items-center justify-center space-x-2">
                    <div class="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
                    <span>Processing...</span>
                </span>
            `;
            loading.classList.remove('hidden');
            result.classList.add('hidden');
            
            try {
                // TEST MODE: Uncomment the next line to simulate success for testing
                // const testMode = true;
                
                let data;
                if (typeof testMode !== 'undefined' && testMode) {
                    // Simulate successful response
                    data = {
                        success: true,
                        file_url: `https://localhost:3000/files/06a75bee-bc99-4894-b184-c497d70ca7f5/video.mp4`
                    };
                } else {
                    // Real API call
                    const response = await fetch('/download', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify({ url: url })
                    });
                    
                    data = await response.json();
                }
                
                if (data.success) {
                    // Store the URL to prevent duplicate downloads
                    lastDownloadedUrl = url;
                    
                    // Extract filename from the URL
                    const fileName = data.file_url.split('/').pop().replaceAll('_', ' ');
                    
                    result.innerHTML = `
                        <div class="bg-green-500/20 border border-green-500/30 rounded-2xl p-6 text-white">
                            <div class="flex items-center space-x-3 mb-4">
                                <div class="w-10 h-10 bg-green-500/30 rounded-full flex items-center justify-center">
                                    <svg class="w-6 h-6 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                    </svg>
                                </div>
                                <h3 class="text-xl font-bold">Download Complete!</h3>
                            </div>
                            <p class="text-white/80 mb-4">Your video has been successfully downloaded and is ready to view.</p>
                            
                            <!-- Download Button -->
                            <div class="text-center mb-4">
                                ${fileName ? `<div class="mb-2 text-sm text-white/70">${fileName}</div>` : ''}
                                <a href="${data.file_url}" target="_blank" class="inline-flex items-center space-x-2 bg-green-500 hover:bg-green-600 text-white font-semibold py-3 px-6 rounded-xl transition-all duration-300 transform hover:scale-105">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                                    </svg>
                                    <span>Download File</span>
                                </a>
                            </div>
                            
                            <!-- Video Player -->
                            <div>
                                <video 
                                    controls 
                                    class="w-full rounded-xl shadow-lg"
                                    poster="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 9'%3E%3Crect width='16' height='9' fill='%23000'/%3E%3C/svg%3E"
                                >
                                    <source src="${data.file_url}" type="video/mp4">
                                    Your browser does not support the video tag.
                                </video>
                            </div>
                        </div>
                    `;
                } else {
                    result.innerHTML = `
                        <div class="bg-red-500/20 border border-red-500/30 rounded-2xl p-6 text-white">
                            <div class="flex items-center space-x-3 mb-4">
                                <div class="w-10 h-10 bg-red-500/30 rounded-full flex items-center justify-center">
                                    <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                    </svg>
                                </div>
                                <h3 class="text-xl font-bold">Download Failed</h3>
                            </div>
                            <p class="text-white/80">${data.error || 'An unknown error occurred while processing your request.'}</p>
                        </div>
                    `;
                }
            } catch (error) {
                result.innerHTML = `
                    <div class="bg-red-500/20 border border-red-500/30 rounded-2xl p-6 text-white">
                        <div class="flex items-center space-x-3 mb-4">
                            <div class="w-10 h-10 bg-red-500/30 rounded-full flex items-center justify-center">
                                <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold">Connection Error</h3>
                        </div>
                        <p class="text-white/80">Failed to connect to the server. Please check your connection and try again.</p>
                    </div>
                `;
            } finally {
                button.disabled = false;
                button.innerHTML = `
                    <span class="flex items-center justify-center space-x-2">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                        </svg>
                        <span>Download Video</span>
                    </span>
                `;
                loading.classList.add('hidden');
                result.classList.remove('hidden');
            }
        });
    </script>
</body>
</html>
        "#,
    )
}
