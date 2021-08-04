#import <Cocoa/Cocoa.h>

void make_delegate(NSWindow *child, NSWindow *parent) {
  [parent setDelegate:(id)child];
  [child orderWindow:NSWindowAbove relativeTo:[parent windowNumber]];
  [child setIgnoresMouseEvents:NO];
  [child makeKeyAndOrderFront:nil];
}

int send_event(void *event, void *data) {
  [(NSWindow *)data sendEvent:(NSEvent *)event];
  return 0;
}