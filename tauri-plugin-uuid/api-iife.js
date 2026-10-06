(function () {
  const __TAURI_INVOKE__ = window.__TAURI_INTERNALS__.invoke
  window.__TAURI__ = window.__TAURI__ || {}
  window.__TAURI__.uuid = {
    v4: () => __TAURI_INVOKE__('plugin:uuid|generate_v4'),
    v7: () => __TAURI_INVOKE__('plugin:uuid|generate_v7'),
  }
})()
