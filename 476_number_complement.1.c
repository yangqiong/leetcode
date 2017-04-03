#include <stdio.h>

int findComplement(int num){
    int mask = ~0;
    while (mask & num) mask <<= 1;
    return ~mask ^ num;
}

int main(){
    int num;
    printf("Input the num:\n");
    scanf("%d", &num);
    printf("The complement of the num %d is %d.\n", num, findComplement(num));
    return 0;
}
