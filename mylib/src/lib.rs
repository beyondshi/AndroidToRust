// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;
// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString, JValue};
// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_com_example_rustandandroid_HelloWord_hello(
    env: JNIEnv,
    class: JClass,
    input: JString)
    -> jstring {
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.

    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String =
        env.get_string(input).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_raw()
}

#[allow(non_snake_case)]
fn call_java(env: &JNIEnv) {
    let File = env.find_class("java/io/File")?;
    // 获取静态字段
    let separator = env.get_static_field(File, "separator", "Ljava/lang/String;")?;
    let separator = env
        .get_string(separator.l()?.into())?
        .to_string_lossy()
        .to_string();
    println!("File.separator: {}", separator);
    assert_eq!(separator, format!("{}", std::path::MAIN_SEPARATOR));
    // env.get_static_field_unchecked(class, field, ty)

    // 创建实例对象
    let file = env.new_object(
        "java/io/File",
        "(Ljava/lang/String;)V",
        &[JValue::Object(env.new_string("")?.into())],
    )?;

    // 调用实例方法
    let abs = env.call_method(file, "getAbsolutePath", "()Ljava/lang/String;", &[])?;
    let abs_path = env
        .get_string(abs.l()?.into())?
        .to_string_lossy()
        .to_string();
    println!("abs_path: {}", abs_path);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_rustandandroid_HelloWord_callback(env: JNIEnv) {
    println!("call java");
    call_java(&env);
    // 创建实例对象
    let obj = env.new_object(
        " com.example.rustandandroid.HelloWord",
        "()V",
        &[],
    );
    env.call_static_method(obj.unwrap(), "rustCallBack", "()V", &[]).unwrap();
}
