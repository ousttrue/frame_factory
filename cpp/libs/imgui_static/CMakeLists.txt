#
# imgui
#
SET(TARGET_NAME imgui_static)
SET(SRC_DIR ${CMAKE_CURRENT_LIST_DIR}/../../_external/imgui)
ADD_LIBRARY(${TARGET_NAME} STATIC
    ${SRC_DIR}/imgui.cpp
    ${SRC_DIR}/imgui_widgets.cpp
    ${SRC_DIR}/imgui_tables.cpp
    ${SRC_DIR}/imgui_draw.cpp
    ${SRC_DIR}/imgui_demo.cpp
    ${SRC_DIR}/backends/imgui_impl_win32.cpp
    ${SRC_DIR}/backends/imgui_impl_sdl.cpp
    ${SRC_DIR}/backends/imgui_impl_dx11.cpp
)
TARGET_INCLUDE_DIRECTORIES(${TARGET_NAME} PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}
    ${SRC_DIR}
PRIVATE
    ${CMAKE_CURRENT_LIST_DIR}/../../_external/SDL/include
)
# TARGET_COMPILE_DEFINITIONS(${TARGET_NAME} PRIVATE
#     "IMGUI_API=__declspec( dllexport )"
# )
# TARGET_LINK_LIBRARIES(${TARGET_NAME} PUBLIC
#     SDL2
# )
TARGET_LINK_OPTIONS(${TARGET_NAME} PUBLIC
    /NODEFAULTLIB:MSVCRTD
)
