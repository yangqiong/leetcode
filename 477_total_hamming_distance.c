#include <stdio.h>

#define MAX_SIZE 10


int totalHammingDistance(int* nums, int numsSize) {
    int sum = 0;
    for (int i = 0; i < 32; i++){
        int num_of_0 = 0;
        int num_of_1 = 0;
        for (int j = 0; j < numsSize; j++){
            if ((nums[j] & 01) == 01){
                ++num_of_1;
            }else{
                ++num_of_0;
            }
            nums[j] >>= 1;
        }
        sum += num_of_0 * num_of_1;
    }
    return sum;
}

int main(){
    int nums[MAX_SIZE];
    int numSize = 0;
    int *p = nums;
    printf("Input the numbers:\n");
    while(scanf("%d", p) != EOF && numSize < MAX_SIZE){
        numSize++;
        p++;
    }
    printf("The sum of total hamming distance is %d.\n", totalHammingDistance(nums, numSize));
    return 0;
}
