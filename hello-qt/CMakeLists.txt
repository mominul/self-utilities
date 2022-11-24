cmake_minimum_required(VERSION 3.0.2)

project("hello-qt")

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(Qt5Core REQUIRED)
find_package(Qt5Widgets REQUIRED)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

add_executable(hello-qt main.cpp)
target_link_libraries(hello-qt Qt5::Core Qt5::Widgets)
