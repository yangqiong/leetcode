#include <stdio.h>
#include <stdbool.h>

bool isPowerOfFour(int num){
    return num > 0 && (num & (num - 1)) == 0 & (num & 0x55555555) == num;
}

int main(){
    int num;
    printf("Input the number:\n");
    scanf("%d", &num);
    printf("The %d is Powser of four is %d.\n", num, isPowerOfFour(num));
    return 0;
}
