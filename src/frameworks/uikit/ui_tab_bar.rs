use crate::frameworks::foundation::NSInteger;
use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITabBar: UIControl

- (())setItems:(NSInteger)items animated:(bool)_animated {
  // TODO
}


@end

};
