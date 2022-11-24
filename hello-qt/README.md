# hello-qt
A Qt5 CMake Hello World template.

## Compiling on Windows
Open Developer Powershell and issue these commands:
```
mkdir build; cd build
cmake .. -DCMAKE_PREFIX_PATH="C:\Qt\5.15.2\msvc2019_64\lib\cmake" # change according to your configuration
msbuild .\hello-qt.sln
```
### Running it
```
$env:Path += ";C:\Qt\5.15.2\msvc2019_64\bin"
.\Debug\hello-qt.exe
```

## Compiling on Linux
Open terminal and issue these commands:
```
mkdir build && cd build
cmake ..
make
```
### Running it
```
./hello-qt
```
