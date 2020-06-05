#include <vector>
#include <stdint.h>
#include <iostream>
#include <string_view>

extern "C" int utf8_to_utf32(const char*, size_t n, std::vector<uint32_t>* buf);

extern "C" void cpp_vector_push(std::vector<uint32_t>* v, uint32_t x) {
  v->push_back(x);
}
extern "C" void cpp_rethrow_panic(const char* s, size_t n) {
  auto sview = std::string_view(s, n);
  std::cerr << sview << std::endl;
  throw sview;
}

int main() {
  std::string s = "Witaj świecie!";
  std::vector<uint32_t> v;
  v.reserve(32);

  int status = utf8_to_utf32(s.data(), s.size(), &v);
  if(status != 0) {
    std::cerr << "ERROR NO " << status << std::endl;
    return 1;
  }

  for(const auto& x : v) {
    std::cout << x << std::endl;
  }
}
