SET(TARGET_NAME imgui)
SET(SRC_DIR ${CMAKE_CURRENT_LIST_DIR}/../_external/imgui)
ADD_LIBRARY(${TARGET_NAME}
    ${SRC_DIR}/imgui.cpp
    ${SRC_DIR}/imgui_widgets.cpp
    ${SRC_DIR}/imgui_tables.cpp
    ${SRC_DIR}/imgui_draw.cpp
    ${SRC_DIR}/imgui_demo.cpp
    ${SRC_DIR}/backends/imgui_impl_win32.cpp
    ${SRC_DIR}/backends/imgui_impl_dx11.cpp
)
TARGET_INCLUDE_DIRECTORIES(${TARGET_NAME} PUBLIC
    ${SRC_DIR}
)
