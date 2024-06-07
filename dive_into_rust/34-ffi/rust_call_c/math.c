int add(int a, int b) {
    return a+b;
}

//gcc -c math.c -o math.o
//gcc -c -Wall -Werror -fpic math.c

//ar rcs libmath.a math.o
//ar rcs math.lib math.o