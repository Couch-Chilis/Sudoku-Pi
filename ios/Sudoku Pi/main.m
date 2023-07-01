@import UIKit;

void run_with_scales(double, double);

int main(int argc, char *argv[]) {
    UIScreen *screen = [UIScreen mainScreen];
    CGFloat scale = screen.scale;
    CGFloat nativeScale = screen.nativeScale;
    
    run_with_scales(scale, nativeScale);
    return 0;
}
