// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
use std::ffi::c_void;
extern crate va_list;
use super::*;

/* SDL_arraysize(array)(sizeof(array)/sizeof(array[0])) */
/* SDL_TABLESIZE(table)SDL_arraysize(table) */
/* SDL_STRINGIFY_ARG(arg)#arg */
/* SDL_reinterpret_cast(type,expression)reinterpret_cast<type>(expression) */
/* SDL_static_cast(type,expression)static_cast<type>(expression) */
/* SDL_const_cast(type,expression)const_cast<type>(expression) */
/* SDL_FOURCC(A,B,C,D)((SDL_static_cast(Uint32,SDL_static_cast(Uint8,(A)))<<0)|(SDL_static_cast(Uint32,SDL_static_cast(Uint8,(B)))<<8)|(SDL_static_cast(Uint32,SDL_static_cast(Uint8,(C)))<<16)|(SDL_static_cast(Uint32,SDL_static_cast(Uint8,(D)))<<24)) */
//SDL_MAX_SINT8 ((Sint8)0x7F)
//SDL_MIN_SINT8 ((Sint8)(~0x7F))
//SDL_MAX_UINT8 ((Uint8)0xFF)
//SDL_MIN_UINT8 ((Uint8)0x00)
//SDL_MAX_SINT16 ((Sint16)0x7FFF)
//SDL_MIN_SINT16 ((Sint16)(~0x7FFF))
//SDL_MAX_UINT16 ((Uint16)0xFFFF)
//SDL_MIN_UINT16 ((Uint16)0x0000)
//SDL_MAX_SINT32 ((Sint32)0x7FFFFFFF)
//SDL_MIN_SINT32 ((Sint32)(~0x7FFFFFFF))
//SDL_MAX_UINT32 ((Uint32)0xFFFFFFFFu)
//SDL_MIN_UINT32 ((Uint32)0x00000000)
//SDL_MAX_SINT64 ((Sint64)0x7FFFFFFFFFFFFFFFll)
//SDL_MIN_SINT64 ((Sint64)(~0x7FFFFFFFFFFFFFFFll))
//SDL_MAX_UINT64 ((Uint64)0xFFFFFFFFFFFFFFFFull)
//SDL_MIN_UINT64 ((Uint64)(0x0000000000000000ull))
//SDL_PRIs64 "I64d"
//SDL_PRIu64 "I64u"
//SDL_PRIx64 "I64x"
//SDL_PRIX64 "I64X"
//SDL_PRIs32 "d"
//SDL_PRIu32 "u"
//SDL_PRIx32 "x"
//SDL_PRIX32 "X"
/* SDL_IN_BYTECAP(x)_In_bytecount_(x) */
/* SDL_INOUT_Z_CAP(x)_Inout_z_cap_(x) */
/* SDL_OUT_Z_CAP(x)_Out_z_cap_(x) */
/* SDL_OUT_CAP(x)_Out_cap_(x) */
/* SDL_OUT_BYTECAP(x)_Out_bytecap_(x) */
/* SDL_OUT_Z_BYTECAP(x)_Out_z_bytecap_(x) */
//SDL_PRINTF_FORMAT_STRING _Printf_format_string_
//SDL_SCANF_FORMAT_STRING _Scanf_format_string_impl_
// SDL_PRINTF_VARARG_FUNC(fmtargnumber)
// SDL_SCANF_VARARG_FUNC(fmtargnumber)
/* SDL_COMPILE_TIME_ASSERT(name,x)typedefintSDL_compile_time_assert_##name[(x)*2-1] */
/* SDL_stack_alloc(type,count)(type*)SDL_malloc(sizeof(type)*(count)) */
/* SDL_stack_free(data)SDL_free(data) */
/* SDL_min(x,y)(((x)<(y))?(x):(y)) */
/* SDL_max(x,y)(((x)>(y))?(x):(y)) */
/* SDL_zero(x)SDL_memset(&(x),0,sizeof((x))) */
/* SDL_zerop(x)SDL_memset((x),0,sizeof(*(x))) */
/* SDL_zeroa(x)SDL_memset((x),0,sizeof((x))) */
//M_PI 3.14159265358979323846264338327950288
//SDL_ICONV_ERROR (size_t)-1
//SDL_ICONV_E2BIG (size_t)-2
//SDL_ICONV_EILSEQ (size_t)-3
//SDL_ICONV_EINVAL (size_t)-4
/* SDL_iconv_utf8_locale(S)SDL_iconv_string("","UTF-8",S,SDL_strlen(S)+1) */
/* SDL_iconv_utf8_ucs2(S)(Uint16*)SDL_iconv_string("UCS-2-INTERNAL","UTF-8",S,SDL_strlen(S)+1) */
/* SDL_iconv_utf8_ucs4(S)(Uint32*)SDL_iconv_string("UCS-4-INTERNAL","UTF-8",S,SDL_strlen(S)+1) */
pub const SDL_FALSE: i32 = 0;
pub const SDL_TRUE: i32 = 0x1;
pub const DUMMY_ENUM_VALUE: i32 = 0;
pub type _SDL_iconv_t = c_void;

#[link(name = "SDL2", kind = "static")]
extern "C" {

    /// * size: 
    pub fn SDL_malloc(
        size: u64,
    ) -> *mut c_void;

    /// * nmemb: 
    /// * size: 
    pub fn SDL_calloc(
        nmemb: u64,
        size: u64,
    ) -> *mut c_void;

    /// * mem: 
    /// * size: 
    pub fn SDL_realloc(
        mem: *mut c_void,
        size: u64,
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
        malloc_func: *mut extern fn(u64,) -> *mut c_void,
        calloc_func: *mut extern fn(u64,u64,) -> *mut c_void,
        realloc_func: *mut extern fn(*mut c_void,u64,) -> *mut c_void,
        free_func: *mut extern fn(*mut c_void,) -> c_void,
    ) -> c_void;

    /// * malloc_func: 
    /// * calloc_func: 
    /// * realloc_func: 
    /// * free_func: 
    pub fn SDL_SetMemoryFunctions(
        malloc_func: extern fn(u64,) -> *mut c_void,
        calloc_func: extern fn(u64,u64,) -> *mut c_void,
        realloc_func: extern fn(*mut c_void,u64,) -> *mut c_void,
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

    /// * base: 
    /// * nmemb: 
    /// * size: 
    /// * compare: 
    pub fn SDL_qsort(
        base: *mut c_void,
        nmemb: u64,
        size: u64,
        compare: *mut extern fn(*mut c_void,*mut c_void,) -> i32,
    ) -> c_void;

    /// * : 
    /// * : 
    #[link_name = "?compare@?1??SDL_qsort@@9@3P6AHPEBX0@ZEA"]
    pub fn compare(
        _0: *const c_void,
        _1: *const c_void,
    ) -> i32;

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
        len: u64,
    ) -> u32;

    /// * dst: 
    /// * c: 
    /// * len: 
    pub fn SDL_memset(
        dst: *mut c_void,
        c: i32,
        len: u64,
    ) -> *mut c_void;

    /// * dst: 
    /// * val: 
    /// * dwords: 
    pub fn SDL_memset4(
        dst: *mut c_void,
        val: u32,
        dwords: u64,
    ) -> c_void;

    /// * dst: 
    /// * src: 
    /// * len: 
    pub fn SDL_memcpy(
        dst: *mut c_void,
        src: *const c_void,
        len: u64,
    ) -> *mut c_void;

    /// * dst: 
    /// * src: 
    /// * len: 
    pub fn SDL_memmove(
        dst: *mut c_void,
        src: *const c_void,
        len: u64,
    ) -> *mut c_void;

    /// * s1: 
    /// * s2: 
    /// * len: 
    pub fn SDL_memcmp(
        s1: *const c_void,
        s2: *const c_void,
        len: u64,
    ) -> i32;

    /// * wstr: 
    pub fn SDL_wcslen(
        wstr: *const u16,
    ) -> u64;

    /// * dst: 
    /// * src: 
    /// * maxlen: 
    pub fn SDL_wcslcpy(
        dst: *mut u16,
        src: *const u16,
        maxlen: u64,
    ) -> u64;

    /// * dst: 
    /// * src: 
    /// * maxlen: 
    pub fn SDL_wcslcat(
        dst: *mut u16,
        src: *const u16,
        maxlen: u64,
    ) -> u64;

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
        maxlen: u64,
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
        len: u64,
    ) -> i32;

    /// * str: 
    pub fn SDL_strlen(
        str: *const i8,
    ) -> u64;

    /// * dst: 
    /// * src: 
    /// * maxlen: 
    pub fn SDL_strlcpy(
        dst: *mut i8,
        src: *const i8,
        maxlen: u64,
    ) -> u64;

    /// * dst: 
    /// * src: 
    /// * dst_bytes: 
    pub fn SDL_utf8strlcpy(
        dst: *mut i8,
        src: *const i8,
        dst_bytes: u64,
    ) -> u64;

    /// * dst: 
    /// * src: 
    /// * maxlen: 
    pub fn SDL_strlcat(
        dst: *mut i8,
        src: *const i8,
        maxlen: u64,
    ) -> u64;

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
    ) -> u64;

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
        value: i64,
        str: *mut i8,
        radix: i32,
    ) -> *mut i8;

    /// * value: 
    /// * str: 
    /// * radix: 
    pub fn SDL_ulltoa(
        value: u64,
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
    ) -> i64;

    /// * str: 
    /// * endp: 
    /// * base: 
    pub fn SDL_strtoull(
        str: *const i8,
        endp: *mut *mut i8,
        base: i32,
    ) -> u64;

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
        maxlen: u64,
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
        len: u64,
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
        ap: *mut i8,
    ) -> i32;

    /// * text: 
    /// * maxlen: 
    /// * fmt: 
    pub fn SDL_snprintf(
        text: *mut i8,
        maxlen: u64,
        fmt: *const i8,
    ) -> i32;

    /// * text: 
    /// * maxlen: 
    /// * fmt: 
    /// * ap: 
    pub fn SDL_vsnprintf(
        text: *mut i8,
        maxlen: u64,
        fmt: *const i8,
        ap: *mut i8,
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
    ) -> *mut _SDL_iconv_t;

    /// * cd: 
    pub fn SDL_iconv_close(
        cd: *mut _SDL_iconv_t,
    ) -> i32;

    /// * cd: 
    /// * inbuf: 
    /// * inbytesleft: 
    /// * outbuf: 
    /// * outbytesleft: 
    pub fn SDL_iconv(
        cd: *mut _SDL_iconv_t,
        inbuf: *const *mut i8,
        inbytesleft: *mut u64,
        outbuf: *mut *mut i8,
        outbytesleft: *mut u64,
    ) -> u64;

    /// * tocode: 
    /// * fromcode: 
    /// * inbuf: 
    /// * inbytesleft: 
    pub fn SDL_iconv_string(
        tocode: *const i8,
        fromcode: *const i8,
        inbuf: *const i8,
        inbytesleft: u64,
    ) -> *mut i8;

    /// * dst: 
    /// * src: 
    /// * dwords: 
    pub fn SDL_memcpy4(
        dst: *mut c_void,
        src: *const c_void,
        dwords: u64,
    ) -> *mut c_void;
}
