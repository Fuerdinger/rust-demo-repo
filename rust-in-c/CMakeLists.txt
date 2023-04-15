cmake_minimum_required (VERSION 3.9)

project("rust-in-c"
	VERSION 1.0.0
	DESCRIPTION "Rust in C"
	HOMEPAGE_URL ""
	LANGUAGES C
)

add_subdirectory(rust-foo)

add_executable(${PROJECT_NAME} src/foo.c)
target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/rust-foo/target)

target_link_libraries(${PROJECT_NAME} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/rust-foo/target/debug/rust_foo.lib)

#list obtained from: rustc --crate-type staticlib --print native-static-libs src/lib.rs
target_link_libraries(${PROJECT_NAME} PRIVATE kernel32.lib advapi32.lib userenv.lib ws2_32.lib bcrypt.lib msvcrt.lib legacy_stdio_definitions.lib)
