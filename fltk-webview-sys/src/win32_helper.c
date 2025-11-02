#include "webview.h"
#include "WebView2.h"

void move_focus(webview_t webview)
{
  if (webview)
  {
    ICoreWebView2Controller *controller_ptr =
        (ICoreWebView2Controller *)webview_get_native_handle(
            webview, WEBVIEW_NATIVE_HANDLE_KIND_BROWSER_CONTROLLER);
    if (controller_ptr)
    {
      controller_ptr->lpVtbl->MoveFocus(
          controller_ptr, COREWEBVIEW2_MOVE_FOCUS_REASON_PROGRAMMATIC);
    }
  }
}