#include <stdio.h>
#include <stdlib.h>

int main(){
	int a; // variavel de entrada
    int b; // variavel de entrada
    int x; // variavel de saída
	
	
     // x<=not(not (a and b));
	a = 0;
	b = 0;
	x = !(!(a && b));  
    printf("\n %d<=not(not %d and %d))", x,a,b);
	

     // x<=not(not (a and b));
	a = 0;
	b = 1;
	x = !(!(a && b));
    printf("\n %d<=not(not %d and %d))", x,a,b);


     // x<=not(not (a and b));
	a = 1;
	b = 0;
	x = !(!(a && b));
    printf("\n %d<=not(not %d and %d))", x,a,b);
		
        
     // x<=not(not (a and b));    
	a = 1;
	b = 1;
	x = !(!(a && b));
    printf("\n %d<=not(not %d and %d))", x,a,b);
	
	return 0;
}