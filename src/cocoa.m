#define OBJC_OLD_DISPATCH_PROTOTYPES 1
#import <Cocoa/Cocoa.h>
#include <assert.h>
#include <objc/message.h>
#include <objc/runtime.h>

@implementation NSWindow (KeyWindow)

- (BOOL)my_canBecomeKeyWindow {
  return YES;
}

@end

static id get_nsstring(const char *c_str) {
  return [NSString stringWithUTF8String:c_str];
}

static id create_menu_item(id title, const char *action, const char *key) {
  id item =
      objc_msgSend((id)objc_getClass("NSMenuItem"), sel_registerName("alloc"));
  objc_msgSend(item, sel_registerName("initWithTitle:action:keyEquivalent:"),
               title, sel_registerName(action), get_nsstring(key));
  objc_msgSend(item, sel_registerName("autorelease"));

  return item;
}

void add_nsmenu(bool val) {
  if (val) {
    id menubar =
        objc_msgSend((id)objc_getClass("NSMenu"), sel_registerName("alloc"));
    objc_msgSend(menubar, sel_registerName("initWithTitle:"), get_nsstring(""));
    objc_msgSend(menubar, sel_registerName("autorelease"));
    id editMenu =
        objc_msgSend((id)objc_getClass("NSMenu"), sel_registerName("alloc"));
    objc_msgSend(editMenu, sel_registerName("initWithTitle:"),
                 get_nsstring("Edit"));
    objc_msgSend(editMenu, sel_registerName("autorelease"));
    id editMenuItem = objc_msgSend((id)objc_getClass("NSMenuItem"),
                                   sel_registerName("alloc"));
    objc_msgSend(editMenuItem, sel_registerName("setSubmenu:"), editMenu);
    objc_msgSend(menubar, sel_registerName("addItem:"), editMenuItem);
    id title = objc_msgSend(get_nsstring("Hide "),
                            sel_registerName("stringByAppendingString:"),
                            get_nsstring("Webview"));
    id item = create_menu_item(title, "hide:", "h");
    id appMenu =
        objc_msgSend((id)objc_getClass("NSMenu"), sel_registerName("alloc"));
    objc_msgSend(appMenu, sel_registerName("addItem:"), item);

    item = create_menu_item(get_nsstring("Cut"), "cut:", "x");
    objc_msgSend(editMenu, sel_registerName("addItem:"), item);

    item = create_menu_item(get_nsstring("Copy"), "copy:", "c");
    objc_msgSend(editMenu, sel_registerName("addItem:"), item);

    item = create_menu_item(get_nsstring("Paste"), "paste:", "v");
    objc_msgSend(editMenu, sel_registerName("addItem:"), item);

    item = create_menu_item(get_nsstring("Select All"), "selectAll:", "a");
    objc_msgSend(editMenu, sel_registerName("addItem:"), item);

    objc_msgSend(objc_msgSend((id)objc_getClass("NSApplication"),
                              sel_registerName("sharedApplication")),
                 sel_registerName("setMainMenu:"), menubar);
  }
}

void make_delegate(NSWindow *child, NSWindow *parent) {
  [parent setDelegate:(id)child];
  [child orderWindow:NSWindowAbove relativeTo:[parent windowNumber]];
  Method old_method =
      class_getInstanceMethod([child class], @selector(canBecomeKeyWindow));
  Method new_method =
      class_getInstanceMethod([child class], @selector(my_canBecomeKeyWindow));
  assert(new_method);
  method_exchangeImplementations(old_method, new_method);
  [child setIgnoresMouseEvents:NO];
  [child makeKeyAndOrderFront:nil];
  add_nsmenu(true);
}

int send_event(void *event, void *data) {
  [(NSWindow *)data sendEvent:(NSEvent *)event];
  return 0;
}
