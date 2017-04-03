#include <stdio.h>

int findComplement(int num){
    int max = 2;
    while((max - 1) < num){
        max *= 2;
    }
    return max - 1 - num;
}

int main(){
    int num;
    printf("Input the num:\n");
    scanf("%d", &num);
    printf("The complement of the num %d is %d.\n", num, findComplement(num));
    return 0;
}
