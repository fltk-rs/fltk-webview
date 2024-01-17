#import <Cocoa/Cocoa.h>
#include <assert.h>
#include <objc/runtime.h>

@implementation NSWindow (KeyWindow)

- (BOOL)my_canBecomeKeyWindow {
  return YES;
}

@end

@implementation NSView (FLView)
-(BOOL)reset_aux_bitmap {
  return YES;
}
@end

void add_nsmenu(bool val) {
  if (val) {
    id menubar = [[NSMenu alloc] initWithTitle:@""];
    id editMenu = [[NSMenu alloc] initWithTitle:@"Edit"];
    id editMenuItem = [NSMenuItem alloc];
    [editMenuItem setSubmenu:editMenu];
    [menubar addItem:editMenuItem];

    id item = [[NSMenuItem alloc] initWithTitle:@""
                                         action:@selector(hide:)
                                  keyEquivalent:@"h"];

    id appMenu = [NSMenu alloc];
    [appMenu addItem:item];

    item = [[NSMenuItem alloc] initWithTitle:@"Cut"
                                      action:@selector(cut:)
                               keyEquivalent:@"x"];
    [editMenu addItem:item];

    item = [[NSMenuItem alloc] initWithTitle:@"Copy"
                                      action:@selector(copy:)
                               keyEquivalent:@"c"];
    [editMenu addItem:item];

    item = [[NSMenuItem alloc] initWithTitle:@"Paste"
                                      action:@selector(paste:)
                               keyEquivalent:@"v"];
    [editMenu addItem:item];

    item = [[NSMenuItem alloc] initWithTitle:@"Select All"
                                      action:@selector(selectAll:)
                               keyEquivalent:@"a"];
    [editMenu addItem:item];
    [menubar autorelease];

    [[NSApplication sharedApplication] setMainMenu:menubar];
  }
}

void make_delegate(NSWindow *child, NSWindow *parent, add_menu: int) {
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
  add_nsmenu(add_menu);
}

void my_close_win(NSWindow *win) {
  NSView *view = [win contentView];
  [view removeFromSuperview];
  [win close];
}