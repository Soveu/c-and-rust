#include <vector>
#include <stdint.h>
#include <iostream>
#include <string_view>
#include <limits>

/* Imports */
extern "C" bool utf8_to_utf32(const char*, size_t n, std::vector<uint32_t>* buf);

/* Exports */
extern "C" void cpp_vector_push_u32(std::vector<uint32_t>* v, uint32_t x) {
  v->push_back(x);
}
extern "C" void cpp_rethrow_panic(const char* s, size_t n, uint32_t line) {
  auto filename = std::string_view(s, n);

  if(line == std::numeric_limits<uint32_t>::max()) {
    std::cerr << "Rust panic occured" << std::endl;
  } else {
    std::cerr << "Rust panic occured in file '" << filename 
              << "' at line " << line << std::endl;
  }

  abort();
}

int main() {
  std::string s = "Witaj świecie!";
  std::vector<uint32_t> v;
  v.reserve(32);

  bool ok = utf8_to_utf32(s.data(), s.size(), &v);
  if(!ok) {
    std::cerr << "ERROR" << std::endl;
    return 1;
  }

  for(const auto& x : v) {
    std::cout << x << ' ';
  }
  std::cout << std::endl;
}
