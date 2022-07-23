# rust-jni-lib
rust-jni-lib

```java
package com.doopp.news.boot.util;

import com.doopp.news.util.NativeUtil;

import java.io.IOException;

public class JniUtil {

    static {
        try {
            /**
             * @see <a href="https://github.com/adamheinrich/native-utils">https://github.com/adamheinrich/native-utils</a>
             */
            NativeUtil.loadLibraryFromJar("/jni_lib/libjniutil.so");
        }
        catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    // get the string , need trim() in java
    public static native String dpDec(String encText);
}
```