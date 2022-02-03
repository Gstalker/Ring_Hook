cmake_minimum_required(VERSION 3.4.1)



add_library(dobby STATIC IMPORTED)

set_target_properties(dobby
        PROPERTIES IMPORTED_LOCATION
        ${CMAKE_CURRENT_SOURCE_DIR}/../../../JniLibs/${CMAKE_ANDROID_ARCH_ABI}/libdobby.a)



include_directories(
        ${CMAKE_CURRENT_SOURCE_DIR}/include
        ${CMAKE_CURRENT_SOURCE_DIR}
)

aux_source_directory(${CMAKE_CURRENT_SOURCE_DIR} RING_CORE)

add_library(ringCore STATIC ${RING_CORE})

target_link_libraries(ringCore dobby)