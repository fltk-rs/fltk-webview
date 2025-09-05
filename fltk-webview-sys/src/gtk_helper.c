#include <X11/Xlib.h>
#include <X11/extensions/Xfixes.h>
#include <gdk/gdk.h>
#include <gdk/gdkx.h>
#include <gtk/gtk.h>

long my_get_xid(GdkWindow *win) { return GDK_WINDOW_XID(win); }

GdkWindow *my_get_win(GtkWindow *win) {
  GdkWindow *w = gtk_widget_get_window(GTK_WIDGET(win));
  return w;
}

void x_init(Display *disp, Window child, Window parent) {
  // Make the child unmanaged (override-redirect) to avoid WM interference (e.g., Mutter)
  XUnmapWindow(disp, child);
  XSetWindowAttributes attrs;
  attrs.override_redirect = True;
  XChangeWindowAttributes(disp, child, CWOverrideRedirect, &attrs);

  // Position and reparent under the FLTK window
  XMoveWindow(disp, child, 0, 0);
  XReparentWindow(disp, child, parent, 0, 0);

  // Ensure the child remains mapped with the parent
  XFixesChangeSaveSet(disp, child, SetModeInsert, SaveSetRoot, SaveSetUnmap);
  XMapWindow(disp, child);
  XSync(disp, False);

  // Send a synthetic ConfigureNotify to ensure size is correct
  XEvent client_event;
  XWindowAttributes childAttributes;
  XWindowAttributes parentAttributes;
  XGetWindowAttributes(disp, child, &childAttributes);
  XGetWindowAttributes(disp, parent, &parentAttributes);

  client_event.type = ConfigureNotify;
  client_event.xconfigure.send_event = True;
  client_event.xconfigure.display = disp;
  client_event.xconfigure.event = child;
  client_event.xconfigure.window = child;
  client_event.xconfigure.x = 0;
  client_event.xconfigure.y = 0;
  client_event.xconfigure.width = childAttributes.width;
  client_event.xconfigure.height = childAttributes.height;
  client_event.xconfigure.border_width = 0;
  client_event.xconfigure.above = None;
  client_event.xconfigure.override_redirect = True;

  XSendEvent(disp, child, False, StructureNotifyMask, &client_event);
}

void x_focus(Display *disp, Window child) {
  XSetInputFocus(disp, child, RevertToParent, CurrentTime);
}

int my_gtk_events_pending() { return gtk_events_pending(); }
