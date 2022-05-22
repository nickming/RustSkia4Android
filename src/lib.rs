mod canvas;

use canvas::Canvas;

use std::fs::File;
use std::io::Write;
use jni::objects::{JClass, JObject};
use jni::JNIEnv;
use jni::sys::{jbyteArray, jint};

use std::{sync::mpsc, thread, time::Duration};

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
    let width = width as i32;
    let height = height as i32;
    let mut canvas = Canvas::new(width, height);
    let half_size = (width / 2) as f32;
    canvas.move_to(1.0, 1.0);
    canvas.line_to(1.0, half_size);
    canvas.line_to(half_size, half_size);
    canvas.line_to(half_size, 1.0);
    canvas.set_line_width(2.0);
    canvas.stroke();
    let data = canvas.data();
    let bytes = data.as_bytes();
    env.byte_array_from_slice(bytes).unwrap()
}

pub unsafe extern fn Java_com_example_myapplication_SkiaLibrary_drawAsync(env: JNIEnv, _: JClass, width: jint, height: jint, callback: JObject) {
    let jvm = env.get_java_vm().unwrap();
    let width = width as i32;
    let height = height as i32;
    let callback = env.new_global_ref(callback).unwrap();

    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        tx.send(()).unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let mut canvas = Canvas::new(width, height);
        let half_size = (width / 2) as f32;
        canvas.move_to(1.0, 1.0);
        canvas.line_to(1.0, half_size);
        canvas.line_to(half_size, half_size);
        canvas.line_to(half_size, 1.0);
        canvas.set_line_width(2.0);
        canvas.stroke();
        let data = canvas.data();
        let bytes = data.as_bytes();
        let j_bytes = env.byte_array_from_slice(bytes).unwrap();
        env.call_method(&callback, "onSuccess", "(II)[B", &[j_bytes.into()]).unwrap();
    });
    rx.recv().unwrap()
}