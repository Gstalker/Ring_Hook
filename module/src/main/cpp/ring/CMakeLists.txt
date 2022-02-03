cmake_minimum_required(VERSION 3.4.1)

add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/core)
add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/manager)

include_directories(
        ${CMAKE_CURRENT_SOURCE_DIR}/include
        ${CMAKE_CURRENT_SOURCE_DIR}
)

aux_source_directory(${CMAKE_CURRENT_SOURCE_DIR} RING_INNER)

add_library(ringInner STATIC ${RING_INNER})

target_link_libraries(ringInner ringCore ringManager)