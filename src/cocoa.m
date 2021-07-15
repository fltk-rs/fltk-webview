#import <Cocoa/Cocoa.h>

void make_delegate(NSWindow *child, NSWindow *parent) {
    [parent setDelegate:(id)child];
    [child makeKeyAndOrderFront:nil];
}