#include <stdio.h>
#include <stdint.h>

int hammingWeight(uint32_t n){
    int i = 0;
    while(n){
        ++i;
        n &= n - 1;
    };
    return i;
}

int main(){
    int n;
    printf("Input the number:\n");
    scanf("%d", &n);
    printf("The number of 1 Bits of %d is %d.", n, hammingWeight(n));
    return 0;
}
