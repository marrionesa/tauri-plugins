(function () {
  const __TAURI_INVOKE__ = window.__TAURI_INTERNALS__.invoke
  window.__TAURI__ = window.__TAURI__ || {}
  window.__TAURI__.hash = {
    hashString: (input, algorithm) => __TAURI_INVOKE__('plugin:hash|hash_string', { input, algorithm }),
    hashFile: (path, algorithm) => __TAURI_INVOKE__('plugin:hash|hash_file', { path, algorithm }),
  }
})()
