(function () {
  const __TAURI_INVOKE__ = window.__TAURI_INTERNALS__.invoke
  window.__TAURI__ = window.__TAURI__ || {}
  window.__TAURI__.mimetype = {
    fromExtension: (extension) => __TAURI_INVOKE__('plugin:mimetype|from_extension', { extension }),
    fromPath: (path) => __TAURI_INVOKE__('plugin:mimetype|from_path', { path }),
  }
})()
