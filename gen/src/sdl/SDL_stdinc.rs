// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;
pub const SDL_MAX_SINT8: i8 = 0x7F;
//SDL_MIN_SINT8 ((Sint8)(~0x7F))
pub const SDL_MAX_UINT8: u8 = 0xFF;
pub const SDL_MIN_UINT8: u8 = 0x00;
pub const SDL_MAX_SINT16: i16 = 0x7FFF;
//SDL_MIN_SINT16 ((Sint16)(~0x7FFF))
pub const SDL_MAX_UINT16: u16 = 0xFFFF;
pub const SDL_MIN_UINT16: u16 = 0x0000;
pub const SDL_MAX_SINT32: i32 = 0x7FFFFFFF;
//SDL_MIN_SINT32 ((Sint32)(~0x7FFFFFFF))
pub const SDL_MAX_UINT32: u32 = 0xFFFFFFFF;
pub const SDL_MIN_UINT32: u32 = 0x00000000;
pub const SDL_MAX_SINT64: i64 = 0x7FFFFFFFFFFFFFFF;
pub const SDL_MIN_SINT64: i64 = !0x7FFFFFFFFFFFFFFF;
pub const SDL_MAX_UINT64: u64 = 0xFFFFFFFFFFFFFFFF;
pub const SDL_MIN_UINT64: u32 = 0x0000000000000000;
//SDL_PRIs64 "I64d"
//SDL_PRIu64 "I64u"
//SDL_PRIx64 "I64x"
//SDL_PRIX64 "I64X"
//SDL_PRIs32 "d"
//SDL_PRIu32 "u"
//SDL_PRIx32 "x"
//SDL_PRIX32 "X"
//SDL_PRINTF_FORMAT_STRING _Printf_format_string_
//SDL_SCANF_FORMAT_STRING _Scanf_format_string_impl_
//M_PI 3.14159265358979323846264338327950288
//SDL_ICONV_ERROR (size_t)-1
//SDL_ICONV_E2BIG (size_t)-2
//SDL_ICONV_EILSEQ (size_t)-3
//SDL_ICONV_EINVAL (size_t)-4

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_bool {
    SDL_FALSE = 0,
    SDL_TRUE = 0x1,
}
pub type Sint8 = i8;
pub type Uint8 = u8;
pub type Sint16 = i16;
pub type Uint16 = u16;
pub type Sint32 = i32;
pub type Uint32 = u32;
pub type Sint64 = i64;
pub type Uint64 = u64;
pub type SDL_compile_time_assert_uint8 = [i32; 1];
pub type SDL_compile_time_assert_sint8 = [i32; 1];
pub type SDL_compile_time_assert_uint16 = [i32; 1];
pub type SDL_compile_time_assert_sint16 = [i32; 1];
pub type SDL_compile_time_assert_uint32 = [i32; 1];
pub type SDL_compile_time_assert_sint32 = [i32; 1];
pub type SDL_compile_time_assert_uint64 = [i32; 1];
pub type SDL_compile_time_assert_sint64 = [i32; 1];

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_DUMMY_ENUM {
    DUMMY_ENUM_VALUE = 0,
}
pub type SDL_compile_time_assert_enum = [i32; 1];
pub type SDL_iconv_t = *mut _SDL_iconv_t;
pub type _SDL_iconv_t = c_void;

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * size: 
    pub fn SDL_malloc(
        size: usize,
    ) -> *mut c_void;

    /// * nmemb: 
    /// * size: 
    pub fn SDL_calloc(
        nmemb: usize,
        size: usize,
    ) -> *mut c_void;

    /// * mem: 
    /// * size: 
    pub fn SDL_realloc(
        mem: *mut c_void,
        size: usize,
    ) -> *mut c_void;

    /// * mem: 
    pub fn SDL_free(
        mem: *mut c_void,
    ) -> c_void;

    /// * malloc_func: 
    /// * calloc_func: 
    /// * realloc_func: 
    /// * free_func: 
    pub fn SDL_GetMemoryFunctions(
        malloc_func: *mut extern fn(usize,) -> *mut c_void,
        calloc_func: *mut extern fn(usize,usize,) -> *mut c_void,
        realloc_func: *mut extern fn(*mut c_void,usize,) -> *mut c_void,
        free_func: *mut extern fn(*mut c_void,) -> c_void,
    ) -> c_void;

    /// * malloc_func: 
    /// * calloc_func: 
    /// * realloc_func: 
    /// * free_func: 
    pub fn SDL_SetMemoryFunctions(
        malloc_func: extern fn(usize,) -> *mut c_void,
        calloc_func: extern fn(usize,usize,) -> *mut c_void,
        realloc_func: extern fn(*mut c_void,usize,) -> *mut c_void,
        free_func: extern fn(*mut c_void,) -> c_void,
    ) -> i32;

    pub fn SDL_GetNumAllocations() -> i32;

    /// * name: 
    pub fn SDL_getenv(
        name: *const i8,
    ) -> *mut i8;

    /// * name: 
    /// * value: 
    /// * overwrite: 
    pub fn SDL_setenv(
        name: *const i8,
        value: *const i8,
        overwrite: i32,
    ) -> i32;

    /// * : 
    /// * : 
    #[link_name = "?compare@?1??SDL_qsort@@9@3P6AHPEBX0@ZEA"]
    pub fn compare(
        _0: *const c_void,
        _1: *const c_void,
    ) -> i32;

    /// * base: 
    /// * nmemb: 
    /// * size: 
    /// * compare: 
    pub fn SDL_qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compare: *mut extern fn(*mut c_void,*mut c_void,) -> i32,
    ) -> c_void;

    /// * x: 
    pub fn SDL_abs(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_isalpha(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_isalnum(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_isblank(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_iscntrl(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_isdigit(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_isxdigit(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_ispunct(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_isspace(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_isupper(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_islower(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_isprint(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_isgraph(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_toupper(
        x: i32,
    ) -> i32;

    /// * x: 
    pub fn SDL_tolower(
        x: i32,
    ) -> i32;

    /// * crc: 
    /// * data: 
    /// * len: 
    pub fn SDL_crc32(
        crc: u32,
        data: *const c_void,
        len: usize,
    ) -> u32;

    /// * dst: 
    /// * c: 
    /// * len: 
    pub fn SDL_memset(
        dst: *mut c_void,
        c: i32,
        len: usize,
    ) -> *mut c_void;

    /// * dst: 
    /// * val: 
    /// * dwords: 
    pub fn SDL_memset4(
        dst: *mut c_void,
        val: u32,
        dwords: usize,
    ) -> c_void;

    /// * dst: 
    /// * src: 
    /// * len: 
    pub fn SDL_memcpy(
        dst: *mut c_void,
        src: *const c_void,
        len: usize,
    ) -> *mut c_void;

    /// * dst: 
    /// * src: 
    /// * len: 
    pub fn SDL_memmove(
        dst: *mut c_void,
        src: *const c_void,
        len: usize,
    ) -> *mut c_void;

    /// * s1: 
    /// * s2: 
    /// * len: 
    pub fn SDL_memcmp(
        s1: *const c_void,
        s2: *const c_void,
        len: usize,
    ) -> i32;

    /// * wstr: 
    pub fn SDL_wcslen(
        wstr: *const u16,
    ) -> usize;

    /// * dst: 
    /// * src: 
    /// * maxlen: 
    pub fn SDL_wcslcpy(
        dst: *mut u16,
        src: *const u16,
        maxlen: usize,
    ) -> usize;

    /// * dst: 
    /// * src: 
    /// * maxlen: 
    pub fn SDL_wcslcat(
        dst: *mut u16,
        src: *const u16,
        maxlen: usize,
    ) -> usize;

    /// * wstr: 
    pub fn SDL_wcsdup(
        wstr: *const u16,
    ) -> *mut u16;

    /// * haystack: 
    /// * needle: 
    pub fn SDL_wcsstr(
        haystack: *const u16,
        needle: *const u16,
    ) -> *mut u16;

    /// * str1: 
    /// * str2: 
    pub fn SDL_wcscmp(
        str1: *const u16,
        str2: *const u16,
    ) -> i32;

    /// * str1: 
    /// * str2: 
    /// * maxlen: 
    pub fn SDL_wcsncmp(
        str1: *const u16,
        str2: *const u16,
        maxlen: usize,
    ) -> i32;

    /// * str1: 
    /// * str2: 
    pub fn SDL_wcscasecmp(
        str1: *const u16,
        str2: *const u16,
    ) -> i32;

    /// * str1: 
    /// * str2: 
    /// * len: 
    pub fn SDL_wcsncasecmp(
        str1: *const u16,
        str2: *const u16,
        len: usize,
    ) -> i32;

    /// * str: 
    pub fn SDL_strlen(
        str: *const i8,
    ) -> usize;

    /// * dst: 
    /// * src: 
    /// * maxlen: 
    pub fn SDL_strlcpy(
        dst: *mut i8,
        src: *const i8,
        maxlen: usize,
    ) -> usize;

    /// * dst: 
    /// * src: 
    /// * dst_bytes: 
    pub fn SDL_utf8strlcpy(
        dst: *mut i8,
        src: *const i8,
        dst_bytes: usize,
    ) -> usize;

    /// * dst: 
    /// * src: 
    /// * maxlen: 
    pub fn SDL_strlcat(
        dst: *mut i8,
        src: *const i8,
        maxlen: usize,
    ) -> usize;

    /// * str: 
    pub fn SDL_strdup(
        str: *const i8,
    ) -> *mut i8;

    /// * str: 
    pub fn SDL_strrev(
        str: *mut i8,
    ) -> *mut i8;

    /// * str: 
    pub fn SDL_strupr(
        str: *mut i8,
    ) -> *mut i8;

    /// * str: 
    pub fn SDL_strlwr(
        str: *mut i8,
    ) -> *mut i8;

    /// * str: 
    /// * c: 
    pub fn SDL_strchr(
        str: *const i8,
        c: i32,
    ) -> *mut i8;

    /// * str: 
    /// * c: 
    pub fn SDL_strrchr(
        str: *const i8,
        c: i32,
    ) -> *mut i8;

    /// * haystack: 
    /// * needle: 
    pub fn SDL_strstr(
        haystack: *const i8,
        needle: *const i8,
    ) -> *mut i8;

    /// * s1: 
    /// * s2: 
    /// * saveptr: 
    pub fn SDL_strtokr(
        s1: *mut i8,
        s2: *const i8,
        saveptr: *mut *mut i8,
    ) -> *mut i8;

    /// * str: 
    pub fn SDL_utf8strlen(
        str: *const i8,
    ) -> usize;

    /// * value: 
    /// * str: 
    /// * radix: 
    pub fn SDL_itoa(
        value: i32,
        str: *mut i8,
        radix: i32,
    ) -> *mut i8;

    /// * value: 
    /// * str: 
    /// * radix: 
    pub fn SDL_uitoa(
        value: u32,
        str: *mut i8,
        radix: i32,
    ) -> *mut i8;

    /// * value: 
    /// * str: 
    /// * radix: 
    pub fn SDL_ltoa(
        value: i32,
        str: *mut i8,
        radix: i32,
    ) -> *mut i8;

    /// * value: 
    /// * str: 
    /// * radix: 
    pub fn SDL_ultoa(
        value: u32,
        str: *mut i8,
        radix: i32,
    ) -> *mut i8;

    /// * value: 
    /// * str: 
    /// * radix: 
    pub fn SDL_lltoa(
        value: Sint64,
        str: *mut i8,
        radix: i32,
    ) -> *mut i8;

    /// * value: 
    /// * str: 
    /// * radix: 
    pub fn SDL_ulltoa(
        value: Uint64,
        str: *mut i8,
        radix: i32,
    ) -> *mut i8;

    /// * str: 
    pub fn SDL_atoi(
        str: *const i8,
    ) -> i32;

    /// * str: 
    pub fn SDL_atof(
        str: *const i8,
    ) -> f64;

    /// * str: 
    /// * endp: 
    /// * base: 
    pub fn SDL_strtol(
        str: *const i8,
        endp: *mut *mut i8,
        base: i32,
    ) -> i32;

    /// * str: 
    /// * endp: 
    /// * base: 
    pub fn SDL_strtoul(
        str: *const i8,
        endp: *mut *mut i8,
        base: i32,
    ) -> u32;

    /// * str: 
    /// * endp: 
    /// * base: 
    pub fn SDL_strtoll(
        str: *const i8,
        endp: *mut *mut i8,
        base: i32,
    ) -> Sint64;

    /// * str: 
    /// * endp: 
    /// * base: 
    pub fn SDL_strtoull(
        str: *const i8,
        endp: *mut *mut i8,
        base: i32,
    ) -> Uint64;

    /// * str: 
    /// * endp: 
    pub fn SDL_strtod(
        str: *const i8,
        endp: *mut *mut i8,
    ) -> f64;

    /// * str1: 
    /// * str2: 
    pub fn SDL_strcmp(
        str1: *const i8,
        str2: *const i8,
    ) -> i32;

    /// * str1: 
    /// * str2: 
    /// * maxlen: 
    pub fn SDL_strncmp(
        str1: *const i8,
        str2: *const i8,
        maxlen: usize,
    ) -> i32;

    /// * str1: 
    /// * str2: 
    pub fn SDL_strcasecmp(
        str1: *const i8,
        str2: *const i8,
    ) -> i32;

    /// * str1: 
    /// * str2: 
    /// * len: 
    pub fn SDL_strncasecmp(
        str1: *const i8,
        str2: *const i8,
        len: usize,
    ) -> i32;

    /// * text: 
    /// * fmt: 
    pub fn SDL_sscanf(
        text: *const i8,
        fmt: *const i8,
    ) -> i32;

    /// * text: 
    /// * fmt: 
    /// * ap: 
    pub fn SDL_vsscanf(
        text: *const i8,
        fmt: *const i8,
        ap: va_list::VaList,
    ) -> i32;

    /// * text: 
    /// * maxlen: 
    /// * fmt: 
    pub fn SDL_snprintf(
        text: *mut i8,
        maxlen: usize,
        fmt: *const i8,
    ) -> i32;

    /// * text: 
    /// * maxlen: 
    /// * fmt: 
    /// * ap: 
    pub fn SDL_vsnprintf(
        text: *mut i8,
        maxlen: usize,
        fmt: *const i8,
        ap: va_list::VaList,
    ) -> i32;

    /// * x: 
    pub fn SDL_acos(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_acosf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_asin(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_asinf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_atan(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_atanf(
        x: f32,
    ) -> f32;

    /// * x: 
    /// * y: 
    pub fn SDL_atan2(
        x: f64,
        y: f64,
    ) -> f64;

    /// * x: 
    /// * y: 
    pub fn SDL_atan2f(
        x: f32,
        y: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_ceil(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_ceilf(
        x: f32,
    ) -> f32;

    /// * x: 
    /// * y: 
    pub fn SDL_copysign(
        x: f64,
        y: f64,
    ) -> f64;

    /// * x: 
    /// * y: 
    pub fn SDL_copysignf(
        x: f32,
        y: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_cos(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_cosf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_exp(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_expf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_fabs(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_fabsf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_floor(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_floorf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_trunc(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_truncf(
        x: f32,
    ) -> f32;

    /// * x: 
    /// * y: 
    pub fn SDL_fmod(
        x: f64,
        y: f64,
    ) -> f64;

    /// * x: 
    /// * y: 
    pub fn SDL_fmodf(
        x: f32,
        y: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_log(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_logf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_log10(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_log10f(
        x: f32,
    ) -> f32;

    /// * x: 
    /// * y: 
    pub fn SDL_pow(
        x: f64,
        y: f64,
    ) -> f64;

    /// * x: 
    /// * y: 
    pub fn SDL_powf(
        x: f32,
        y: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_round(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_roundf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_lround(
        x: f64,
    ) -> i32;

    /// * x: 
    pub fn SDL_lroundf(
        x: f32,
    ) -> i32;

    /// * x: 
    /// * n: 
    pub fn SDL_scalbn(
        x: f64,
        n: i32,
    ) -> f64;

    /// * x: 
    /// * n: 
    pub fn SDL_scalbnf(
        x: f32,
        n: i32,
    ) -> f32;

    /// * x: 
    pub fn SDL_sin(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_sinf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_sqrt(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_sqrtf(
        x: f32,
    ) -> f32;

    /// * x: 
    pub fn SDL_tan(
        x: f64,
    ) -> f64;

    /// * x: 
    pub fn SDL_tanf(
        x: f32,
    ) -> f32;

    /// * tocode: 
    /// * fromcode: 
    pub fn SDL_iconv_open(
        tocode: *const i8,
        fromcode: *const i8,
    ) -> SDL_iconv_t;

    /// * cd: 
    pub fn SDL_iconv_close(
        cd: SDL_iconv_t,
    ) -> i32;

    /// * cd: 
    /// * inbuf: 
    /// * inbytesleft: 
    /// * outbuf: 
    /// * outbytesleft: 
    pub fn SDL_iconv(
        cd: SDL_iconv_t,
        inbuf: *const *mut i8,
        inbytesleft: *mut usize,
        outbuf: *mut *mut i8,
        outbytesleft: *mut usize,
    ) -> usize;

    /// * tocode: 
    /// * fromcode: 
    /// * inbuf: 
    /// * inbytesleft: 
    pub fn SDL_iconv_string(
        tocode: *const i8,
        fromcode: *const i8,
        inbuf: *const i8,
        inbytesleft: usize,
    ) -> *mut i8;

    /// * dst: 
    /// * src: 
    /// * dwords: 
    pub fn SDL_memcpy4(
        dst: *mut c_void,
        src: *const c_void,
        dwords: usize,
    ) -> *mut c_void;
}
