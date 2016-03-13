#!/usr/bin/sh

echo "/// Generated automatically at build time" > src/libtextcatpath.rs
echo "pub const LIBTEXTCAT_LANGUAGES_PATH : &'static str = \"$1\";" >> src/libtextcatpath.rs

