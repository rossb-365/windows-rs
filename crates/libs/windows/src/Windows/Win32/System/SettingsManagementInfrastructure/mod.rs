#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct IItemEnumerator(::windows::core::IUnknown);
impl IItemEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Current(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Current)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IItemEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IItemEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IItemEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IItemEnumerator {}
impl ::core::fmt::Debug for IItemEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IItemEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IItemEnumerator {
    type Vtable = IItemEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IItemEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb7_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Current: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsContext(::windows::core::IUnknown);
impl ISettingsContext {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<'a, P0, P1>(&self, pstream: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITargetInfo>>,
    {
        (::windows::core::Vtable::vtable(self).Serialize)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), ptarget.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deserialize<'a, P0, P1>(&self, pstream: P0, ptarget: P1, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITargetInfo>>,
    {
        (::windows::core::Vtable::vtable(self).Deserialize)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), ptarget.into().abi(), ::core::mem::transmute(pppresults), ::core::mem::transmute(pcresultcount)).ok()
    }
    pub unsafe fn SetUserData(&self, puserdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUserData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(puserdata)).ok()
    }
    pub unsafe fn GetUserData(&self) -> ::windows::core::Result<*mut ::core::ffi::c_void> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUserData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::core::ffi::c_void>(result__)
    }
    pub unsafe fn GetNamespaces(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNamespaces)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    pub unsafe fn GetStoredSettings<'a, P0>(&self, pidentity: P0, ppaddedsettings: *mut ::core::option::Option<IItemEnumerator>, ppmodifiedsettings: *mut ::core::option::Option<IItemEnumerator>, ppdeletedsettings: *mut ::core::option::Option<IItemEnumerator>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISettingsIdentity>>,
    {
        (::windows::core::Vtable::vtable(self).GetStoredSettings)(::windows::core::Vtable::as_raw(self), pidentity.into().abi(), ::core::mem::transmute(ppaddedsettings), ::core::mem::transmute(ppmodifiedsettings), ::core::mem::transmute(ppdeletedsettings)).ok()
    }
    pub unsafe fn RevertSetting<'a, P0, P1>(&self, pidentity: P0, pwzsetting: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISettingsIdentity>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).RevertSetting)(::windows::core::Vtable::as_raw(self), pidentity.into().abi(), pwzsetting.into()).ok()
    }
}
::windows::core::interface_hierarchy!(ISettingsContext, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISettingsContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsContext {}
impl ::core::fmt::Debug for ISettingsContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISettingsContext {
    type Vtable = ISettingsContext_Vtbl;
}
unsafe impl ::windows::core::Interface for ISettingsContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bbd_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, pppresults: *mut *mut *mut ::core::ffi::c_void, pcresultcount: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    pub SetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puserdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnamespaceids: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStoredSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: *mut ::core::ffi::c_void, ppaddedsettings: *mut *mut ::core::ffi::c_void, ppmodifiedsettings: *mut *mut ::core::ffi::c_void, ppdeletedsettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevertSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: *mut ::core::ffi::c_void, pwzsetting: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsEngine(::windows::core::IUnknown);
impl ISettingsEngine {
    pub unsafe fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNamespaces)(::windows::core::Vtable::as_raw(self), flags, ::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    pub unsafe fn GetNamespace<'a, P0>(&self, settingsid: P0, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsNamespace>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISettingsIdentity>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNamespace)(::windows::core::Vtable::as_raw(self), settingsid.into().abi(), access, ::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsNamespace>(result__)
    }
    pub unsafe fn GetErrorDescription(&self, hresult: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetErrorDescription)(::windows::core::Vtable::as_raw(self), hresult, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn CreateSettingsIdentity(&self) -> ::windows::core::Result<ISettingsIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSettingsIdentity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsIdentity>(result__)
    }
    pub unsafe fn GetStoreStatus(&self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<WcmUserStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStoreStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmUserStatus>(result__)
    }
    pub unsafe fn LoadStore(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).LoadStore)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
    pub unsafe fn UnloadStore(&self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnloadStore)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterNamespace<'a, P0, P1, P2>(&self, settingsid: P0, stream: P1, pushsettings: P2) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISettingsIdentity>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IStream>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterNamespace)(::windows::core::Vtable::as_raw(self), settingsid.into().abi(), stream.into().abi(), pushsettings.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterNamespace<'a, P0, P1>(&self, settingsid: P0, removesettings: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISettingsIdentity>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).UnregisterNamespace)(::windows::core::Vtable::as_raw(self), settingsid.into().abi(), removesettings.into()).ok()
    }
    pub unsafe fn CreateTargetInfo(&self) -> ::windows::core::Result<ITargetInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTargetInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITargetInfo>(result__)
    }
    pub unsafe fn GetTargetInfo(&self) -> ::windows::core::Result<ITargetInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTargetInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITargetInfo>(result__)
    }
    pub unsafe fn SetTargetInfo<'a, P0>(&self, target: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITargetInfo>>,
    {
        (::windows::core::Vtable::vtable(self).SetTargetInfo)(::windows::core::Vtable::as_raw(self), target.into().abi()).ok()
    }
    pub unsafe fn CreateSettingsContext(&self, flags: u32, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSettingsContext)(::windows::core::Vtable::as_raw(self), flags, ::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsContext>(result__)
    }
    pub unsafe fn SetSettingsContext<'a, P0>(&self, settingscontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISettingsContext>>,
    {
        (::windows::core::Vtable::vtable(self).SetSettingsContext)(::windows::core::Vtable::as_raw(self), settingscontext.into().abi()).ok()
    }
    pub unsafe fn ApplySettingsContext<'a, P0>(&self, settingscontext: P0, pppwzidentities: *mut *mut ::windows::core::PWSTR, pcidentities: *mut usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISettingsContext>>,
    {
        (::windows::core::Vtable::vtable(self).ApplySettingsContext)(::windows::core::Vtable::as_raw(self), settingscontext.into().abi(), ::core::mem::transmute(pppwzidentities), ::core::mem::transmute(pcidentities)).ok()
    }
    pub unsafe fn GetSettingsContext(&self) -> ::windows::core::Result<ISettingsContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSettingsContext)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsContext>(result__)
    }
}
::windows::core::interface_hierarchy!(ISettingsEngine, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISettingsEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsEngine {}
impl ::core::fmt::Debug for ISettingsEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsEngine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISettingsEngine {
    type Vtable = ISettingsEngine_Vtbl;
}
unsafe impl ::windows::core::Interface for ISettingsEngine {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb9_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsEngine_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, message: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub CreateSettingsIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStoreStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows::core::HRESULT,
    pub LoadStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub UnloadStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, results: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterNamespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, removesettings: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterNamespace: usize,
    pub CreateTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ApplySettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: *mut ::core::ffi::c_void, pppwzidentities: *mut *mut ::windows::core::PWSTR, pcidentities: *mut usize) -> ::windows::core::HRESULT,
    pub GetSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsIdentity(::windows::core::IUnknown);
impl ISettingsIdentity {
    pub unsafe fn GetAttribute<'a, P0>(&self, reserved: *const ::core::ffi::c_void, name: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAttribute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved), name.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetAttribute<'a, P0, P1>(&self, reserved: *const ::core::ffi::c_void, name: P0, value: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetAttribute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(reserved), name.into(), value.into()).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFlags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFlags)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
}
::windows::core::interface_hierarchy!(ISettingsIdentity, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISettingsIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsIdentity {}
impl ::core::fmt::Debug for ISettingsIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsIdentity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISettingsIdentity {
    type Vtable = ISettingsIdentity_Vtbl;
}
unsafe impl ::windows::core::Interface for ISettingsIdentity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb6_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsIdentity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsItem(::windows::core::IUnknown);
impl ISettingsItem {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, value: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn GetSettingType(&self) -> ::windows::core::Result<WcmSettingType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSettingType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmSettingType>(result__)
    }
    pub unsafe fn GetDataType(&self) -> ::windows::core::Result<WcmDataType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDataType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmDataType>(result__)
    }
    pub unsafe fn GetValueRaw(&self, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetValueRaw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(data), ::core::mem::transmute(datasize)).ok()
    }
    pub unsafe fn SetValueRaw(&self, datatype: i32, data: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValueRaw)(::windows::core::Vtable::as_raw(self), datatype, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasChild(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HasChild)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Children(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Children)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    pub unsafe fn GetChild<'a, P0>(&self, name: P0) -> ::windows::core::Result<ISettingsItem>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetChild)(::windows::core::Vtable::as_raw(self), name.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn GetSettingByPath<'a, P0>(&self, path: P0) -> ::windows::core::Result<ISettingsItem>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSettingByPath)(::windows::core::Vtable::as_raw(self), path.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn CreateSettingByPath<'a, P0>(&self, path: P0) -> ::windows::core::Result<ISettingsItem>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSettingByPath)(::windows::core::Vtable::as_raw(self), path.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn RemoveSettingByPath<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).RemoveSettingByPath)(::windows::core::Vtable::as_raw(self), path.into()).ok()
    }
    pub unsafe fn GetListKeyInformation(&self, keyname: *mut ::windows::core::BSTR, datatype: *mut WcmDataType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetListKeyInformation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(keyname), ::core::mem::transmute(datatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateListElement(&self, keydata: *const super::Com::VARIANT) -> ::windows::core::Result<ISettingsItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateListElement)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(keydata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn RemoveListElement<'a, P0>(&self, elementname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).RemoveListElement)(::windows::core::Vtable::as_raw(self), elementname.into()).ok()
    }
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Attributes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttribute<'a, P0>(&self, name: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAttribute)(::windows::core::Vtable::as_raw(self), name.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetRestrictionFacets(&self) -> ::windows::core::Result<WcmRestrictionFacets> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRestrictionFacets)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmRestrictionFacets>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRestriction)(::windows::core::Vtable::as_raw(self), restrictionfacet, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetKeyValue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetKeyValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
::windows::core::interface_hierarchy!(ISettingsItem, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISettingsItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsItem {}
impl ::core::fmt::Debug for ISettingsItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISettingsItem {
    type Vtable = ISettingsItem_Vtbl;
}
unsafe impl ::windows::core::Interface for ISettingsItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bbb_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsItem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub GetSettingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut WcmSettingType) -> ::windows::core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut WcmDataType) -> ::windows::core::HRESULT,
    pub GetValueRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::HRESULT,
    pub SetValueRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChild: usize,
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, child: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetListKeyInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>, datatype: *mut WcmDataType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateListElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keydata: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, child: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateListElement: usize,
    pub RemoveListElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elementname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetRestrictionFacets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRestriction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRestriction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetKeyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetKeyValue: usize,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsNamespace(::windows::core::IUnknown);
impl ISettingsNamespace {
    pub unsafe fn GetIdentity(&self) -> ::windows::core::Result<ISettingsIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIdentity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsIdentity>(result__)
    }
    pub unsafe fn Settings(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Settings)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, P0>(&self, pushsettings: P0) -> ::windows::core::Result<ISettingsResult>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self), pushsettings.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsResult>(result__)
    }
    pub unsafe fn GetSettingByPath<'a, P0>(&self, path: P0) -> ::windows::core::Result<ISettingsItem>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSettingByPath)(::windows::core::Vtable::as_raw(self), path.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn CreateSettingByPath<'a, P0>(&self, path: P0) -> ::windows::core::Result<ISettingsItem>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSettingByPath)(::windows::core::Vtable::as_raw(self), path.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn RemoveSettingByPath<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).RemoveSettingByPath)(::windows::core::Vtable::as_raw(self), path.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttribute<'a, P0>(&self, name: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAttribute)(::windows::core::Vtable::as_raw(self), name.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
::windows::core::interface_hierarchy!(ISettingsNamespace, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISettingsNamespace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsNamespace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsNamespace {}
impl ::core::fmt::Debug for ISettingsNamespace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsNamespace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISettingsNamespace {
    type Vtable = ISettingsNamespace_Vtbl;
}
unsafe impl ::windows::core::Interface for ISettingsNamespace {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bba_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsNamespace_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ISettingsResult(::windows::core::IUnknown);
impl ISettingsResult {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetErrorCode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetContextDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContextDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetLine(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetColumn(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetColumn)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSource)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
}
::windows::core::interface_hierarchy!(ISettingsResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISettingsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsResult {}
impl ::core::fmt::Debug for ISettingsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISettingsResult {
    type Vtable = ISettingsResult_Vtbl;
}
unsafe impl ::windows::core::Interface for ISettingsResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bbc_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrout: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetContextDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwline: *mut u32) -> ::windows::core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcolumn: *mut u32) -> ::windows::core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
pub struct ITargetInfo(::windows::core::IUnknown);
impl ITargetInfo {
    pub unsafe fn GetTargetMode(&self) -> ::windows::core::Result<WcmTargetMode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTargetMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmTargetMode>(result__)
    }
    pub unsafe fn SetTargetMode(&self, targetmode: WcmTargetMode) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTargetMode)(::windows::core::Vtable::as_raw(self), targetmode).ok()
    }
    pub unsafe fn GetTemporaryStoreLocation(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTemporaryStoreLocation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetTemporaryStoreLocation<'a, P0>(&self, temporarystorelocation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetTemporaryStoreLocation)(::windows::core::Vtable::as_raw(self), temporarystorelocation.into()).ok()
    }
    pub unsafe fn GetTargetID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTargetID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetTargetID(&self, targetid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTargetID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(targetid)).ok()
    }
    pub unsafe fn GetTargetProcessorArchitecture(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTargetProcessorArchitecture)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetTargetProcessorArchitecture<'a, P0>(&self, processorarchitecture: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetTargetProcessorArchitecture)(::windows::core::Vtable::as_raw(self), processorarchitecture.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty<'a, P0, P1>(&self, offline: P0, property: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), offline.into(), property.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty<'a, P0, P1, P2>(&self, offline: P0, property: P1, value: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetProperty)(::windows::core::Vtable::as_raw(self), offline.into(), property.into(), value.into()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEnumerator)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpandTarget<'a, P0, P1>(&self, offline: P0, location: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExpandTarget)(::windows::core::Vtable::as_raw(self), offline.into(), location.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpandTargetPath<'a, P0, P1>(&self, offline: P0, location: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExpandTargetPath)(::windows::core::Vtable::as_raw(self), offline.into(), location.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetModulePath<'a, P0, P1>(&self, module: P0, path: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetModulePath)(::windows::core::Vtable::as_raw(self), module.into(), path.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadModule<'a, P0>(&self, module: P0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadModule)(::windows::core::Vtable::as_raw(self), module.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HINSTANCE>(result__)
    }
    pub unsafe fn SetWow64Context<'a, P0>(&self, installermodule: P0, wow64context: *const u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetWow64Context)(::windows::core::Vtable::as_raw(self), installermodule.into(), ::core::mem::transmute(wow64context)).ok()
    }
    pub unsafe fn TranslateWow64<'a, P0, P1>(&self, clientarchitecture: P0, value: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TranslateWow64)(::windows::core::Vtable::as_raw(self), clientarchitecture.into(), value.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetSchemaHiveLocation<'a, P0>(&self, pwzhivedir: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetSchemaHiveLocation)(::windows::core::Vtable::as_raw(self), pwzhivedir.into()).ok()
    }
    pub unsafe fn GetSchemaHiveLocation(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSchemaHiveLocation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn SetSchemaHiveMountName<'a, P0>(&self, pwzmountname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).SetSchemaHiveMountName)(::windows::core::Vtable::as_raw(self), pwzmountname.into()).ok()
    }
    pub unsafe fn GetSchemaHiveMountName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSchemaHiveMountName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
}
::windows::core::interface_hierarchy!(ITargetInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITargetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetInfo {}
impl ::core::fmt::Debug for ITargetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITargetInfo {
    type Vtable = ITargetInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ITargetInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb8_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: *mut WcmTargetMode) -> ::windows::core::HRESULT,
    pub SetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: WcmTargetMode) -> ::windows::core::HRESULT,
    pub GetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temporarystorelocation: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temporarystorelocation: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetTargetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetTargetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processorarchitecture: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processorarchitecture: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows::core::PCWSTR, value: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProperty: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows::core::PCWSTR, expandedlocation: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTargetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows::core::PCWSTR, expandedlocation: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTargetPath: usize,
    pub SetModulePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: ::windows::core::PCWSTR, path: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: ::windows::core::PCWSTR, modulehandle: *mut super::super::Foundation::HINSTANCE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadModule: usize,
    pub SetWow64Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, installermodule: ::windows::core::PCWSTR, wow64context: *const u8) -> ::windows::core::HRESULT,
    pub TranslateWow64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientarchitecture: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR, translatedvalue: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSchemaHiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzhivedir: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSchemaHiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phivelocation: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSchemaHiveMountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzmountname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSchemaHiveMountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmountname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const SettingsEngine: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7d7bb5_20b3_11da_81a5_0030f1642e3c);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ABORTOPERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255384i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ASSERTIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255398i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ATTRIBUTENOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255420i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_ATTRIBUTENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255421i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_CONFLICTINGASSERTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255399i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_CYCLICREFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255389i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_DUPLICATENAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255397i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_EXPRESSIONNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255408i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_HANDLERNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255394i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INTERNALERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255424i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255385i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDDATATYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255416i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255401i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDHANDLERSYNTAX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255393i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDKEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255396i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDLANGUAGEFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255410i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDPATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255413i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDPROCESSORFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255382i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDSTREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255395i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255419i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVALUEFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255418i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_INVALIDVERSIONFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255411i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_KEYNOTCHANGEABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255409i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MANIFESTCOMPILATIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255390i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MISSINGCONFIGURATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255383i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_MIXTYPEASSERTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255388i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NAMESPACEALREADYREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255403i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NAMESPACENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255404i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTIFICATIONNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255400i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTPOSITIONED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255415i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_NOTSUPPORTEDFUNCTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255387i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_READONLYITEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255414i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_RESTRICTIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255391i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_SOURCEMANEMPTYVALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255381i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STATENODENOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255422i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STATENODENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255423i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_STORECORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255402i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_SUBSTITUTIONNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255407i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_TYPENOTSPECIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255417i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_UNKNOWNRESULT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145251325i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_USERALREADYREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255406i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_USERNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255405i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_VALIDATIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255392i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_VALUETOOBIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255386i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_E_WRONGESCAPESTRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145255412i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_ARCHITECTURE: ::windows::core::PCWSTR = ::windows::w!("architecture");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_LANGUAGE: ::windows::core::PCWSTR = ::windows::w!("language");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_NAME: ::windows::core::PCWSTR = ::windows::w!("name");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_TOKEN: ::windows::core::PCWSTR = ::windows::w!("token");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_URI: ::windows::core::PCWSTR = ::windows::w!("uri");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_VERSION: ::windows::core::PCWSTR = ::windows::w!("version");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_SETTINGS_ID_VERSION_SCOPE: ::windows::core::PCWSTR = ::windows::w!("versionScope");
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_ATTRIBUTENOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(2232325i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_ATTRIBUTENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(2232321i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_INTERNALERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(2232320i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(2232324i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_LEGACYSETTINGWARNING: ::windows::core::HRESULT = ::windows::core::HRESULT(2232322i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const WCM_S_NAMESPACENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(2232326i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmDataType(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeByte: WcmDataType = WcmDataType(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeSByte: WcmDataType = WcmDataType(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt16: WcmDataType = WcmDataType(3i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt16: WcmDataType = WcmDataType(4i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt32: WcmDataType = WcmDataType(5i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt32: WcmDataType = WcmDataType(6i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeUInt64: WcmDataType = WcmDataType(7i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeInt64: WcmDataType = WcmDataType(8i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeBoolean: WcmDataType = WcmDataType(11i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeString: WcmDataType = WcmDataType(12i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const dataTypeFlagArray: WcmDataType = WcmDataType(32768i32);
impl ::core::marker::Copy for WcmDataType {}
impl ::core::clone::Clone for WcmDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmDataType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmDataType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmDataType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmNamespaceAccess(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const ReadOnlyAccess: WcmNamespaceAccess = WcmNamespaceAccess(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const ReadWriteAccess: WcmNamespaceAccess = WcmNamespaceAccess(2i32);
impl ::core::marker::Copy for WcmNamespaceAccess {}
impl ::core::clone::Clone for WcmNamespaceAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmNamespaceAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmNamespaceAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmNamespaceAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmNamespaceAccess").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmNamespaceEnumerationFlags(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const SharedEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const AllEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(3i32);
impl ::core::marker::Copy for WcmNamespaceEnumerationFlags {}
impl ::core::clone::Clone for WcmNamespaceEnumerationFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmNamespaceEnumerationFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmNamespaceEnumerationFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmNamespaceEnumerationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmNamespaceEnumerationFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmRestrictionFacets(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMaxLength: WcmRestrictionFacets = WcmRestrictionFacets(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetEnumeration: WcmRestrictionFacets = WcmRestrictionFacets(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMaxInclusive: WcmRestrictionFacets = WcmRestrictionFacets(4i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const restrictionFacetMinInclusive: WcmRestrictionFacets = WcmRestrictionFacets(8i32);
impl ::core::marker::Copy for WcmRestrictionFacets {}
impl ::core::clone::Clone for WcmRestrictionFacets {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmRestrictionFacets {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmRestrictionFacets {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmRestrictionFacets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmRestrictionFacets").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmSettingType(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeScalar: WcmSettingType = WcmSettingType(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeComplex: WcmSettingType = WcmSettingType(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const settingTypeList: WcmSettingType = WcmSettingType(3i32);
impl ::core::marker::Copy for WcmSettingType {}
impl ::core::clone::Clone for WcmSettingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmSettingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmSettingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmSettingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmSettingType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmTargetMode(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const OfflineMode: WcmTargetMode = WcmTargetMode(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const OnlineMode: WcmTargetMode = WcmTargetMode(2i32);
impl ::core::marker::Copy for WcmTargetMode {}
impl ::core::clone::Clone for WcmTargetMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmTargetMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmTargetMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmTargetMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmTargetMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WcmUserStatus(pub i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UnknownStatus: WcmUserStatus = WcmUserStatus(0i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserRegistered: WcmUserStatus = WcmUserStatus(1i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserUnregistered: WcmUserStatus = WcmUserStatus(2i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserLoaded: WcmUserStatus = WcmUserStatus(3i32);
#[doc = "*Required features: `\"Win32_System_SettingsManagementInfrastructure\"`*"]
pub const UserUnloaded: WcmUserStatus = WcmUserStatus(4i32);
impl ::core::marker::Copy for WcmUserStatus {}
impl ::core::clone::Clone for WcmUserStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmUserStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WcmUserStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmUserStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmUserStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
