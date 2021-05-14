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

void customwin_set_win(CustomWin *self, GdkWindow *w) {
  self->win = w;
}