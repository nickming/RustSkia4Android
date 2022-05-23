use std::{sync::mpsc, thread};

use android_logger::Config;
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::{jbyteArray, jint};
use log::{info, Level};

use canvas::Canvas;

mod canvas;

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
    let bytes = draw_canvas(width, height);
    let bytes = bytes.as_slice();
    env.byte_array_from_slice(bytes).unwrap()
}

#[no_mangle]
#[allow(non_snake_case)]
#[link(name = "log")]
pub unsafe extern fn Java_com_example_myapplication_SkiaLibrary_drawAsync(env: JNIEnv, _: JClass, width: jint, height: jint, callback: JObject) {
    let jvm = env.get_java_vm().unwrap();
    let width = width as i32;
    let height = height as i32;
    let callback = env.new_global_ref(callback).unwrap();

    android_logger::init_once(Config::default().with_min_level(Level::Debug).with_tag("skia_rust_lib"));
    info!("draw async start");

    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        tx.send(()).unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let bytes = draw_canvas(width, height);
        let bytes = bytes.as_slice();
        let j_bytes = env.byte_array_from_slice(bytes).unwrap();
        match env.call_method(&callback, "onSuccess", "([B)V", &[j_bytes.into()]) {
            Ok(_) => {
                info!("call native method success!");
            }
            Err(e) => {
                info!("call native method failed:{:?}",e);
            }
        }
    });
    rx.recv().unwrap()
}

fn draw_canvas(width: i32, height: i32) -> Vec<u8> {
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
    Vec::from(bytes)
}