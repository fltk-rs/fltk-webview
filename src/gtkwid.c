#include <gdk/gdkx.h>
#include <assert.h>

#include "gtkwid.h"


G_DEFINE_TYPE(CustomWin, customwin, GTK_TYPE_WIDGET);

static void customwin_class_init(CustomWinClass *klass) {
  GtkWidgetClass *parent_widget = GTK_WIDGET_CLASS(klass);
}

static void customwin_init(CustomWin *self) {}

GtkWidget *customwin_new() {
  GtkWidget *w = g_object_new(customwin_get_type(), NULL);
  return w;
}

void customwin_set_win(CustomWin *self, long xid) {
  GdkDisplayManager *mn = gdk_display_manager_get();
  assert(mn);
  const gchar *env = getenv("DISPLAY");
  assert(env);
  GdkDisplay *disp = gdk_display_manager_open_display(mn, env);
  assert(disp);
  GdkWindow *win = gdk_x11_window_foreign_new_for_display(disp, xid);
  assert(win);
  self->win = win;
}