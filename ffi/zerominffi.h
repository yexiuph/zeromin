#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uintptr_t BLOCKSIZE = 64;

constexpr static const uintptr_t SHA256LENGTH = 8;

struct ZeroSHA {
    uint8_t _antizero[0];
    uint32_t m_aui_buf[SHA256LENGTH];
    uint32_t m_aui_bits[2];
    uint8_t m_auc_in[BLOCKSIZE];
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

extern "C" {

ZeroSHA *create_zerosha();

void data_free(ZStruct *ptr);

ZStruct *data_new();

int32_t get_file_version(const ZRijndael *rijndael);

const char *get_rijndael_version(const ZRijndael *rijndael);

void perform_async_operation(const char *char_name, uint64_t seconds, void (*callback)(const char*,
                                                                                       uintptr_t));

void zeropath_delete(ZeroPath *zp);

ZeroPath *zeropath_init();

} // extern "C"
