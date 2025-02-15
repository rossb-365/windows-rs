#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICachedFileUpdaterStatics {
    type Vtable = ICachedFileUpdaterStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICachedFileUpdaterStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc90920_7bcf_4888_a81e_102d7034d7ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetUpdateInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICachedFileUpdaterUI {
    type Vtable = ICachedFileUpdaterUI_Vtbl;
}
unsafe impl ::windows::core::Interface for ICachedFileUpdaterUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6f41e6_baf2_4a97_b600_9333f5df80fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UpdateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CachedFileTarget) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FileUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UIRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UIRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUIRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUIRequested: usize,
    pub UIStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UIStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterUI2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICachedFileUpdaterUI2 {
    type Vtable = ICachedFileUpdaterUI2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICachedFileUpdaterUI2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8856a21c_8699_4340_9f49_f7cad7fe8991);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UpdateRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFileUpdateRequest {
    type Vtable = IFileUpdateRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IFileUpdateRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40c82536_c1fe_4d93_a792_1e736bc70837);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileUpdateStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FileUpdateStatus) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateLocalFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFileUpdateRequest2 {
    type Vtable = IFileUpdateRequest2_Vtbl;
}
unsafe impl ::windows::core::Interface for IFileUpdateRequest2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82484648_bdbe_447b_a2ee_7afe6a032a94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UserInputNeededMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetUserInputNeededMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFileUpdateRequestDeferral {
    type Vtable = IFileUpdateRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for IFileUpdateRequestDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffcedb2b_8ade_44a5_bb00_164c4e72f13a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFileUpdateRequestedEventArgs {
    type Vtable = IFileUpdateRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IFileUpdateRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b0a9342_3905_438d_aaef_78ae265f8dd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderError(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderError {
    type Vtable = IStorageProviderError_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47f2780b_ef7f_5910_bf83_331d89256615);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderError_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PrimaryAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrimaryAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SecondaryAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSecondaryAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InformationalLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetInformationalLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderErrorCommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderErrorCommand {
    type Vtable = IStorageProviderErrorCommand_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderErrorCommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b18aed_bb65_5f26_86e4_1d3e34d54477);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderErrorCommand_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActionUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActionUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderErrorCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderErrorCommandFactory {
    type Vtable = IStorageProviderErrorCommandFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderErrorCommandFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc1f555_3ab4_556f_8bb2_7e5515eed8dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderErrorCommandFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, actionuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderErrorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderErrorFactory {
    type Vtable = IStorageProviderErrorFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderErrorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97d6f240_61ab_51dc_9921_18bd0dbef79e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderErrorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderFileTypeInfo {
    type Vtable = IStorageProviderFileTypeInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderFileTypeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1955b9c1_0184_5a88_87df_4544f464365d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FileExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfoFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderFileTypeInfoFactory {
    type Vtable = IStorageProviderFileTypeInfoFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderFileTypeInfoFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fa12c6f_cce6_5d5d_80b1_389e7cf92dbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfoFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconresource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderGetContentInfoForPathResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderGetContentInfoForPathResult {
    type Vtable = IStorageProviderGetContentInfoForPathResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderGetContentInfoForPathResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2564711d_aa89_4d12_82e3_f72a92e33966);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetContentInfoForPathResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub ContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderGetPathForContentUriResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderGetPathForContentUriResult {
    type Vtable = IStorageProviderGetPathForContentUriResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderGetPathForContentUriResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63711a9d_4118_45a6_acb6_22c49d019f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetPathForContentUriResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderHandlerFactory(::windows::core::IUnknown);
impl IStorageProviderHandlerFactory {
    pub fn GetStatusSource(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<IStorageProviderStatusSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatusSource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(syncrootid), result__.as_mut_ptr()).from_abi::<IStorageProviderStatusSource>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IStorageProviderHandlerFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IStorageProviderHandlerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderHandlerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderHandlerFactory {}
impl ::core::fmt::Debug for IStorageProviderHandlerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderHandlerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderHandlerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6154dc3a-fc1d-5aae-9e23-e8659a22c5f6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IStorageProviderHandlerFactory {
    type Vtable = IStorageProviderHandlerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderHandlerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6154dc3a_fc1d_5aae_9e23_e8659a22c5f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderHandlerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStatusSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderItemPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderItemPropertiesStatics {
    type Vtable = IStorageProviderItemPropertiesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertiesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d2c1c97_2704_4729_8fa9_7e6b8e158c2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, itemproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderItemProperty(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderItemProperty {
    type Vtable = IStorageProviderItemProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderItemProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x476cb558_730b_4188_b7b5_63b716ed476d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemProperty_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderItemPropertyDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderItemPropertyDefinition {
    type Vtable = IStorageProviderItemPropertyDefinition_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertyDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5b383bb_ff1f_4298_831e_ff1c08089690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertyDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderItemPropertySource(::windows::core::IUnknown);
impl IStorageProviderItemPropertySource {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemProperties(&self, itempath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetItemProperties)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(itempath), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IStorageProviderItemPropertySource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IStorageProviderItemPropertySource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderItemPropertySource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderItemPropertySource {}
impl ::core::fmt::Debug for IStorageProviderItemPropertySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderItemPropertySource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderItemPropertySource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8f6f9c3e-f632-4a9b-8d99-d2d7a11df56a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IStorageProviderItemPropertySource {
    type Vtable = IStorageProviderItemPropertySource_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertySource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f6f9c3e_f632_4a9b_8d99_d2d7a11df56a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertySource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itempath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemProperties: usize,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderPropertyCapabilities(::windows::core::IUnknown);
impl IStorageProviderPropertyCapabilities {
    pub fn IsPropertySupported(&self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPropertySupported)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertycanonicalname), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IStorageProviderPropertyCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IStorageProviderPropertyCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderPropertyCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderPropertyCapabilities {}
impl ::core::fmt::Debug for IStorageProviderPropertyCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderPropertyCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderPropertyCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{658d2f0e-63b7-4567-acf9-51abe301dda5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IStorageProviderPropertyCapabilities {
    type Vtable = IStorageProviderPropertyCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderPropertyCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x658d2f0e_63b7_4567_acf9_51abe301dda5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderPropertyCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPropertySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertycanonicalname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderStatus {
    type Vtable = IStorageProviderStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff6e761d_fb8b_56c3_9e7a_05309d191fb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatus_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ErrorMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ErrorMessages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderStatusFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderStatusFactory {
    type Vtable = IStorageProviderStatusFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderStatusFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd64828c5_9b7a_5fa4_b126_90bd18936c7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: StorageProviderState, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: StorageProviderState, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, errormessages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance2: usize,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderStatusSource(::windows::core::IUnknown);
impl IStorageProviderStatusSource {
    pub fn GetStatus(&self) -> ::windows::core::Result<StorageProviderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed(&self, handler: &super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Changed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
::windows::core::interface_hierarchy!(IStorageProviderStatusSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IStorageProviderStatusSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderStatusSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderStatusSource {}
impl ::core::fmt::Debug for IStorageProviderStatusSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderStatusSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderStatusSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2e316bb2-fd43-5335-b3c4-a962ee31d17e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IStorageProviderStatusSource {
    type Vtable = IStorageProviderStatusSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderStatusSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e316bb2_fd43_5335_b3c4_a962ee31d17e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderSyncRootInfo {
    type Vtable = IStorageProviderSyncRootInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c1305c4_99f9_41ac_8904_ab055d654926);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Context: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContext: usize,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HydrationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHydrationPolicy) -> ::windows::core::HRESULT,
    pub SetHydrationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderHydrationPolicy) -> ::windows::core::HRESULT,
    pub HydrationPolicyModifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT,
    pub SetHydrationPolicyModifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT,
    pub PopulationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderPopulationPolicy) -> ::windows::core::HRESULT,
    pub SetPopulationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderPopulationPolicy) -> ::windows::core::HRESULT,
    pub InSyncPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderInSyncPolicy) -> ::windows::core::HRESULT,
    pub SetInSyncPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderInSyncPolicy) -> ::windows::core::HRESULT,
    pub HardlinkPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT,
    pub SetHardlinkPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT,
    pub ShowSiblingsAsGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowSiblingsAsGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProtectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderProtectionMode) -> ::windows::core::HRESULT,
    pub SetProtectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderProtectionMode) -> ::windows::core::HRESULT,
    pub AllowPinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowPinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageProviderItemPropertyDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageProviderItemPropertyDefinitions: usize,
    #[cfg(feature = "Foundation")]
    pub RecycleBinUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecycleBinUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetRecycleBinUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRecycleBinUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderSyncRootInfo2 {
    type Vtable = IStorageProviderSyncRootInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf51b023_7cf1_5166_bdba_efd95f529e31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderSyncRootInfo3 {
    type Vtable = IStorageProviderSyncRootInfo3_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507a6617_bef6_56fd_855e_75ace2e45cf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FallbackFileTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FallbackFileTypeInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderSyncRootManagerStatics {
    type Vtable = IStorageProviderSyncRootManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e99fbbf_8fe3_4b40_abc7_f6fc3d74c98e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootinformation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetSyncRootInformationForFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSyncRootInformationForId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentSyncRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentSyncRoots: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageProviderSyncRootManagerStatics2 {
    type Vtable = IStorageProviderSyncRootManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefb6cfee_1374_544e_9df1_5598d2e9cfdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderUriSource(::windows::core::IUnknown);
impl IStorageProviderUriSource {
    pub fn GetPathForContentUri(&self, contenturi: &::windows::core::HSTRING, result: &StorageProviderGetPathForContentUriResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetPathForContentUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contenturi), ::core::mem::transmute_copy(result)).ok() }
    }
    pub fn GetContentInfoForPath(&self, path: &::windows::core::HSTRING, result: &StorageProviderGetContentInfoForPathResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetContentInfoForPath)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(path), ::core::mem::transmute_copy(result)).ok() }
    }
}
::windows::core::interface_hierarchy!(IStorageProviderUriSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IStorageProviderUriSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderUriSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderUriSource {}
impl ::core::fmt::Debug for IStorageProviderUriSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderUriSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderUriSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b29806d1-8be0-4962-8bb6-0d4c2e14d47a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IStorageProviderUriSource {
    type Vtable = IStorageProviderUriSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageProviderUriSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb29806d1_8be0_4962_8bb6_0d4c2e14d47a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderUriSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetPathForContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenturi: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContentInfoForPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
pub struct CachedFileUpdater;
impl CachedFileUpdater {
    pub fn SetUpdateInformation<'a, P0, E0>(file: P0, contentid: &::windows::core::HSTRING, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICachedFileUpdaterStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetUpdateInformation)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(contentid), readmode, writemode, options).ok() })
    }
    #[doc(hidden)]
    pub fn ICachedFileUpdaterStatics<R, F: FnOnce(&ICachedFileUpdaterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CachedFileUpdater, ICachedFileUpdaterStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CachedFileUpdater {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdater";
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct CachedFileUpdaterUI(::windows::core::IUnknown);
impl CachedFileUpdaterUI {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn UpdateTarget(&self) -> ::windows::core::Result<CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateTarget)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<CachedFileTarget>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FileUpdateRequested(&self, handler: &super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FileUpdateRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileUpdateRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFileUpdateRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UIRequested(&self, handler: &super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UIRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUIRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUIRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn UIStatus(&self) -> ::windows::core::Result<UIStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UIStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<UIStatus>(result__)
        }
    }
    pub fn UpdateRequest(&self) -> ::windows::core::Result<FileUpdateRequest> {
        let this = &::windows::core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FileUpdateRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral> {
        let this = &::windows::core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FileUpdateRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CachedFileUpdaterUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterUI {}
impl ::core::fmt::Debug for CachedFileUpdaterUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileUpdaterUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CachedFileUpdaterUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.CachedFileUpdaterUI;{9e6f41e6-baf2-4a97-b600-9333f5df80fd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CachedFileUpdaterUI {
    type Vtable = ICachedFileUpdaterUI_Vtbl;
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterUI {
    const IID: ::windows::core::GUID = <ICachedFileUpdaterUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CachedFileUpdaterUI {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdaterUI";
}
::windows::core::interface_hierarchy!(CachedFileUpdaterUI, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateRequest(::windows::core::IUnknown);
impl FileUpdateRequest {
    pub fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn File(&self) -> ::windows::core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).File)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageFile>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<FileUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FileUpdateStatus>(result__)
        }
    }
    pub fn SetStatus(&self, value: FileUpdateStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStatus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FileUpdateRequestDeferral>(result__)
        }
    }
    pub fn UpdateLocalFile<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateLocalFile)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn UserInputNeededMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserInputNeededMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetUserInputNeededMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUserInputNeededMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for FileUpdateRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileUpdateRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUpdateRequest {}
impl ::core::fmt::Debug for FileUpdateRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequest;{40c82536-c1fe-4d93-a792-1e736bc70837})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FileUpdateRequest {
    type Vtable = IFileUpdateRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for FileUpdateRequest {
    const IID: ::windows::core::GUID = <IFileUpdateRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileUpdateRequest {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequest";
}
::windows::core::interface_hierarchy!(FileUpdateRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateRequestDeferral(::windows::core::IUnknown);
impl FileUpdateRequestDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for FileUpdateRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileUpdateRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUpdateRequestDeferral {}
impl ::core::fmt::Debug for FileUpdateRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequestDeferral;{ffcedb2b-8ade-44a5-bb00-164c4e72f13a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FileUpdateRequestDeferral {
    type Vtable = IFileUpdateRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for FileUpdateRequestDeferral {
    const IID: ::windows::core::GUID = <IFileUpdateRequestDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileUpdateRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestDeferral";
}
::windows::core::interface_hierarchy!(FileUpdateRequestDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateRequestedEventArgs(::windows::core::IUnknown);
impl FileUpdateRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<FileUpdateRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for FileUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileUpdateRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUpdateRequestedEventArgs {}
impl ::core::fmt::Debug for FileUpdateRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequestedEventArgs;{7b0a9342-3905-438d-aaef-78ae265f8dd2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FileUpdateRequestedEventArgs {
    type Vtable = IFileUpdateRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for FileUpdateRequestedEventArgs {
    const IID: ::windows::core::GUID = <IFileUpdateRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestedEventArgs";
}
::windows::core::interface_hierarchy!(FileUpdateRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderError(::windows::core::IUnknown);
impl StorageProviderError {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FilePath)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetFilePath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFilePath)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PrimaryAction(&self) -> ::windows::core::Result<StorageProviderErrorCommand> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrimaryAction)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderErrorCommand>(result__)
        }
    }
    pub fn SetPrimaryAction(&self, value: &StorageProviderErrorCommand) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrimaryAction)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SecondaryAction(&self) -> ::windows::core::Result<StorageProviderErrorCommand> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SecondaryAction)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderErrorCommand>(result__)
        }
    }
    pub fn SetSecondaryAction(&self, value: &StorageProviderErrorCommand) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSecondaryAction)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn InformationalLink(&self) -> ::windows::core::Result<StorageProviderErrorCommand> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InformationalLink)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderErrorCommand>(result__)
        }
    }
    pub fn SetInformationalLink(&self, value: &StorageProviderErrorCommand) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInformationalLink)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateInstance(id: &::windows::core::HSTRING, title: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderError> {
        Self::IStorageProviderErrorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(message), result__.as_mut_ptr()).from_abi::<StorageProviderError>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderErrorFactory<R, F: FnOnce(&IStorageProviderErrorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderError, IStorageProviderErrorFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for StorageProviderError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderError {}
impl ::core::fmt::Debug for StorageProviderError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderError;{47f2780b-ef7f-5910-bf83-331d89256615})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageProviderError {
    type Vtable = IStorageProviderError_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageProviderError {
    const IID: ::windows::core::GUID = <IStorageProviderError as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderError {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderError";
}
::windows::core::interface_hierarchy!(StorageProviderError, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderError {}
unsafe impl ::core::marker::Sync for StorageProviderError {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderErrorCommand(::windows::core::IUnknown);
impl StorageProviderErrorCommand {
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Label)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActionUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActionUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstance(label: &::windows::core::HSTRING, actionuri: &super::super::Foundation::Uri) -> ::windows::core::Result<StorageProviderErrorCommand> {
        Self::IStorageProviderErrorCommandFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(label), ::core::mem::transmute_copy(actionuri), result__.as_mut_ptr()).from_abi::<StorageProviderErrorCommand>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderErrorCommandFactory<R, F: FnOnce(&IStorageProviderErrorCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderErrorCommand, IStorageProviderErrorCommandFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for StorageProviderErrorCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderErrorCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderErrorCommand {}
impl ::core::fmt::Debug for StorageProviderErrorCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderErrorCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderErrorCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderErrorCommand;{b6b18aed-bb65-5f26-86e4-1d3e34d54477})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageProviderErrorCommand {
    type Vtable = IStorageProviderErrorCommand_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageProviderErrorCommand {
    const IID: ::windows::core::GUID = <IStorageProviderErrorCommand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderErrorCommand {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderErrorCommand";
}
::windows::core::interface_hierarchy!(StorageProviderErrorCommand, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderErrorCommand {}
unsafe impl ::core::marker::Sync for StorageProviderErrorCommand {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderFileTypeInfo(::windows::core::IUnknown);
impl StorageProviderFileTypeInfo {
    pub fn FileExtension(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FileExtension)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IconResource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CreateInstance(fileextension: &::windows::core::HSTRING, iconresource: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderFileTypeInfo> {
        Self::IStorageProviderFileTypeInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(fileextension), ::core::mem::transmute_copy(iconresource), result__.as_mut_ptr()).from_abi::<StorageProviderFileTypeInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderFileTypeInfoFactory<R, F: FnOnce(&IStorageProviderFileTypeInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderFileTypeInfo, IStorageProviderFileTypeInfoFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for StorageProviderFileTypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderFileTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderFileTypeInfo {}
impl ::core::fmt::Debug for StorageProviderFileTypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderFileTypeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderFileTypeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderFileTypeInfo;{1955b9c1-0184-5a88-87df-4544f464365d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageProviderFileTypeInfo {
    type Vtable = IStorageProviderFileTypeInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageProviderFileTypeInfo {
    const IID: ::windows::core::GUID = <IStorageProviderFileTypeInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderFileTypeInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderFileTypeInfo";
}
::windows::core::interface_hierarchy!(StorageProviderFileTypeInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderFileTypeInfo {}
unsafe impl ::core::marker::Sync for StorageProviderFileTypeInfo {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderGetContentInfoForPathResult(::windows::core::IUnknown);
impl StorageProviderGetContentInfoForPathResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderGetContentInfoForPathResult, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderUriSourceStatus>(result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStatus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ContentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContentUri(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContentId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for StorageProviderGetContentInfoForPathResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderGetContentInfoForPathResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderGetContentInfoForPathResult {}
impl ::core::fmt::Debug for StorageProviderGetContentInfoForPathResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderGetContentInfoForPathResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderGetContentInfoForPathResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult;{2564711d-aa89-4d12-82e3-f72a92e33966})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageProviderGetContentInfoForPathResult {
    type Vtable = IStorageProviderGetContentInfoForPathResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageProviderGetContentInfoForPathResult {
    const IID: ::windows::core::GUID = <IStorageProviderGetContentInfoForPathResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderGetContentInfoForPathResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult";
}
::windows::core::interface_hierarchy!(StorageProviderGetContentInfoForPathResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderGetContentInfoForPathResult {}
unsafe impl ::core::marker::Sync for StorageProviderGetContentInfoForPathResult {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderGetPathForContentUriResult(::windows::core::IUnknown);
impl StorageProviderGetPathForContentUriResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderGetPathForContentUriResult, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderUriSourceStatus>(result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStatus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Path)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPath)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for StorageProviderGetPathForContentUriResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderGetPathForContentUriResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderGetPathForContentUriResult {}
impl ::core::fmt::Debug for StorageProviderGetPathForContentUriResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderGetPathForContentUriResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderGetPathForContentUriResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderGetPathForContentUriResult;{63711a9d-4118-45a6-acb6-22c49d019f40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageProviderGetPathForContentUriResult {
    type Vtable = IStorageProviderGetPathForContentUriResult_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageProviderGetPathForContentUriResult {
    const IID: ::windows::core::GUID = <IStorageProviderGetPathForContentUriResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderGetPathForContentUriResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetPathForContentUriResult";
}
::windows::core::interface_hierarchy!(StorageProviderGetPathForContentUriResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderGetPathForContentUriResult {}
unsafe impl ::core::marker::Sync for StorageProviderGetPathForContentUriResult {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
pub struct StorageProviderItemProperties;
impl StorageProviderItemProperties {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAsync<'a, P0, E0, P1, E1>(item: P0, itemproperties: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageItem>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IStorageProviderItemPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAsync)(::windows::core::Vtable::as_raw(this), item.try_into().map_err(|e| e.into())?.abi(), itemproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderItemPropertiesStatics<R, F: FnOnce(&IStorageProviderItemPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderItemProperties, IStorageProviderItemPropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StorageProviderItemProperties {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperties";
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderItemProperty(::windows::core::IUnknown);
impl StorageProviderItemProperty {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderItemProperty, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetIconResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIconResource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IconResource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageProviderItemProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderItemProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderItemProperty {}
impl ::core::fmt::Debug for StorageProviderItemProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderItemProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderItemProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderItemProperty;{476cb558-730b-4188-b7b5-63b716ed476d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageProviderItemProperty {
    type Vtable = IStorageProviderItemProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageProviderItemProperty {
    const IID: ::windows::core::GUID = <IStorageProviderItemProperty as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderItemProperty {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperty";
}
::windows::core::interface_hierarchy!(StorageProviderItemProperty, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderItemProperty {}
unsafe impl ::core::marker::Sync for StorageProviderItemProperty {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderItemPropertyDefinition(::windows::core::IUnknown);
impl StorageProviderItemPropertyDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderItemPropertyDefinition, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayNameResource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayNameResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayNameResource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for StorageProviderItemPropertyDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderItemPropertyDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderItemPropertyDefinition {}
impl ::core::fmt::Debug for StorageProviderItemPropertyDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderItemPropertyDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderItemPropertyDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderItemPropertyDefinition;{c5b383bb-ff1f-4298-831e-ff1c08089690})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageProviderItemPropertyDefinition {
    type Vtable = IStorageProviderItemPropertyDefinition_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageProviderItemPropertyDefinition {
    const IID: ::windows::core::GUID = <IStorageProviderItemPropertyDefinition as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderItemPropertyDefinition {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemPropertyDefinition";
}
::windows::core::interface_hierarchy!(StorageProviderItemPropertyDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderItemPropertyDefinition {}
unsafe impl ::core::marker::Sync for StorageProviderItemPropertyDefinition {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderStatus(::windows::core::IUnknown);
impl StorageProviderStatus {
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<StorageProviderState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderState>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ErrorMessages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderError>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorMessages)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<StorageProviderError>>(result__)
        }
    }
    pub fn CreateInstance(state: StorageProviderState, message: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderStatus> {
        Self::IStorageProviderStatusFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), state, ::core::mem::transmute_copy(message), result__.as_mut_ptr()).from_abi::<StorageProviderStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance2<'a, P0, E0>(state: StorageProviderState, message: &::windows::core::HSTRING, errormessages: P0) -> ::windows::core::Result<StorageProviderStatus>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<StorageProviderError>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IStorageProviderStatusFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance2)(::windows::core::Vtable::as_raw(this), state, ::core::mem::transmute_copy(message), errormessages.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<StorageProviderStatus>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderStatusFactory<R, F: FnOnce(&IStorageProviderStatusFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderStatus, IStorageProviderStatusFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for StorageProviderStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderStatus {}
impl ::core::fmt::Debug for StorageProviderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderStatus;{ff6e761d-fb8b-56c3-9e7a-05309d191fb4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageProviderStatus {
    type Vtable = IStorageProviderStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageProviderStatus {
    const IID: ::windows::core::GUID = <IStorageProviderStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderStatus {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderStatus";
}
::windows::core::interface_hierarchy!(StorageProviderStatus, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderStatus {}
unsafe impl ::core::marker::Sync for StorageProviderStatus {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderSyncRootInfo(::windows::core::IUnknown);
impl StorageProviderSyncRootInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderSyncRootInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Context(&self) -> ::windows::core::Result<super::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Context)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetContext<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContext)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Path(&self) -> ::windows::core::Result<super::IStorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Path)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::IStorageFolder>(result__)
        }
    }
    pub fn SetPath<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPath)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayNameResource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayNameResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayNameResource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IconResource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetIconResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIconResource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HydrationPolicy(&self) -> ::windows::core::Result<StorageProviderHydrationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HydrationPolicy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderHydrationPolicy>(result__)
        }
    }
    pub fn SetHydrationPolicy(&self, value: StorageProviderHydrationPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHydrationPolicy)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn HydrationPolicyModifier(&self) -> ::windows::core::Result<StorageProviderHydrationPolicyModifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HydrationPolicyModifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderHydrationPolicyModifier>(result__)
        }
    }
    pub fn SetHydrationPolicyModifier(&self, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHydrationPolicyModifier)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PopulationPolicy(&self) -> ::windows::core::Result<StorageProviderPopulationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PopulationPolicy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderPopulationPolicy>(result__)
        }
    }
    pub fn SetPopulationPolicy(&self, value: StorageProviderPopulationPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPopulationPolicy)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InSyncPolicy(&self) -> ::windows::core::Result<StorageProviderInSyncPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InSyncPolicy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderInSyncPolicy>(result__)
        }
    }
    pub fn SetInSyncPolicy(&self, value: StorageProviderInSyncPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInSyncPolicy)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn HardlinkPolicy(&self) -> ::windows::core::Result<StorageProviderHardlinkPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HardlinkPolicy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderHardlinkPolicy>(result__)
        }
    }
    pub fn SetHardlinkPolicy(&self, value: StorageProviderHardlinkPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHardlinkPolicy)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ShowSiblingsAsGroup(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowSiblingsAsGroup)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShowSiblingsAsGroup(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetShowSiblingsAsGroup)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Version)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVersion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProtectionMode(&self) -> ::windows::core::Result<StorageProviderProtectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageProviderProtectionMode>(result__)
        }
    }
    pub fn SetProtectionMode(&self, value: StorageProviderProtectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetProtectionMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AllowPinning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowPinning)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowPinning(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowPinning)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorageProviderItemPropertyDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StorageProviderItemPropertyDefinitions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecycleBinUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RecycleBinUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRecycleBinUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRecycleBinUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SetProviderId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetProviderId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FallbackFileTypeInfo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>> {
        let this = &::windows::core::Interface::cast::<IStorageProviderSyncRootInfo3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FallbackFileTypeInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageProviderSyncRootInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderSyncRootInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderSyncRootInfo {}
impl ::core::fmt::Debug for StorageProviderSyncRootInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderSyncRootInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderSyncRootInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderSyncRootInfo;{7c1305c4-99f9-41ac-8904-ab055d654926})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageProviderSyncRootInfo {
    type Vtable = IStorageProviderSyncRootInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageProviderSyncRootInfo {
    const IID: ::windows::core::GUID = <IStorageProviderSyncRootInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderSyncRootInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootInfo";
}
::windows::core::interface_hierarchy!(StorageProviderSyncRootInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorageProviderSyncRootInfo {}
unsafe impl ::core::marker::Sync for StorageProviderSyncRootInfo {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
pub struct StorageProviderSyncRootManager;
impl StorageProviderSyncRootManager {
    pub fn Register(syncrootinformation: &StorageProviderSyncRootInfo) -> ::windows::core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).Register)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(syncrootinformation)).ok() })
    }
    pub fn Unregister(id: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).Unregister)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id)).ok() })
    }
    pub fn GetSyncRootInformationForFolder<'a, P0, E0>(folder: P0) -> ::windows::core::Result<StorageProviderSyncRootInfo>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSyncRootInformationForFolder)(::windows::core::Vtable::as_raw(this), folder.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<StorageProviderSyncRootInfo>(result__)
        })
    }
    pub fn GetSyncRootInformationForId(id: &::windows::core::HSTRING) -> ::windows::core::Result<StorageProviderSyncRootInfo> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSyncRootInformationForId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<StorageProviderSyncRootInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentSyncRoots() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentSyncRoots)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IStorageProviderSyncRootManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderSyncRootManagerStatics<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStorageProviderSyncRootManagerStatics2<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StorageProviderSyncRootManager {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootManager";
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CachedFileOptions(pub u32);
impl CachedFileOptions {
    pub const None: Self = Self(0u32);
    pub const RequireUpdateOnAccess: Self = Self(1u32);
    pub const UseCachedFileWhenOffline: Self = Self(2u32);
    pub const DenyAccessWhenOffline: Self = Self(4u32);
}
impl ::core::marker::Copy for CachedFileOptions {}
impl ::core::clone::Clone for CachedFileOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CachedFileOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CachedFileOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for CachedFileOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CachedFileOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CachedFileOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CachedFileOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CachedFileOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CachedFileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CachedFileOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CachedFileTarget(pub i32);
impl CachedFileTarget {
    pub const Local: Self = Self(0i32);
    pub const Remote: Self = Self(1i32);
}
impl ::core::marker::Copy for CachedFileTarget {}
impl ::core::clone::Clone for CachedFileTarget {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CachedFileTarget {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CachedFileTarget {
    type Abi = Self;
}
impl ::core::fmt::Debug for CachedFileTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CachedFileTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileTarget;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FileUpdateStatus(pub i32);
impl FileUpdateStatus {
    pub const Incomplete: Self = Self(0i32);
    pub const Complete: Self = Self(1i32);
    pub const UserInputNeeded: Self = Self(2i32);
    pub const CurrentlyUnavailable: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
    pub const CompleteAndRenamed: Self = Self(5i32);
}
impl ::core::marker::Copy for FileUpdateStatus {}
impl ::core::clone::Clone for FileUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileUpdateStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FileUpdateStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for FileUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.FileUpdateStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ReadActivationMode(pub i32);
impl ReadActivationMode {
    pub const NotNeeded: Self = Self(0i32);
    pub const BeforeAccess: Self = Self(1i32);
}
impl ::core::marker::Copy for ReadActivationMode {}
impl ::core::clone::Clone for ReadActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ReadActivationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ReadActivationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ReadActivationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReadActivationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReadActivationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.ReadActivationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderHardlinkPolicy(pub u32);
impl StorageProviderHardlinkPolicy {
    pub const None: Self = Self(0u32);
    pub const Allowed: Self = Self(1u32);
}
impl ::core::marker::Copy for StorageProviderHardlinkPolicy {}
impl ::core::clone::Clone for StorageProviderHardlinkPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderHardlinkPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderHardlinkPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderHardlinkPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderHardlinkPolicy").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderHardlinkPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderHardlinkPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderHardlinkPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHardlinkPolicy;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderHydrationPolicy(pub i32);
impl StorageProviderHydrationPolicy {
    pub const Partial: Self = Self(0i32);
    pub const Progressive: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const AlwaysFull: Self = Self(3i32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicy {}
impl ::core::clone::Clone for StorageProviderHydrationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderHydrationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderHydrationPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderHydrationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderHydrationPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderHydrationPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderHydrationPolicyModifier(pub u32);
impl StorageProviderHydrationPolicyModifier {
    pub const None: Self = Self(0u32);
    pub const ValidationRequired: Self = Self(1u32);
    pub const StreamingAllowed: Self = Self(2u32);
    pub const AutoDehydrationAllowed: Self = Self(4u32);
    pub const AllowFullRestartHydration: Self = Self(8u32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicyModifier {}
impl ::core::clone::Clone for StorageProviderHydrationPolicyModifier {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderHydrationPolicyModifier {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderHydrationPolicyModifier {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderHydrationPolicyModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderHydrationPolicyModifier").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderHydrationPolicyModifier {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderHydrationPolicyModifier {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderHydrationPolicyModifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicyModifier;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderInSyncPolicy(pub u32);
impl StorageProviderInSyncPolicy {
    pub const Default: Self = Self(0u32);
    pub const FileCreationTime: Self = Self(1u32);
    pub const FileReadOnlyAttribute: Self = Self(2u32);
    pub const FileHiddenAttribute: Self = Self(4u32);
    pub const FileSystemAttribute: Self = Self(8u32);
    pub const DirectoryCreationTime: Self = Self(16u32);
    pub const DirectoryReadOnlyAttribute: Self = Self(32u32);
    pub const DirectoryHiddenAttribute: Self = Self(64u32);
    pub const DirectorySystemAttribute: Self = Self(128u32);
    pub const FileLastWriteTime: Self = Self(256u32);
    pub const DirectoryLastWriteTime: Self = Self(512u32);
    pub const PreserveInsyncForSyncEngine: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for StorageProviderInSyncPolicy {}
impl ::core::clone::Clone for StorageProviderInSyncPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderInSyncPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderInSyncPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderInSyncPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderInSyncPolicy").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderInSyncPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderInSyncPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageProviderInSyncPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderInSyncPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderInSyncPolicy;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderPopulationPolicy(pub i32);
impl StorageProviderPopulationPolicy {
    pub const Full: Self = Self(1i32);
    pub const AlwaysFull: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderPopulationPolicy {}
impl ::core::clone::Clone for StorageProviderPopulationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderPopulationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderPopulationPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderPopulationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderPopulationPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderPopulationPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderPopulationPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderProtectionMode(pub i32);
impl StorageProviderProtectionMode {
    pub const Unknown: Self = Self(0i32);
    pub const Personal: Self = Self(1i32);
}
impl ::core::marker::Copy for StorageProviderProtectionMode {}
impl ::core::clone::Clone for StorageProviderProtectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderProtectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderProtectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderProtectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderProtectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderProtectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderProtectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderState(pub i32);
impl StorageProviderState {
    pub const InSync: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Warning: Self = Self(4i32);
    pub const Offline: Self = Self(5i32);
}
impl ::core::marker::Copy for StorageProviderState {}
impl ::core::clone::Clone for StorageProviderState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorageProviderUriSourceStatus(pub i32);
impl StorageProviderUriSourceStatus {
    pub const Success: Self = Self(0i32);
    pub const NoSyncRoot: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderUriSourceStatus {}
impl ::core::clone::Clone for StorageProviderUriSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderUriSourceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderUriSourceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderUriSourceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderUriSourceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderUriSourceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderUriSourceStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIStatus(pub i32);
impl UIStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
    pub const Complete: Self = Self(3i32);
}
impl ::core::marker::Copy for UIStatus {}
impl ::core::clone::Clone for UIStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UIStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UIStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UIStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.UIStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WriteActivationMode(pub i32);
impl WriteActivationMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const NotNeeded: Self = Self(1i32);
    pub const AfterWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for WriteActivationMode {}
impl ::core::clone::Clone for WriteActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WriteActivationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WriteActivationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for WriteActivationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WriteActivationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WriteActivationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.WriteActivationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
