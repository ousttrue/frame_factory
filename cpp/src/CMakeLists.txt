SET(TARGET_NAME "cpp_sample")
ADD_EXECUTABLE(${TARGET_NAME} WIN32
    main.cpp
    window_util.cpp
    dx11.cpp
    imgui_app.cpp
)

TARGET_LINK_DIRECTORIES(${TARGET_NAME} PRIVATE
    ${CMAKE_CURRENT_LIST_DIR}/../../rust/target/debug
)

TARGET_LINK_LIBRARIES(${TARGET_NAME} PRIVATE
    frame_factory.dll.lib
    #
    imgui
    json
    #
    d3d11
    winmm
)
