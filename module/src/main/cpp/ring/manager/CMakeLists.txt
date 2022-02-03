cmake_minimum_required(VERSION 3.4.1)

include_directories(
        ${CMAKE_CURRENT_SOURCE_DIR}/include
        ${CMAKE_CURRENT_SOURCE_DIR}
)

aux_source_directory(${CMAKE_CURRENT_SOURCE_DIR} RING_MANAGER)
add_library(ringManager STATIC ${RING_MANAGER})