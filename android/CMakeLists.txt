# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html

# Sets the minimum version of CMake required to build the native library.

cmake_minimum_required(VERSION 3.4.1)

include(ExternalProject)

# Creates and names a library, sets it as either STATIC
# or SHARED, and provides the relative paths to its source code.
# You can define multiple libraries, and CMake builds them for you.
# Gradle automatically packages shared libraries with your APK.

add_library( # Sets the name of the library.
             native-lib

             # Sets the library as a shared library.
             SHARED

             # Provides a relative path to your source file(s).
             akriti/src/main/cpp/native-lib.cpp )


if ( ${CMAKE_BUILD_TYPE} STREQUAL "Debug" )
    SET ( CARGO_BUILD_DIR "debug" )
    SET ( CARGO_BUILD_FLAG "")
elseif ( ${CMAKE_BUILD_TYPE} STREQUAL "Release" )
    SET ( CARGO_BUILD_DIR "release" )
    SET ( CARGO_BUILD_FLAG "--release")
else()
    message( FATAL_ERROR "Unknown build mode ${CMAKE_BUILD_TYPE}" )
endif()

if ( ${ANDROID_ABI} STREQUAL "x86" OR ${ANDROID_ABI} STREQUAL "x86_64" )
    SET ( CARGO_TARGET i686-linux-android )
elseif ( ${ANDROID_ABI} STREQUAL "armeabi-v7a" )
    SET ( CARGO_TARGET armv7-linux-androideabi )
elseif ( ${ANDROID_ABI} STREQUAL "arm64-v8a" )
    SET ( CARGO_TARGET aarch64-linux-android )
else()
    message( FATAL_ERROR "Unknown architecture ${ANDROID_ABI}" )
endif()

set ( CARGO_PROJECT_DIR ${CMAKE_SOURCE_DIR} )
set ( CARGO_BUILD_DIR ${CARGO_PROJECT_DIR}/target/${CARGO_TARGET}/${CARGO_BUILD_DIR} )
set ( AKRITI_CORE_LIB ${CARGO_BUILD_DIR}/libakriti_android.a )

MESSAGE ( STATUS ${AKRITI_CORE_LIB} )

ExternalProject_Add(
    akriti-core-build
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    PREFIX "${CARGO_PROJECT_DIR}"
    BUILD_COMMAND cargo build ${CARGO_BUILD_FLAG} --target ${CARGO_TARGET}
    BUILD_BYPRODUCTS ${AKRITI_CORE_LIB}
    BINARY_DIR "${CARGO_BUILD_DIR}"
    INSTALL_COMMAND ""
    ALWAYS 1
    LOG_BUILD ON)

add_dependencies(native-lib akriti-core-build)

add_library(akriti-core STATIC IMPORTED)
set_target_properties( akriti-core PROPERTIES IMPORTED_LOCATION ${AKRITI_CORE_LIB} )

# Searches for a specified prebuilt library and stores the path as a
# variable. Because CMake includes system libraries in the search path by
# default, you only need to specify the name of the public NDK library
# you want to add. CMake verifies that the library exists before
# completing its build.

find_library( log-lib log )

# Specifies libraries CMake should link to your target library. You
# can link multiple libraries, such as libraries you define in this
# build script, prebuilt third-party libraries, or system libraries.

target_link_libraries( # Specifies the target library.
                       native-lib

                       akriti-core

                       # Links the target library to the log library
                       # included in the NDK.
                       ${log-lib} )