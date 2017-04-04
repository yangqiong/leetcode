#include <stdio.h>
#include <stdbool.h>

bool isPowerOfTwo(int n){
    while(n){
        if ((n & 01) == 01){
            return n == 01;
        }
        n >>= 1;
    }
    return false;
}

int main(){
    int n;
    printf("Input the num:\n");
    scanf("%d", &n);
    printf("The %d is power of two %d.\n", n, isPowerOfTwo(n));
    return 0;
}
