(function () {
  const __TAURI_INVOKE__ = window.__TAURI_INTERNALS__.invoke
  window.__TAURI__ = window.__TAURI__ || {}
  window.__TAURI__.beep = {
    beep: () => __TAURI_INVOKE__('plugin:beep|beep'),
  }
})()
