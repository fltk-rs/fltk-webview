#ifndef __CUSTOMWIN_H__
#define __CUSTOMWIN_H__

#include <gdk/gdk.h>
#include <gtk/gtk.h>

G_BEGIN_DECLS

typedef struct _CustomWinClass {
  GtkWidgetClass parent_class;
} CustomWinClass;

typedef struct _CustomWin {
  GtkWidget parent;
  GdkWindow *win;
} CustomWin;

#define CUSTOMWIN_TYPE (customwin_get_type())
#define CUSTOMWIN(obj) GTK_CHECK_CAST(obj, customwin_get_type(), CustomWin)
#define IS_CUSTOMWIN(obj) GTK_CHECK_TYPE(obj, customwin_get_type())
#define CUSTOMWIN_CLASS(klass)                                                 \
  GTK_CHECK_CLASS_CAST(klass, customwin_get_type(), CustomWin)

GType customwin_get_type(void);
GtkWidget *customwin_new(void);
void customwin_set_win(CustomWin *self, GdkWindow *w);

G_END_DECLS

#endif