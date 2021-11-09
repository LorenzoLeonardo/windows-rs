#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IIndexedResourceCandidate(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIndexedResourceCandidate {
    type Vtable = IIndexedResourceCandidate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(241278707, 64236, 17428, [169, 215, 84, 172, 213, 149, 63, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceCandidate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IndexedResourceType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, qualifiername: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIndexedResourceQualifier(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIndexedResourceQualifier {
    type Vtable = IIndexedResourceQualifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3672357787, 54020, 18815, [161, 104, 163, 64, 4, 44, 138, 219]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexedResourceQualifier_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceIndexer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceIndexer {
    type Vtable = IResourceIndexer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(760019365, 58159, 19122, [135, 72, 150, 53, 10, 1, 109, 163]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceIndexerFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceIndexerFactory {
    type Vtable = IResourceIndexerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3101572873, 12749, 19863, [189, 48, 141, 57, 247, 66, 188, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, projectroot: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceIndexerFactory2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceIndexerFactory2 {
    type Vtable = IResourceIndexerFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1614868877, 54757, 19296, [146, 1, 205, 39, 156, 188, 254, 217]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceIndexerFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, projectroot: ::windows::runtime::RawPtr, extensiondllpath: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `ApplicationModel_Resources_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IndexedResourceCandidate(pub ::windows::runtime::IInspectable);
impl IndexedResourceCandidate {
    #[doc = "*Required features: `ApplicationModel_Resources_Management`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<IndexedResourceType> {
        let this = self;
        unsafe {
            let mut result__: IndexedResourceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IndexedResourceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Resources_Management`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Management`, `Foundation_Collections`*"]
    pub fn Metadata(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Resources_Management`, `Foundation_Collections`*"]
    pub fn Qualifiers(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Management`*"]
    pub fn ValueAsString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Management`*"]
    pub fn GetQualifierValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, qualifiername: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), qualifiername.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IndexedResourceCandidate {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.IndexedResourceCandidate;{0e619ef3-faec-4414-a9d7-54acd5953f29})");
}
unsafe impl ::windows::runtime::Interface for IndexedResourceCandidate {
    type Vtable = IIndexedResourceCandidate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(241278707, 64236, 17428, [169, 215, 84, 172, 213, 149, 63, 41]);
}
impl ::windows::runtime::RuntimeName for IndexedResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceCandidate";
}
impl ::core::convert::From<IndexedResourceCandidate> for ::windows::runtime::IUnknown {
    fn from(value: IndexedResourceCandidate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IndexedResourceCandidate> for ::windows::runtime::IUnknown {
    fn from(value: &IndexedResourceCandidate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IndexedResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IndexedResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IndexedResourceCandidate> for ::windows::runtime::IInspectable {
    fn from(value: IndexedResourceCandidate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IndexedResourceCandidate> for ::windows::runtime::IInspectable {
    fn from(value: &IndexedResourceCandidate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IndexedResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IndexedResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IndexedResourceCandidate {}
unsafe impl ::core::marker::Sync for IndexedResourceCandidate {}
#[doc = "*Required features: `ApplicationModel_Resources_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IndexedResourceQualifier(pub ::windows::runtime::IInspectable);
impl IndexedResourceQualifier {
    #[doc = "*Required features: `ApplicationModel_Resources_Management`*"]
    pub fn QualifierName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources_Management`*"]
    pub fn QualifierValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IndexedResourceQualifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.IndexedResourceQualifier;{dae3bb9b-d304-497f-a168-a340042c8adb})");
}
unsafe impl ::windows::runtime::Interface for IndexedResourceQualifier {
    type Vtable = IIndexedResourceQualifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3672357787, 54020, 18815, [161, 104, 163, 64, 4, 44, 138, 219]);
}
impl ::windows::runtime::RuntimeName for IndexedResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IndexedResourceQualifier";
}
impl ::core::convert::From<IndexedResourceQualifier> for ::windows::runtime::IUnknown {
    fn from(value: IndexedResourceQualifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IndexedResourceQualifier> for ::windows::runtime::IUnknown {
    fn from(value: &IndexedResourceQualifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IndexedResourceQualifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IndexedResourceQualifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IndexedResourceQualifier> for ::windows::runtime::IInspectable {
    fn from(value: IndexedResourceQualifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IndexedResourceQualifier> for ::windows::runtime::IInspectable {
    fn from(value: &IndexedResourceQualifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IndexedResourceQualifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IndexedResourceQualifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for IndexedResourceQualifier {}
unsafe impl ::core::marker::Sync for IndexedResourceQualifier {}
#[doc = "*Required features: `ApplicationModel_Resources_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IndexedResourceType(pub i32);
impl IndexedResourceType {
    pub const String: IndexedResourceType = IndexedResourceType(0i32);
    pub const Path: IndexedResourceType = IndexedResourceType(1i32);
    pub const EmbeddedData: IndexedResourceType = IndexedResourceType(2i32);
}
impl ::core::convert::From<i32> for IndexedResourceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IndexedResourceType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IndexedResourceType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Management.IndexedResourceType;i4)");
}
impl ::windows::runtime::DefaultType for IndexedResourceType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Resources_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceIndexer(pub ::windows::runtime::IInspectable);
impl ResourceIndexer {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Resources_Management`, `Foundation`*"]
    pub fn IndexFilePath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, filepath: Param0) -> ::windows::runtime::Result<IndexedResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<IndexedResourceCandidate>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Resources_Management`, `Foundation`, `Foundation_Collections`*"]
    pub fn IndexFileContentsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, file: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Resources_Management`, `Foundation`*"]
    pub fn CreateResourceIndexer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(projectroot: Param0) -> ::windows::runtime::Result<ResourceIndexer> {
        Self::IResourceIndexerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), projectroot.into_param().abi(), &mut result__).from_abi::<ResourceIndexer>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Resources_Management`, `Foundation`*"]
    pub fn CreateResourceIndexerWithExtension<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(projectroot: Param0, extensiondllpath: Param1) -> ::windows::runtime::Result<ResourceIndexer> {
        Self::IResourceIndexerFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), projectroot.into_param().abi(), extensiondllpath.into_param().abi(), &mut result__).from_abi::<ResourceIndexer>(result__)
        })
    }
    pub fn IResourceIndexerFactory<R, F: FnOnce(&IResourceIndexerFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceIndexer, IResourceIndexerFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceIndexerFactory2<R, F: FnOnce(&IResourceIndexerFactory2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceIndexer, IResourceIndexerFactory2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceIndexer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Management.ResourceIndexer;{2d4cf9a5-e32f-4ab2-8748-96350a016da3})");
}
unsafe impl ::windows::runtime::Interface for ResourceIndexer {
    type Vtable = IResourceIndexer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(760019365, 58159, 19122, [135, 72, 150, 53, 10, 1, 109, 163]);
}
impl ::windows::runtime::RuntimeName for ResourceIndexer {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.ResourceIndexer";
}
impl ::core::convert::From<ResourceIndexer> for ::windows::runtime::IUnknown {
    fn from(value: ResourceIndexer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceIndexer> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceIndexer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceIndexer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceIndexer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceIndexer> for ::windows::runtime::IInspectable {
    fn from(value: ResourceIndexer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceIndexer> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceIndexer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceIndexer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceIndexer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceIndexer {}
unsafe impl ::core::marker::Sync for ResourceIndexer {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct ResourceIndexerContract(pub u8);
