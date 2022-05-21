mod canvas;

use canvas::Canvas;

use std::fs::File;
use std::io::Write;
use jni::objects::JClass;
use jni::JNIEnv;
use jni::sys::{jbyteArray, jint};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


#[no_mangle]
pub unsafe extern fn Java_com_example_myapplication_SkiaLibrary_draw(env: JNIEnv, _: JClass, width: jint, height: jint) -> jbyteArray {
    let width = width as u32;
    let height = height as u32;
    let mut canvas = Canvas::new(2560, 1280);
    canvas.scale(1.2, 1.2);
    canvas.move_to(36.0, 48.0);
    canvas.quad_to(660.0, 880.0, 1200.0, 360.0);
    canvas.translate(10.0, 10.0);
    canvas.set_line_width(20.0);
    canvas.stroke();
    canvas.save();
    canvas.move_to(30.0, 90.0);
    canvas.line_to(110.0, 20.0);
    canvas.line_to(240.0, 130.0);
    canvas.line_to(60.0, 130.0);
    canvas.line_to(190.0, 20.0);
    canvas.line_to(270.0, 90.0);
    canvas.fill();
    let data = canvas.data();
    let bytes = data.as_bytes();
    env.byte_array_from_slice(bytes).unwrap()
}