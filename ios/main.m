#import <UIKit/UIKit.h>

// Entry point implemented in Rust and exported from libsudoku_pi.a (src/lib.rs).
// This must NOT call UIApplicationMain itself: winit's iOS event loop calls
// UIApplicationMain when the Rust side runs, and it asserts that UIApplication
// has not already been created.
extern void sudoku_pi_start(void);

int main(int argc, char *argv[]) {
    @autoreleasepool {
        sudoku_pi_start();
    }
    return 0;
}
