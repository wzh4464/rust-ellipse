use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // 输出所有环境变量到 stderr
    for (key, value) in env::vars() {
        eprintln!("{}: {}", key, value);
    }
    
    // 确认 make 工具在系统路径中
    let make = env::var("MAKE").unwrap_or_else(|_| "make".to_string());

    // 运行 Makefile 编译 C 代码
    let status = Command::new(make)
        .arg("shared")
        .current_dir("ELSDc_c")
        .status()
        .expect("Failed to run make");

    if !status.success() {
        panic!("Failed to build ELSDc_c library");
    }

    // 告诉 cargo 链接编译后的共享库
    println!("cargo:rustc-link-search=native=ELSDc_c");
    println!("cargo:rustc-link-lib=dylib=elsdc");

    // 生成绑定代码
    let bindings = bindgen::Builder::default()
        .header("ELSDc_c/src/elsdc.h")
        .clang_arg("-IELSDc_c/src")
        .generate()
        .expect("Failed to generate bindings");

    // 输出绑定代码到文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}



// #
// #  Copyright (c) 2012 viorica patraucean (vpatrauc@gmail.com)
// #
// #  This program is free software: you can redistribute it and/or modify
// #  it under the terms of the GNU Affero General Public License as
// #  published by the Free Software Foundation, either version 3 of the
// #  License, or (at your option) any later version.
// #
// #  This program is distributed in the hope that it will be useful,
// #  but WITHOUT ANY WARRANTY; without even the implied warranty of
// #  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// #  GNU Affero General Public License for more details.
// #
// #  You should have received a copy of the GNU Affero General Public License
// #  along with this program. If not, see <http://www.gnu.org/licenses/>.
// #
// #  makefile - This file belongs to ELSDc project (Ellipse and Line Segment
// #             Detector with continuous validation).

// elsdc:
// 	make -C src
// 	mv src/elsdc .

// shared:
// 	make -C src shared
// 	mv src/libelsdc.so .

// test:
// 	./elsdc shapes.pgm

// clean:
// 	rm -f elsdc
// 	rm -f libelsdc.so
// 	rm -f src/*.o

// #
// #  Copyright (c) 2012 viorica patraucean (vpatrauc@gmail.com)
// #
// #  This program is free software: you can redistribute it and/or modify
// #  it under the terms of the GNU Affero General Public License as
// #  published by the Free Software Foundation, either version 3 of the
// #  License, or (at your option) any later version.
// #
// #  This program is distributed in the hope that it will be useful,
// #  but WITHOUT ANY WARRANTY; without even the implied warranty of
// #  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// #  GNU Affero General Public License for more details.
// #
// #  You should have received a copy of the GNU Affero General Public License
// #  along with this program. If not, see <http://www.gnu.org/licenses/>.
// #
// #  makefile - This file belongs to ELSDc project (Ellipse and Line Segment
// #             Detector with continuous validation).

// # You may need to indicate the location of Lapack library in your system.
// # For that, uncomment the following line and replace `/usr/lib` with
// # the directory where the library is located.

// # if comupter name is mercury, use the following line
// ifeq ($(shell hostname),mercury)
// LIB= -L/home/zihan/lib
// endif
// OPT= -O3
// # OPT= -g

// ifeq ($(shell uname),Darwin)
// LDFLAGS=-L/opt/homebrew/opt/lapack/lib
// CPPFLAGS=-I/opt/homebrew/opt/lapack/include
// LAPACK_FLAGS=-DUSE_LAPACKE
// LAPACK_LIBS=-llapacke
// else
// LAPACK_LIBS=-llapack
// endif

// elsdc: main.c pgm.c svg.c elsdc.c gauss.c curve_grow.c polygon.c ring.c ellipse_fit.c rectangle.c iterator.c image.c lapack_wrapper.c misc.c
// 	if [ "$(shell hostname)" = "mercury" ]; then \
// 		cc $(OPT) $(LIB) -o $@ $^ -llapack -lm -lblas -lgfortran; \
// 	else \
// 		cc $(OPT) $(LIB) $(CPPFLAGS) $(LDFLAGS) $(LAPACK_FLAGS) -o $@ $^ $(LAPACK_LIBS) -lm; \
// 	fi

// shared: python_interface.c pgm.c svg.c elsdc.c gauss.c curve_grow.c polygon.c ring.c ellipse_fit.c rectangle.c iterator.c image.c lapack_wrapper.c misc.c
// 	if [ "$(shell hostname)" = "mercury" ]; then \
// 		echo "mercury, use gfortran"; \
// 		cc -c $(OPT) $(LIB) -fpic $^; \
// 		cc -shared $(OPT) $(LIB) -o libelsdc.so python_interface.o pgm.o svg.o elsdc.o gauss.o curve_grow.o polygon.o ring.o ellipse_fit.o rectangle.o iterator.o image.o lapack_wrapper.o misc.o -llapack -lm -lblas -lgfortran; \
// 	else \
// 		cc -c $(OPT) $(LIB) $(CPPFLAGS) $(LAPACK_FLAGS) -fpic $^; \
// 		cc -shared $(OPT) $(LIB) $(CPPFLAGS) $(LDFLAGS) $(LAPACK_FLAGS) -o libelsdc.so python_interface.o pgm.o svg.o elsdc.o gauss.o curve_grow.o polygon.o ring.o ellipse_fit.o rectangle.o iterator.o image.o lapack_wrapper.o misc.o $(LAPACK_LIBS) -lm; \
// 	fi

// clean:
// 	rm elsdc
// 	rm *.o
// 	rm *.so
