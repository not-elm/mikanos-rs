/**
 * @file font.cpp
 *
 * フォント描画のプログラムを集めたファイル.
 */
//
#include <cstdint>


extern const uint8_t _binary_hankaku_bin_start;
extern const uint8_t _binary_hankaku_bin_end;
extern const uint8_t _binary_hankaku_bin_size;
extern "C" {
    const uint8_t* get_font(char c);
}
const uint8_t* get_font(char c) {
   auto index = 16 * static_cast<unsigned int>(c);
   if (index >= reinterpret_cast<uintptr_t>(&_binary_hankaku_bin_size)) {
       return nullptr;
   }
   return &_binary_hankaku_bin_start + index;
}

