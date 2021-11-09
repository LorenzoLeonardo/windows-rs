#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AssignProcessToJobObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, hprocess: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AssignProcessToJobObject(hjob: super::super::Foundation::HANDLE, hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AssignProcessToJobObject(hjob.into_param().abi(), hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateJobObjectA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateJobObjectA(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateJobObjectA(::core::mem::transmute(lpjobattributes), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateJobObjectW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateJobObjectW(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateJobObjectW(::core::mem::transmute(lpjobattributes), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`*"]
#[inline]
pub unsafe fn FreeMemoryJobObject(buffer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeMemoryJobObject(buffer: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(FreeMemoryJobObject(::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(processhandle: Param0, jobhandle: Param1, result: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInJob(processhandle: super::super::Foundation::HANDLE, jobhandle: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsProcessInJob(processhandle.into_param().abi(), jobhandle.into_param().abi(), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: super::super::Foundation::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: super::SystemServices::JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl JOBOBJECT_IO_RATE_CONTROL_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION")
            .field("MaxIops", &self.MaxIops)
            .field("MaxBandwidth", &self.MaxBandwidth)
            .field("ReservationIops", &self.ReservationIops)
            .field("VolumeName", &self.VolumeName)
            .field("BaseIoSize", &self.BaseIoSize)
            .field("ControlFlags", &self.ControlFlags)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIops == other.MaxIops && self.MaxBandwidth == other.MaxBandwidth && self.ReservationIops == other.ReservationIops && self.VolumeName == other.VolumeName && self.BaseIoSize == other.BaseIoSize && self.ControlFlags == other.ControlFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_JobObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_LIMIT(pub u32);
pub const JOB_OBJECT_LIMIT_WORKINGSET: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1u32);
pub const JOB_OBJECT_LIMIT_PROCESS_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2u32);
pub const JOB_OBJECT_LIMIT_JOB_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(4u32);
pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(8u32);
pub const JOB_OBJECT_LIMIT_AFFINITY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(16u32);
pub const JOB_OBJECT_LIMIT_PRIORITY_CLASS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32u32);
pub const JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(64u32);
pub const JOB_OBJECT_LIMIT_SCHEDULING_CLASS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(128u32);
pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(256u32);
pub const JOB_OBJECT_LIMIT_JOB_MEMORY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(512u32);
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(512u32);
pub const JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1024u32);
pub const JOB_OBJECT_LIMIT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2048u32);
pub const JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(4096u32);
pub const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(8192u32);
pub const JOB_OBJECT_LIMIT_SUBSET_AFFINITY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(16384u32);
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_LOW: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32768u32);
pub const JOB_OBJECT_LIMIT_JOB_READ_BYTES: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(65536u32);
pub const JOB_OBJECT_LIMIT_JOB_WRITE_BYTES: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(131072u32);
pub const JOB_OBJECT_LIMIT_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(262144u32);
pub const JOB_OBJECT_LIMIT_CPU_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(262144u32);
pub const JOB_OBJECT_LIMIT_IO_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(524288u32);
pub const JOB_OBJECT_LIMIT_NET_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1048576u32);
pub const JOB_OBJECT_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(524287u32);
pub const JOB_OBJECT_BASIC_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(255u32);
pub const JOB_OBJECT_EXTENDED_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32767u32);
pub const JOB_OBJECT_NOTIFICATION_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2064900u32);
impl ::core::convert::From<u32> for JOB_OBJECT_LIMIT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JOB_OBJECT_LIMIT {
    type Abi = Self;
}
impl ::core::ops::BitOr for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_LIMIT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_LIMIT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_JobObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_SECURITY(pub u32);
pub const JOB_OBJECT_SECURITY_NO_ADMIN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(1u32);
pub const JOB_OBJECT_SECURITY_RESTRICTED_TOKEN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(2u32);
pub const JOB_OBJECT_SECURITY_ONLY_TOKEN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(4u32);
pub const JOB_OBJECT_SECURITY_FILTER_TOKENS: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(8u32);
pub const JOB_OBJECT_SECURITY_VALID_FLAGS: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(15u32);
impl ::core::convert::From<u32> for JOB_OBJECT_SECURITY {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JOB_OBJECT_SECURITY {
    type Abi = Self;
}
impl ::core::ops::BitOr for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_SECURITY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_SECURITY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_JobObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_UILIMIT(pub u32);
pub const JOB_OBJECT_UILIMIT_NONE: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(0u32);
pub const JOB_OBJECT_UILIMIT_HANDLES: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(1u32);
pub const JOB_OBJECT_UILIMIT_READCLIPBOARD: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(2u32);
pub const JOB_OBJECT_UILIMIT_WRITECLIPBOARD: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(4u32);
pub const JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(8u32);
pub const JOB_OBJECT_UILIMIT_DISPLAYSETTINGS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(16u32);
pub const JOB_OBJECT_UILIMIT_GLOBALATOMS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(32u32);
pub const JOB_OBJECT_UILIMIT_DESKTOP: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(64u32);
pub const JOB_OBJECT_UILIMIT_EXITWINDOWS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(128u32);
impl ::core::convert::From<u32> for JOB_OBJECT_UILIMIT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JOB_OBJECT_UILIMIT {
    type Abi = Self;
}
impl ::core::ops::BitOr for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_UILIMIT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_UILIMIT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenJobObjectA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenJobObjectA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenJobObjectA(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenJobObjectW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenJobObjectW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenJobObjectW(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn QueryInformationJobObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, jobobjectinformationclass: super::SystemServices::JOBOBJECTINFOCLASS, lpjobobjectinformation: *mut ::core::ffi::c_void, cbjobobjectinformationlength: u32, lpreturnlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryInformationJobObject(hjob: super::super::Foundation::HANDLE, jobobjectinformationclass: super::SystemServices::JOBOBJECTINFOCLASS, lpjobobjectinformation: *mut ::core::ffi::c_void, cbjobobjectinformationlength: u32, lpreturnlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryInformationJobObject(hjob.into_param().abi(), ::core::mem::transmute(jobobjectinformationclass), ::core::mem::transmute(lpjobobjectinformation), ::core::mem::transmute(cbjobobjectinformationlength), ::core::mem::transmute(lpreturnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn QueryIoRateControlInformationJobObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hjob: Param0, volumename: Param1, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryIoRateControlInformationJobObject(hjob: super::super::Foundation::HANDLE, volumename: super::super::Foundation::PWSTR, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut u32) -> u32;
        }
        ::core::mem::transmute(QueryIoRateControlInformationJobObject(hjob.into_param().abi(), volumename.into_param().abi(), ::core::mem::transmute(infoblocks), ::core::mem::transmute(infoblockcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn SetInformationJobObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, jobobjectinformationclass: super::SystemServices::JOBOBJECTINFOCLASS, lpjobobjectinformation: *const ::core::ffi::c_void, cbjobobjectinformationlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetInformationJobObject(hjob: super::super::Foundation::HANDLE, jobobjectinformationclass: super::SystemServices::JOBOBJECTINFOCLASS, lpjobobjectinformation: *const ::core::ffi::c_void, cbjobobjectinformationlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetInformationJobObject(hjob.into_param().abi(), ::core::mem::transmute(jobobjectinformationclass), ::core::mem::transmute(lpjobobjectinformation), ::core::mem::transmute(cbjobobjectinformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn SetIoRateControlInformationJobObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIoRateControlInformationJobObject(hjob: super::super::Foundation::HANDLE, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32;
        }
        ::core::mem::transmute(SetIoRateControlInformationJobObject(hjob.into_param().abi(), ::core::mem::transmute(ioratecontrolinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateJobObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, uexitcode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateJobObject(hjob: super::super::Foundation::HANDLE, uexitcode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TerminateJobObject(hjob.into_param().abi(), ::core::mem::transmute(uexitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserHandleGrantAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(huserhandle: Param0, hjob: Param1, bgrant: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserHandleGrantAccess(huserhandle: super::super::Foundation::HANDLE, hjob: super::super::Foundation::HANDLE, bgrant: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UserHandleGrantAccess(huserhandle.into_param().abi(), hjob.into_param().abi(), bgrant.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
