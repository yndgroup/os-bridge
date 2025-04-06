#ifdef __cplusplus
extern "C" {
#endif

int add(int a, int b);

#ifdef __cplusplus
}
#endif
// 1、在 C++ 编译器下（#ifdef __cplusplus 成立）：
// extern "C" { ... } 会告诉 C++ 编译器：这些函数使用 C 的链接方式（name mangling 规则），确保函数名在二进制文件中保持原样（如 add 而不是 _Z3addii）。
// 这样 C++ 代码可以正确调用 C 编写的函数（或反过来）。
// 2、在 C 编译器下（#ifdef __cplusplus 不成立）：
// 由于 C 没有 extern "C" 语法，预处理器会直接跳过这部分，只保留 int add(int a, int b); 声明。
// C 编译器本身不会对函数名进行 name mangling，所以函数名直接就是 add。