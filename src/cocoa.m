#import <Cocoa/Cocoa.h>

void make_delegate(NSWindow *child, NSWindow *parent) {
  [parent setDelegate:(id)child];
  [child orderWindow:NSWindowAbove relativeTo:[parent windowNumber]];
  [child setIgnoresMouseEvents:NO];
  [child makeKeyAndOrderFront:nil];
}