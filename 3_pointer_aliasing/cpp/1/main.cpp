#include <iostream>
#include <memory>
#include "main.hpp"

using namespace std;

int main(void) {
    funcA();
    funcB();
    funcC();
}

// Demonstrates the necessity of explicit move semantics
void funcA(void) {
    auto foo = make_unique<string>("bar");
    cout << "funcA(1)" << *foo << endl; 
    /* Uncommenting final line in this function gives this (intended) compilation error:
     * "error: call to implicitly-deleted copy constructor of ...unique_ptr...
     *  note: copy constructor is implicitly deleted because ...unique_ptr... has a
     *  user-declared move constructor."
     */
    //take_ownership(foo);
}

// Demonstrates run-time errors for using moved values
void funcB(void) {
    auto foo = make_unique<string>("bar");
    cout << "funcB(1): " << *foo << endl; 
    take_ownership(move(foo));
    /* Uncommenting the following line will create a runtime error for 
     * dereferencing nullptr.
     */
    //cout << "funcB(2): " << *foo << endl;
}

// Demonstrates nullptr for moved values
void funcC(void) {
    auto foo = make_unique<string>("bar");
    cout << *foo << endl; 
    take_ownership(move(foo));
    if ( nullptr == foo ) {
        cout << "foo is null now" << endl;
    }
}

template <typename T>
inline void take_ownership(T baz) { }