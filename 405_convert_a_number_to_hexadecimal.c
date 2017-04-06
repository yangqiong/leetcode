#include <stdio.h>
#include <stdlib.h>

char * toHex(int num){
    if (num == 0) return "0";
    char str[9] = {'0','0','0','0','0','0','0','0','\0'};
    char * strResult = malloc(9 * sizeof(char));
    char str16[16] = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'};
    int i = 7;
    while (num && (i >= 0)){
        str[i] = str16[num & 0xF];
    printf("%s\n", str);
        --i;
        num >>= 4;
    }
    int j = 0;
    int flag = 0;
    for (i = 0; i < 8; i++){
        if (str[i] != '0' || flag){
            strResult[j] = str[i];
            j++;
            flag = 1;
        }
    }
    strResult[j] = '\0';
    return strResult;
}

int main(){
    int num;
    printf("Input a number:\n");
    scanf("%d", &num);
    printf("The Hexadecimal of %d is %s.\n", num, toHex(num));
    return 0;
}
