package com.example.rustandandroid;

public class HelloWord {
    // a native library.
    private static native String hello(String input);

    private static native String callback();

    public static void rustCallBack() {
        System.out.println("\n rustCallBack");
    }

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.load("/Users/home/Android/RustAndAndroid/mylib/target/debug/libmylib.dylib");
    }

    public static void main(String[] args) {
        String output = HelloWord.hello("josh");

        HelloWord.callback();

        System.out.println(output);
        System.out.println("text");
    }
}
