#include <stdio.h>

#define MAX_SIZE 10

int missingNumber(int * nums, int numsSize){
    int res = numsSize;
    for (int i = 0; i < numsSize; i++){
        res ^= i;
        res ^= nums[i];
    }
    return res;
}

int main(){
    int num[MAX_SIZE];
    int *p = num;
    int numsSize = 0;
    printf("Input numbers:\n");
    while(scanf("%d",p) != EOF && numsSize < MAX_SIZE){
        numsSize++;
        p++;
    }
    printf("The missing number is %d.\n", missingNumber(num, numsSize));
    return 0;
}
