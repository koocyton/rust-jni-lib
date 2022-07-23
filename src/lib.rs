// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping this context
// and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function.
// We can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

use std::error::Error;
use crypto::aes::KeySize::KeySize128;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};

const DP_CBC_KEY: &str    = "1111111111111111";
const DP_CBC_OFFSET: &str = "1111111111111111";

#[no_mangle]
pub extern "system" fn Java_com_doopp_news_boot_util_JniUtil_dpEnc(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    input: JString,
) -> jstring {
    // First, we have to get the string out of java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();
    // Then we have to create a new java string to return. Again, more info
    // in the `strings` module.
    let enc_str: String = encrypt(DP_CBC_KEY, DP_CBC_OFFSET, input.as_str()).unwrap();
    let output = env
        .new_string(enc_str)
        .expect("Couldn't get encrypt string!");
    // Finally, extract the raw pointer to return.
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_doopp_news_boot_util_JniUtil_dpDec(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    input: JString,
) -> jstring {
    // First, we have to get the string out of java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();
    // Then we have to create a new java string to return. Again, more info
    // in the `strings` module.
    let dec_str: String = decrypt(DP_CBC_KEY, DP_CBC_OFFSET, input.as_str()).unwrap();
    let output = env
        .new_string(dec_str)
        .expect("Couldn't get decrypt string!");
    // Finally, extract the raw pointer to return.
    output.into_inner()
}

/// 加密
fn encrypt(key: &str, iv: &str, text: &str) -> Result<String, Box<dyn Error>> {
    let mut encrypt = crypto::aes::cbc_encryptor(
        KeySize128,
        key.as_bytes(),
        iv.as_bytes(),
        PkcsPadding
    );
    let mut read_buffer = RefReadBuffer::new(text.as_bytes());
    let mut result = vec![0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut result);
    encrypt.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    Ok(base64::encode_config(trim(&result)?, base64::STANDARD))
}

/// 解密
fn decrypt(key: &str, iv: &str, input: &str) -> Result<String, Box<dyn Error>> {
    let mut decrypt = crypto::aes::cbc_decryptor(
        KeySize128,
        key.as_bytes(),
        iv.as_bytes(),
        PkcsPadding
    );
    let base_text = base64::decode_config(input, base64::STANDARD)?;
    let mut read_buffer = RefReadBuffer::new(&base_text);
    let mut result = vec![0; input.len()];
    let mut write_buffer = RefWriteBuffer::new(&mut result);
    decrypt.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    let dec_str = String::from_utf8(trim(&result)?)?;
    Ok(dec_str.trim().to_string())
}

fn trim(input: &Vec<u8>)  -> Result<Vec<u8>, Box<dyn Error>> {
    let zero : u8 = 0;
    let mut fz_idx = input.len();
    for (idx, val) in input.iter().rev().enumerate() {
        if &zero!=val
        {
            fz_idx = fz_idx - idx;
            break;
        }
    }
    Ok(input[0..fz_idx].to_vec())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}