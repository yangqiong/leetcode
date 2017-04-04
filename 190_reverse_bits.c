#include <stdio.h>
#include <stdint.h>

uint32_t reverseBits(uint32_t n){
    uint32_t m = 0;
    int i = 32;
    while(i > 0){  // 通过循环32位去判断，而不是n
        m <<= 1;
        if ((n & 01) == 01){
            m += 01;
        }
        n >>= 1;
        --i;
    }
    return m;
}

int main(){
    uint32_t n;
    printf("Input a number:\n");
    scanf("%d", &n);
    printf("The num %d of reverse bits is %d.\n", n, reverseBits(n));
    return 0;
}

