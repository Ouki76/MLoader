if(CMAKE_SIZEOF_VOID_P EQUAL 8)
    set(ARCH "amd64")
else()
    set(ARCH "")
endif()

set(DIA_PATH "${CMAKE_CURRENT_SOURCE_DIR}/blackbone/src/3rd_party/DIA/lib/${ARCH}")
set(BLACKBONE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/blackbone/src")
set(WNETWRAP_PATH "${CMAKE_CURRENT_SOURCE_DIR}/wnetwrap")

target_sources(injector PRIVATE ${WNETWRAP_PATH}/wnetwrap.cpp ${WNETWRAP_PATH}/wnetwrap.h)

target_include_directories(injector PRIVATE ${BLACKBONE_PATH} ${DIA_PATH} ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(injector BlackBone ${DIA_PATH}/diaguids.lib)

add_subdirectory(${BLACKBONE_PATH})