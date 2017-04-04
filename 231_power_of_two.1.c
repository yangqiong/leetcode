#include <stdio.h>
#include <stdbool.h>

bool isPowerOfTwo(int n){
    if (n <= 0) return false;
    return !(n & (n-1));
}

int main(){
    int n;
    printf("Input the num:\n");
    scanf("%d", &n);
    printf("The %d is power of two %d.\n", n, isPowerOfTwo(n));
    return 0;
}
