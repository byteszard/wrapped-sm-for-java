use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jobjectArray, jstring};


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm2_1generate_1keypair(env: JNIEnv, _: JClass) -> jobjectArray {
    let (private_key, public_key) = {
        let (private_key, public_key) = yarism::sm2::generate_keypair();
        let prk = env.new_string(private_key).unwrap();
        let puk = env.new_string(public_key).unwrap();
        (prk, puk)
    };

    let out = {
        let e = "java/lang/String";
        let t = env.new_object_array(2, e, JObject::null()).unwrap();

        env.set_object_array_element(t, 0, private_key).unwrap();
        env.set_object_array_element(t, 1, public_key).unwrap();
        t
    };

    out
}

#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm2_1encrypt(env: JNIEnv, _: JClass, public_key: JString, plain: JString) -> jstring {
    let key: String = env.get_string(public_key).unwrap().into();
    let plain: String = env.get_string(plain).unwrap().into();
    let cipher = yarism::sm2::encrypt(&key, &plain);
    let out = env.new_string(cipher).unwrap();
    out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm2_1decrypt(env: JNIEnv, _: JClass, private_key: JString, cipher: JString) -> jstring {
    let key: String = env.get_string(private_key).unwrap().into();
    let cipher: String = env.get_string(cipher).unwrap().into();
    let text = yarism::sm2::decrypt(&key, &cipher);
    let out = env.new_string(text).unwrap();
    out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm2_1encrypt_1c1c2c3(env: JNIEnv, _: JClass, public_key: JString, plain: JString) -> jstring {
    let key: String = env.get_string(public_key).unwrap().into();
    let plain: String = env.get_string(plain).unwrap().into();
    let cipher = yarism::sm2::encrypt_c1c2c3(&key, &plain);
    let out = env.new_string(cipher).unwrap();
    out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm2_1decrypt_1c1c2c3(env: JNIEnv, _: JClass, private_key: JString, cipher: JString) -> jstring {
    let key: String = env.get_string(private_key).unwrap().into();
    let cipher: String = env.get_string(cipher).unwrap().into();
    let text = yarism::sm2::decrypt_c1c2c3(&key, &cipher);
    let out = env.new_string(text).unwrap();
    out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm2_1sign(env: JNIEnv, _: JClass, private_key: JString, public_key: JString, plain: JString) -> jstring {
    let prk: String = env.get_string(private_key).unwrap().into();
    let puk: String = env.get_string(public_key).unwrap().into();
    let plain: String = env.get_string(plain).unwrap().into();
    let signature = yarism::sm2::sign(&prk, &puk, &plain);
    let out = env.new_string(signature).unwrap();
    out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm2_1verify(env: JNIEnv, _: JClass, public_key: JString, plain: JString, signature: JString) -> jboolean {
    let key: String = env.get_string(public_key).unwrap().into();
    let plain: String = env.get_string(plain).unwrap().into();
    let signature: String = env.get_string(signature).unwrap().into();
    let valid = yarism::sm2::verify(&key, &plain, &signature);
    jboolean::from(valid)
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm3_1digest(env: JNIEnv, _: JClass, plain: JString) -> jstring {
    let plain: String = env.get_string(plain).unwrap().into();
    let cipher = yarism::sm3::digest(&plain);
    let out = env.new_string(cipher).unwrap();
    out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1generate_1key(env: JNIEnv, _: JClass) -> jstring {
    let key = yarism::sm4::generate_key();
    let out = env.new_string(key).unwrap();
    out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1generate_1iv(env: JNIEnv, _: JClass) -> jstring {
    let iv = yarism::sm4::generate_iv();
    let out = env.new_string(iv).unwrap();
    out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1encrypt_1ecb(env: JNIEnv, _: JClass, key: JString, plain: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let plain: String = env.get_string(plain).unwrap().into();
    let cipher = yarism::sm4::encrypt_ecb(key, plain);
    let out = env.new_string(cipher).unwrap();
    out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1decrypt_1ecb(env: JNIEnv, _: JClass, key: JString, cipher: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let cipher: String = env.get_string(cipher).unwrap().into();
    let plain = yarism::sm4::decrypt_ecb(key, cipher);
    let out = env.new_string(plain).unwrap();
    out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1encrypt_1cbc(env: JNIEnv, _: JClass, key: JString, iv: JString, plain: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let iv: String = env.get_string(iv).unwrap().into();
    let plain: String = env.get_string(plain).unwrap().into();
    let cipher = yarism::sm4::encrypt_cbc(key, iv, plain);
    let out = env.new_string(cipher).unwrap();
    out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1decrypt_1cbc(env: JNIEnv, _: JClass, key: JString, iv: JString, cipher: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let iv: String = env.get_string(iv).unwrap().into();
    let cipher: String = env.get_string(cipher).unwrap().into();
    let plain = yarism::sm4::decrypt_cbc(key, iv, cipher);
    let out = env.new_string(plain).unwrap();
    out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1encrypt_1cfb(env: JNIEnv, _: JClass, key: JString, iv: JString, plain: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let iv: String = env.get_string(iv).unwrap().into();
    let plain: String = env.get_string(plain).unwrap().into();
    let cipher = yarism::sm4::encrypt_cfb(key, iv, plain);
    let out = env.new_string(cipher).unwrap();
    out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1decrypt_1cfb(env: JNIEnv, _: JClass, key: JString, iv: JString, cipher: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let iv: String = env.get_string(iv).unwrap().into();
    let cipher: String = env.get_string(cipher).unwrap().into();
    let plain = yarism::sm4::decrypt_cfb(key, iv, cipher);
    let out = env.new_string(plain).unwrap();
    out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1encrypt_1ofb(env: JNIEnv, _: JClass, key: JString, iv: JString, plain: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let iv: String = env.get_string(iv).unwrap().into();
    let plain: String = env.get_string(plain).unwrap().into();
    let cipher = yarism::sm4::encrypt_ofb(key, iv, plain);
    let out = env.new_string(cipher).unwrap();
    out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1decrypt_1ofb(env: JNIEnv, _: JClass, key: JString, iv: JString, cipher: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let iv: String = env.get_string(iv).unwrap().into();
    let cipher: String = env.get_string(cipher).unwrap().into();
    let plain = yarism::sm4::decrypt_ofb(key, iv, cipher);
    let out = env.new_string(plain).unwrap();
    out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1encrypt_1ctr(env: JNIEnv, _: JClass, key: JString, iv: JString, plain: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let iv: String = env.get_string(iv).unwrap().into();
    let plain: String = env.get_string(plain).unwrap().into();
    let cipher = yarism::sm4::encrypt_ctr(key, iv, plain);
    let out = env.new_string(cipher).unwrap();
    out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_wonders_wsm4j_Rust_sm4_1decrypt_1ctr(env: JNIEnv, _: JClass, key: JString, iv: JString, cipher: JString) -> jstring {
    let key: String = env.get_string(key).unwrap().into();
    let iv: String = env.get_string(iv).unwrap().into();
    let cipher: String = env.get_string(cipher).unwrap().into();
    let plain = yarism::sm4::decrypt_ctr(key, iv, cipher);
    let out = env.new_string(plain).unwrap();
    out.into_inner()
}