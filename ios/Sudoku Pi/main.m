@import UIKit;

void run_with_scales_and_padding(double, double, double);

int main(int argc, char *argv[]) {
    UIWindow *window = UIApplication.sharedApplication.windows.firstObject;
    CGFloat topPadding = window.safeAreaInsets.top;

    UIScreen *screen = [UIScreen mainScreen];
    CGFloat scale = screen.scale;
    CGFloat nativeScale = screen.nativeScale;

    run_with_scales_and_padding(scale, nativeScale, topPadding);
    return 0;
}
