# Install script for directory: /home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/csc/met-io-rs/met-io-rs/target/release/build/hdf5-src-a855198d21d93c8f/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xheadersx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/hdf5.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5api_adpt.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5public.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Apublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5ACpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Cpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Dpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Epubgen.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Epublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Fpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDcore.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDdirect.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDfamily.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDhdfs.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDlog.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDmpi.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDmpio.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDmulti.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDros3.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDs3comms.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDsec2.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDstdio.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5FDwindows.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Gpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Ipublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Lpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5MMpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Opublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Ppublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5PLextern.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5PLpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Rpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Spublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Tpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Zpublic.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5Epubgen.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5version.h"
    "/home/csc/met-io-rs/met-io-rs/hdf5-rust/hdf5-src/ext/hdf5/src/H5overflow.h"
    "/home/csc/met-io-rs/met-io-rs/target/release/build/hdf5-src-a855198d21d93c8f/out/build/H5pubconf.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xlibrariesx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/csc/met-io-rs/met-io-rs/target/release/build/hdf5-src-a855198d21d93c8f/out/build/bin/libhdf5.a")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xlibrariesx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/home/csc/met-io-rs/met-io-rs/target/release/build/hdf5-src-a855198d21d93c8f/out/build/CMakeFiles/hdf5-1.10.6.pc")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xlibrariesx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE FILE PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE FILES "/home/csc/met-io-rs/met-io-rs/target/release/build/hdf5-src-a855198d21d93c8f/out/build/CMakeFiles/h5cc")
endif()

