@import UIKit;

void run_with_scales_and_padding(double, double, double);

int main(int argc, char *argv[]) {
    //CGSize statusBarSize = [[UIApplication sharedApplication] statusBarFrame].size;
    CGFloat topPadding = 44.0;

    UIScreen *screen = [UIScreen mainScreen];
    CGFloat scale = screen.scale;
    CGFloat nativeScale = screen.nativeScale;

    run_with_scales_and_padding(scale, nativeScale, topPadding);
    return 0;
}
