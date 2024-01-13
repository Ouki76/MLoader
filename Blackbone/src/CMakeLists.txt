set(INJ_PATH "${CMAKE_CURRENT_SOURCE_DIR}/injector")

target_sources(injector PRIVATE ${INJ_PATH}/injector.cpp ${INJ_PATH}/injector.hpp)