@import UIKit;

void run_with_fixed_sizes(double, double, double, double, double, bool);

int main(int argc, char *argv[]) {
    //CGSize statusBarSize = [[UIApplication sharedApplication] statusBarFrame].size;
    CGFloat topPadding = 44.0;

    UIScreen *screen = [UIScreen mainScreen];
    CGFloat width = screen.bounds.size.width;
    CGFloat height = screen.bounds.size.height;
    CGFloat scale = screen.scale;
    CGFloat nativeScale = screen.nativeScale;

    run_with_fixed_sizes(
        width,
        height,
        scale,
        nativeScale,
        topPadding,
        [[UIDevice currentDevice].model hasPrefix:@"iPad"]
    );
    return 0;
}
