WIP notes for sqlcipher
=======================

This branch attempts to get mentat using sqlcipher. It could probably be
done using a "feature" in the same way [bundled_sqlite3] is, but this is
just trying to make sure it works.

Only x86 is supported - see the new `build.rs` - it's probably easy to
have that script support other targets.

You need to download http://repo1.maven.org/maven2/net/zetetic/android-database-sqlcipher/3.5.9/android-database-sqlcipher-3.5.9.aar,
and unzip it (it's just a .zip file) into build/external/ - so there should be a file
build/external/android-database-sqlcipher-3.5.9/jni/x86/libsqlcipher.so

Then `cargo build -p mentat_ffi --target=i686-linux-android --release` should work!

I then started trying to work out how to get toodle using it too and
have so far failed - the builds always seem to fail with:

`Builds with bundled SQLCipher are not supported`

Even though I've changed the rusqlite from `features = ["bundled", "limits"]`
to `features = ["sqlcipher", "limits"]` and changed `Cargo.toml` to point at
a local mentat.