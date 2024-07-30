/* automatically generated by rust-bindgen 0.69.4 */

pub const _SYS_TIME_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const __GLIBC_USE_C2X_STRTOL: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 39;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __sigset_t_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const _BITS_ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const _BITS_ENDIANNESS_H: u32 = 1;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const FD_SETSIZE: u32 = 1024;
pub const PCM_OUT: u32 = 0;
pub const PCM_IN: u32 = 268435456;
pub const PCM_MMAP: u32 = 1;
pub const PCM_NOIRQ: u32 = 2;
pub const PCM_NORESTART: u32 = 4;
pub const PCM_MONOTONIC: u32 = 8;
pub const PCM_STATE_RUNNING: u32 = 3;
pub const PCM_STATE_XRUN: u32 = 4;
pub const PCM_STATE_DRAINING: u32 = 5;
pub const PCM_STATE_SUSPENDED: u32 = 7;
pub const PCM_STATE_DISCONNECTED: u32 = 8;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    const UNINIT: ::std::mem::MaybeUninit<__fsid_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type time_t = __time_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    const UNINIT: ::std::mem::MaybeUninit<timeval> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_usec) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
pub type suseconds_t = __suseconds_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    const UNINIT: ::std::mem::MaybeUninit<__sigset_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sigset_t>(),
        128usize,
        concat!("Size of: ", stringify!(__sigset_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigset_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigset_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigset_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    const UNINIT: ::std::mem::MaybeUninit<timespec> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_nsec) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
#[test]
fn bindgen_test_layout_fd_set() {
    const UNINIT: ::std::mem::MaybeUninit<fd_set> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(fd_set))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fds_bits) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fd_set),
            "::",
            stringify!(__fds_bits)
        )
    );
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: ::std::os::raw::c_int,
    pub tz_dsttime: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_timezone() {
    const UNINIT: ::std::mem::MaybeUninit<timezone> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<timezone>(),
        8usize,
        concat!("Size of: ", stringify!(timezone))
    );
    assert_eq!(
        ::std::mem::align_of::<timezone>(),
        4usize,
        concat!("Alignment of ", stringify!(timezone))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tz_minuteswest) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timezone),
            "::",
            stringify!(tz_minuteswest)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tz_dsttime) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(timezone),
            "::",
            stringify!(tz_dsttime)
        )
    );
}
extern "C" {
    pub fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn settimeofday(__tv: *const timeval, __tz: *const timezone) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn adjtime(__delta: *const timeval, __olddelta: *mut timeval) -> ::std::os::raw::c_int;
}
pub const __itimer_which_ITIMER_REAL: __itimer_which = 0;
pub const __itimer_which_ITIMER_VIRTUAL: __itimer_which = 1;
pub const __itimer_which_ITIMER_PROF: __itimer_which = 2;
pub type __itimer_which = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
#[test]
fn bindgen_test_layout_itimerval() {
    const UNINIT: ::std::mem::MaybeUninit<itimerval> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<itimerval>(),
        32usize,
        concat!("Size of: ", stringify!(itimerval))
    );
    assert_eq!(
        ::std::mem::align_of::<itimerval>(),
        8usize,
        concat!("Alignment of ", stringify!(itimerval))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).it_interval) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerval),
            "::",
            stringify!(it_interval)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).it_value) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerval),
            "::",
            stringify!(it_value)
        )
    );
}
pub type __itimer_which_t = ::std::os::raw::c_int;
extern "C" {
    pub fn getitimer(__which: __itimer_which_t, __value: *mut itimerval) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn utimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lutimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn futimes(__fd: ::std::os::raw::c_int, __tvp: *const timeval) -> ::std::os::raw::c_int;
}
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    const UNINIT: ::std::mem::MaybeUninit<max_align_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce2) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcm {
    _unused: [u8; 0],
}
pub const pcm_format_PCM_FORMAT_S16_LE: pcm_format = 0;
pub const pcm_format_PCM_FORMAT_S32_LE: pcm_format = 1;
pub const pcm_format_PCM_FORMAT_S8: pcm_format = 2;
pub const pcm_format_PCM_FORMAT_S24_LE: pcm_format = 3;
pub const pcm_format_PCM_FORMAT_MAX: pcm_format = 4;
pub type pcm_format = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcm_mask {
    pub bits: [::std::os::raw::c_uint; 8usize],
}
#[test]
fn bindgen_test_layout_pcm_mask() {
    const UNINIT: ::std::mem::MaybeUninit<pcm_mask> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pcm_mask>(),
        32usize,
        concat!("Size of: ", stringify!(pcm_mask))
    );
    assert_eq!(
        ::std::mem::align_of::<pcm_mask>(),
        4usize,
        concat!("Alignment of ", stringify!(pcm_mask))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bits) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pcm_mask),
            "::",
            stringify!(bits)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcm_config {
    pub channels: ::std::os::raw::c_uint,
    pub rate: ::std::os::raw::c_uint,
    pub period_size: ::std::os::raw::c_uint,
    pub period_count: ::std::os::raw::c_uint,
    pub format: pcm_format,
    pub start_threshold: ::std::os::raw::c_ulong,
    pub stop_threshold: ::std::os::raw::c_ulong,
    pub silence_threshold: ::std::os::raw::c_ulong,
    // pub start_threshold: ::std::os::raw::c_uint,
    // pub stop_threshold: ::std::os::raw::c_uint,
    // pub silence_threshold: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_pcm_config() {
    const UNINIT: ::std::mem::MaybeUninit<pcm_config> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pcm_config>(),
        32usize,
        concat!("Size of: ", stringify!(pcm_config))
    );
    assert_eq!(
        ::std::mem::align_of::<pcm_config>(),
        4usize,
        concat!("Alignment of ", stringify!(pcm_config))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).channels) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pcm_config),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rate) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(pcm_config),
            "::",
            stringify!(rate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).period_size) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(pcm_config),
            "::",
            stringify!(period_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).period_count) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(pcm_config),
            "::",
            stringify!(period_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(pcm_config),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).start_threshold) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(pcm_config),
            "::",
            stringify!(start_threshold)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stop_threshold) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(pcm_config),
            "::",
            stringify!(stop_threshold)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).silence_threshold) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(pcm_config),
            "::",
            stringify!(silence_threshold)
        )
    );
}
pub const pcm_param_PCM_PARAM_ACCESS: pcm_param = 0;
pub const pcm_param_PCM_PARAM_FORMAT: pcm_param = 1;
pub const pcm_param_PCM_PARAM_SUBFORMAT: pcm_param = 2;
pub const pcm_param_PCM_PARAM_SAMPLE_BITS: pcm_param = 3;
pub const pcm_param_PCM_PARAM_FRAME_BITS: pcm_param = 4;
pub const pcm_param_PCM_PARAM_CHANNELS: pcm_param = 5;
pub const pcm_param_PCM_PARAM_RATE: pcm_param = 6;
pub const pcm_param_PCM_PARAM_PERIOD_TIME: pcm_param = 7;
pub const pcm_param_PCM_PARAM_PERIOD_SIZE: pcm_param = 8;
pub const pcm_param_PCM_PARAM_PERIOD_BYTES: pcm_param = 9;
pub const pcm_param_PCM_PARAM_PERIODS: pcm_param = 10;
pub const pcm_param_PCM_PARAM_BUFFER_TIME: pcm_param = 11;
pub const pcm_param_PCM_PARAM_BUFFER_SIZE: pcm_param = 12;
pub const pcm_param_PCM_PARAM_BUFFER_BYTES: pcm_param = 13;
pub const pcm_param_PCM_PARAM_TICK_TIME: pcm_param = 14;
pub type pcm_param = ::std::os::raw::c_uint;
pub const mixer_ctl_type_MIXER_CTL_TYPE_BOOL: mixer_ctl_type = 0;
pub const mixer_ctl_type_MIXER_CTL_TYPE_INT: mixer_ctl_type = 1;
pub const mixer_ctl_type_MIXER_CTL_TYPE_ENUM: mixer_ctl_type = 2;
pub const mixer_ctl_type_MIXER_CTL_TYPE_BYTE: mixer_ctl_type = 3;
pub const mixer_ctl_type_MIXER_CTL_TYPE_IEC958: mixer_ctl_type = 4;
pub const mixer_ctl_type_MIXER_CTL_TYPE_INT64: mixer_ctl_type = 5;
pub const mixer_ctl_type_MIXER_CTL_TYPE_UNKNOWN: mixer_ctl_type = 6;
pub const mixer_ctl_type_MIXER_CTL_TYPE_MAX: mixer_ctl_type = 7;
pub type mixer_ctl_type = ::std::os::raw::c_uint;

#[link(name = "tinyalsa")]
extern "C" {
    pub fn pcm_open(
        card: ::std::os::raw::c_uint,
        device: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_uint,
        config: *mut pcm_config,
    ) -> *mut pcm;
    pub fn pcm_close(pcm: *mut pcm) -> ::std::os::raw::c_int;
    pub fn pcm_is_ready(pcm: *mut pcm) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcm_params {
    _unused: [u8; 0],
}
#[link(name = "tinyalsa")]
extern "C" {
    pub fn pcm_params_get(
        card: ::std::os::raw::c_uint,
        device: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_uint,
    ) -> *mut pcm_params;
    pub fn pcm_params_free(pcm_params: *mut pcm_params);
    pub fn pcm_params_get_mask(pcm_params: *mut pcm_params, param: pcm_param) -> *mut pcm_mask;
    pub fn pcm_params_get_min(
        pcm_params: *mut pcm_params,
        param: pcm_param,
    ) -> ::std::os::raw::c_uint;
    pub fn pcm_params_get_max(
        pcm_params: *mut pcm_params,
        param: pcm_param,
    ) -> ::std::os::raw::c_uint;
    pub fn pcm_get_error(pcm: *mut pcm) -> *const ::std::os::raw::c_char;
    pub fn pcm_format_to_bits(format: pcm_format) -> ::std::os::raw::c_uint;
    pub fn pcm_get_buffer_size(pcm: *mut pcm) -> ::std::os::raw::c_uint;
    pub fn pcm_frames_to_bytes(
        pcm: *mut pcm,
        frames: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
    pub fn pcm_bytes_to_frames(
        pcm: *mut pcm,
        bytes: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
    pub fn pcm_get_htimestamp(
        pcm: *mut pcm,
        avail: *mut ::std::os::raw::c_uint,
        tstamp: *mut timespec,
    ) -> ::std::os::raw::c_int;
    pub fn pcm_write(
        pcm: *mut pcm,
        data: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn pcm_readi(
        pcm: *mut pcm,
        data: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn pcm_read(
        pcm: *mut pcm,
        data: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn pcm_mmap_write(
        pcm: *mut pcm,
        data: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn pcm_mmap_read(
        pcm: *mut pcm,
        data: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn pcm_mmap_begin(
        pcm: *mut pcm,
        areas: *mut *mut ::std::os::raw::c_void,
        offset: *mut ::std::os::raw::c_uint,
        frames: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn pcm_mmap_commit(
        pcm: *mut pcm,
        offset: ::std::os::raw::c_uint,
        frames: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn pcm_prepare(pcm: *mut pcm) -> ::std::os::raw::c_int;
    pub fn pcm_start(pcm: *mut pcm) -> ::std::os::raw::c_int;
    pub fn pcm_stop(pcm: *mut pcm) -> ::std::os::raw::c_int;
    pub fn pcm_wait(pcm: *mut pcm, timeout: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mixer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mixer_ctl {
    _unused: [u8; 0],
}
extern "C" {
    pub fn mixer_open(card: ::std::os::raw::c_uint) -> *mut mixer;
    pub fn mixer_close(mixer: *mut mixer);
    pub fn mixer_get_name(mixer: *mut mixer) -> *const ::std::os::raw::c_char;
    pub fn mixer_get_num_ctls(mixer: *mut mixer) -> ::std::os::raw::c_uint;
    pub fn mixer_get_ctl(mixer: *mut mixer, id: ::std::os::raw::c_uint) -> *mut mixer_ctl;
    pub fn mixer_get_ctl_by_name(
        mixer: *mut mixer,
        name: *const ::std::os::raw::c_char,
    ) -> *mut mixer_ctl;
    pub fn mixer_ctl_get_name(ctl: *mut mixer_ctl) -> *const ::std::os::raw::c_char;
    pub fn mixer_ctl_get_type(ctl: *mut mixer_ctl) -> mixer_ctl_type;
    pub fn mixer_ctl_get_type_string(ctl: *mut mixer_ctl) -> *const ::std::os::raw::c_char;
    pub fn mixer_ctl_get_num_values(ctl: *mut mixer_ctl) -> ::std::os::raw::c_uint;
    pub fn mixer_ctl_get_num_enums(ctl: *mut mixer_ctl) -> ::std::os::raw::c_uint;
    pub fn mixer_ctl_get_enum_string(
        ctl: *mut mixer_ctl,
        enum_id: ::std::os::raw::c_uint,
    ) -> *const ::std::os::raw::c_char;
    pub fn mixer_ctl_update(ctl: *mut mixer_ctl);
    pub fn mixer_ctl_get_percent(
        ctl: *mut mixer_ctl,
        id: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn mixer_ctl_set_percent(
        ctl: *mut mixer_ctl,
        id: ::std::os::raw::c_uint,
        percent: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn mixer_ctl_get_value(
        ctl: *mut mixer_ctl,
        id: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn mixer_ctl_get_array(
        ctl: *mut mixer_ctl,
        array: *mut ::std::os::raw::c_void,
        count: usize,
    ) -> ::std::os::raw::c_int;
    pub fn mixer_ctl_set_value(
        ctl: *mut mixer_ctl,
        id: ::std::os::raw::c_uint,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn mixer_ctl_set_array(
        ctl: *mut mixer_ctl,
        array: *const ::std::os::raw::c_void,
        count: usize,
    ) -> ::std::os::raw::c_int;
    pub fn mixer_ctl_set_enum_by_string(
        ctl: *mut mixer_ctl,
        string: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn mixer_ctl_get_range_min(ctl: *mut mixer_ctl) -> ::std::os::raw::c_int;
    pub fn mixer_ctl_get_range_max(ctl: *mut mixer_ctl) -> ::std::os::raw::c_int;
}
