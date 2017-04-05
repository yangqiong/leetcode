#include <stdio.h>
#include <stdbool.h>

bool isPowerOfFour(int num){
    if (num < 1) return false;
    if (num == 1) return true;
    while(num && (num & 03) == 0){
        num >>= 2;
    }
    return num == 1 ? true: false;
}

int main(){
    int num;
    printf("Input the number:\n");
    scanf("%d", &num);
    printf("The %d is Powser of four is %d.\n", num, isPowerOfFour(num));
    return 0;
}
