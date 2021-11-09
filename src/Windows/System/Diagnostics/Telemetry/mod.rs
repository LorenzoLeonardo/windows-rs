#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformTelemetryClientStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlatformTelemetryClientStatics {
    type Vtable = IPlatformTelemetryClientStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2616455773, 54723, 20202, [141, 190, 156, 141, 187, 13, 157, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryClientStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, settings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlatformTelemetryRegistrationResult {
    type Vtable = IPlatformTelemetryRegistrationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1300568235, 8850, 18877, [161, 90, 61, 113, 210, 20, 81, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlatformTelemetryRegistrationStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationSettings(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlatformTelemetryRegistrationSettings {
    type Vtable = IPlatformTelemetryRegistrationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2174387586, 51737, 16734, [187, 121, 156, 34, 75, 250, 58, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
pub struct PlatformTelemetryClient {}
impl PlatformTelemetryClient {
    #[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
    pub fn Register<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(id: Param0) -> ::windows::runtime::Result<PlatformTelemetryRegistrationResult> {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<PlatformTelemetryRegistrationResult>(result__)
        })
    }
    #[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
    pub fn RegisterWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PlatformTelemetryRegistrationSettings>>(id: Param0, settings: Param1) -> ::windows::runtime::Result<PlatformTelemetryRegistrationResult> {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), settings.into_param().abi(), &mut result__).from_abi::<PlatformTelemetryRegistrationResult>(result__)
        })
    }
    pub fn IPlatformTelemetryClientStatics<R, F: FnOnce(&IPlatformTelemetryClientStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlatformTelemetryClient, IPlatformTelemetryClientStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PlatformTelemetryClient {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient";
}
#[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlatformTelemetryRegistrationResult(pub ::windows::runtime::IInspectable);
impl PlatformTelemetryRegistrationResult {
    #[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PlatformTelemetryRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: PlatformTelemetryRegistrationStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlatformTelemetryRegistrationStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlatformTelemetryRegistrationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult;{4d8518ab-2292-49bd-a15a-3d71d2145112})");
}
unsafe impl ::windows::runtime::Interface for PlatformTelemetryRegistrationResult {
    type Vtable = IPlatformTelemetryRegistrationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1300568235, 8850, 18877, [161, 90, 61, 113, 210, 20, 81, 18]);
}
impl ::windows::runtime::RuntimeName for PlatformTelemetryRegistrationResult {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult";
}
impl ::core::convert::From<PlatformTelemetryRegistrationResult> for ::windows::runtime::IUnknown {
    fn from(value: PlatformTelemetryRegistrationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationResult> for ::windows::runtime::IUnknown {
    fn from(value: &PlatformTelemetryRegistrationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlatformTelemetryRegistrationResult> for ::windows::runtime::IInspectable {
    fn from(value: PlatformTelemetryRegistrationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationResult> for ::windows::runtime::IInspectable {
    fn from(value: &PlatformTelemetryRegistrationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlatformTelemetryRegistrationResult {}
unsafe impl ::core::marker::Sync for PlatformTelemetryRegistrationResult {}
#[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlatformTelemetryRegistrationSettings(pub ::windows::runtime::IInspectable);
impl PlatformTelemetryRegistrationSettings {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlatformTelemetryRegistrationSettings, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
    pub fn StorageSize(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
    pub fn SetStorageSize(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
    pub fn UploadQuotaSize(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
    pub fn SetUploadQuotaSize(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlatformTelemetryRegistrationSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings;{819a8582-ca19-415e-bb79-9c224bfa3a73})");
}
unsafe impl ::windows::runtime::Interface for PlatformTelemetryRegistrationSettings {
    type Vtable = IPlatformTelemetryRegistrationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2174387586, 51737, 16734, [187, 121, 156, 34, 75, 250, 58, 115]);
}
impl ::windows::runtime::RuntimeName for PlatformTelemetryRegistrationSettings {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings";
}
impl ::core::convert::From<PlatformTelemetryRegistrationSettings> for ::windows::runtime::IUnknown {
    fn from(value: PlatformTelemetryRegistrationSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationSettings> for ::windows::runtime::IUnknown {
    fn from(value: &PlatformTelemetryRegistrationSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlatformTelemetryRegistrationSettings> for ::windows::runtime::IInspectable {
    fn from(value: PlatformTelemetryRegistrationSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationSettings> for ::windows::runtime::IInspectable {
    fn from(value: &PlatformTelemetryRegistrationSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlatformTelemetryRegistrationSettings {}
unsafe impl ::core::marker::Sync for PlatformTelemetryRegistrationSettings {}
#[doc = "*Required features: `System_Diagnostics_Telemetry`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationStatus(pub i32);
impl PlatformTelemetryRegistrationStatus {
    pub const Success: PlatformTelemetryRegistrationStatus = PlatformTelemetryRegistrationStatus(0i32);
    pub const SettingsOutOfRange: PlatformTelemetryRegistrationStatus = PlatformTelemetryRegistrationStatus(1i32);
    pub const UnknownFailure: PlatformTelemetryRegistrationStatus = PlatformTelemetryRegistrationStatus(2i32);
}
impl ::core::convert::From<i32> for PlatformTelemetryRegistrationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlatformTelemetryRegistrationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlatformTelemetryRegistrationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationStatus;i4)");
}
impl ::windows::runtime::DefaultType for PlatformTelemetryRegistrationStatus {
    type DefaultType = Self;
}
