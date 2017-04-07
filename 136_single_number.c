#include <stdio.h>
#define MAX_SIZE 10

int singleNumber(int * nums, int numsSize){
    int n;
    for (int i = 0; i < numsSize; i++){
        n ^= nums[i];
    }   
    return n;
}

int main(){
    int numsSize = 0;
    int nums[MAX_SIZE];
    printf("Input the number:\n");
    for (int i = 0; i < MAX_SIZE && (scanf("%d", &nums[i])) != EOF; i++, numsSize++){}
    printf("The single number is %d.\n", singleNumber(nums, numsSize));
    return 0;
}
