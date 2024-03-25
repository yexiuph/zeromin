#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct ZValue {
  enum class Tag {
    _Int,
    _Float,
  };

  struct _Int_Body {
    int32_t _0;
  };

  struct _Float_Body {
    double _0;
  };

  Tag tag;
  union {
    _Int_Body int_;
    _Float_Body float_;
  };
};

struct ZStruct {
  const char *name;
  ZValue value;
};

struct ZRijndael {
  int32_t ran_file_version;
  const char *rijn_version;
};

struct ZeroSHA {
  uint8_t _private[0];
};

extern "C" {

void perform_async_operation(const char *char_name, uint64_t seconds, void (*callback)(const char*,
                                                                                       uintptr_t));

ZStruct *data_new();

void data_free(ZStruct *ptr);

int32_t get_file_version(const ZRijndael *rijndael);

const char *get_rijndael_version(const ZRijndael *rijndael);

ZeroSHA *create_zerosha();

} // extern "C"
