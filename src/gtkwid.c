#include <X11/Xlib.h>
#include <X11/extensions/Xfixes.h>
#include <gdk/gdk.h>
#include <gdk/gdkx.h>
#include <gtk/gtk.h>
#include <unistd.h>

long my_get_xid(GdkWindow *win) { return GDK_WINDOW_XID(win); }

GdkWindow *my_get_win(GtkWindow *win) {
  GdkWindow *w = gtk_widget_get_window(GTK_WIDGET(win));
  return w;
}

// copied from https://stackoverflow.com/a/40902444
void x_init(Display *display, Window childWindowId, Window parentWindowId) {
  XMoveWindow(display, childWindowId, 0, 0);

  XReparentWindow(display, childWindowId, parentWindowId, 0, 0);

  XFixesChangeSaveSet(display, childWindowId, SetModeInsert, SaveSetRoot,
                      SaveSetUnmap);

  XEvent client_event;
  XWindowAttributes childAttributes;
  XWindowAttributes parentAttributes;
  XGetWindowAttributes(display, childWindowId, &childAttributes);
  XGetWindowAttributes(display, parentWindowId, &parentAttributes);

  client_event.type = ConfigureNotify;
  client_event.xconfigure.send_event = True;
  client_event.xconfigure.display = display;
  client_event.xconfigure.event = childWindowId;
  client_event.xconfigure.window = childWindowId;
  client_event.xconfigure.width = childAttributes.width;
  client_event.xconfigure.height = childAttributes.height;
  client_event.xconfigure.border_width = 0;
  client_event.xconfigure.above = None;
  client_event.xconfigure.override_redirect = True;

  XSendEvent(display, childWindowId, False, StructureNotifyMask, &client_event);
}

void x_reparent(Display *display, Window childWin, Window parentWin) {
  Window root, parent, *ch;
  unsigned int nch;
  XQueryTree(display, childWin, &root, &parent, &ch, &nch);
  if (parent != parentWin) {
    XReparentWindow(display, childWin, parentWin, 0, 0);
  }
  if (nch > 0) {
    XFree(ch);
  }
  XFlush(display);
  // usleep(3e5);
}