(function () {
  const __TAURI_INVOKE__ = window.__TAURI_INTERNALS__.invoke
  window.__TAURI__ = window.__TAURI__ || {}
  window.__TAURI__.hostname = {
    hostname: () => __TAURI_INVOKE__('plugin:hostname|hostname'),
    username: () => __TAURI_INVOKE__('plugin:hostname|username'),
  }
})()
