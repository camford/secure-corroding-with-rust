#include <iostream>
#include <memory>
#include "main.hpp"

using namespace std;

int main(void) {
    func();
}

// Demonstrates returning moved values
void func(void) {
    auto foo = make_unique<string>("bar");
    cout << "func(1): " << *foo << endl; 
    auto qux = return_ownership(move(foo));
    cout << "func(2): " << *qux << endl;
    // An alternative would be to re-assign foo
    /* foo = return_ownership(move(foo));
     * cout << *foo << endl;
     */
}

template <typename T>
inline void take_ownership(T baz) { }

template <typename T>
inline T return_ownership(T baz) { return baz; }
