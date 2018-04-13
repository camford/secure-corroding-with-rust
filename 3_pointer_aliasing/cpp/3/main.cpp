#include <iostream>
#include <memory>
#include "main.hpp"

using namespace std;

int main(void) {
    funcA();
    funcB();
}

// Demonstrates raw pointer access
void funcA(void) {
    cout << "------------------\nunqiue_ptr : get()\n------------------" << endl;
    auto foo = make_unique<string>("bar");
    cout << *foo << endl; 
    auto fooptr = foo.get();
    if ( nullptr == foo ) {
        cout << "foo is null after get()" << endl;
    } else {
        cout << "foo is not null after get()" << endl;
    }
    cout << "raw before move: \"" << *fooptr << "\""<< endl;
    take_ownership(move(foo));
    if ( nullptr == foo ) {
        cout << "foo is null after move()" << endl;
    } else {
        cout << "foo is not null after move()" << endl;
    }
    cout << "raw after move: \"" << *fooptr << "\""<< endl;
}

// Demonstrates raw pointer access
void funcB(void) {
    cout << "------------------\nunqiue_ptr : release()\n------------------" << endl;
    auto foo = make_unique<string>("bar");
    cout << *foo << endl; 
    auto fooptr = foo.release();
    if ( nullptr == foo ) {
        cout << "foo is null after release()" << endl;
    } else {
        cout << "foo is not null after release()" << endl;
    }
    cout << "raw before move: \"" << *fooptr << "\""<< endl;
    take_ownership(move(foo));
    if ( nullptr == foo ) {
        cout << "foo is null after move()" << endl;
    } else {
        cout << "foo is not null after move()" << endl;
    }
    cout << "raw after move: \"" << *fooptr << "\""<< endl;
}

template <typename T>
inline void take_ownership(T baz) { }