# rust-jni-lib
rust-jni-lib

```java
package com.doopp.news.boot.util;

import com.doopp.news.util.NativeUtil;

import java.io.IOException;

public class JniUtil {

    static {
        String osName = System.getProperties().getProperty("os.name");
        // other linux
        String nativeLibrary = "/jni_lib/libjniutil.so";
        // win
        if (osName.contains("Win")) {
            nativeLibrary = "/jni_lib/jniutil.dll";
        }
        // mac
        else if (osName.contains("Mac")) {
            nativeLibrary = "/jni_lib/libjniutil.dylib";
        }
        try {
            NativeUtil.loadLibraryFromJar(nativeLibrary);
        }
        catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    // encrypt
    public static native String dpEnc(String text);

    // decrypt
    public static native String dpDec(String encText);
}
```
