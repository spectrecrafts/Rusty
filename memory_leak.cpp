// The below code is the example of memory leak which is possible in c++. OS assigns the memory to the string str but does not delete the memory allocation from the heap. The reference pointing to the stack do gets deleted but not the actual memory allocation in the heap. The delete[] operator has to be used which is upon the discretion of the developer whether he does it or not.

// Rust handles these memory leaks very efficiently. It forces the developers

// fn create_string()
// {
//     Allocate the string on the heap
//         let s : String = String::from("Hello world");
//     Print the string
//         pritnln !("{}", s);
// }
// fn main()
// {
//     Call the function
//     create_string()
// }
// Rust uses ownership rules to manage memory deallocation

#include <iostream>
#include <cstring>
using namespace std;

// Function that allocates memory on a heap and deletes the string
void createString()
{
    // Allocate memory of a string on a heap
    char *str = new char[30];

    // Copy a string to the allocated memory
    strcpy(str, "Hello , heap memory leak");

    // Print the string
    cout << str << endl;

    // Free the allocated memory
    // delete[] str;
}

int main()
{
    createString();
    return 0;
}