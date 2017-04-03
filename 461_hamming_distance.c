#include <stdio.h>

int hammingDistance(int x, int y) {
    int number = 0;
    int z = x ^ y;
    while(z > 0){
        if ((z & 1) == 1){
            number++;
        }
        z >>= 1;
    }
    return number;
}

int main(){
    int x, y;
    printf("Input two number:\n");
    scanf("%d%d", &x, &y);
    printf("the hamming distance of %d, %d is %d\n", x, y, hammingDistance(x, y));
    return 0;
}
