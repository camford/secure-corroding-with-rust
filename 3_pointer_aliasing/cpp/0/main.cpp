#include <iostream>
#include <memory>
#include <iomanip>
#include <sstream>
#include "main.hpp"

using namespace std;

int main(void) {
    func();
}

void func(void) {
    string *foo = new string("bar");
    string *ptr = foo;
    print(foo, ptr);
    delete foo;
    foo = nullptr;
    print(foo, ptr);
}

void print(string* foo, string* ptr) {
    std::ostringstream s;
    cout << left << setw(10) << "foo";
    cout << left << "ptr" << endl;
    cout << left << setw(10) << foo;
    cout << left << ptr << endl;
    if (foo == nullptr) {
        cout << left << setw(10) << "<invalid>";
    } else {
        s << "\"" << *foo << "\"";
        cout << left << setw(10) << s.str();
    }
    cout << left << "\"" << *ptr << "\"" << "\n" << endl;
}
