# clearly this file shouldn't be here!
# But it is so I don't forget how to build the sample!
cargo build -p loginsapi_ffi --target=i686-linux-android --release
# and this is a bit of a mess - the android sample app is outside
# this directory :(
cp target/i686-linux-android/release/libloginsapi_ffi.so ../application-services/logins-api/android/LoginsApi/loginsapi/src/main/jniLibs/x86/.
