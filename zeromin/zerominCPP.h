#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uintptr_t BLOCKSIZE = 64;

constexpr static const uintptr_t SHA256LENGTH = 8;

constexpr static const unsigned long ZERO_MSG_BASE = 888;

constexpr static const unsigned long ZERO_MSG_LOGIN = (ZERO_MSG_BASE + (unsigned long)450);

constexpr static const unsigned long ZERO_MSG_LOBBY = (ZERO_MSG_BASE + (unsigned long)950);

constexpr static const unsigned long ZERO_MSG_LOBBY_MAX = (ZERO_MSG_BASE + (unsigned long)1450);

constexpr static const unsigned long ZERO_MSG_GAME_CONTROL = (ZERO_MSG_BASE + (unsigned long)1900);

constexpr static const unsigned long ZERO_MSG_GAME_CONTROL_MAX = (ZERO_MSG_BASE + (unsigned long)3900);

struct ZeroMsg;

struct ZeroPath {
  uint8_t _antizero[0];
  const char *glogic;
  const char *glevel;
  const char *gnpctalk;
  const char *gquest;
  const char *ganimation;
  const char *geffect;
  const char *gchareffect;
  const char *gskinobject;
};

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
  uint8_t _antizero[0];
  int32_t ran_file_version;
  const char *rijn_version;
};

struct ZeroSHA {
  uint8_t _antizero[0];
  uint32_t m_aui_buf[SHA256LENGTH];
  uint32_t m_aui_bits[2];
  uint8_t m_auc_in[BLOCKSIZE];
};

struct ZeroMsgGeneric {
  unsigned long dw_size;
  ZeroMsg n_type;
};

extern "C" {

void perform_async_operation(const char *char_name, uint64_t seconds, void (*callback)(const char*,
                                                                                       uintptr_t));

void zeropath_delete(ZeroPath *zp);

ZeroPath *zeropath_init();

ZStruct *data_new();

void data_free(ZStruct *ptr);

int32_t get_file_version(const ZRijndael *rijndael);

const char *get_rijndael_version(const ZRijndael *rijndael);

ZeroSHA *new();

ZeroSHA *create_zerosha();

ZeroMsgGeneric *construct_zero_msg(unsigned long size, ZeroMsg zero_msg_type);

void deconstruct_zero_msg(ZeroMsgGeneric *ptr);

} // extern "C"
