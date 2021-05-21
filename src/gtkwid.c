#include <gdk/gdk.h>
#include <gtk/gtk.h>
#include <gdk/gdkx.h>

long my_get_xid(GdkWindow *win) { return GDK_WINDOW_XID(win); }

GdkWindow *my_get_win(GtkWindow *win) {
  GdkWindow *w = gtk_widget_get_window(GTK_WIDGET(win));
  return w;
}