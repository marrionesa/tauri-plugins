(function () {
  const __TAURI_INVOKE__ = window.__TAURI_INTERNALS__.invoke
  window.__TAURI__ = window.__TAURI__ || {}
  window.__TAURI__.timezone = {
    getTimezone: () => __TAURI_INVOKE__('plugin:timezone|get_timezone'),
    getUtcOffsetSeconds: () => __TAURI_INVOKE__('plugin:timezone|get_utc_offset_seconds'),
  }
})()
