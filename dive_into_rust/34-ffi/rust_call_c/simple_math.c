int add(int a, int b) {
    return a+b;
}

//gcc -c add.c -o add.o
//gcc -c -Wall -Werror -fpic add.c

//ar rcs libadd.a add.o