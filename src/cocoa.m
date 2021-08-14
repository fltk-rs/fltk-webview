#import <Cocoa/Cocoa.h>
#import <objc/runtime.h>
#include <assert.h>

@implementation NSWindow (KeyWindow)

- (BOOL)my_canBecomeKeyWindow {
  return YES;
}

@end

void make_delegate(NSWindow *child, NSWindow *parent) {
  [parent setDelegate:(id)child];
  [child orderWindow:NSWindowAbove relativeTo:[parent windowNumber]];
  Method old_method = class_getInstanceMethod([child class], @selector(canBecomeKeyWindow));
  Method new_method = class_getInstanceMethod([child class], @selector(my_canBecomeKeyWindow));
  assert(new_method);
  method_exchangeImplementations(old_method, new_method);
  [child setIgnoresMouseEvents:NO];
  [child makeKeyAndOrderFront:nil];
}

int send_event(void *event, void *data) {
  [(NSWindow *)data sendEvent:(NSEvent *)event];
  return 0;
}
