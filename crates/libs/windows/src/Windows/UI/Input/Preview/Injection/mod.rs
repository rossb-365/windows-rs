#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputGamepadInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInjectedInputGamepadInfo {
    type Vtable = IInjectedInputGamepadInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IInjectedInputGamepadInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20ae9a3f_df11_4572_a9ab_d75b8a5e48ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputGamepadInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Gaming_Input")]
    pub Buttons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))]
    Buttons: usize,
    #[cfg(feature = "Gaming_Input")]
    pub SetButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))]
    SetButtons: usize,
    pub LeftThumbstickX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetLeftThumbstickX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub LeftThumbstickY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetLeftThumbstickY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub LeftTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetLeftTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RightThumbstickX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRightThumbstickX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RightThumbstickY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRightThumbstickY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RightTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRightTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputGamepadInfoFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInjectedInputGamepadInfoFactory {
    type Vtable = IInjectedInputGamepadInfoFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IInjectedInputGamepadInfoFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59596876_6c39_4ec4_8b2a_29ef7de18aca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputGamepadInfoFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Gaming_Input")]
    pub CreateInstanceFromGamepadReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reading: super::super::super::super::Gaming::Input::GamepadReading, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))]
    CreateInstanceFromGamepadReading: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputKeyboardInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInjectedInputKeyboardInfo {
    type Vtable = IInjectedInputKeyboardInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IInjectedInputKeyboardInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b46d140_2b6a_5ffa_7eae_bd077b052acd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputKeyboardInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub KeyOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputKeyOptions) -> ::windows::core::HRESULT,
    pub SetKeyOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputKeyOptions) -> ::windows::core::HRESULT,
    pub ScanCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SetScanCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT,
    pub VirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SetVirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputMouseInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInjectedInputMouseInfo {
    type Vtable = IInjectedInputMouseInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IInjectedInputMouseInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96f56e6b_e47a_5cf4_418d_8a5fb9670c7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputMouseInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MouseOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputMouseOptions) -> ::windows::core::HRESULT,
    pub SetMouseOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputMouseOptions) -> ::windows::core::HRESULT,
    pub MouseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMouseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DeltaY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetDeltaY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub DeltaX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetDeltaX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub TimeOffsetInMilliseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTimeOffsetInMilliseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputPenInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInjectedInputPenInfo {
    type Vtable = IInjectedInputPenInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IInjectedInputPenInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b40ad03_ca1e_5527_7e02_2828540bb1d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputPenInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PointerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPointerInfo) -> ::windows::core::HRESULT,
    pub SetPointerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputPointerInfo) -> ::windows::core::HRESULT,
    pub PenButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPenButtons) -> ::windows::core::HRESULT,
    pub SetPenButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputPenButtons) -> ::windows::core::HRESULT,
    pub PenParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPenParameters) -> ::windows::core::HRESULT,
    pub SetPenParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputPenParameters) -> ::windows::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub TiltX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetTiltX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub TiltY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetTiltY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInjectedInputTouchInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInjectedInputTouchInfo {
    type Vtable = IInjectedInputTouchInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IInjectedInputTouchInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x224fd1df_43e8_5ef5_510a_69ca8c9b4c28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputTouchInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputRectangle) -> ::windows::core::HRESULT,
    pub SetContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputRectangle) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub PointerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputPointerInfo) -> ::windows::core::HRESULT,
    pub SetPointerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputPointerInfo) -> ::windows::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub TouchParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InjectedInputTouchParameters) -> ::windows::core::HRESULT,
    pub SetTouchParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InjectedInputTouchParameters) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputInjector(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInputInjector {
    type Vtable = IInputInjector_Vtbl;
}
unsafe impl ::windows::core::Interface for IInputInjector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ec26f84_0b02_4bd2_ad7a_3d4658be3e18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub InjectKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InjectKeyboardInput: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InjectMouseInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InjectMouseInput: usize,
    pub InitializeTouchInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visualmode: InjectedInputVisualizationMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub InjectTouchInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InjectTouchInput: usize,
    pub UninitializeTouchInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InitializePenInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visualmode: InjectedInputVisualizationMode) -> ::windows::core::HRESULT,
    pub InjectPenInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UninitializePenInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InjectShortcut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortcut: InjectedInputShortcut) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputInjector2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInputInjector2 {
    type Vtable = IInputInjector2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInputInjector2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e7a905d_1453_43a7_9bcb_06d6d7b305f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjector2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InitializeGamepadInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InjectGamepadInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UninitializeGamepadInjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputInjectorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInputInjectorStatics {
    type Vtable = IInputInjectorStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IInputInjectorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdeae6943_7402_4141_a5c6_0c01aa57b16a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjectorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputInjectorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInputInjectorStatics2 {
    type Vtable = IInputInjectorStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IInputInjectorStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4db38fb_dd8c_414f_95ea_f87ef4c0ae6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjectorStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryCreateForAppBroadcastOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
pub struct InjectedInputGamepadInfo(::windows::core::IUnknown);
impl InjectedInputGamepadInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InjectedInputGamepadInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Gaming_Input\"`*"]
    #[cfg(feature = "Gaming_Input")]
    pub fn Buttons(&self) -> ::windows::core::Result<super::super::super::super::Gaming::Input::GamepadButtons> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Buttons)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::super::Gaming::Input::GamepadButtons>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input\"`*"]
    #[cfg(feature = "Gaming_Input")]
    pub fn SetButtons(&self, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetButtons)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LeftThumbstickX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LeftThumbstickX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLeftThumbstickX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLeftThumbstickX)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LeftThumbstickY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LeftThumbstickY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLeftThumbstickY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLeftThumbstickY)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LeftTrigger(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LeftTrigger)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLeftTrigger(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLeftTrigger)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RightThumbstickX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RightThumbstickX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRightThumbstickX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRightThumbstickX)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RightThumbstickY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RightThumbstickY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRightThumbstickY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRightThumbstickY)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RightTrigger(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RightTrigger)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRightTrigger(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRightTrigger)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Gaming_Input\"`*"]
    #[cfg(feature = "Gaming_Input")]
    pub fn CreateInstanceFromGamepadReading(reading: super::super::super::super::Gaming::Input::GamepadReading) -> ::windows::core::Result<InjectedInputGamepadInfo> {
        Self::IInjectedInputGamepadInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstanceFromGamepadReading)(::windows::core::Vtable::as_raw(this), reading, result__.as_mut_ptr()).from_abi::<InjectedInputGamepadInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInjectedInputGamepadInfoFactory<R, F: FnOnce(&IInjectedInputGamepadInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InjectedInputGamepadInfo, IInjectedInputGamepadInfoFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InjectedInputGamepadInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputGamepadInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputGamepadInfo {}
impl ::core::fmt::Debug for InjectedInputGamepadInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputGamepadInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputGamepadInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo;{20ae9a3f-df11-4572-a9ab-d75b8a5e48ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InjectedInputGamepadInfo {
    type Vtable = IInjectedInputGamepadInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for InjectedInputGamepadInfo {
    const IID: ::windows::core::GUID = <IInjectedInputGamepadInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InjectedInputGamepadInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo";
}
::windows::core::interface_hierarchy!(InjectedInputGamepadInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
pub struct InjectedInputKeyboardInfo(::windows::core::IUnknown);
impl InjectedInputKeyboardInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InjectedInputKeyboardInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn KeyOptions(&self) -> ::windows::core::Result<InjectedInputKeyOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyOptions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputKeyOptions>(result__)
        }
    }
    pub fn SetKeyOptions(&self, value: InjectedInputKeyOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetKeyOptions)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ScanCode(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScanCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SetScanCode(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScanCode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn VirtualKey(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VirtualKey)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SetVirtualKey(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVirtualKey)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InjectedInputKeyboardInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputKeyboardInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputKeyboardInfo {}
impl ::core::fmt::Debug for InjectedInputKeyboardInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputKeyboardInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputKeyboardInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo;{4b46d140-2b6a-5ffa-7eae-bd077b052acd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InjectedInputKeyboardInfo {
    type Vtable = IInjectedInputKeyboardInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for InjectedInputKeyboardInfo {
    const IID: ::windows::core::GUID = <IInjectedInputKeyboardInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InjectedInputKeyboardInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo";
}
::windows::core::interface_hierarchy!(InjectedInputKeyboardInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
pub struct InjectedInputMouseInfo(::windows::core::IUnknown);
impl InjectedInputMouseInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InjectedInputMouseInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MouseOptions(&self) -> ::windows::core::Result<InjectedInputMouseOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MouseOptions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputMouseOptions>(result__)
        }
    }
    pub fn SetMouseOptions(&self, value: InjectedInputMouseOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMouseOptions)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MouseData(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MouseData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMouseData(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMouseData)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DeltaY(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeltaY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDeltaY(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDeltaY)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DeltaX(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeltaX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDeltaX(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDeltaX)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn TimeOffsetInMilliseconds(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeOffsetInMilliseconds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTimeOffsetInMilliseconds(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTimeOffsetInMilliseconds)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InjectedInputMouseInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputMouseInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputMouseInfo {}
impl ::core::fmt::Debug for InjectedInputMouseInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputMouseInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputMouseInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo;{96f56e6b-e47a-5cf4-418d-8a5fb9670c7d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InjectedInputMouseInfo {
    type Vtable = IInjectedInputMouseInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for InjectedInputMouseInfo {
    const IID: ::windows::core::GUID = <IInjectedInputMouseInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InjectedInputMouseInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo";
}
::windows::core::interface_hierarchy!(InjectedInputMouseInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
pub struct InjectedInputPenInfo(::windows::core::IUnknown);
impl InjectedInputPenInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InjectedInputPenInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PointerInfo(&self) -> ::windows::core::Result<InjectedInputPointerInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputPointerInfo>(result__)
        }
    }
    pub fn SetPointerInfo(&self, value: InjectedInputPointerInfo) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPointerInfo)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PenButtons(&self) -> ::windows::core::Result<InjectedInputPenButtons> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PenButtons)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputPenButtons>(result__)
        }
    }
    pub fn SetPenButtons(&self, value: InjectedInputPenButtons) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPenButtons)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PenParameters(&self) -> ::windows::core::Result<InjectedInputPenParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PenParameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputPenParameters>(result__)
        }
    }
    pub fn SetPenParameters(&self, value: InjectedInputPenParameters) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPenParameters)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Pressure(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pressure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetPressure(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPressure)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rotation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotation(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRotation)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn TiltX(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TiltX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetTiltX(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTiltX)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn TiltY(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TiltY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetTiltY(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTiltY)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InjectedInputPenInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputPenInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputPenInfo {}
impl ::core::fmt::Debug for InjectedInputPenInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputPenInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputPenInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputPenInfo;{6b40ad03-ca1e-5527-7e02-2828540bb1d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InjectedInputPenInfo {
    type Vtable = IInjectedInputPenInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for InjectedInputPenInfo {
    const IID: ::windows::core::GUID = <IInjectedInputPenInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InjectedInputPenInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputPenInfo";
}
::windows::core::interface_hierarchy!(InjectedInputPenInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
pub struct InjectedInputTouchInfo(::windows::core::IUnknown);
impl InjectedInputTouchInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InjectedInputTouchInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Contact(&self) -> ::windows::core::Result<InjectedInputRectangle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Contact)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputRectangle>(result__)
        }
    }
    pub fn SetContact(&self, value: InjectedInputRectangle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContact)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Orientation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetOrientation(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOrientation)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PointerInfo(&self) -> ::windows::core::Result<InjectedInputPointerInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputPointerInfo>(result__)
        }
    }
    pub fn SetPointerInfo(&self, value: InjectedInputPointerInfo) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPointerInfo)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Pressure(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pressure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetPressure(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPressure)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn TouchParameters(&self) -> ::windows::core::Result<InjectedInputTouchParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TouchParameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InjectedInputTouchParameters>(result__)
        }
    }
    pub fn SetTouchParameters(&self, value: InjectedInputTouchParameters) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTouchParameters)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InjectedInputTouchInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InjectedInputTouchInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InjectedInputTouchInfo {}
impl ::core::fmt::Debug for InjectedInputTouchInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputTouchInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputTouchInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo;{224fd1df-43e8-5ef5-510a-69ca8c9b4c28})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InjectedInputTouchInfo {
    type Vtable = IInjectedInputTouchInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for InjectedInputTouchInfo {
    const IID: ::windows::core::GUID = <IInjectedInputTouchInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InjectedInputTouchInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo";
}
::windows::core::interface_hierarchy!(InjectedInputTouchInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
pub struct InputInjector(::windows::core::IUnknown);
impl InputInjector {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InjectKeyboardInput<'a, P0, E0>(&self, input: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InjectKeyboardInput)(::windows::core::Vtable::as_raw(this), input.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InjectMouseInput<'a, P0, E0>(&self, input: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InjectMouseInput)(::windows::core::Vtable::as_raw(this), input.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn InitializeTouchInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InitializeTouchInjection)(::windows::core::Vtable::as_raw(this), visualmode).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InjectTouchInput<'a, P0, E0>(&self, input: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InjectTouchInput)(::windows::core::Vtable::as_raw(this), input.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn UninitializeTouchInjection(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UninitializeTouchInjection)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn InitializePenInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InitializePenInjection)(::windows::core::Vtable::as_raw(this), visualmode).ok() }
    }
    pub fn InjectPenInput(&self, input: &InjectedInputPenInfo) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InjectPenInput)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn UninitializePenInjection(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UninitializePenInjection)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn InjectShortcut(&self, shortcut: InjectedInputShortcut) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InjectShortcut)(::windows::core::Vtable::as_raw(this), shortcut).ok() }
    }
    pub fn InitializeGamepadInjection(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InitializeGamepadInjection)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn InjectGamepadInput(&self, input: &InjectedInputGamepadInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InjectGamepadInput)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn UninitializeGamepadInjection(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).UninitializeGamepadInjection)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn TryCreate() -> ::windows::core::Result<InputInjector> {
        Self::IInputInjectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InputInjector>(result__)
        })
    }
    pub fn TryCreateForAppBroadcastOnly() -> ::windows::core::Result<InputInjector> {
        Self::IInputInjectorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateForAppBroadcastOnly)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InputInjector>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputInjectorStatics<R, F: FnOnce(&IInputInjectorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InputInjector, IInputInjectorStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInputInjectorStatics2<R, F: FnOnce(&IInputInjectorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InputInjector, IInputInjectorStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InputInjector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputInjector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputInjector {}
impl ::core::fmt::Debug for InputInjector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputInjector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputInjector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InputInjector;{8ec26f84-0b02-4bd2-ad7a-3d4658be3e18})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InputInjector {
    type Vtable = IInputInjector_Vtbl;
}
unsafe impl ::windows::core::Interface for InputInjector {
    const IID: ::windows::core::GUID = <IInputInjector as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputInjector {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InputInjector";
}
::windows::core::interface_hierarchy!(InputInjector, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InjectedInputButtonChangeKind(pub i32);
impl InjectedInputButtonChangeKind {
    pub const None: Self = Self(0i32);
    pub const FirstButtonDown: Self = Self(1i32);
    pub const FirstButtonUp: Self = Self(2i32);
    pub const SecondButtonDown: Self = Self(3i32);
    pub const SecondButtonUp: Self = Self(4i32);
    pub const ThirdButtonDown: Self = Self(5i32);
    pub const ThirdButtonUp: Self = Self(6i32);
    pub const FourthButtonDown: Self = Self(7i32);
    pub const FourthButtonUp: Self = Self(8i32);
    pub const FifthButtonDown: Self = Self(9i32);
    pub const FifthButtonUp: Self = Self(10i32);
}
impl ::core::marker::Copy for InjectedInputButtonChangeKind {}
impl ::core::clone::Clone for InjectedInputButtonChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputButtonChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InjectedInputButtonChangeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputButtonChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputButtonChangeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputButtonChangeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputButtonChangeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InjectedInputKeyOptions(pub u32);
impl InjectedInputKeyOptions {
    pub const None: Self = Self(0u32);
    pub const ExtendedKey: Self = Self(1u32);
    pub const KeyUp: Self = Self(2u32);
    pub const ScanCode: Self = Self(8u32);
    pub const Unicode: Self = Self(4u32);
}
impl ::core::marker::Copy for InjectedInputKeyOptions {}
impl ::core::clone::Clone for InjectedInputKeyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputKeyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InjectedInputKeyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputKeyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputKeyOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputKeyOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputKeyOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputKeyOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputKeyOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputKeyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputKeyOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputKeyOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InjectedInputMouseOptions(pub u32);
impl InjectedInputMouseOptions {
    pub const None: Self = Self(0u32);
    pub const Move: Self = Self(1u32);
    pub const LeftDown: Self = Self(2u32);
    pub const LeftUp: Self = Self(4u32);
    pub const RightDown: Self = Self(8u32);
    pub const RightUp: Self = Self(16u32);
    pub const MiddleDown: Self = Self(32u32);
    pub const MiddleUp: Self = Self(64u32);
    pub const XDown: Self = Self(128u32);
    pub const XUp: Self = Self(256u32);
    pub const Wheel: Self = Self(2048u32);
    pub const HWheel: Self = Self(4096u32);
    pub const MoveNoCoalesce: Self = Self(8192u32);
    pub const VirtualDesk: Self = Self(16384u32);
    pub const Absolute: Self = Self(32768u32);
}
impl ::core::marker::Copy for InjectedInputMouseOptions {}
impl ::core::clone::Clone for InjectedInputMouseOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputMouseOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InjectedInputMouseOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputMouseOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputMouseOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputMouseOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputMouseOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputMouseOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputMouseOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputMouseOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputMouseOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputMouseOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InjectedInputPenButtons(pub u32);
impl InjectedInputPenButtons {
    pub const None: Self = Self(0u32);
    pub const Barrel: Self = Self(1u32);
    pub const Inverted: Self = Self(2u32);
    pub const Eraser: Self = Self(4u32);
}
impl ::core::marker::Copy for InjectedInputPenButtons {}
impl ::core::clone::Clone for InjectedInputPenButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputPenButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InjectedInputPenButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputPenButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputPenButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputPenButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputPenButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputPenButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputPenButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputPenButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputPenButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPenButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InjectedInputPenParameters(pub u32);
impl InjectedInputPenParameters {
    pub const None: Self = Self(0u32);
    pub const Pressure: Self = Self(1u32);
    pub const Rotation: Self = Self(2u32);
    pub const TiltX: Self = Self(4u32);
    pub const TiltY: Self = Self(8u32);
}
impl ::core::marker::Copy for InjectedInputPenParameters {}
impl ::core::clone::Clone for InjectedInputPenParameters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputPenParameters {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InjectedInputPenParameters {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputPenParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputPenParameters").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputPenParameters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputPenParameters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputPenParameters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputPenParameters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputPenParameters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputPenParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPenParameters;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InjectedInputPointerOptions(pub u32);
impl InjectedInputPointerOptions {
    pub const None: Self = Self(0u32);
    pub const New: Self = Self(1u32);
    pub const InRange: Self = Self(2u32);
    pub const InContact: Self = Self(4u32);
    pub const FirstButton: Self = Self(16u32);
    pub const SecondButton: Self = Self(32u32);
    pub const Primary: Self = Self(8192u32);
    pub const Confidence: Self = Self(16384u32);
    pub const Canceled: Self = Self(32768u32);
    pub const PointerDown: Self = Self(65536u32);
    pub const Update: Self = Self(131072u32);
    pub const PointerUp: Self = Self(262144u32);
    pub const CaptureChanged: Self = Self(2097152u32);
}
impl ::core::marker::Copy for InjectedInputPointerOptions {}
impl ::core::clone::Clone for InjectedInputPointerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputPointerOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InjectedInputPointerOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputPointerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputPointerOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputPointerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputPointerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputPointerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputPointerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputPointerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputPointerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InjectedInputShortcut(pub i32);
impl InjectedInputShortcut {
    pub const Back: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Search: Self = Self(2i32);
}
impl ::core::marker::Copy for InjectedInputShortcut {}
impl ::core::clone::Clone for InjectedInputShortcut {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputShortcut {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InjectedInputShortcut {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputShortcut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputShortcut").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputShortcut {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputShortcut;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InjectedInputTouchParameters(pub u32);
impl InjectedInputTouchParameters {
    pub const None: Self = Self(0u32);
    pub const Contact: Self = Self(1u32);
    pub const Orientation: Self = Self(2u32);
    pub const Pressure: Self = Self(4u32);
}
impl ::core::marker::Copy for InjectedInputTouchParameters {}
impl ::core::clone::Clone for InjectedInputTouchParameters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputTouchParameters {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InjectedInputTouchParameters {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputTouchParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputTouchParameters").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InjectedInputTouchParameters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InjectedInputTouchParameters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InjectedInputTouchParameters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InjectedInputTouchParameters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InjectedInputTouchParameters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputTouchParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputTouchParameters;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InjectedInputVisualizationMode(pub i32);
impl InjectedInputVisualizationMode {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Indirect: Self = Self(2i32);
}
impl ::core::marker::Copy for InjectedInputVisualizationMode {}
impl ::core::clone::Clone for InjectedInputVisualizationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InjectedInputVisualizationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InjectedInputVisualizationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InjectedInputVisualizationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InjectedInputVisualizationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InjectedInputVisualizationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputVisualizationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
pub struct InjectedInputPoint {
    pub PositionX: i32,
    pub PositionY: i32,
}
impl ::core::marker::Copy for InjectedInputPoint {}
impl ::core::clone::Clone for InjectedInputPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for InjectedInputPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InjectedInputPoint").field("PositionX", &self.PositionX).field("PositionY", &self.PositionY).finish()
    }
}
unsafe impl ::windows::core::Abi for InjectedInputPoint {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InjectedInputPoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputPoint;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for InjectedInputPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<InjectedInputPoint>()) == 0 }
    }
}
impl ::core::cmp::Eq for InjectedInputPoint {}
impl ::core::default::Default for InjectedInputPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
pub struct InjectedInputPointerInfo {
    pub PointerId: u32,
    pub PointerOptions: InjectedInputPointerOptions,
    pub PixelLocation: InjectedInputPoint,
    pub TimeOffsetInMilliseconds: u32,
    pub PerformanceCount: u64,
}
impl ::core::marker::Copy for InjectedInputPointerInfo {}
impl ::core::clone::Clone for InjectedInputPointerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for InjectedInputPointerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InjectedInputPointerInfo").field("PointerId", &self.PointerId).field("PointerOptions", &self.PointerOptions).field("PixelLocation", &self.PixelLocation).field("TimeOffsetInMilliseconds", &self.TimeOffsetInMilliseconds).field("PerformanceCount", &self.PerformanceCount).finish()
    }
}
unsafe impl ::windows::core::Abi for InjectedInputPointerInfo {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InjectedInputPointerInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputPointerInfo;u4;enum(Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions;u4);struct(Windows.UI.Input.Preview.Injection.InjectedInputPoint;i4;i4);u4;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for InjectedInputPointerInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<InjectedInputPointerInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for InjectedInputPointerInfo {}
impl ::core::default::Default for InjectedInputPointerInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Input_Preview_Injection\"`*"]
pub struct InjectedInputRectangle {
    pub Left: i32,
    pub Top: i32,
    pub Bottom: i32,
    pub Right: i32,
}
impl ::core::marker::Copy for InjectedInputRectangle {}
impl ::core::clone::Clone for InjectedInputRectangle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for InjectedInputRectangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("InjectedInputRectangle").field("Left", &self.Left).field("Top", &self.Top).field("Bottom", &self.Bottom).field("Right", &self.Right).finish()
    }
}
unsafe impl ::windows::core::Abi for InjectedInputRectangle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InjectedInputRectangle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputRectangle;i4;i4;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for InjectedInputRectangle {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<InjectedInputRectangle>()) == 0 }
    }
}
impl ::core::cmp::Eq for InjectedInputRectangle {}
impl ::core::default::Default for InjectedInputRectangle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
