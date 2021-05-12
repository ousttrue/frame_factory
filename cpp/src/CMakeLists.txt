SET(TARGET_NAME "cpp_sample")
ADD_EXECUTABLE(${TARGET_NAME} WIN32
    main.cpp
    window_util.cpp
)

TARGET_LINK_DIRECTORIES(${TARGET_NAME} PRIVATE
    ${CMAKE_CURRENT_LIST_DIR}/../../rust/target/debug
)

TARGET_LINK_LIBRARIES(${TARGET_NAME} PRIVATE
    renderer.dll.lib
    imgui
    d3d11
)
