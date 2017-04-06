fn apply<F>(f:F) where
    F:Fn(){
        f();
    }


fn main() {
    let x = 7;
    let priint = || println!("{}",x);
    apply(priint);    
}

git submodule update --init --recursive

cmake -DCMAKE_C_COMPILER=D:/TDM-GCC-64/bin/gcc -DCMAKE_CXX_COMPILER=D:/TDM-GCC-64/bin/g++


I don't know what's going on on your system, but you need to figure out where your compilers are. 
For me, they're in /usr/bin, 
so I would do this:
cmake .. -DCMAKE_C_COMPILER=/usr/bin/gcc -DCMAKE_CXX_COMPILER=/usr/bin/g++

 cmake -D CMAKE_CXX_COMPILER="g++" CMAKE -D CMAKE_BUILD_TYPE=Release -D CMAKE_INSTALL_PREFIX:PATH="/usr/local" 
 
 
 CMakeLists.txt