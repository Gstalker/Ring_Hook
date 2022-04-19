cmake_minimum_required(VERSION 3.4.1)

find_library(log-lib log)
include_directories(
        ${CMAKE_CURRENT_SOURCE_DIR}/
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

aux_source_directory(
        ${CMAKE_CURRENT_SOURCE_DIR}
        GSTALKER_SOURCE
)

add_library(
        gstalker

        SHARED

        ${GSTALKER_SOURCE}
)

target_link_libraries(
        gstalker
        ${log-lib}
)