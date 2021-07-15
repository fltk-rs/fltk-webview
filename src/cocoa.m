#import <Cocoa/Cocoa.h>

void cocoa_reparent(NSWindow *child, NSWindow *parent) {
    NSView *parent_view = [parent contentView];
    NSView *child_view = [[child contentView] retain];
    [child_view removeFromSuperview];
    [parent_view addSubview:child_view positioned:NSWindowAbove relativeTo:nil];
    [child_view acceptsFirstResponder];
    [child close];
}