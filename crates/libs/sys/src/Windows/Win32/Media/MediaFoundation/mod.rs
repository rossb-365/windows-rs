#[cfg_attr(windows, link(name = "windows"))]
extern "system" {
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn CreateNamedPropertyStore(ppstore: *mut super::super::UI::Shell::PropertiesSystem::INamedPropertyStore) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn CreatePropertyStore(ppstore: *mut super::super::UI::Shell::PropertiesSystem::IPropertyStore) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn DXVA2CreateDirect3DDeviceManager9(presettoken: *mut u32, ppdevicemanager: *mut IDirect3DDeviceManager9) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn DXVA2CreateVideoService(pdd: super::super::Graphics::Direct3D9::IDirect3DDevice9, riid: *const ::windows_sys::core::GUID, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn DXVAHD_CreateDevice(pd3ddevice: super::super::Graphics::Direct3D9::IDirect3DDevice9Ex, pcontentdesc: *const DXVAHD_CONTENT_DESC, usage: DXVAHD_DEVICE_USAGE, pplugin: PDXVAHDSW_Plugin, ppdevice: *mut IDXVAHD_Device) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFAddPeriodicCallback(callback: MFPERIODICCALLBACK, pcontext: ::windows_sys::core::IUnknown, pdwkey: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFAllocateSerialWorkQueue(dwworkqueue: u32, pdwworkqueue: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFAllocateWorkQueue(pdwworkqueue: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFAllocateWorkQueueEx(workqueuetype: MFASYNC_WORKQUEUE_TYPE, pdwworkqueue: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFAverageTimePerFrameToFrameRate(unaveragetimeperframe: u64, punnumerator: *mut u32, pundenominator: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFBeginCreateFile(accessmode: MF_FILE_ACCESSMODE, openmode: MF_FILE_OPENMODE, fflags: MF_FILE_FLAGS, pwszfilepath: ::windows_sys::core::PCWSTR, pcallback: IMFAsyncCallback, pstate: ::windows_sys::core::IUnknown, ppcancelcookie: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFBeginRegisterWorkQueueWithMMCSS(dwworkqueueid: u32, wszclass: ::windows_sys::core::PCWSTR, dwtaskid: u32, pdonecallback: IMFAsyncCallback, pdonestate: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFBeginRegisterWorkQueueWithMMCSSEx(dwworkqueueid: u32, wszclass: ::windows_sys::core::PCWSTR, dwtaskid: u32, lpriority: i32, pdonecallback: IMFAsyncCallback, pdonestate: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFBeginUnregisterWorkQueueWithMMCSS(dwworkqueueid: u32, pdonecallback: IMFAsyncCallback, pdonestate: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MFCalculateBitmapImageSize(pbmih: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, cbbufsize: u32, pcbimagesize: *mut u32, pbknown: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCalculateImageSize(guidsubtype: *const ::windows_sys::core::GUID, unwidth: u32, unheight: u32, pcbimagesize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCancelCreateFile(pcancelcookie: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCancelWorkItem(key: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCombineSamples(psample: IMFSample, psampletoadd: IMFSample, dwmaxmergeddurationinms: u32, pmerged: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCompareFullToPartialMediaType(pmftypefull: IMFMediaType, pmftypepartial: IMFMediaType) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFConvertColorInfoFromDXVA(ptoformat: *mut MFVIDEOFORMAT, dwfromdxva: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFConvertColorInfoToDXVA(pdwtodxva: *mut u32, pfromformat: *const MFVIDEOFORMAT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFConvertFromFP16Array(pdest: *mut f32, psrc: *const u16, dwcount: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFConvertToFP16Array(pdest: *mut u16, psrc: *const f32, dwcount: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCopyImage(pdest: *mut u8, ldeststride: i32, psrc: *const u8, lsrcstride: i32, dwwidthinbytes: u32, dwlines: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreate2DMediaBuffer(dwwidth: u32, dwheight: u32, dwfourcc: u32, fbottomup: super::super::Foundation::BOOL, ppbuffer: *mut IMFMediaBuffer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreate3GPMediaSink(pibytestream: IMFByteStream, pvideomediatype: IMFMediaType, paudiomediatype: IMFMediaType, ppimediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateAC3MediaSink(ptargetbytestream: IMFByteStream, paudiomediatype: IMFMediaType, ppmediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateADTSMediaSink(ptargetbytestream: IMFByteStream, paudiomediatype: IMFMediaType, ppmediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateAMMediaTypeFromMFMediaType(pmftype: IMFMediaType, guidformatblocktype: ::windows_sys::core::GUID, ppamtype: *mut *mut AM_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFContentInfo(ppicontentinfo: *mut IMFASFContentInfo) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFIndexer(ppiindexer: *mut IMFASFIndexer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFIndexerByteStream(picontentbytestream: IMFByteStream, cbindexstartoffset: u64, piindexbytestream: *mut IMFByteStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFMediaSink(pibytestream: IMFByteStream, ppimediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFMediaSinkActivate(pwszfilename: ::windows_sys::core::PCWSTR, pcontentinfo: IMFASFContentInfo, ppiactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFMultiplexer(ppimultiplexer: *mut IMFASFMultiplexer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFProfile(ppiprofile: *mut IMFASFProfile) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFProfileFromPresentationDescriptor(pipd: IMFPresentationDescriptor, ppiprofile: *mut IMFASFProfile) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFSplitter(ppisplitter: *mut IMFASFSplitter) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFStreamSelector(piasfprofile: IMFASFProfile, ppselector: *mut IMFASFStreamSelector) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFStreamingMediaSink(pibytestream: IMFByteStream, ppimediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateASFStreamingMediaSinkActivate(pbytestreamactivate: IMFActivate, pcontentinfo: IMFASFContentInfo, ppiactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateAVIMediaSink(pibytestream: IMFByteStream, pvideomediatype: IMFMediaType, paudiomediatype: IMFMediaType, ppimediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateAggregateSource(psourcecollection: IMFCollection, ppaggsource: *mut IMFMediaSource) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateAlignedMemoryBuffer(cbmaxlength: u32, cbaligment: u32, ppbuffer: *mut IMFMediaBuffer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateAsyncResult(punkobject: ::windows_sys::core::IUnknown, pcallback: IMFAsyncCallback, punkstate: ::windows_sys::core::IUnknown, ppasyncresult: *mut IMFAsyncResult) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateAttributes(ppmfattributes: *mut IMFAttributes, cinitialsize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub fn MFCreateAudioMediaType(paudioformat: *const super::Audio::WAVEFORMATEX, ppiaudiomediatype: *mut IMFAudioMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateAudioRenderer(paudioattributes: IMFAttributes, ppsink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateAudioRendererActivate(ppactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateCameraOcclusionStateMonitor(symboliclink: ::windows_sys::core::PCWSTR, callback: IMFCameraOcclusionStateReportCallback, occlusionstatemonitor: *mut IMFCameraOcclusionStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateCollection(ppimfcollection: *mut IMFCollection) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateContentDecryptorContext(guidmediaprotectionsystemid: *const ::windows_sys::core::GUID, pd3dmanager: IMFDXGIDeviceManager, pcontentprotectiondevice: IMFContentProtectionDevice, ppcontentdecryptorcontext: *mut IMFContentDecryptorContext) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateContentProtectionDevice(protectionsystemid: *const ::windows_sys::core::GUID, contentprotectiondevice: *mut IMFContentProtectionDevice) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateCredentialCache(ppcache: *mut IMFNetCredentialCache) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn MFCreateD3D12SynchronizationObject(pdevice: super::super::Graphics::Direct3D12::ID3D12Device, riid: *const ::windows_sys::core::GUID, ppvsyncobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateDXGIDeviceManager(resettoken: *mut u32, ppdevicemanager: *mut IMFDXGIDeviceManager) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateDXGISurfaceBuffer(riid: *const ::windows_sys::core::GUID, punksurface: ::windows_sys::core::IUnknown, usubresourceindex: u32, fbottomupwhenlinear: super::super::Foundation::BOOL, ppbuffer: *mut IMFMediaBuffer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateDXSurfaceBuffer(riid: *const ::windows_sys::core::GUID, punksurface: ::windows_sys::core::IUnknown, fbottomupwhenlinear: super::super::Foundation::BOOL, ppbuffer: *mut IMFMediaBuffer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateDeviceSource(pattributes: IMFAttributes, ppsource: *mut IMFMediaSource) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateDeviceSourceActivate(pattributes: IMFAttributes, ppactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFCreateEncryptedMediaExtensionsStoreActivate(pmphost: IMFPMPHostApp, objectstream: super::super::System::Com::IStream, classid: ::windows_sys::core::PCWSTR, activate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateEventQueue(ppmediaeventqueue: *mut IMFMediaEventQueue) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateExtendedCameraIntrinsicModel(distortionmodeltype: MFCameraIntrinsic_DistortionModelType, ppextendedcameraintrinsicmodel: *mut IMFExtendedCameraIntrinsicModel) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateExtendedCameraIntrinsics(ppextendedcameraintrinsics: *mut IMFExtendedCameraIntrinsics) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateFMPEG4MediaSink(pibytestream: IMFByteStream, pvideomediatype: IMFMediaType, paudiomediatype: IMFMediaType, ppimediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateFile(accessmode: MF_FILE_ACCESSMODE, openmode: MF_FILE_OPENMODE, fflags: MF_FILE_FLAGS, pwszfileurl: ::windows_sys::core::PCWSTR, ppibytestream: *mut IMFByteStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Media_DxMediaObjects\"`*"]
    #[cfg(feature = "Win32_Media_DxMediaObjects")]
    pub fn MFCreateLegacyMediaBufferOnMFMediaBuffer(psample: IMFSample, pmfmediabuffer: IMFMediaBuffer, cboffset: u32, ppmediabuffer: *mut super::DxMediaObjects::IMediaBuffer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFCreateMFByteStreamOnStream(pstream: super::super::System::Com::IStream, ppbytestream: *mut IMFByteStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMFByteStreamOnStreamEx(punkstream: ::windows_sys::core::IUnknown, ppbytestream: *mut IMFByteStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMFByteStreamWrapper(pstream: IMFByteStream, ppstreamwrapper: *mut IMFByteStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateMFVideoFormatFromMFMediaType(pmftype: IMFMediaType, ppmfvf: *mut *mut MFVIDEOFORMAT, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMP3MediaSink(ptargetbytestream: IMFByteStream, ppmediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMPEG4MediaSink(pibytestream: IMFByteStream, pvideomediatype: IMFMediaType, paudiomediatype: IMFMediaType, ppimediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMediaBufferFromMediaType(pmediatype: IMFMediaType, llduration: i64, dwminlength: u32, dwminalignment: u32, ppbuffer: *mut IMFMediaBuffer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMediaBufferWrapper(pbuffer: IMFMediaBuffer, cboffset: u32, dwlength: u32, ppbuffer: *mut IMFMediaBuffer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFCreateMediaEvent(met: u32, guidextendedtype: *const ::windows_sys::core::GUID, hrstatus: ::windows_sys::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppevent: *mut IMFMediaEvent) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMediaExtensionActivate(szactivatableclassid: ::windows_sys::core::PCWSTR, pconfiguration: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMediaSession(pconfiguration: IMFAttributes, ppmediasession: *mut IMFMediaSession) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMediaType(ppmftype: *mut IMFMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMediaTypeFromProperties(punkstream: ::windows_sys::core::IUnknown, ppmediatype: *mut IMFMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMediaTypeFromRepresentation(guidrepresentation: ::windows_sys::core::GUID, pvrepresentation: *const ::core::ffi::c_void, ppimediatype: *mut IMFMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMemoryBuffer(cbmaxlength: u32, ppbuffer: *mut IMFMediaBuffer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMuxSink(guidoutputsubtype: ::windows_sys::core::GUID, poutputattributes: IMFAttributes, poutputbytestream: IMFByteStream, ppmuxsink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMuxStreamAttributes(pattributestomux: IMFCollection, ppmuxattribs: *mut IMFAttributes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMuxStreamMediaType(pmediatypestomux: IMFCollection, ppmuxmediatype: *mut IMFMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateMuxStreamSample(psamplestomux: IMFCollection, ppmuxsample: *mut IMFSample) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateNetSchemePlugin(riid: *const ::windows_sys::core::GUID, ppvhandler: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreatePMPMediaSession(dwcreationflags: u32, pconfiguration: IMFAttributes, ppmediasession: *mut IMFMediaSession, ppenableractivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreatePMPServer(dwcreationflags: u32, pppmpserver: *mut IMFPMPServer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreatePresentationClock(pppresentationclock: *mut IMFPresentationClock) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreatePresentationDescriptor(cstreamdescriptors: u32, apstreamdescriptors: *const IMFStreamDescriptor, pppresentationdescriptor: *mut IMFPresentationDescriptor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreatePresentationDescriptorFromASFProfile(piprofile: IMFASFProfile, ppipd: *mut IMFPresentationDescriptor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreatePropertiesFromMediaType(pmediatype: IMFMediaType, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateProtectedEnvironmentAccess(ppaccess: *mut IMFProtectedEnvironmentAccess) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn MFCreateProxyLocator(pszprotocol: ::windows_sys::core::PCWSTR, pproxyconfig: super::super::UI::Shell::PropertiesSystem::IPropertyStore, ppproxylocator: *mut IMFNetProxyLocator) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateRelativePanelWatcher(videodeviceid: ::windows_sys::core::PCWSTR, displaymonitordeviceid: ::windows_sys::core::PCWSTR, pprelativepanelwatcher: *mut IMFRelativePanelWatcher) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateRemoteDesktopPlugin(ppplugin: *mut IMFRemoteDesktopPlugin) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSample(ppimfsample: *mut IMFSample) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSampleCopierMFT(ppcopiermft: *mut IMFTransform) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSampleGrabberSinkActivate(pimfmediatype: IMFMediaType, pimfsamplegrabbersinkcallback: IMFSampleGrabberSinkCallback, ppiactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSensorActivityMonitor(pcallback: IMFSensorActivitiesReportCallback, ppactivitymonitor: *mut IMFSensorActivityMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSensorGroup(sensorgroupsymboliclink: ::windows_sys::core::PCWSTR, ppsensorgroup: *mut IMFSensorGroup) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSensorProfile(profiletype: *const ::windows_sys::core::GUID, profileindex: u32, constraints: ::windows_sys::core::PCWSTR, ppprofile: *mut IMFSensorProfile) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSensorProfileCollection(ppsensorprofile: *mut IMFSensorProfileCollection) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSensorStream(streamid: u32, pattributes: IMFAttributes, pmediatypecollection: IMFCollection, ppstream: *mut IMFSensorStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFCreateSequencerSegmentOffset(dwid: u32, hnsoffset: i64, pvarsegmentoffset: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSequencerSource(preserved: ::windows_sys::core::IUnknown, ppsequencersource: *mut IMFSequencerSource) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSimpleTypeHandler(pphandler: *mut IMFMediaTypeHandler) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSinkWriterFromMediaSink(pmediasink: IMFMediaSink, pattributes: IMFAttributes, ppsinkwriter: *mut IMFSinkWriter) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSinkWriterFromURL(pwszoutputurl: ::windows_sys::core::PCWSTR, pbytestream: IMFByteStream, pattributes: IMFAttributes, ppsinkwriter: *mut IMFSinkWriter) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSourceReaderFromByteStream(pbytestream: IMFByteStream, pattributes: IMFAttributes, ppsourcereader: *mut IMFSourceReader) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSourceReaderFromMediaSource(pmediasource: IMFMediaSource, pattributes: IMFAttributes, ppsourcereader: *mut IMFSourceReader) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSourceReaderFromURL(pwszurl: ::windows_sys::core::PCWSTR, pattributes: IMFAttributes, ppsourcereader: *mut IMFSourceReader) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSourceResolver(ppisourceresolver: *mut IMFSourceResolver) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateStandardQualityManager(ppqualitymanager: *mut IMFQualityManager) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateStreamDescriptor(dwstreamidentifier: u32, cmediatypes: u32, apmediatypes: *const IMFMediaType, ppdescriptor: *mut IMFStreamDescriptor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFCreateStreamOnMFByteStream(pbytestream: IMFByteStream, ppstream: *mut super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateStreamOnMFByteStreamEx(pbytestream: IMFByteStream, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateSystemTimeSource(ppsystemtimesource: *mut IMFPresentationTimeSource) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTempFile(accessmode: MF_FILE_ACCESSMODE, openmode: MF_FILE_OPENMODE, fflags: MF_FILE_FLAGS, ppibytestream: *mut IMFByteStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTopoLoader(ppobj: *mut IMFTopoLoader) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTopology(pptopo: *mut IMFTopology) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTopologyNode(nodetype: MF_TOPOLOGY_TYPE, ppnode: *mut IMFTopologyNode) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTrackedSample(ppmfsample: *mut IMFTrackedSample) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTranscodeProfile(pptranscodeprofile: *mut IMFTranscodeProfile) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTranscodeSinkActivate(ppactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTranscodeTopology(psrc: IMFMediaSource, pwszoutputfilepath: ::windows_sys::core::PCWSTR, pprofile: IMFTranscodeProfile, pptranscodetopo: *mut IMFTopology) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTranscodeTopologyFromByteStream(psrc: IMFMediaSource, poutputstream: IMFByteStream, pprofile: IMFTranscodeProfile, pptranscodetopo: *mut IMFTopology) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateTransformActivate(ppactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateVideoMediaType(pvideoformat: *const MFVIDEOFORMAT, ppivideomediatype: *mut IMFVideoMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn MFCreateVideoMediaTypeFromBitMapInfoHeader(pbmihbitmapinfoheader: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, dwpixelaspectratiox: u32, dwpixelaspectratioy: u32, interlacemode: MFVideoInterlaceMode, videoflags: u64, qwframespersecondnumerator: u64, qwframesperseconddenominator: u64, dwmaxbitrate: u32, ppivideomediatype: *mut IMFVideoMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn MFCreateVideoMediaTypeFromBitMapInfoHeaderEx(pbmihbitmapinfoheader: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, cbbitmapinfoheader: u32, dwpixelaspectratiox: u32, dwpixelaspectratioy: u32, interlacemode: MFVideoInterlaceMode, videoflags: u64, dwframespersecondnumerator: u32, dwframesperseconddenominator: u32, dwmaxbitrate: u32, ppivideomediatype: *mut IMFVideoMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateVideoMediaTypeFromSubtype(pamsubtype: *const ::windows_sys::core::GUID, ppivideomediatype: *mut IMFVideoMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateVideoMixer(powner: ::windows_sys::core::IUnknown, riiddevice: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateVideoMixerAndPresenter(pmixerowner: ::windows_sys::core::IUnknown, ppresenterowner: ::windows_sys::core::IUnknown, riidmixer: *const ::windows_sys::core::GUID, ppvvideomixer: *mut *mut ::core::ffi::c_void, riidpresenter: *const ::windows_sys::core::GUID, ppvvideopresenter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateVideoPresenter(powner: ::windows_sys::core::IUnknown, riiddevice: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppvideopresenter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateVideoRenderer(riidrenderer: *const ::windows_sys::core::GUID, ppvideorenderer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateVideoRendererActivate(hwndvideo: super::super::Foundation::HWND, ppactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateVideoSampleAllocator(riid: *const ::windows_sys::core::GUID, ppsampleallocator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateVideoSampleAllocatorEx(riid: *const ::windows_sys::core::GUID, ppsampleallocator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateVideoSampleFromSurface(punksurface: ::windows_sys::core::IUnknown, ppsample: *mut IMFSample) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateVirtualCamera(r#type: MFVirtualCameraType, lifetime: MFVirtualCameraLifetime, access: MFVirtualCameraAccess, friendlyname: ::windows_sys::core::PCWSTR, sourceid: ::windows_sys::core::PCWSTR, categories: *const ::windows_sys::core::GUID, categorycount: u32, virtualcamera: *mut IMFVirtualCamera) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateWAVEMediaSink(ptargetbytestream: IMFByteStream, paudiomediatype: IMFMediaType, ppmediasink: *mut IMFMediaSink) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFCreateWICBitmapBuffer(riid: *const ::windows_sys::core::GUID, punksurface: ::windows_sys::core::IUnknown, ppbuffer: *mut IMFMediaBuffer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn MFCreateWMAEncoderActivate(pmediatype: IMFMediaType, pencodingconfigurationproperties: super::super::UI::Shell::PropertiesSystem::IPropertyStore, ppactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn MFCreateWMVEncoderActivate(pmediatype: IMFMediaType, pencodingconfigurationproperties: super::super::UI::Shell::PropertiesSystem::IPropertyStore, ppactivate: *mut IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub fn MFCreateWaveFormatExFromMFMediaType(pmftype: IMFMediaType, ppwf: *mut *mut super::Audio::WAVEFORMATEX, pcbsize: *mut u32, flags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFDeserializeAttributesFromStream(pattr: IMFAttributes, dwoptions: u32, pstm: super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFDeserializePresentationDescriptor(cbdata: u32, pbdata: *const u8, pppd: *mut IMFPresentationDescriptor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFEndCreateFile(presult: IMFAsyncResult, ppfile: *mut IMFByteStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFEndRegisterWorkQueueWithMMCSS(presult: IMFAsyncResult, pdwtaskid: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFEndUnregisterWorkQueueWithMMCSS(presult: IMFAsyncResult) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFEnumDeviceSources(pattributes: IMFAttributes, pppsourceactivate: *mut *mut IMFActivate, pcsourceactivate: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFFrameRateToAverageTimePerFrame(unnumerator: u32, undenominator: u32, punaveragetimeperframe: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetAttributesAsBlob(pattributes: IMFAttributes, pbuf: *mut u8, cbbufsize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetAttributesAsBlobSize(pattributes: IMFAttributes, pcbbufsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetContentProtectionSystemCLSID(guidprotectionsystemid: *const ::windows_sys::core::GUID, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetLocalId(verifier: *const u8, size: u32, id: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetMFTMerit(pmft: ::windows_sys::core::IUnknown, cbverifier: u32, verifier: *const u8, merit: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetPlaneSize(format: u32, dwwidth: u32, dwheight: u32, pdwplanesize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetPluginControl(ppplugincontrol: *mut IMFPluginControl) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetService(punkobject: ::windows_sys::core::IUnknown, guidservice: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetStrideForBitmapInfoHeader(format: u32, dwwidth: u32, pstride: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFGetSupportedMimeTypes(ppropvarmimetypearray: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFGetSupportedSchemes(ppropvarschemearray: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetSystemId(ppid: *mut IMFSystemId) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetSystemTime() -> i64;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetTimerPeriodicity(periodicity: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetTopoNodeCurrentType(pnode: IMFTopologyNode, dwstreamindex: u32, foutput: super::super::Foundation::BOOL, pptype: *mut IMFMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetUncompressedVideoFormat(pvideoformat: *const MFVIDEOFORMAT) -> u32;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetWorkQueueMMCSSClass(dwworkqueueid: u32, pwszclass: ::windows_sys::core::PWSTR, pcchclass: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetWorkQueueMMCSSPriority(dwworkqueueid: u32, lpriority: *mut i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFGetWorkQueueMMCSSTaskId(dwworkqueueid: u32, pdwtaskid: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFHeapAlloc(nsize: usize, dwflags: u32, pszfile: ::windows_sys::core::PCSTR, line: i32, eat: EAllocationType) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFHeapFree(pv: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitAMMediaTypeFromMFMediaType(pmftype: IMFMediaType, guidformatblocktype: ::windows_sys::core::GUID, pamtype: *mut AM_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFInitAttributesFromBlob(pattributes: IMFAttributes, pbuf: *const u8, cbbufsize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitMediaTypeFromAMMediaType(pmftype: IMFMediaType, pamtype: *const AM_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitMediaTypeFromMFVideoFormat(pmftype: IMFMediaType, pmfvf: *const MFVIDEOFORMAT, cbbufsize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MFInitMediaTypeFromMPEG1VideoInfo(pmftype: IMFMediaType, pmp1vi: *const MPEG1VIDEOINFO, cbbufsize: u32, psubtype: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MFInitMediaTypeFromMPEG2VideoInfo(pmftype: IMFMediaType, pmp2vi: *const MPEG2VIDEOINFO, cbbufsize: u32, psubtype: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MFInitMediaTypeFromVideoInfoHeader(pmftype: IMFMediaType, pvih: *const VIDEOINFOHEADER, cbbufsize: u32, psubtype: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MFInitMediaTypeFromVideoInfoHeader2(pmftype: IMFMediaType, pvih2: *const VIDEOINFOHEADER2, cbbufsize: u32, psubtype: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub fn MFInitMediaTypeFromWaveFormatEx(pmftype: IMFMediaType, pwaveformat: *const super::Audio::WAVEFORMATEX, cbbufsize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitVideoFormat(pvideoformat: *const MFVIDEOFORMAT, r#type: MFStandardVideoFormat) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitVideoFormat_RGB(pvideoformat: *const MFVIDEOFORMAT, dwwidth: u32, dwheight: u32, d3dfmt: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFInvokeCallback(pasyncresult: IMFAsyncResult) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFIsContentProtectionDeviceSupported(protectionsystemid: *const ::windows_sys::core::GUID, issupported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFIsFormatYUV(format: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFIsVirtualCameraTypeSupported(r#type: MFVirtualCameraType, supported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFLoadSignedLibrary(pszname: ::windows_sys::core::PCWSTR, pplib: *mut IMFSignedLibrary) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFLockDXGIDeviceManager(presettoken: *mut u32, ppmanager: *mut IMFDXGIDeviceManager) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFLockPlatform() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFLockSharedWorkQueue(wszclass: ::windows_sys::core::PCWSTR, basepriority: i32, pdwtaskid: *mut u32, pid: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFLockWorkQueue(dwworkqueue: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub fn MFMapDX9FormatToDXGIFormat(dx9: u32) -> super::super::Graphics::Dxgi::Common::DXGI_FORMAT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub fn MFMapDXGIFormatToDX9Format(dx11: super::super::Graphics::Dxgi::Common::DXGI_FORMAT) -> u32;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFPCreateMediaPlayer(pwszurl: ::windows_sys::core::PCWSTR, fstartplayback: super::super::Foundation::BOOL, creationoptions: MFP_CREATION_OPTIONS, pcallback: IMFPMediaPlayerCallback, hwnd: super::super::Foundation::HWND, ppmediaplayer: *mut IMFPMediaPlayer) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFPutWaitingWorkItem(hevent: super::super::Foundation::HANDLE, priority: i32, presult: IMFAsyncResult, pkey: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFPutWorkItem(dwqueue: u32, pcallback: IMFAsyncCallback, pstate: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFPutWorkItem2(dwqueue: u32, priority: i32, pcallback: IMFAsyncCallback, pstate: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFPutWorkItemEx(dwqueue: u32, presult: IMFAsyncResult) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFPutWorkItemEx2(dwqueue: u32, priority: i32, presult: IMFAsyncResult) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFRegisterLocalByteStreamHandler(szfileextension: ::windows_sys::core::PCWSTR, szmimetype: ::windows_sys::core::PCWSTR, pactivate: IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFRegisterLocalSchemeHandler(szscheme: ::windows_sys::core::PCWSTR, pactivate: IMFActivate) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFRegisterPlatformWithMMCSS(wszclass: ::windows_sys::core::PCWSTR, pdwtaskid: *mut u32, lpriority: i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFRemovePeriodicCallback(dwkey: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFRequireProtectedEnvironment(ppresentationdescriptor: IMFPresentationDescriptor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFScheduleWorkItem(pcallback: IMFAsyncCallback, pstate: ::windows_sys::core::IUnknown, timeout: i64, pkey: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFScheduleWorkItemEx(presult: IMFAsyncResult, timeout: i64, pkey: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFSerializeAttributesToStream(pattr: IMFAttributes, dwoptions: u32, pstm: super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFSerializePresentationDescriptor(ppd: IMFPresentationDescriptor, pcbdata: *mut u32, ppbdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFShutdown() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFShutdownObject(punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFSplitSample(psample: IMFSample, poutputsamples: *mut IMFSample, dwoutputsamplemaxcount: u32, pdwoutputsamplecount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFStartup(version: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFTEnum(guidcategory: ::windows_sys::core::GUID, flags: u32, pinputtype: *const MFT_REGISTER_TYPE_INFO, poutputtype: *const MFT_REGISTER_TYPE_INFO, pattributes: IMFAttributes, ppclsidmft: *mut *mut ::windows_sys::core::GUID, pcmfts: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFTEnum2(guidcategory: ::windows_sys::core::GUID, flags: MFT_ENUM_FLAG, pinputtype: *const MFT_REGISTER_TYPE_INFO, poutputtype: *const MFT_REGISTER_TYPE_INFO, pattributes: IMFAttributes, pppmftactivate: *mut *mut IMFActivate, pnummftactivate: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFTEnumEx(guidcategory: ::windows_sys::core::GUID, flags: MFT_ENUM_FLAG, pinputtype: *const MFT_REGISTER_TYPE_INFO, poutputtype: *const MFT_REGISTER_TYPE_INFO, pppmftactivate: *mut *mut IMFActivate, pnummftactivate: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFTGetInfo(clsidmft: ::windows_sys::core::GUID, pszname: *mut ::windows_sys::core::PWSTR, ppinputtypes: *mut *mut MFT_REGISTER_TYPE_INFO, pcinputtypes: *mut u32, ppoutputtypes: *mut *mut MFT_REGISTER_TYPE_INFO, pcoutputtypes: *mut u32, ppattributes: *mut IMFAttributes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFTRegister(clsidmft: ::windows_sys::core::GUID, guidcategory: ::windows_sys::core::GUID, pszname: ::windows_sys::core::PCWSTR, flags: u32, cinputtypes: u32, pinputtypes: *const MFT_REGISTER_TYPE_INFO, coutputtypes: u32, poutputtypes: *const MFT_REGISTER_TYPE_INFO, pattributes: IMFAttributes) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFTRegisterLocal(pclassfactory: super::super::System::Com::IClassFactory, guidcategory: *const ::windows_sys::core::GUID, pszname: ::windows_sys::core::PCWSTR, flags: u32, cinputtypes: u32, pinputtypes: *const MFT_REGISTER_TYPE_INFO, coutputtypes: u32, poutputtypes: *const MFT_REGISTER_TYPE_INFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFTRegisterLocalByCLSID(clisdmft: *const ::windows_sys::core::GUID, guidcategory: *const ::windows_sys::core::GUID, pszname: ::windows_sys::core::PCWSTR, flags: u32, cinputtypes: u32, pinputtypes: *const MFT_REGISTER_TYPE_INFO, coutputtypes: u32, poutputtypes: *const MFT_REGISTER_TYPE_INFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFTUnregister(clsidmft: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFTUnregisterLocal(pclassfactory: super::super::System::Com::IClassFactory) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFTUnregisterLocalByCLSID(clsidmft: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFTranscodeGetAudioOutputAvailableTypes(guidsubtype: *const ::windows_sys::core::GUID, dwmftflags: u32, pcodecconfig: IMFAttributes, ppavailabletypes: *mut IMFCollection) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFUnlockDXGIDeviceManager() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFUnlockPlatform() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFUnlockWorkQueue(dwworkqueue: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFUnregisterPlatformFromMMCSS() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFUnwrapMediaType(pwrap: IMFMediaType, pporig: *mut IMFMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFValidateMediaTypeSize(formattype: ::windows_sys::core::GUID, pblock: *const u8, cbsize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFWrapMediaType(porig: IMFMediaType, majortype: *const ::windows_sys::core::GUID, subtype: *const ::windows_sys::core::GUID, ppwrap: *mut IMFMediaType) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn MFllMulDiv(a: i64, b: i64, c: i64, d: i64) -> i64;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OPMGetVideoOutputForTarget(padapterluid: *const super::super::Foundation::LUID, vidpntarget: u32, vos: OPM_VIDEO_OUTPUT_SEMANTICS, ppopmvideooutput: *mut IOPMVideoOutput) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OPMGetVideoOutputsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, vos: OPM_VIDEO_OUTPUT_SEMANTICS, pulnumvideooutputs: *mut u32, pppopmvideooutputarray: *mut *mut IOPMVideoOutput) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn OPMGetVideoOutputsFromIDirect3DDevice9Object(pdirect3ddevice9: super::super::Graphics::Direct3D9::IDirect3DDevice9, vos: OPM_VIDEO_OUTPUT_SEMANTICS, pulnumvideooutputs: *mut u32, pppopmvideooutputarray: *mut *mut IOPMVideoOutput) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn OPMXboxEnableHDCP(hdcptype: OPM_HDCP_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn OPMXboxGetHDCPStatus(phdcpstatus: *mut OPM_HDCP_STATUS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
    pub fn OPMXboxGetHDCPStatusAndType(phdcpstatus: *mut OPM_HDCP_STATUS, phdcptype: *mut OPM_HDCP_TYPE) -> ::windows_sys::core::HRESULT;
}
pub type IAdvancedMediaCapture = *mut ::core::ffi::c_void;
pub type IAdvancedMediaCaptureInitializationSettings = *mut ::core::ffi::c_void;
pub type IAdvancedMediaCaptureSettings = *mut ::core::ffi::c_void;
pub type IAudioSourceProvider = *mut ::core::ffi::c_void;
pub type IClusterDetector = *mut ::core::ffi::c_void;
pub type ICodecAPI = *mut ::core::ffi::c_void;
pub type ID3D12VideoDecodeCommandList = *mut ::core::ffi::c_void;
pub type ID3D12VideoDecodeCommandList1 = *mut ::core::ffi::c_void;
pub type ID3D12VideoDecodeCommandList2 = *mut ::core::ffi::c_void;
pub type ID3D12VideoDecoder = *mut ::core::ffi::c_void;
pub type ID3D12VideoDecoder1 = *mut ::core::ffi::c_void;
pub type ID3D12VideoDecoderHeap = *mut ::core::ffi::c_void;
pub type ID3D12VideoDecoderHeap1 = *mut ::core::ffi::c_void;
pub type ID3D12VideoDevice = *mut ::core::ffi::c_void;
pub type ID3D12VideoDevice1 = *mut ::core::ffi::c_void;
pub type ID3D12VideoDevice2 = *mut ::core::ffi::c_void;
pub type ID3D12VideoDevice3 = *mut ::core::ffi::c_void;
pub type ID3D12VideoEncodeCommandList = *mut ::core::ffi::c_void;
pub type ID3D12VideoEncodeCommandList1 = *mut ::core::ffi::c_void;
pub type ID3D12VideoEncodeCommandList2 = *mut ::core::ffi::c_void;
pub type ID3D12VideoEncoder = *mut ::core::ffi::c_void;
pub type ID3D12VideoEncoderHeap = *mut ::core::ffi::c_void;
pub type ID3D12VideoExtensionCommand = *mut ::core::ffi::c_void;
pub type ID3D12VideoMotionEstimator = *mut ::core::ffi::c_void;
pub type ID3D12VideoMotionVectorHeap = *mut ::core::ffi::c_void;
pub type ID3D12VideoProcessCommandList = *mut ::core::ffi::c_void;
pub type ID3D12VideoProcessCommandList1 = *mut ::core::ffi::c_void;
pub type ID3D12VideoProcessCommandList2 = *mut ::core::ffi::c_void;
pub type ID3D12VideoProcessor = *mut ::core::ffi::c_void;
pub type ID3D12VideoProcessor1 = *mut ::core::ffi::c_void;
pub type IDXVAHD_Device = *mut ::core::ffi::c_void;
pub type IDXVAHD_VideoProcessor = *mut ::core::ffi::c_void;
pub type IDirect3D9ExOverlayExtension = *mut ::core::ffi::c_void;
pub type IDirect3DAuthenticatedChannel9 = *mut ::core::ffi::c_void;
pub type IDirect3DCryptoSession9 = *mut ::core::ffi::c_void;
pub type IDirect3DDevice9Video = *mut ::core::ffi::c_void;
pub type IDirect3DDeviceManager9 = *mut ::core::ffi::c_void;
pub type IDirectXVideoAccelerationService = *mut ::core::ffi::c_void;
pub type IDirectXVideoDecoder = *mut ::core::ffi::c_void;
pub type IDirectXVideoDecoderService = *mut ::core::ffi::c_void;
pub type IDirectXVideoMemoryConfiguration = *mut ::core::ffi::c_void;
pub type IDirectXVideoProcessor = *mut ::core::ffi::c_void;
pub type IDirectXVideoProcessorService = *mut ::core::ffi::c_void;
pub type IEVRFilterConfig = *mut ::core::ffi::c_void;
pub type IEVRFilterConfigEx = *mut ::core::ffi::c_void;
pub type IEVRTrustedVideoPlugin = *mut ::core::ffi::c_void;
pub type IEVRVideoStreamControl = *mut ::core::ffi::c_void;
pub type IFileClient = *mut ::core::ffi::c_void;
pub type IFileIo = *mut ::core::ffi::c_void;
pub type IMF2DBuffer = *mut ::core::ffi::c_void;
pub type IMF2DBuffer2 = *mut ::core::ffi::c_void;
pub type IMFASFContentInfo = *mut ::core::ffi::c_void;
pub type IMFASFIndexer = *mut ::core::ffi::c_void;
pub type IMFASFMultiplexer = *mut ::core::ffi::c_void;
pub type IMFASFMutualExclusion = *mut ::core::ffi::c_void;
pub type IMFASFProfile = *mut ::core::ffi::c_void;
pub type IMFASFSplitter = *mut ::core::ffi::c_void;
pub type IMFASFStreamConfig = *mut ::core::ffi::c_void;
pub type IMFASFStreamPrioritization = *mut ::core::ffi::c_void;
pub type IMFASFStreamSelector = *mut ::core::ffi::c_void;
pub type IMFActivate = *mut ::core::ffi::c_void;
pub type IMFAsyncCallback = *mut ::core::ffi::c_void;
pub type IMFAsyncCallbackLogging = *mut ::core::ffi::c_void;
pub type IMFAsyncResult = *mut ::core::ffi::c_void;
pub type IMFAttributes = *mut ::core::ffi::c_void;
pub type IMFAudioMediaType = *mut ::core::ffi::c_void;
pub type IMFAudioPolicy = *mut ::core::ffi::c_void;
pub type IMFAudioStreamVolume = *mut ::core::ffi::c_void;
pub type IMFBufferListNotify = *mut ::core::ffi::c_void;
pub type IMFByteStream = *mut ::core::ffi::c_void;
pub type IMFByteStreamBuffering = *mut ::core::ffi::c_void;
pub type IMFByteStreamCacheControl = *mut ::core::ffi::c_void;
pub type IMFByteStreamCacheControl2 = *mut ::core::ffi::c_void;
pub type IMFByteStreamHandler = *mut ::core::ffi::c_void;
pub type IMFByteStreamProxyClassFactory = *mut ::core::ffi::c_void;
pub type IMFByteStreamTimeSeek = *mut ::core::ffi::c_void;
pub type IMFCameraOcclusionStateMonitor = *mut ::core::ffi::c_void;
pub type IMFCameraOcclusionStateReport = *mut ::core::ffi::c_void;
pub type IMFCameraOcclusionStateReportCallback = *mut ::core::ffi::c_void;
pub type IMFCameraSyncObject = *mut ::core::ffi::c_void;
pub type IMFCaptureEngine = *mut ::core::ffi::c_void;
pub type IMFCaptureEngineClassFactory = *mut ::core::ffi::c_void;
pub type IMFCaptureEngineOnEventCallback = *mut ::core::ffi::c_void;
pub type IMFCaptureEngineOnSampleCallback = *mut ::core::ffi::c_void;
pub type IMFCaptureEngineOnSampleCallback2 = *mut ::core::ffi::c_void;
pub type IMFCapturePhotoConfirmation = *mut ::core::ffi::c_void;
pub type IMFCapturePhotoSink = *mut ::core::ffi::c_void;
pub type IMFCapturePreviewSink = *mut ::core::ffi::c_void;
pub type IMFCaptureRecordSink = *mut ::core::ffi::c_void;
pub type IMFCaptureSink = *mut ::core::ffi::c_void;
pub type IMFCaptureSink2 = *mut ::core::ffi::c_void;
pub type IMFCaptureSource = *mut ::core::ffi::c_void;
pub type IMFCdmSuspendNotify = *mut ::core::ffi::c_void;
pub type IMFClock = *mut ::core::ffi::c_void;
pub type IMFClockConsumer = *mut ::core::ffi::c_void;
pub type IMFClockStateSink = *mut ::core::ffi::c_void;
pub type IMFCollection = *mut ::core::ffi::c_void;
pub type IMFContentDecryptionModule = *mut ::core::ffi::c_void;
pub type IMFContentDecryptionModuleAccess = *mut ::core::ffi::c_void;
pub type IMFContentDecryptionModuleFactory = *mut ::core::ffi::c_void;
pub type IMFContentDecryptionModuleSession = *mut ::core::ffi::c_void;
pub type IMFContentDecryptionModuleSessionCallbacks = *mut ::core::ffi::c_void;
pub type IMFContentDecryptorContext = *mut ::core::ffi::c_void;
pub type IMFContentEnabler = *mut ::core::ffi::c_void;
pub type IMFContentProtectionDevice = *mut ::core::ffi::c_void;
pub type IMFContentProtectionManager = *mut ::core::ffi::c_void;
pub type IMFD3D12SynchronizationObject = *mut ::core::ffi::c_void;
pub type IMFD3D12SynchronizationObjectCommands = *mut ::core::ffi::c_void;
pub type IMFDLNASinkInit = *mut ::core::ffi::c_void;
pub type IMFDRMNetHelper = *mut ::core::ffi::c_void;
pub type IMFDXGIBuffer = *mut ::core::ffi::c_void;
pub type IMFDXGIDeviceManager = *mut ::core::ffi::c_void;
pub type IMFDXGIDeviceManagerSource = *mut ::core::ffi::c_void;
pub type IMFDesiredSample = *mut ::core::ffi::c_void;
pub type IMFDeviceTransform = *mut ::core::ffi::c_void;
pub type IMFDeviceTransformCallback = *mut ::core::ffi::c_void;
pub type IMFExtendedCameraControl = *mut ::core::ffi::c_void;
pub type IMFExtendedCameraController = *mut ::core::ffi::c_void;
pub type IMFExtendedCameraIntrinsicModel = *mut ::core::ffi::c_void;
pub type IMFExtendedCameraIntrinsics = *mut ::core::ffi::c_void;
pub type IMFExtendedCameraIntrinsicsDistortionModel6KT = *mut ::core::ffi::c_void;
pub type IMFExtendedCameraIntrinsicsDistortionModelArcTan = *mut ::core::ffi::c_void;
pub type IMFExtendedDRMTypeSupport = *mut ::core::ffi::c_void;
pub type IMFFieldOfUseMFTUnlock = *mut ::core::ffi::c_void;
pub type IMFFinalizableMediaSink = *mut ::core::ffi::c_void;
pub type IMFGetService = *mut ::core::ffi::c_void;
pub type IMFHDCPStatus = *mut ::core::ffi::c_void;
pub type IMFHttpDownloadRequest = *mut ::core::ffi::c_void;
pub type IMFHttpDownloadSession = *mut ::core::ffi::c_void;
pub type IMFHttpDownloadSessionProvider = *mut ::core::ffi::c_void;
pub type IMFImageSharingEngine = *mut ::core::ffi::c_void;
pub type IMFImageSharingEngineClassFactory = *mut ::core::ffi::c_void;
pub type IMFInputTrustAuthority = *mut ::core::ffi::c_void;
pub type IMFLocalMFTRegistration = *mut ::core::ffi::c_void;
pub type IMFMediaBuffer = *mut ::core::ffi::c_void;
pub type IMFMediaEngine = *mut ::core::ffi::c_void;
pub type IMFMediaEngineAudioEndpointId = *mut ::core::ffi::c_void;
pub type IMFMediaEngineClassFactory = *mut ::core::ffi::c_void;
pub type IMFMediaEngineClassFactory2 = *mut ::core::ffi::c_void;
pub type IMFMediaEngineClassFactory3 = *mut ::core::ffi::c_void;
pub type IMFMediaEngineClassFactory4 = *mut ::core::ffi::c_void;
pub type IMFMediaEngineClassFactoryEx = *mut ::core::ffi::c_void;
pub type IMFMediaEngineEME = *mut ::core::ffi::c_void;
pub type IMFMediaEngineEMENotify = *mut ::core::ffi::c_void;
pub type IMFMediaEngineEx = *mut ::core::ffi::c_void;
pub type IMFMediaEngineExtension = *mut ::core::ffi::c_void;
pub type IMFMediaEngineNeedKeyNotify = *mut ::core::ffi::c_void;
pub type IMFMediaEngineNotify = *mut ::core::ffi::c_void;
pub type IMFMediaEngineOPMInfo = *mut ::core::ffi::c_void;
pub type IMFMediaEngineProtectedContent = *mut ::core::ffi::c_void;
pub type IMFMediaEngineSrcElements = *mut ::core::ffi::c_void;
pub type IMFMediaEngineSrcElementsEx = *mut ::core::ffi::c_void;
pub type IMFMediaEngineSupportsSourceTransfer = *mut ::core::ffi::c_void;
pub type IMFMediaEngineTransferSource = *mut ::core::ffi::c_void;
pub type IMFMediaEngineWebSupport = *mut ::core::ffi::c_void;
pub type IMFMediaError = *mut ::core::ffi::c_void;
pub type IMFMediaEvent = *mut ::core::ffi::c_void;
pub type IMFMediaEventGenerator = *mut ::core::ffi::c_void;
pub type IMFMediaEventQueue = *mut ::core::ffi::c_void;
pub type IMFMediaKeySession = *mut ::core::ffi::c_void;
pub type IMFMediaKeySession2 = *mut ::core::ffi::c_void;
pub type IMFMediaKeySessionNotify = *mut ::core::ffi::c_void;
pub type IMFMediaKeySessionNotify2 = *mut ::core::ffi::c_void;
pub type IMFMediaKeySystemAccess = *mut ::core::ffi::c_void;
pub type IMFMediaKeys = *mut ::core::ffi::c_void;
pub type IMFMediaKeys2 = *mut ::core::ffi::c_void;
pub type IMFMediaSession = *mut ::core::ffi::c_void;
pub type IMFMediaSharingEngine = *mut ::core::ffi::c_void;
pub type IMFMediaSharingEngineClassFactory = *mut ::core::ffi::c_void;
pub type IMFMediaSink = *mut ::core::ffi::c_void;
pub type IMFMediaSinkPreroll = *mut ::core::ffi::c_void;
pub type IMFMediaSource = *mut ::core::ffi::c_void;
pub type IMFMediaSource2 = *mut ::core::ffi::c_void;
pub type IMFMediaSourceEx = *mut ::core::ffi::c_void;
pub type IMFMediaSourceExtension = *mut ::core::ffi::c_void;
pub type IMFMediaSourceExtensionLiveSeekableRange = *mut ::core::ffi::c_void;
pub type IMFMediaSourceExtensionNotify = *mut ::core::ffi::c_void;
pub type IMFMediaSourcePresentationProvider = *mut ::core::ffi::c_void;
pub type IMFMediaSourceTopologyProvider = *mut ::core::ffi::c_void;
pub type IMFMediaStream = *mut ::core::ffi::c_void;
pub type IMFMediaStream2 = *mut ::core::ffi::c_void;
pub type IMFMediaStreamSourceSampleRequest = *mut ::core::ffi::c_void;
pub type IMFMediaTimeRange = *mut ::core::ffi::c_void;
pub type IMFMediaType = *mut ::core::ffi::c_void;
pub type IMFMediaTypeHandler = *mut ::core::ffi::c_void;
pub type IMFMetadata = *mut ::core::ffi::c_void;
pub type IMFMetadataProvider = *mut ::core::ffi::c_void;
pub type IMFMuxStreamAttributesManager = *mut ::core::ffi::c_void;
pub type IMFMuxStreamMediaTypeManager = *mut ::core::ffi::c_void;
pub type IMFMuxStreamSampleManager = *mut ::core::ffi::c_void;
pub type IMFNetCredential = *mut ::core::ffi::c_void;
pub type IMFNetCredentialCache = *mut ::core::ffi::c_void;
pub type IMFNetCredentialManager = *mut ::core::ffi::c_void;
pub type IMFNetCrossOriginSupport = *mut ::core::ffi::c_void;
pub type IMFNetProxyLocator = *mut ::core::ffi::c_void;
pub type IMFNetProxyLocatorFactory = *mut ::core::ffi::c_void;
pub type IMFNetResourceFilter = *mut ::core::ffi::c_void;
pub type IMFNetSchemeHandlerConfig = *mut ::core::ffi::c_void;
pub type IMFObjectReferenceStream = *mut ::core::ffi::c_void;
pub type IMFOutputPolicy = *mut ::core::ffi::c_void;
pub type IMFOutputSchema = *mut ::core::ffi::c_void;
pub type IMFOutputTrustAuthority = *mut ::core::ffi::c_void;
pub type IMFPMPClient = *mut ::core::ffi::c_void;
pub type IMFPMPClientApp = *mut ::core::ffi::c_void;
pub type IMFPMPHost = *mut ::core::ffi::c_void;
pub type IMFPMPHostApp = *mut ::core::ffi::c_void;
pub type IMFPMPServer = *mut ::core::ffi::c_void;
pub type IMFPMediaItem = *mut ::core::ffi::c_void;
pub type IMFPMediaPlayer = *mut ::core::ffi::c_void;
pub type IMFPMediaPlayerCallback = *mut ::core::ffi::c_void;
pub type IMFPluginControl = *mut ::core::ffi::c_void;
pub type IMFPluginControl2 = *mut ::core::ffi::c_void;
pub type IMFPresentationClock = *mut ::core::ffi::c_void;
pub type IMFPresentationDescriptor = *mut ::core::ffi::c_void;
pub type IMFPresentationTimeSource = *mut ::core::ffi::c_void;
pub type IMFProtectedEnvironmentAccess = *mut ::core::ffi::c_void;
pub type IMFQualityAdvise = *mut ::core::ffi::c_void;
pub type IMFQualityAdvise2 = *mut ::core::ffi::c_void;
pub type IMFQualityAdviseLimits = *mut ::core::ffi::c_void;
pub type IMFQualityManager = *mut ::core::ffi::c_void;
pub type IMFRateControl = *mut ::core::ffi::c_void;
pub type IMFRateSupport = *mut ::core::ffi::c_void;
pub type IMFReadWriteClassFactory = *mut ::core::ffi::c_void;
pub type IMFRealTimeClient = *mut ::core::ffi::c_void;
pub type IMFRealTimeClientEx = *mut ::core::ffi::c_void;
pub type IMFRelativePanelReport = *mut ::core::ffi::c_void;
pub type IMFRelativePanelWatcher = *mut ::core::ffi::c_void;
pub type IMFRemoteAsyncCallback = *mut ::core::ffi::c_void;
pub type IMFRemoteDesktopPlugin = *mut ::core::ffi::c_void;
pub type IMFRemoteProxy = *mut ::core::ffi::c_void;
pub type IMFSAMIStyle = *mut ::core::ffi::c_void;
pub type IMFSSLCertificateManager = *mut ::core::ffi::c_void;
pub type IMFSample = *mut ::core::ffi::c_void;
pub type IMFSampleAllocatorControl = *mut ::core::ffi::c_void;
pub type IMFSampleGrabberSinkCallback = *mut ::core::ffi::c_void;
pub type IMFSampleGrabberSinkCallback2 = *mut ::core::ffi::c_void;
pub type IMFSampleOutputStream = *mut ::core::ffi::c_void;
pub type IMFSampleProtection = *mut ::core::ffi::c_void;
pub type IMFSaveJob = *mut ::core::ffi::c_void;
pub type IMFSchemeHandler = *mut ::core::ffi::c_void;
pub type IMFSecureBuffer = *mut ::core::ffi::c_void;
pub type IMFSecureChannel = *mut ::core::ffi::c_void;
pub type IMFSeekInfo = *mut ::core::ffi::c_void;
pub type IMFSensorActivitiesReport = *mut ::core::ffi::c_void;
pub type IMFSensorActivitiesReportCallback = *mut ::core::ffi::c_void;
pub type IMFSensorActivityMonitor = *mut ::core::ffi::c_void;
pub type IMFSensorActivityReport = *mut ::core::ffi::c_void;
pub type IMFSensorDevice = *mut ::core::ffi::c_void;
pub type IMFSensorGroup = *mut ::core::ffi::c_void;
pub type IMFSensorProcessActivity = *mut ::core::ffi::c_void;
pub type IMFSensorProfile = *mut ::core::ffi::c_void;
pub type IMFSensorProfileCollection = *mut ::core::ffi::c_void;
pub type IMFSensorStream = *mut ::core::ffi::c_void;
pub type IMFSensorTransformFactory = *mut ::core::ffi::c_void;
pub type IMFSequencerSource = *mut ::core::ffi::c_void;
pub type IMFSharingEngineClassFactory = *mut ::core::ffi::c_void;
pub type IMFShutdown = *mut ::core::ffi::c_void;
pub type IMFSignedLibrary = *mut ::core::ffi::c_void;
pub type IMFSimpleAudioVolume = *mut ::core::ffi::c_void;
pub type IMFSinkWriter = *mut ::core::ffi::c_void;
pub type IMFSinkWriterCallback = *mut ::core::ffi::c_void;
pub type IMFSinkWriterCallback2 = *mut ::core::ffi::c_void;
pub type IMFSinkWriterEncoderConfig = *mut ::core::ffi::c_void;
pub type IMFSinkWriterEx = *mut ::core::ffi::c_void;
pub type IMFSourceBuffer = *mut ::core::ffi::c_void;
pub type IMFSourceBufferAppendMode = *mut ::core::ffi::c_void;
pub type IMFSourceBufferList = *mut ::core::ffi::c_void;
pub type IMFSourceBufferNotify = *mut ::core::ffi::c_void;
pub type IMFSourceOpenMonitor = *mut ::core::ffi::c_void;
pub type IMFSourceReader = *mut ::core::ffi::c_void;
pub type IMFSourceReaderCallback = *mut ::core::ffi::c_void;
pub type IMFSourceReaderCallback2 = *mut ::core::ffi::c_void;
pub type IMFSourceReaderEx = *mut ::core::ffi::c_void;
pub type IMFSourceResolver = *mut ::core::ffi::c_void;
pub type IMFSpatialAudioObjectBuffer = *mut ::core::ffi::c_void;
pub type IMFSpatialAudioSample = *mut ::core::ffi::c_void;
pub type IMFStreamDescriptor = *mut ::core::ffi::c_void;
pub type IMFStreamSink = *mut ::core::ffi::c_void;
pub type IMFStreamingSinkConfig = *mut ::core::ffi::c_void;
pub type IMFSystemId = *mut ::core::ffi::c_void;
pub type IMFTimecodeTranslate = *mut ::core::ffi::c_void;
pub type IMFTimedText = *mut ::core::ffi::c_void;
pub type IMFTimedTextBinary = *mut ::core::ffi::c_void;
pub type IMFTimedTextBouten = *mut ::core::ffi::c_void;
pub type IMFTimedTextCue = *mut ::core::ffi::c_void;
pub type IMFTimedTextCueList = *mut ::core::ffi::c_void;
pub type IMFTimedTextFormattedText = *mut ::core::ffi::c_void;
pub type IMFTimedTextNotify = *mut ::core::ffi::c_void;
pub type IMFTimedTextRegion = *mut ::core::ffi::c_void;
pub type IMFTimedTextRuby = *mut ::core::ffi::c_void;
pub type IMFTimedTextStyle = *mut ::core::ffi::c_void;
pub type IMFTimedTextStyle2 = *mut ::core::ffi::c_void;
pub type IMFTimedTextTrack = *mut ::core::ffi::c_void;
pub type IMFTimedTextTrackList = *mut ::core::ffi::c_void;
pub type IMFTimer = *mut ::core::ffi::c_void;
pub type IMFTopoLoader = *mut ::core::ffi::c_void;
pub type IMFTopology = *mut ::core::ffi::c_void;
pub type IMFTopologyNode = *mut ::core::ffi::c_void;
pub type IMFTopologyNodeAttributeEditor = *mut ::core::ffi::c_void;
pub type IMFTopologyServiceLookup = *mut ::core::ffi::c_void;
pub type IMFTopologyServiceLookupClient = *mut ::core::ffi::c_void;
pub type IMFTrackedSample = *mut ::core::ffi::c_void;
pub type IMFTranscodeProfile = *mut ::core::ffi::c_void;
pub type IMFTranscodeSinkInfoProvider = *mut ::core::ffi::c_void;
pub type IMFTransform = *mut ::core::ffi::c_void;
pub type IMFTrustedInput = *mut ::core::ffi::c_void;
pub type IMFTrustedOutput = *mut ::core::ffi::c_void;
pub type IMFVideoCaptureSampleAllocator = *mut ::core::ffi::c_void;
pub type IMFVideoDeviceID = *mut ::core::ffi::c_void;
pub type IMFVideoDisplayControl = *mut ::core::ffi::c_void;
pub type IMFVideoMediaType = *mut ::core::ffi::c_void;
pub type IMFVideoMixerBitmap = *mut ::core::ffi::c_void;
pub type IMFVideoMixerControl = *mut ::core::ffi::c_void;
pub type IMFVideoMixerControl2 = *mut ::core::ffi::c_void;
pub type IMFVideoPositionMapper = *mut ::core::ffi::c_void;
pub type IMFVideoPresenter = *mut ::core::ffi::c_void;
pub type IMFVideoProcessor = *mut ::core::ffi::c_void;
pub type IMFVideoProcessorControl = *mut ::core::ffi::c_void;
pub type IMFVideoProcessorControl2 = *mut ::core::ffi::c_void;
pub type IMFVideoProcessorControl3 = *mut ::core::ffi::c_void;
pub type IMFVideoRenderer = *mut ::core::ffi::c_void;
pub type IMFVideoRendererEffectControl = *mut ::core::ffi::c_void;
pub type IMFVideoSampleAllocator = *mut ::core::ffi::c_void;
pub type IMFVideoSampleAllocatorCallback = *mut ::core::ffi::c_void;
pub type IMFVideoSampleAllocatorEx = *mut ::core::ffi::c_void;
pub type IMFVideoSampleAllocatorNotify = *mut ::core::ffi::c_void;
pub type IMFVideoSampleAllocatorNotifyEx = *mut ::core::ffi::c_void;
pub type IMFVirtualCamera = *mut ::core::ffi::c_void;
pub type IMFWorkQueueServices = *mut ::core::ffi::c_void;
pub type IMFWorkQueueServicesEx = *mut ::core::ffi::c_void;
pub type IOPMVideoOutput = *mut ::core::ffi::c_void;
pub type IPlayToControl = *mut ::core::ffi::c_void;
pub type IPlayToControlWithCapabilities = *mut ::core::ffi::c_void;
pub type IPlayToSourceClassFactory = *mut ::core::ffi::c_void;
pub type IToc = *mut ::core::ffi::c_void;
pub type ITocCollection = *mut ::core::ffi::c_void;
pub type ITocEntry = *mut ::core::ffi::c_void;
pub type ITocEntryList = *mut ::core::ffi::c_void;
pub type ITocParser = *mut ::core::ffi::c_void;
pub type IValidateBinding = *mut ::core::ffi::c_void;
pub type IWMCodecLeakyBucket = *mut ::core::ffi::c_void;
pub type IWMCodecOutputTimestamp = *mut ::core::ffi::c_void;
pub type IWMCodecPrivateData = *mut ::core::ffi::c_void;
pub type IWMCodecProps = *mut ::core::ffi::c_void;
pub type IWMCodecStrings = *mut ::core::ffi::c_void;
pub type IWMColorConvProps = *mut ::core::ffi::c_void;
pub type IWMColorLegalizerProps = *mut ::core::ffi::c_void;
pub type IWMFrameInterpProps = *mut ::core::ffi::c_void;
pub type IWMInterlaceProps = *mut ::core::ffi::c_void;
pub type IWMResamplerProps = *mut ::core::ffi::c_void;
pub type IWMResizerProps = *mut ::core::ffi::c_void;
pub type IWMSampleExtensionSupport = *mut ::core::ffi::c_void;
pub type IWMValidate = *mut ::core::ffi::c_void;
pub type IWMVideoDecoderHurryup = *mut ::core::ffi::c_void;
pub type IWMVideoDecoderReconBuffer = *mut ::core::ffi::c_void;
pub type IWMVideoForceKeyFrame = *mut ::core::ffi::c_void;
pub type MFASYNCRESULT = *mut ::core::ffi::c_void;
pub const AACMFTEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x93af0c51_2275_45d2_a35b_f2ba21caed00);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AEC_MAX_SYSTEM_MODES: u32 = 6u32;
pub const ALawCodecWrapper: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36cb6e0c_78c1_42b2_9943_846262f31786);
pub const AMPROPSETID_Pin: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9b00f101_1567_11d1_b3f1_00aa003761c5);
pub const AM_MEDIA_TYPE_REPRESENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe2e42ad2_132c_491e_a268_3c7c2dca181f);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AVENC_H263V_LEVELCOUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AVENC_H264V_LEVELCOUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AVENC_H264V_MAX_MBBITS: u32 = 3200u32;
pub const CAC3DecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x03d7c802_ecfa_47d9_b268_5fb3e310dee4);
pub const CAPTION_FORMAT_ATSC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3ed9cb31_fd10_4ade_bccc_fb9105d2f3ef);
pub const CAPTION_FORMAT_DIRECTV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe9ca1ce7_915e_47be_9bb9_bf1d8a13a5ec);
pub const CAPTION_FORMAT_DVB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x12230db4_ff2a_447e_bb88_6841c416d068);
pub const CAPTION_FORMAT_ECHOSTAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xebb1a262_1158_4b99_ae80_92ac776952c4);
pub const CClusterDetectorDmo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36e820c4_165a_4521_863c_619e1160d4d4);
pub const CColorControlDmo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x798059f0_89ca_4160_b325_aeb48efe4f9a);
pub const CColorConvertDMO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x98230571_0087_4204_b020_3282538e57d3);
pub const CColorLegalizerDmo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfdfaa753_e48e_4e33_9c74_98a27fc6726a);
pub const CDTVAudDecoderDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8e269032_fe03_4753_9b17_18253c21722e);
pub const CDTVVidDecoderDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x64777dc8_4e24_4beb_9d19_60a35be1daaf);
pub const CDVDecoderMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe54709c5_1e17_4c8d_94e7_478940433584);
pub const CDVEncoderMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc82ae729_c327_4cce_914d_8171fefebefb);
pub const CDeColorConvMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x49034c05_f43c_400f_84c1_90a683195a3a);
pub const CFrameInterpDMO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0a7cfe1b_6ab5_4334_9ed8_3f97cb37daa1);
pub const CFrameRateConvertDmo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x01f36ce2_0907_4d8b_979d_f151be91c883);
pub const CInterlaceMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb5a89c80_4901_407b_9abc_90d9a644bb46);
pub const CLSID_ACMWrapper: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6a08cf80_0e18_11cf_a24d_0020afd79767);
pub const CLSID_ATSCNetworkPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe3444d16_5ac4_4386_88df_13fd230e1dda);
pub const CLSID_ATSCNetworkProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0dad2fdd_5fd7_11d3_8f50_00c04f7971e2);
pub const CLSID_AVICo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd76e2820_1563_11cf_ac98_00aa004c0fa9);
pub const CLSID_AVIDec: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcf49d4e0_1115_11ce_b03a_0020af0ba770);
pub const CLSID_AVIDoc: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd3588ab0_0781_11ce_b03a_0020af0ba770);
pub const CLSID_AVIDraw: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa888df60_1e90_11cf_ac98_00aa004c0fa9);
pub const CLSID_AVIMIDIRender: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x07b65360_c445_11ce_afde_00aa006c14f4);
pub const CLSID_ActiveMovieCategories: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xda4e3da0_d07d_11d0_bd50_00a0c911ce86);
pub const CLSID_AllocPresenter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x99d54f63_1a69_41ae_aa4d_c976eb3f0713);
pub const CLSID_AllocPresenterDDXclMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4444ac9e_242e_471b_a3c7_45dcd46352bc);
pub const CLSID_AnalogVideoDecoderPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x71f96466_78f3_11d0_a18c_00a0c9118956);
pub const CLSID_AsyncReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436ebb5_524f_11ce_9f53_0020af0ba770);
pub const CLSID_AudioCompressorCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33d9a761_90c8_11d0_bd43_00a0c911ce86);
pub const CLSID_AudioInputDeviceCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33d9a762_90c8_11d0_bd43_00a0c911ce86);
pub const CLSID_AudioInputMixerProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2ca8ca52_3c3f_11d2_b73d_00c04fb6bd3d);
pub const CLSID_AudioProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05589faf_c356_11ce_bf01_00aa0055595a);
pub const CLSID_AudioRecord: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe30629d2_27e5_11ce_875d_00608cb78066);
pub const CLSID_AudioRender: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe30629d1_27e5_11ce_875d_00608cb78066);
pub const CLSID_AudioRendererAdvancedProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x37e92a92_d9aa_11d2_bf84_8ef2b1555aed);
pub const CLSID_AudioRendererCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe0f158e1_cb04_11d0_bd4e_00a0c911ce86);
pub const CLSID_AudioResamplerMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf447b69e_1884_4a7e_8055_346f74d6edb3);
pub const CLSID_AviDest: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe2510970_f137_11ce_8b67_00aa00a3f1a6);
pub const CLSID_AviMuxProptyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc647b5c0_157c_11d0_bd23_00a0c911ce86);
pub const CLSID_AviMuxProptyPage1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0a9ae910_85c0_11d0_bd42_00a0c911ce86);
pub const CLSID_AviReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b544c21_fd0b_11ce_8c63_00aa0044b51e);
pub const CLSID_AviSplitter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b544c20_fd0b_11ce_8c63_00aa0044b51e);
pub const CLSID_CAcmCoClassManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33d9a761_90c8_11d0_bd43_00a0c911ce86);
pub const CLSID_CAsfTocParser: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9b77c0f2_8735_46c5_b90f_5f0b303ef6ab);
pub const CLSID_CAviTocParser: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3adce5cc_13c8_4573_b328_ed438eb694f9);
pub const CLSID_CCAFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3d07a539_35ca_447c_9b05_8d85ce924f9e);
pub const CLSID_CClusterDetectorEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47354492_827e_4b8a_b318_c80eba1381f0);
pub const CLSID_CDeviceMoniker: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4315d437_5b8c_11d0_bd3b_00a0c911ce86);
pub const CLSID_CFileClient: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbfccd195_1244_4840_ab44_480975c4ffe4);
pub const CLSID_CFileIo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x11993195_1244_4840_ab44_480975c4ffe4);
pub const CLSID_CIcmCoClassManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33d9a760_90c8_11d0_bd43_00a0c911ce86);
pub const CLSID_CMidiOutClassManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4efe2452_168a_11d1_bc76_00c04fb9453b);
pub const CLSID_CMpegAudioCodec: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4a2286e0_7bef_11ce_9bd9_0000e202599c);
pub const CLSID_CMpegVideoCodec: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfeb50740_7bef_11ce_9bd9_0000e202599c);
pub const CLSID_CQzFilterClassManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x083863f1_70de_11d0_bd40_00a0c911ce86);
pub const CLSID_CToc: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4fe24495_28ce_4920_a4c4_e556e1f0df2a);
pub const CLSID_CTocCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5058292d_a244_4840_ab44_480975c4ffe4);
pub const CLSID_CTocEntry: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf22f5e05_585c_4def_8523_6555cfbc0cb3);
pub const CLSID_CTocEntryList: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3a8cccbc_0efd_43a3_b838_f38a552ba237);
pub const CLSID_CTocParser: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x499eaeea_2737_4849_8bb6_47f107eaf358);
pub const CLSID_CVidCapClassManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x860bb310_5d01_11d0_bd3b_00a0c911ce86);
pub const CLSID_CWaveOutClassManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe0f158e1_cb04_11d0_bd4e_00a0c911ce86);
pub const CLSID_CWaveinClassManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33d9a762_90c8_11d0_bd43_00a0c911ce86);
pub const CLSID_CameraControlPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x71f96465_78f3_11d0_a18c_00a0c9118956);
pub const CLSID_CaptionsFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2f7ee4b6_6ff5_4eb4_b24a_2bfc41117171);
pub const CLSID_CaptureGraphBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbf87b6e0_8c27_11d0_b3f0_00aa003761c5);
pub const CLSID_CaptureGraphBuilder2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbf87b6e1_8c27_11d0_b3f0_00aa003761c5);
pub const CLSID_CaptureProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b544c22_fd0b_11ce_8c63_00aa0044b51f);
pub const CLSID_Colour: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1643e180_90f5_11ce_97d5_00aa0055595a);
pub const CLSID_CreateMediaExtensionObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xef65a54d_0788_45b8_8b14_bc0f6a6b5137);
pub const CLSID_CrossbarFilterPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x71f96461_78f3_11d0_a18c_00a0c9118956);
pub const CLSID_DShowTVEFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05500280_faa5_4df9_8246_bfc23ac5cea8);
pub const CLSID_DSoundRender: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x79376820_07d0_11cf_a24d_0020afd79767);
pub const CLSID_DVBCNetworkProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdc0c0fe7_0485_4266_b93f_68fbf80ed834);
pub const CLSID_DVBSNetworkProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfa4b375a_45b4_4d45_8440_263957b11623);
pub const CLSID_DVBTNetworkProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x216c62df_6d7f_4e9a_8571_05f14edb766a);
pub const CLSID_DVDHWDecodersCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2721ae20_7e70_11d0_a5d6_28db04c10000);
pub const CLSID_DVDNavigator: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9b8c4620_2c1a_11d0_8493_00a02438ad48);
pub const CLSID_DVDState: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf963c5cf_a659_4a93_9638_caf3cd277d13);
pub const CLSID_DVDecPropertiesPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x101193c0_0bfe_11d0_af91_00aa00b67a42);
pub const CLSID_DVEncPropertiesPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4150f050_bb6f_11d0_afb9_00aa00b67a42);
pub const CLSID_DVMux: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x129d7e40_c10d_11d0_afb9_00aa00b67a42);
pub const CLSID_DVMuxPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4db880e0_c10d_11d0_afb9_00aa00b67a42);
pub const CLSID_DVSplitter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4eb31670_9fc6_11cf_af6e_00aa00b67a42);
pub const CLSID_DVVideoCodec: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb1b77c00_c3e4_11cf_af79_00aa00b67a42);
pub const CLSID_DVVideoEnc: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x13aa3650_bb6f_11d0_afb9_00aa00b67a42);
pub const CLSID_DeviceControlCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcc7bfb46_f175_11d1_a392_00e0291f3959);
pub const CLSID_DirectDrawProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x944d4c00_dd52_11ce_bf0e_00aa0055595a);
pub const CLSID_DirectShowPluginControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8670c736_f614_427b_8ada_bbadc587194b);
pub const CLSID_Dither: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1da08500_9edc_11cf_bc10_00aa00ac74f6);
pub const CLSID_DtvCcFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb056ba0_2502_45b9_8e86_2b40de84ad29);
pub const CLSID_DvdGraphBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfcc152b7_f372_11d0_8e00_00c04fd7c08b);
pub const CLSID_EVRPlaybackPipelineOptimizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62079164_233b_41f8_a80f_f01705f514a8);
pub const CLSID_EVRTearlessWindowPresenter9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0a7a57b_59b2_4919_a694_add0a526c373);
pub const CLSID_EnhancedVideoRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfa10746c_9b63_4b6c_bc49_fc300ea5f256);
pub const CLSID_FGControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436ebb4_524f_11ce_9f53_0020af0ba770);
pub const CLSID_FileSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x701722e0_8ae3_11ce_a85c_00aa002feab5);
pub const CLSID_FileWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8596e5f0_0da5_11d0_bd21_00a0c911ce86);
pub const CLSID_FilterGraph: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436ebb3_524f_11ce_9f53_0020af0ba770);
pub const CLSID_FilterGraphNoThread: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436ebb8_524f_11ce_9f53_0020af0ba770);
pub const CLSID_FilterGraphPrivateThread: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa3ecbc41_581a_4476_b693_a63340462d8b);
pub const CLSID_FilterMapper: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436ebb2_524f_11ce_9f53_0020af0ba770);
pub const CLSID_FilterMapper2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcda42200_bd88_11d0_bd4e_00a0c911ce86);
pub const CLSID_FrameServerNetworkCameraSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7a213aa7_866f_414a_8c1a_275c7283a395);
pub const CLSID_HttpSchemePlugin: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x44cb442b_9da9_49df_b3fd_023777b16e50);
pub const CLSID_ICodecAPIProxy: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7ff0997a_1999_4286_a73c_622b8814e7eb);
pub const CLSID_IVideoEncoderCodecAPIProxy: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb05dabd9_56e5_4fdc_afa4_8a47e91f1c9c);
pub const CLSID_IVideoEncoderProxy: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb43c4eec_8c32_4791_9102_508ada5ee8e7);
pub const CLSID_InfTee: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf8388a40_d5bb_11d0_be5a_0080c706568e);
pub const CLSID_LegacyAmFilterCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x083863f1_70de_11d0_bd40_00a0c911ce86);
pub const CLSID_Line21Decoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6e8d4a20_310c_11d0_b79a_00aa003767a7);
pub const CLSID_Line21Decoder2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe4206432_01a1_4bee_b3e1_3702c8edc574);
pub const CLSID_MFByteStreamProxyClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x770e8e77_4916_441c_a9a7_b342d0eebc71);
pub const CLSID_MFCaptureEngine: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xefce38d3_8914_4674_a7df_ae1b3d654b8a);
pub const CLSID_MFCaptureEngineClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xefce38d3_8914_4674_a7df_ae1b3d654b8a);
pub const CLSID_MFImageSharingEngineClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb22c3339_87f3_4059_a0c5_037aa9707eaf);
pub const CLSID_MFMediaEngineClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb44392da_499b_446b_a4cb_005fead0e6d5);
pub const CLSID_MFMediaSharingEngineClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf8e307fb_6d45_4ad3_9993_66cd5a529659);
pub const CLSID_MFReadWriteClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48e2ed0f_98c2_4a37_bed5_166312ddd83f);
pub const CLSID_MFSinkWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa3bbfb17_8273_4e52_9e0e_9739dc887990);
pub const CLSID_MFSourceReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1777133c_0881_411b_a577_ad545f0714c4);
pub const CLSID_MFSourceResolver: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x90eab60f_e43a_4188_bcc4_e47fdf04868c);
pub const CLSID_MFVideoMixer9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe474e05a_ab65_4f6a_827c_218b1baaf31f);
pub const CLSID_MFVideoPresenter9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x98455561_5136_4d28_ab08_4cee40ea2781);
pub const CLSID_MJPGEnc: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb80ab0a0_7416_11d2_9eeb_006008039e37);
pub const CLSID_MMSPLITTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3ae86b20_7be8_11d1_abe6_00a0c905f375);
pub const CLSID_MOVReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x44584800_f8ee_11ce_b2d4_00dd01101b85);
pub const CLSID_MP3DecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbbeea841_0a63_4f52_a7ab_a9b3a84ed38a);
pub const CLSID_MPEG1Doc: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe4bbd160_4269_11ce_838d_00aa0055595a);
pub const CLSID_MPEG1PacketPlayer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x26c25940_4ca9_11ce_a828_00aa002feab5);
pub const CLSID_MPEG1Splitter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x336475d0_942a_11ce_a870_00aa002feab5);
pub const CLSID_MPEG2ByteStreamPlugin: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x40871c59_ab40_471f_8dc3_1f259d862479);
pub const CLSID_MPEG2DLNASink: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfa5fe7c5_6a1d_4b11_b41f_f959d6c76500);
pub const CLSID_MPEG2Demultiplexer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xafb6c280_2c41_11d3_8a60_0000f81e0e4a);
pub const CLSID_MPEG2Demultiplexer_NoClock: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x687d3367_3644_467a_adfe_6cd7a85c4a2c);
pub const CLSID_MSAACDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32d186a7_218f_4c75_8876_dd77273a8999);
pub const CLSID_MSDDPlusDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x177c0afe_900b_48d4_9e4c_57add250b3d4);
pub const CLSID_MSH264DecoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62ce7e72_4c71_4d20_b15d_452831a87d9d);
pub const CLSID_MSH264EncoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ca50344_051a_4ded_9779_a43305165e35);
pub const CLSID_MSH265DecoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x420a51a3_d605_430c_b4fc_45274fa6c562);
pub const CLSID_MSMPEGAudDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x70707b39_b2ca_4015_abea_f8447d22d88b);
pub const CLSID_MSMPEGDecoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2d709e52_123f_49b5_9cbc_9af5cde28fb9);
pub const CLSID_MSOpusDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x63e17c10_2d43_4c42_8fe3_8d8b63e46a6a);
pub const CLSID_MSVPxDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe3aaf548_c9a4_4c6e_234d_5ada374b0000);
pub const CLSID_MediaEncoderCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7d22e920_5ca9_4787_8c2b_a6779bd11781);
pub const CLSID_MediaMultiplexerCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x236c9559_adce_4736_bf72_bab34e392196);
pub const CLSID_MediaPropertyBag: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcdbd8d00_c193_11d0_bd4e_00a0c911ce86);
pub const CLSID_MemoryAllocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1e651cc0_b199_11d0_8212_00c04fc32c45);
pub const CLSID_MidiRendererCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4efe2452_168a_11d1_bc76_00c04fb9453b);
pub const CLSID_MjpegDec: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x301056d0_6dff_11d2_9eeb_006008039e37);
pub const CLSID_ModexRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x07167665_5011_11cf_bf33_00aa0055595a);
pub const CLSID_Mpeg2VideoStreamAnalyzer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6cfad761_735d_4aa5_8afc_af91a7d61eba);
pub const CLSID_NetSchemePlugin: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe9f4ebab_d97b_463e_a2b1_c54ee3f9414d);
pub const CLSID_NetworkProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb2f3a67c_29da_4c78_8831_091ed509a475);
pub const CLSID_OverlayMixer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcd8743a1_3736_11d0_9e69_00c04fd7c15b);
pub const CLSID_PerformanceProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x59ce6880_acf8_11cf_b56e_0080c7c4b68a);
pub const CLSID_PersistMonikerPID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436ebb7_524f_11ce_9f53_0020af0ba770);
pub const CLSID_PlayToSourceClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xda17539a_3dc3_42c1_a749_a183b51f085e);
pub const CLSID_ProtoFilterGraph: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436ebb0_524f_11ce_9f53_0020af0ba770);
pub const CLSID_QTDec: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfdfe9681_74a3_11d0_afa7_00aa00b67a42);
pub const CLSID_QualityProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x418afb70_f8b8_11ce_aac6_0020af0b99a3);
pub const CLSID_QuickTimeParser: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd51bd5a0_7548_11cf_a520_0080c77ef58a);
pub const CLSID_SBE2File: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x93a094d7_51e8_485b_904a_8d6b97dc6b39);
pub const CLSID_SBE2FileScan: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3e458037_0ca6_41aa_a594_2aa6c02d709b);
pub const CLSID_SBE2MediaTypeProfile: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1f26a602_2b5c_4b63_b8e8_9ea5c1a7dc2e);
pub const CLSID_SBE2Sink: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe2448508_95da_4205_9a27_7ec81e723b1a);
pub const CLSID_SeekingPassThru: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x060af76c_68dd_11d0_8fc1_00c04fd9189d);
pub const CLSID_SmartTee: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcc58e280_8aa1_11d1_b3f1_00aa003761c5);
pub const CLSID_StreamBufferComposeRecording: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd682c4ba_a90a_42fe_b9e1_03109849c423);
pub const CLSID_StreamBufferConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfa8a68b2_c864_4ba2_ad53_d3876a87494b);
pub const CLSID_StreamBufferPropertyHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe37a73f8_fb01_43dc_914e_aaee76095ab9);
pub const CLSID_StreamBufferRecordingAttributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xccaa63ac_1057_4778_ae92_1206ab9acee6);
pub const CLSID_StreamBufferSink: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2db47ae5_cf39_43c2_b4d6_0cd8d90946f4);
pub const CLSID_StreamBufferSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc9f5fe02_f851_4eb5_99ee_ad602af1e619);
pub const CLSID_StreamBufferThumbnailHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x713790ee_5ee1_45ba_8070_a1337d2762fa);
pub const CLSID_SubtitlesFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9f22cfea_ce07_41ab_8ba0_c7364af90af9);
pub const CLSID_SystemClock: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436ebb1_524f_11ce_9f53_0020af0ba770);
pub const CLSID_SystemDeviceEnum: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62be5d10_60eb_11d0_bd3b_00a0c911ce86);
pub const CLSID_TVAudioFilterPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x71f96463_78f3_11d0_a18c_00a0c9118956);
pub const CLSID_TVEFilterCCProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05500282_faa5_4df9_8246_bfc23ac5cea8);
pub const CLSID_TVEFilterStatsProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05500283_faa5_4df9_8246_bfc23ac5cea8);
pub const CLSID_TVEFilterTuneProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05500281_faa5_4df9_8246_bfc23ac5cea8);
pub const CLSID_TVTunerFilterPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x266eee41_6c63_11cf_8a03_00aa006ecb65);
pub const CLSID_TextRender: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe30629d3_27e5_11ce_875d_00608cb78066);
pub const CLSID_TransmitCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcc7bfb41_f175_11d1_a392_00e0291f3959);
pub const CLSID_URLReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436ebb6_524f_11ce_9f53_0020af0ba770);
pub const CLSID_UrlmonSchemePlugin: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9ec4b4f9_3029_45ad_947b_344de2a249e2);
pub const CLSID_VBISurfaces: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x814b9800_1c88_11d1_bad9_00609744111a);
pub const CLSID_VPObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xce292861_fc88_11d0_9e69_00c04fd7c15b);
pub const CLSID_VPVBIObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x814b9801_1c88_11d1_bad9_00609744111a);
pub const CLSID_VfwCapture: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b544c22_fd0b_11ce_8c63_00aa0044b51e);
pub const CLSID_VideoCompressorCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33d9a760_90c8_11d0_bd43_00a0c911ce86);
pub const CLSID_VideoInputDeviceCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x860bb310_5d01_11d0_bd3b_00a0c911ce86);
pub const CLSID_VideoMixingRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb87beb7b_8d29_423f_ae4d_6582c10175ac);
pub const CLSID_VideoMixingRenderer9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x51b4abf3_748f_4e3b_a276_c828330e926a);
pub const CLSID_VideoPortManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6f26a6cd_967b_47fd_874a_7aed2c9d25a2);
pub const CLSID_VideoProcAmpPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x71f96464_78f3_11d0_a18c_00a0c9118956);
pub const CLSID_VideoProcessorMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x88753b26_5b24_49bd_b2e7_0c445c78c982);
pub const CLSID_VideoRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x70e102b0_5556_11ce_97c0_00aa0055595a);
pub const CLSID_VideoRendererDefault: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6bc1cffa_8fc1_4261_ac22_cfb4cc38db50);
pub const CLSID_VideoStreamConfigPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x71f96467_78f3_11d0_a18c_00a0c9118956);
pub const CLSID_WMADecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2eeb4adf_4578_4d10_bca7_bb955f56320a);
pub const CLSID_WMAsfReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x187463a0_5bb7_11d3_acbe_0080c75e246e);
pub const CLSID_WMAsfWriter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7c23220e_55bb_11d3_8b16_00c04fb6bd3d);
pub const CLSID_WMDRMSystemID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8948bb22_11bd_4796_93e3_974d1b575678);
pub const CLSID_WMVDecoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x82d353df_90bd_4382_8bc2_3f6192b76e34);
pub const CLSID_WSTDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x70bc06e0_5666_11d3_a184_00105aef9f33);
pub const CLSID_WstDecoderPropertyPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x04e27f80_91e4_11d3_a184_00105aef9f33);
pub const CMP3DecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbbeea841_0a63_4f52_a7ab_a9b3a84ed38a);
pub const CMPEG2AudDecoderDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe1f1a0b8_beee_490d_ba7c_066c40b5e2b9);
pub const CMPEG2AudioEncoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x46a4dd5c_73f8_4304_94df_308f760974f4);
pub const CMPEG2EncoderAudioDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xacd453bc_c58a_44d1_bbf5_bfb325be2d78);
pub const CMPEG2EncoderDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5f5aff4a_2f7f_4279_88c2_cd88eb39d144);
pub const CMPEG2EncoderVideoDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x42150cd9_ca9a_4ea5_9939_30ee037f6e74);
pub const CMPEG2VidDecoderDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x212690fb_83e5_4526_8fd7_74478b7939cd);
pub const CMPEG2VideoEncoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe6335f02_80b7_4dc4_adfa_dfe7210d20d5);
pub const CMPEGAACDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8dde1772_edad_41c3_b4be_1f30fb4ee0d6);
pub const CMSAACDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32d186a7_218f_4c75_8876_dd77273a8999);
pub const CMSAC3Enc: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc6b400e2_20a7_4e58_a2fe_24619682ce6c);
pub const CMSALACDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc0cd7d12_31fc_4bbc_b363_7322ee3e1879);
pub const CMSALACEncMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9ab6a28c_748e_4b6a_bfff_cc443b8e8fb4);
pub const CMSDDPlusDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x177c0afe_900b_48d4_9e4c_57add250b3d4);
pub const CMSDolbyDigitalEncMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac3315c9_f481_45d7_826c_0b406c1f64b8);
pub const CMSFLACDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6b0b3e6b_a2c5_4514_8055_afe8a95242d9);
pub const CMSFLACEncMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x128509e9_c44e_45dc_95e9_c255b8f466a6);
pub const CMSH263EncoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc47fcfe_98a0_4f27_bb07_698af24f2b38);
pub const CMSH264DecoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62ce7e72_4c71_4d20_b15d_452831a87d9d);
pub const CMSH264EncoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ca50344_051a_4ded_9779_a43305165e35);
pub const CMSH264RemuxMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05a47ebb_8bf0_4cbf_ad2f_3b71d75866f5);
pub const CMSH265EncoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2f84074_8bca_40bd_9159_e880f673dd3b);
pub const CMSMPEGAudDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x70707b39_b2ca_4015_abea_f8447d22d88b);
pub const CMSMPEGDecoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2d709e52_123f_49b5_9cbc_9af5cde28fb9);
pub const CMSOpusDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x63e17c10_2d43_4c42_8fe3_8d8b63e46a6a);
pub const CMSSCDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7bafb3b1_d8f4_4279_9253_27da423108de);
pub const CMSSCEncMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8cb9cc06_d139_4ae6_8bb4_41e612e141d5);
pub const CMSSCEncMediaObject2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf7ffe0a0_a4f5_44b5_949e_15ed2bc66f9d);
pub const CMSVPXEncoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaeb6c755_2546_4881_82cc_e15ae5ebff3d);
pub const CMSVideoDSPMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x51571744_7fe4_4ff2_a498_2dc34ff74f1b);
pub const CMpeg2DecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x863d66cd_cdce_4617_b47f_c8929cfc28a6);
pub const CMpeg43DecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcba9e78b_49a3_49ea_93d4_6bcba8c4de07);
pub const CMpeg4DecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf371728a_6052_4d47_827c_d039335dfe0a);
pub const CMpeg4EncMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x24f258d8_c651_4042_93e4_ca654abb682c);
pub const CMpeg4sDecMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5686a0d9_fe39_409f_9dff_3fdbc849f9f5);
pub const CMpeg4sDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a11bae2_fe6e_4249_864b_9e9ed6e8dbc2);
pub const CMpeg4sEncMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ec5a7be_d81e_4f9e_ada3_cd1bf262b6d8);
pub const CNokiaAACCCDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeabf7a6f_ccba_4d60_8620_b152cc977263);
pub const CNokiaAACDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb2bde4_4e29_4c44_a73e_2d7c2c46d6ec);
pub const CODECAPI_ALLSETTINGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6a577e92_83e1_4113_adc2_4fcec32f83a1);
pub const CODECAPI_AUDIO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb9d19a3e_f897_429c_bc46_8138b7272b2d);
pub const CODECAPI_AVAudioChannelConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x17f89cb3_c38d_4368_9ede_63b94d177f9f);
pub const CODECAPI_AVAudioChannelCount: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1d3583c4_1583_474e_b71a_5ee463c198e4);
pub const CODECAPI_AVAudioSampleRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x971d2723_1acb_42e7_855c_520a4b70a5f2);
pub const CODECAPI_AVDDSurroundMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x99f2f386_98d1_4452_a163_abc78a6eb770);
pub const CODECAPI_AVDSPLoudnessEqualization: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8afd1a15_1812_4cbf_9319_433a5b2a3b27);
pub const CODECAPI_AVDSPSpeakerFill: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5612bca1_56da_4582_8da1_ca8090f92768);
pub const CODECAPI_AVDecAACDownmixMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x01274475_f6bb_4017_b084_81a763c942d4);
pub const CODECAPI_AVDecAudioDualMono: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4a52cda8_30f8_4216_be0f_ba0b2025921d);
pub const CODECAPI_AVDecAudioDualMonoReproMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa5106186_cc94_4bc9_8cd9_aa2f61f6807e);
pub const CODECAPI_AVDecCommonInputFormat: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe5005239_bd89_4be3_9c0f_5dde317988cc);
pub const CODECAPI_AVDecCommonMeanBitRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x59488217_007a_4f7a_8e41_5c48b1eac5c6);
pub const CODECAPI_AVDecCommonMeanBitRateInterval: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0ee437c6_38a7_4c5c_944c_68ab42116b85);
pub const CODECAPI_AVDecCommonOutputFormat: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3c790028_c0ce_4256_b1a2_1b0fc8b1dcdc);
pub const CODECAPI_AVDecDDDynamicRangeScaleHigh: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50196c21_1f33_4af5_b296_11426d6c8789);
pub const CODECAPI_AVDecDDDynamicRangeScaleLow: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x044e62e4_11a5_42d5_a3b2_3bb2c7c2d7cf);
pub const CODECAPI_AVDecDDMatrixDecodingMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xddc811a5_04ed_4bf3_a0ca_d00449f9355f);
pub const CODECAPI_AVDecDDOperationalMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd6d6c6d1_064e_4fdd_a40e_3ecbfcb7ebd0);
pub const CODECAPI_AVDecDDStereoDownMixMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ce4122c_3ee9_4182_b4ae_c10fc088649d);
pub const CODECAPI_AVDecDisableVideoPostProcessing: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf8749193_667a_4f2c_a9e8_5d4af924f08f);
pub const CODECAPI_AVDecHEAACDynamicRangeControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x287c8abe_69a4_4d39_8080_d3d9712178a0);
pub const CODECAPI_AVDecMmcssClass: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe0ad4828_df66_4893_9f33_788aa4ec4082);
pub const CODECAPI_AVDecNumWorkerThreads: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9561c3e8_ea9e_4435_9b1e_a93e691894d8);
pub const CODECAPI_AVDecSoftwareDynamicFormatChange: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x862e2f0a_507b_47ff_af47_01e2624298b7);
pub const CODECAPI_AVDecVideoAcceleration_H264: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf7db8a2f_4f48_4ee8_ae31_8b6ebe558ae2);
pub const CODECAPI_AVDecVideoAcceleration_MPEG2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf7db8a2e_4f48_4ee8_ae31_8b6ebe558ae2);
pub const CODECAPI_AVDecVideoAcceleration_VC1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf7db8a30_4f48_4ee8_ae31_8b6ebe558ae2);
pub const CODECAPI_AVDecVideoCodecType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x434528e5_21f0_46b6_b62c_9b1b6b658cd1);
pub const CODECAPI_AVDecVideoDXVABusEncryption: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x42153c8b_fd0b_4765_a462_ddd9e8bcc388);
pub const CODECAPI_AVDecVideoDXVAMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf758f09e_7337_4ae7_8387_73dc2d54e67d);
pub const CODECAPI_AVDecVideoDropPicWithMissingRef: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf8226383_14c2_4567_9734_5004e96ff887);
pub const CODECAPI_AVDecVideoFastDecodeMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6b529f7d_d3b1_49c6_a999_9ec6911bedbf);
pub const CODECAPI_AVDecVideoH264ErrorConcealment: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xececace8_3436_462c_9294_cd7bacd758a9);
pub const CODECAPI_AVDecVideoImageSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5ee5747c_6801_4cab_aaf1_6248fa841ba4);
pub const CODECAPI_AVDecVideoInputScanType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x38477e1f_0ea7_42cd_8cd1_130ced57c580);
pub const CODECAPI_AVDecVideoMPEG2ErrorConcealment: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d2bfe18_728d_48d2_b358_bc7e436c6674);
pub const CODECAPI_AVDecVideoMaxCodedHeight: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7262a16a_d2dc_4e75_9ba8_65c0c6d32b13);
pub const CODECAPI_AVDecVideoMaxCodedWidth: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5ae557b8_77af_41f5_9fa6_4db2fe1d4bca);
pub const CODECAPI_AVDecVideoPixelAspectRatio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb0cf8245_f32d_41df_b02c_87bd304d12ab);
pub const CODECAPI_AVDecVideoProcDeinterlaceCSC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf7db8a31_4f48_4ee8_ae31_8b6ebe558ae2);
pub const CODECAPI_AVDecVideoSWPowerLevel: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb5d2347_4dd8_4509_aed0_db5fa9aa93f4);
pub const CODECAPI_AVDecVideoSoftwareDeinterlaceMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0c08d1ce_9ced_4540_bae3_ceb380141109);
pub const CODECAPI_AVDecVideoThumbnailGenerationMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2efd8eee_1150_4328_9cf5_66dce933fcf4);
pub const CODECAPI_AVEnableInLoopDeblockFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd2e8e399_0623_4bf3_92a8_4d1818529ded);
pub const CODECAPI_AVEncAdaptiveMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4419b185_da1f_4f53_bc76_097d0c1efb1e);
pub const CODECAPI_AVEncAudioDualMono: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3648126b_a3e8_4329_9b3a_5ce566a43bd3);
pub const CODECAPI_AVEncAudioInputContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3e226c2b_60b9_4a39_b00b_a7b40f70d566);
pub const CODECAPI_AVEncAudioIntervalToEncode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x866e4b4d_725a_467c_bb01_b496b23b25f9);
pub const CODECAPI_AVEncAudioIntervalToSkip: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x88c15f94_c38c_4796_a9e8_96e967983f26);
pub const CODECAPI_AVEncAudioMapDestChannel0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b60_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b61_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b6a_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel11: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b6b_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel12: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b6c_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel13: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b6d_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel14: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b6e_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel15: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b6f_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b62_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b63_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b64_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel5: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b65_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel6: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b66_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel7: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b67_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel8: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b68_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMapDestChannel9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc5d0b69_df6a_4e16_9803_b82007a30c8d);
pub const CODECAPI_AVEncAudioMeanBitRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x921295bb_4fca_4679_aab8_9e2a1d753384);
pub const CODECAPI_AVEncChromaEncodeMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8a47ab5a_4798_4c93_b5a5_554f9a3b9f50);
pub const CODECAPI_AVEncChromaUpdateTime: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4b4fd998_4274_40bb_8ee4_07553e7e2d3a);
pub const CODECAPI_AVEncCodecType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x08af4ac1_f3f2_4c74_9dcf_37f2ec79f826);
pub const CODECAPI_AVEncCommonAllowFrameDrops: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd8477dcb_9598_48e3_8d0c_752bf206093e);
pub const CODECAPI_AVEncCommonBufferInLevel: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd9c5c8db_fc74_4064_94e9_cd19f947ed45);
pub const CODECAPI_AVEncCommonBufferOutLevel: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xccae7f49_d0bc_4e3d_a57e_fb5740140069);
pub const CODECAPI_AVEncCommonBufferSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0db96574_b6a4_4c8b_8106_3773de0310cd);
pub const CODECAPI_AVEncCommonFormatConstraint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cbb9b8_116f_4951_b40c_c2a035ed8f17);
pub const CODECAPI_AVEncCommonLowLatency: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d3ecd55_89e8_490a_970a_0c9548d5a56e);
pub const CODECAPI_AVEncCommonMaxBitRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9651eae4_39b9_4ebf_85ef_d7f444ec7465);
pub const CODECAPI_AVEncCommonMeanBitRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf7222374_2144_4815_b550_a37f8e12ee52);
pub const CODECAPI_AVEncCommonMeanBitRateInterval: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbfaa2f0c_cb82_4bc0_8474_f06a8a0d0258);
pub const CODECAPI_AVEncCommonMinBitRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x101405b2_2083_4034_a806_efbeddd7c9ff);
pub const CODECAPI_AVEncCommonMultipassMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x22533d4c_47e1_41b5_9352_a2b7780e7ac4);
pub const CODECAPI_AVEncCommonPassEnd: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0e3d01bc_c85c_467d_8b60_c41012ee3bf6);
pub const CODECAPI_AVEncCommonPassStart: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6a67739f_4eb5_4385_9928_f276a939ef95);
pub const CODECAPI_AVEncCommonQuality: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfcbf57a3_7ea5_4b0c_9644_69b40c39c391);
pub const CODECAPI_AVEncCommonQualityVsSpeed: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x98332df8_03cd_476b_89fa_3f9e442dec9f);
pub const CODECAPI_AVEncCommonRateControlMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1c0608e9_370c_4710_8a58_cb6181c42423);
pub const CODECAPI_AVEncCommonRealTime: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x143a0ff6_a131_43da_b81e_98fbb8ec378e);
pub const CODECAPI_AVEncCommonStreamEndHandling: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6aad30af_6ba8_4ccc_8fca_18d19beaeb1c);
pub const CODECAPI_AVEncCommonTranscodeEncodingProfile: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6947787c_f508_4ea9_b1e9_a1fe3a49fbc9);
pub const CODECAPI_AVEncDDAtoDConverterType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x719f9612_81a1_47e0_9a05_d94ad5fca948);
pub const CODECAPI_AVEncDDCentreDownMixLevel: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe285072c_c958_4a81_afd2_e5e0daf1b148);
pub const CODECAPI_AVEncDDChannelBWLowPassFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe197821d_d2e7_43e2_ad2c_00582f518545);
pub const CODECAPI_AVEncDDCopyright: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8694f076_cd75_481d_a5c6_a904dcc828f0);
pub const CODECAPI_AVEncDDDCHighPassFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9565239f_861c_4ac8_bfda_e00cb4db8548);
pub const CODECAPI_AVEncDDDialogNormalization: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd7055acf_f125_437d_a704_79c79f0404a8);
pub const CODECAPI_AVEncDDDigitalDeemphasis: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe024a2c2_947c_45ac_87d8_f1030c5c0082);
pub const CODECAPI_AVEncDDDynamicRangeCompressionControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcfc2ff6d_79b8_4b8d_a8aa_a0c9bd1c2940);
pub const CODECAPI_AVEncDDHeadphoneMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4052dbec_52f5_42f5_9b00_d134b1341b9d);
pub const CODECAPI_AVEncDDLFELowPassFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd3b80f6f_9d15_45e5_91be_019c3fab1f01);
pub const CODECAPI_AVEncDDLoRoCenterMixLvl_x10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1cfba222_25b3_4bf4_9bfd_e7111267858c);
pub const CODECAPI_AVEncDDLoRoSurroundMixLvl_x10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe725cff6_eb56_40c7_8450_2b9367e91555);
pub const CODECAPI_AVEncDDLtRtCenterMixLvl_x10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdca128a2_491f_4600_b2da_76e3344b4197);
pub const CODECAPI_AVEncDDLtRtSurroundMixLvl_x10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x212246c7_3d2c_4dfa_bc21_652a9098690d);
pub const CODECAPI_AVEncDDOriginalBitstream: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x966ae800_5bd3_4ff9_95b9_d30566273856);
pub const CODECAPI_AVEncDDPreferredStereoDownMixMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7f4e6b31_9185_403d_b0a2_763743e6f063);
pub const CODECAPI_AVEncDDProductionInfoExists: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb0b7fe5f_b6ab_4f40_964d_8d91f17c19e8);
pub const CODECAPI_AVEncDDProductionMixLevel: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x301d103a_cbf9_4776_8899_7c15b461ab26);
pub const CODECAPI_AVEncDDProductionRoomType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdad7ad60_23d8_4ab7_a284_556986d8a6fe);
pub const CODECAPI_AVEncDDRFPreEmphasisFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x21af44c0_244e_4f3d_a2cc_3d3068b2e73f);
pub const CODECAPI_AVEncDDService: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd2e1bec7_5172_4d2a_a50e_2f3b82b1ddf8);
pub const CODECAPI_AVEncDDSurround3dBAttenuation: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4d43b99d_31e2_48b9_bf2e_5cbf1a572784);
pub const CODECAPI_AVEncDDSurround90DegreeePhaseShift: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x25ecec9d_3553_42c0_bb56_d25792104f80);
pub const CODECAPI_AVEncDDSurroundDownMixLevel: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b20d6e5_0bcf_4273_a487_506b047997e9);
pub const CODECAPI_AVEncDDSurroundExMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x91607cee_dbdd_4eb6_bca2_aadfafa3dd68);
pub const CODECAPI_AVEncEnableVideoProcessing: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x006f4bf6_0ea3_4d42_8702_b5d8be0f7a92);
pub const CODECAPI_AVEncH264CABACEnable: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xee6cad62_d305_4248_a50e_e1b255f7caf8);
pub const CODECAPI_AVEncH264PPSID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbfe29ec2_056c_4d68_a38d_ae5944c8582e);
pub const CODECAPI_AVEncH264SPSID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50f38f51_2b79_40e3_b39c_7e9fa0770501);
pub const CODECAPI_AVEncInputVideoSystem: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbede146d_b616_4dc7_92b2_f5d9fa9298f7);
pub const CODECAPI_AVEncLowPowerEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb668d582_8bad_4f6a_9141_375a95358b6d);
pub const CODECAPI_AVEncMP12MuxDVDNavPacks: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc7607ced_8cf1_4a99_83a1_ee5461be3574);
pub const CODECAPI_AVEncMP12MuxEarliestPTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x157232b6_f809_474e_9464_a7f93014a817);
pub const CODECAPI_AVEncMP12MuxInitialSCR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3433ad21_1b91_4a0b_b190_2b77063b63a4);
pub const CODECAPI_AVEncMP12MuxLargestPacketSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x35ceb711_f461_4b92_a4ef_17b6841ed254);
pub const CODECAPI_AVEncMP12MuxMuxRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xee047c72_4bdb_4a9d_8e21_41926c823da7);
pub const CODECAPI_AVEncMP12MuxNumStreams: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf7164a41_dced_4659_a8f2_fb693f2a4cd0);
pub const CODECAPI_AVEncMP12MuxPackSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf916053a_1ce8_4faf_aa0b_ba31c80034b8);
pub const CODECAPI_AVEncMP12MuxPacketOverhead: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe40bd720_3955_4453_acf9_b79132a38fa0);
pub const CODECAPI_AVEncMP12MuxSysAudioLock: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0fbb5752_1d43_47bf_bd79_f2293d8ce337);
pub const CODECAPI_AVEncMP12MuxSysCSPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7952ff45_9c0d_4822_bc82_8ad772e02993);
pub const CODECAPI_AVEncMP12MuxSysFixed: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcefb987e_894f_452e_8f89_a4ef8cec063a);
pub const CODECAPI_AVEncMP12MuxSysRateBound: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05f0428a_ee30_489d_ae28_205c72446710);
pub const CODECAPI_AVEncMP12MuxSysSTDBufferBound: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x35746903_b545_43e7_bb35_c5e0a7d5093c);
pub const CODECAPI_AVEncMP12MuxSysVideoLock: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb8296408_2430_4d37_a2a1_95b3e435a91d);
pub const CODECAPI_AVEncMP12MuxTargetPacketizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd862212a_2015_45dd_9a32_1b3aa88205a0);
pub const CODECAPI_AVEncMP12PktzCopyright: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc8f4b0c1_094c_43c7_8e68_a595405a6ef8);
pub const CODECAPI_AVEncMP12PktzInitialPTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a4f2065_9a63_4d20_ae22_0a1bc896a315);
pub const CODECAPI_AVEncMP12PktzOriginal: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6b178416_31b9_4964_94cb_6bff866cdf83);
pub const CODECAPI_AVEncMP12PktzPacketSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xab71347a_1332_4dde_a0e5_ccf7da8a0f22);
pub const CODECAPI_AVEncMP12PktzSTDBuffer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0b751bd0_819e_478c_9435_75208926b377);
pub const CODECAPI_AVEncMP12PktzStreamID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc834d038_f5e8_4408_9b60_88f36493fedf);
pub const CODECAPI_AVEncMPACodingMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb16ade03_4b93_43d7_a550_90b4fe224537);
pub const CODECAPI_AVEncMPACopyright: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa6ae762a_d0a9_4454_b8ef_f2dbeefdd3bd);
pub const CODECAPI_AVEncMPAEmphasisType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2d59fcda_bf4e_4ed6_b5df_5b03b36b0a1f);
pub const CODECAPI_AVEncMPAEnableRedundancyProtection: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5e54b09e_b2e7_4973_a89b_0b3650a3beda);
pub const CODECAPI_AVEncMPALayer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d377230_f91b_453d_9ce0_78445414c22d);
pub const CODECAPI_AVEncMPAOriginalBitstream: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cfb7855_9cc9_47ff_b829_b36786c92346);
pub const CODECAPI_AVEncMPAPrivateUserBit: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xafa505ce_c1e3_4e3d_851b_61b700e5e6cc);
pub const CODECAPI_AVEncMPVAddSeqEndCode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa823178f_57df_4c7a_b8fd_e5ec8887708d);
pub const CODECAPI_AVEncMPVDefaultBPictureCount: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8d390aac_dc5c_4200_b57f_814d04babab2);
pub const CODECAPI_AVEncMPVFrameFieldMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xacb5de96_7b93_4c2f_8825_b0295fa93bf4);
pub const CODECAPI_AVEncMPVGOPOpen: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb1d5d4a6_3300_49b1_ae61_a09937ab0e49);
pub const CODECAPI_AVEncMPVGOPSInSeq: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x993410d4_2691_4192_9978_98dc2603669f);
pub const CODECAPI_AVEncMPVGOPSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x95f31b26_95a4_41aa_9303_246a7fc6eef1);
pub const CODECAPI_AVEncMPVGOPSizeMax: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfe7de4c4_1936_4fe2_bdf7_1f18ca1d001f);
pub const CODECAPI_AVEncMPVGOPSizeMin: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7155cf20_d440_4852_ad0f_9c4abfe37a6a);
pub const CODECAPI_AVEncMPVGenerateHeaderPicDispExt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc6412f84_c03f_4f40_a00c_4293df8395bb);
pub const CODECAPI_AVEncMPVGenerateHeaderPicExt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b8464ab_944f_45f0_b74e_3a58dad11f37);
pub const CODECAPI_AVEncMPVGenerateHeaderSeqDispExt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6437aa6f_5a3c_4de9_8a16_53d9c4ad326f);
pub const CODECAPI_AVEncMPVGenerateHeaderSeqExt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd5e78611_082d_4e6b_98af_0f51ab139222);
pub const CODECAPI_AVEncMPVGenerateHeaderSeqScaleExt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0722d62f_dd59_4a86_9cd5_644f8e2653d8);
pub const CODECAPI_AVEncMPVIntraDCPrecision: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0116151_cbc8_4af3_97dc_d00cceb82d79);
pub const CODECAPI_AVEncMPVIntraVLCTable: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa2b83ff5_1a99_405a_af95_c5997d558d3a);
pub const CODECAPI_AVEncMPVLevel: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ee40c40_a60c_41ef_8f50_37c2249e2cb3);
pub const CODECAPI_AVEncMPVProfile: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdabb534a_1d99_4284_975a_d90e2239baa1);
pub const CODECAPI_AVEncMPVQScaleType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2b79ebb7_f484_4af7_bb58_a2a188c5cbbe);
pub const CODECAPI_AVEncMPVQuantMatrixChromaIntra: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9eb9ecd4_018d_4ffd_8f2d_39e49f07b17a);
pub const CODECAPI_AVEncMPVQuantMatrixChromaNonIntra: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1415b6b1_362a_4338_ba9a_1ef58703c05b);
pub const CODECAPI_AVEncMPVQuantMatrixIntra: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9bea04f3_6621_442c_8ba1_3ac378979698);
pub const CODECAPI_AVEncMPVQuantMatrixNonIntra: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x87f441d8_0997_4beb_a08e_8573d409cf75);
pub const CODECAPI_AVEncMPVScanPattern: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7f8a478e_7bbb_4ae2_b2fc_96d17fc4a2d6);
pub const CODECAPI_AVEncMPVSceneDetection: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x552799f1_db4c_405b_8a3a_c93f2d0674dc);
pub const CODECAPI_AVEncMPVUseConcealmentMotionVectors: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xec770cf3_6908_4b4b_aa30_7fb986214fea);
pub const CODECAPI_AVEncMaxFrameRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb98e1b31_19fa_4d4f_9931_d6a5b8aab93c);
pub const CODECAPI_AVEncMuxOutputStreamType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcedd9e8f_34d3_44db_a1d8_f81520254f3e);
pub const CODECAPI_AVEncNoInputCopy: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd2b46a2a_e8ee_4ec5_869e_449b6c62c81a);
pub const CODECAPI_AVEncNumWorkerThreads: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb0c8bf60_16f7_4951_a30b_1db1609293d6);
pub const CODECAPI_AVEncProgressiveUpdateTime: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x649faf66_afc6_4828_8fdc_0771cd9ab17d);
pub const CODECAPI_AVEncSliceControlMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe9e782ef_5f18_44c9_a90b_e9c3c2c17b0b);
pub const CODECAPI_AVEncSliceControlSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x92f51df3_07a5_4172_aefe_c69ca3b60e35);
pub const CODECAPI_AVEncSliceGenerationMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8a6bc67f_9497_4286_b46b_02db8d60edbc);
pub const CODECAPI_AVEncStatAudioAverageBPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xca6724db_7059_4351_8b43_f82198826a14);
pub const CODECAPI_AVEncStatAudioAveragePCMValue: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x979272f8_d17f_4e32_bb73_4e731c68ba2d);
pub const CODECAPI_AVEncStatAudioPeakPCMValue: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdce7fd34_dc00_4c16_821b_35d9eb00fb1a);
pub const CODECAPI_AVEncStatAverageBPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xca6724db_7059_4351_8b43_f82198826a14);
pub const CODECAPI_AVEncStatCommonCompletedPasses: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3e5de533_9df7_438c_854f_9f7dd3683d34);
pub const CODECAPI_AVEncStatHardwareBandwidthUtilitization: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0124ba9b_dc41_4826_b45f_18ac01b3d5a8);
pub const CODECAPI_AVEncStatHardwareProcessorUtilitization: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x995dc027_cb95_49e6_b91b_5967753cdcb8);
pub const CODECAPI_AVEncStatMPVSkippedEmptyFrames: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32195fd3_590d_4812_a7ed_6d639a1f9711);
pub const CODECAPI_AVEncStatVideoCodedFrames: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd47f8d61_6f5a_4a26_bb9f_cd9518462bcd);
pub const CODECAPI_AVEncStatVideoOutputFrameRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbe747849_9ab4_4a63_98fe_f143f04f8ee9);
pub const CODECAPI_AVEncStatVideoTotalFrames: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfdaa9916_119a_4222_9ad6_3f7cab99cc8b);
pub const CODECAPI_AVEncStatWMVCBAvg: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6aa6229f_d602_4b9d_b68c_c1ad78884bef);
pub const CODECAPI_AVEncStatWMVCBMax: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe976bef8_00fe_44b4_b625_8f238bc03499);
pub const CODECAPI_AVEncStatWMVDecoderComplexityProfile: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x89e69fc3_0f9b_436c_974a_df821227c90d);
pub const CODECAPI_AVEncVideoCBRMotionTradeoff: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0d49451e_18d5_4367_a4ef_3240df1693c4);
pub const CODECAPI_AVEncVideoCTBSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd47db8b2_e73b_4cb9_8c3e_bd877d06d77b);
pub const CODECAPI_AVEncVideoCodedVideoAccessUnitSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb4b10c15_14a7_4ce8_b173_dc90a0b4fcdb);
pub const CODECAPI_AVEncVideoContentType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x66117aca_eb77_459d_930c_a48d9d0683fc);
pub const CODECAPI_AVEncVideoDefaultUpperFieldDominant: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x810167c4_0bc1_47ca_8fc2_57055a1474a5);
pub const CODECAPI_AVEncVideoDirtyRectEnabled: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8acb8fdd_5e0c_4c66_8729_b8f629ab04fb);
pub const CODECAPI_AVEncVideoDisplayDimension: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xde053668_f4ec_47a9_86d0_836770f0c1d5);
pub const CODECAPI_AVEncVideoEncodeDimension: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1074df28_7e0f_47a4_a453_cdd73870f5ce);
pub const CODECAPI_AVEncVideoEncodeFrameTypeQP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaa70b610_e03f_450c_ad07_07314e639ce7);
pub const CODECAPI_AVEncVideoEncodeOffsetOrigin: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6bc098fe_a71a_4454_852e_4d2ddeb2cd24);
pub const CODECAPI_AVEncVideoEncodeQP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2cb5696b_23fb_4ce1_a0f9_ef5b90fd55ca);
pub const CODECAPI_AVEncVideoFieldSwap: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfefd7569_4e0a_49f2_9f2b_360ea48c19a2);
pub const CODECAPI_AVEncVideoForceKeyFrame: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x398c1b98_8353_475a_9ef2_8f265d260345);
pub const CODECAPI_AVEncVideoForceSourceScanType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1ef2065f_058a_4765_a4fc_8a864c103012);
pub const CODECAPI_AVEncVideoGradualIntraRefresh: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8f347dee_cb0d_49ba_b462_db6927ee2101);
pub const CODECAPI_AVEncVideoHeaderDropFrame: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ed9e124_7925_43fe_971b_e019f62222b4);
pub const CODECAPI_AVEncVideoHeaderFrames: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xafd5f567_5c1b_4adc_bdaf_735610381436);
pub const CODECAPI_AVEncVideoHeaderHours: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2acc7702_e2da_4158_bf9b_88880129d740);
pub const CODECAPI_AVEncVideoHeaderMinutes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdc1a99ce_0307_408b_880b_b8348ee8ca7f);
pub const CODECAPI_AVEncVideoHeaderSeconds: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4a2e1a05_a780_4f58_8120_9a449d69656b);
pub const CODECAPI_AVEncVideoInputChromaResolution: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbb0cec33_16f1_47b0_8a88_37815bee1739);
pub const CODECAPI_AVEncVideoInputChromaSubsampling: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa8e73a39_4435_4ec3_a6ea_98300f4b36f7);
pub const CODECAPI_AVEncVideoInputColorLighting: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x46a99549_0015_4a45_9c30_1d5cfa258316);
pub const CODECAPI_AVEncVideoInputColorNominalRange: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x16cf25c6_a2a6_48e9_ae80_21aec41d427e);
pub const CODECAPI_AVEncVideoInputColorPrimaries: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc24d783f_7ce6_4278_90ab_28a4f1e5f86c);
pub const CODECAPI_AVEncVideoInputColorTransferFunction: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8c056111_a9c3_4b08_a0a0_ce13f8a27c75);
pub const CODECAPI_AVEncVideoInputColorTransferMatrix: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x52ed68b9_72d5_4089_958d_f5405d55081c);
pub const CODECAPI_AVEncVideoInstantTemporalUpSwitching: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa3308307_0d96_4ba4_b1f0_b91a5e49df10);
pub const CODECAPI_AVEncVideoIntraLayerPrediction: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd3af46b8_bf47_44bb_a283_69f0b0228ff9);
pub const CODECAPI_AVEncVideoInverseTelecineEnable: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2ea9098b_e76d_4ccd_a030_d3b889c1b64c);
pub const CODECAPI_AVEncVideoInverseTelecineThreshold: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x40247d84_e895_497f_b44c_b74560acfe27);
pub const CODECAPI_AVEncVideoLTRBufferControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa4a0e93d_4cbc_444c_89f4_826d310e92a7);
pub const CODECAPI_AVEncVideoMarkLTRFrame: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe42f4748_a06d_4ef9_8cea_3d05fde3bd3b);
pub const CODECAPI_AVEncVideoMaxCTBSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x822363ff_cec8_43e5_92fd_e097488485e9);
pub const CODECAPI_AVEncVideoMaxKeyframeDistance: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2987123a_ba93_4704_b489_ec1e5f25292c);
pub const CODECAPI_AVEncVideoMaxNumRefFrame: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x964829ed_94f9_43b4_b74d_ef40944b69a0);
pub const CODECAPI_AVEncVideoMaxQP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3daf6f66_a6a7_45e0_a8e5_f2743f46a3a2);
pub const CODECAPI_AVEncVideoMaxTemporalLayers: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9c668cfe_08e1_424a_934e_b764b064802a);
pub const CODECAPI_AVEncVideoMeanAbsoluteDifference: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe5c0c10f_81a4_422d_8c3f_b474a4581336);
pub const CODECAPI_AVEncVideoMinQP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0ee22c6a_a37c_4568_b5f1_9d4c2b3ab886);
pub const CODECAPI_AVEncVideoNoOfFieldsToEncode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x61e4bbe2_4ee0_40e7_80ab_51ddeebe6291);
pub const CODECAPI_AVEncVideoNoOfFieldsToSkip: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa97e1240_1427_4c16_a7f7_3dcfd8ba4cc5);
pub const CODECAPI_AVEncVideoNumGOPsPerIDR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x83bc5bdb_5b89_4521_8f66_33151c373176);
pub const CODECAPI_AVEncVideoOutputChromaResolution: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6097b4c9_7c1d_4e64_bfcc_9e9765318ae7);
pub const CODECAPI_AVEncVideoOutputChromaSubsampling: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfa561c6c_7d17_44f0_83c9_32ed12e96343);
pub const CODECAPI_AVEncVideoOutputColorLighting: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0e5aaac6_ace6_4c5c_998e_1a8c9c6c0f89);
pub const CODECAPI_AVEncVideoOutputColorNominalRange: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x972835ed_87b5_4e95_9500_c73958566e54);
pub const CODECAPI_AVEncVideoOutputColorPrimaries: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbe95907c_9d04_4921_8985_a6d6d87d1a6c);
pub const CODECAPI_AVEncVideoOutputColorTransferFunction: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4a7f884a_ea11_460d_bf57_b88bc75900de);
pub const CODECAPI_AVEncVideoOutputColorTransferMatrix: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa9b90444_af40_4310_8fbe_ed6d933f892b);
pub const CODECAPI_AVEncVideoOutputFrameRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xea85e7c3_9567_4d99_87c4_02c1c278ca7c);
pub const CODECAPI_AVEncVideoOutputFrameRateConversion: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8c068bf4_369a_4ba3_82fd_b2518fb3396e);
pub const CODECAPI_AVEncVideoOutputScanType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x460b5576_842e_49ab_a62d_b36f7312c9db);
pub const CODECAPI_AVEncVideoPixelAspectRatio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cdc718f_b3e9_4eb6_a57f_cf1f1b321b87);
pub const CODECAPI_AVEncVideoROIEnabled: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd74f7f18_44dd_4b85_aba3_05d9f42a8280);
pub const CODECAPI_AVEncVideoRateControlParams: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x87d43767_7645_44ec_b438_d3322fbca29f);
pub const CODECAPI_AVEncVideoSelectLayer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeb1084f5_6aaa_4914_bb2f_6147227f12e7);
pub const CODECAPI_AVEncVideoSourceFilmContent: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1791c64b_ccfc_4827_a0ed_2557793b2b1c);
pub const CODECAPI_AVEncVideoSourceIsBW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x42ffc49b_1812_4fdc_8d24_7054c521e6eb);
pub const CODECAPI_AVEncVideoSupportedControls: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd3f40fdd_77b9_473d_8196_061259e69cff);
pub const CODECAPI_AVEncVideoTemporalLayerCount: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x19caebff_b74d_4cfd_8c27_c2f9d97d5f52);
pub const CODECAPI_AVEncVideoUsage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1f636849_5dc1_49f1_b1d8_ce3cf62ea385);
pub const CODECAPI_AVEncVideoUseLTRFrame: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00752db8_55f7_4f80_895b_27639195f2ad);
pub const CODECAPI_AVEncWMVDecoderComplexity: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf32c0dab_f3cb_4217_b79f_8762768b5f67);
pub const CODECAPI_AVEncWMVInterlacedEncoding: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe3d00f8a_c6f5_4e14_a588_0ec87a726f9b);
pub const CODECAPI_AVEncWMVKeyFrameBufferLevelMarker: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x51ff1115_33ac_426c_a1b1_09321bdf96b4);
pub const CODECAPI_AVEncWMVKeyFrameDistance: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5569055e_e268_4771_b83e_9555ea28aed3);
pub const CODECAPI_AVEncWMVProduceDummyFrames: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd669d001_183c_42e3_a3ca_2f4586d2396c);
pub const CODECAPI_AVLowLatencyMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9c27891a_ed7a_40e1_88e8_b22727a024ee);
pub const CODECAPI_AVPriorityControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x54ba3dc8_bdde_4329_b187_2018bc5c2ba1);
pub const CODECAPI_AVRealtimeControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6f440632_c4ad_4bf7_9e52_456942b454b0);
pub const CODECAPI_AVScenarioInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb28a6e64_3ff9_446a_8a4b_0d7a53413236);
pub const CODECAPI_CHANGELISTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62b12acf_f6b0_47d9_9456_96f22c4e0b9d);
pub const CODECAPI_CURRENTCHANGELIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1cb14e83_7d72_4657_83fd_47a2c5b9d13d);
pub const CODECAPI_GUID_AVDecAudioInputAAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x97df7828_b94a_47e2_a4bc_51194db22a4d);
pub const CODECAPI_GUID_AVDecAudioInputDTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x600bc0ca_6a1f_4e91_b241_1bbeb1cb19e0);
pub const CODECAPI_GUID_AVDecAudioInputDolby: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8e4228a0_f000_4e0b_8f54_ab8d24ad61a2);
pub const CODECAPI_GUID_AVDecAudioInputDolbyDigitalPlus: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0803e185_8f5d_47f5_9908_19a5bbc9fe34);
pub const CODECAPI_GUID_AVDecAudioInputHEAAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x16efb4aa_330e_4f5c_98a8_cf6ac55cbe60);
pub const CODECAPI_GUID_AVDecAudioInputMPEG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x91106f36_02c5_4f75_9719_3b7abf75e1f6);
pub const CODECAPI_GUID_AVDecAudioInputPCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2421da5_bbb4_4cd5_a996_933c6b5d1347);
pub const CODECAPI_GUID_AVDecAudioInputWMA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc95e8dcf_4058_4204_8c42_cb24d91e4b9b);
pub const CODECAPI_GUID_AVDecAudioInputWMAPro: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0128b7c7_da72_4fe3_bef8_5c52e3557704);
pub const CODECAPI_GUID_AVDecAudioOutputFormat_PCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x696e1d31_548f_4036_825f_7026c60011bd);
pub const CODECAPI_GUID_AVDecAudioOutputFormat_PCM_Headphones: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x696e1d34_548f_4036_825f_7026c60011bd);
pub const CODECAPI_GUID_AVDecAudioOutputFormat_PCM_Stereo_Auto: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x696e1d35_548f_4036_825f_7026c60011bd);
pub const CODECAPI_GUID_AVDecAudioOutputFormat_PCM_Stereo_MatrixEncoded: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x696e1d30_548f_4036_825f_7026c60011bd);
pub const CODECAPI_GUID_AVDecAudioOutputFormat_SPDIF_Bitstream: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x696e1d33_548f_4036_825f_7026c60011bd);
pub const CODECAPI_GUID_AVDecAudioOutputFormat_SPDIF_PCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x696e1d32_548f_4036_825f_7026c60011bd);
pub const CODECAPI_GUID_AVEncCommonFormatATSC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8d7b897c_a019_4670_aa76_2edcac7ac296);
pub const CODECAPI_GUID_AVEncCommonFormatDVB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x71830d8f_6c33_430d_844b_c2705baae6db);
pub const CODECAPI_GUID_AVEncCommonFormatDVD_DashVR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe55199d6_044c_4dae_a488_531ed306235b);
pub const CODECAPI_GUID_AVEncCommonFormatDVD_PlusVR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe74c6f2e_ec37_478d_9af4_a5e135b6271c);
pub const CODECAPI_GUID_AVEncCommonFormatDVD_V: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcc9598c4_e7fe_451d_b1ca_761bc840b7f3);
pub const CODECAPI_GUID_AVEncCommonFormatHighMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1eabe760_fb2b_4928_90d1_78db88eee889);
pub const CODECAPI_GUID_AVEncCommonFormatHighMPV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa2d25db8_b8f9_42c2_8bc7_0b93cf604788);
pub const CODECAPI_GUID_AVEncCommonFormatMP3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x349733cd_eb08_4dc2_8197_e49835ef828b);
pub const CODECAPI_GUID_AVEncCommonFormatSVCD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x51d85818_8220_448c_8066_d69bed16c9ad);
pub const CODECAPI_GUID_AVEncCommonFormatUnSpecified: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaf46a35a_6024_4525_a48a_094b97f5b3c2);
pub const CODECAPI_GUID_AVEncCommonFormatVCD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x95035bf7_9d90_40ff_ad5c_5cf8cf71ca1d);
pub const CODECAPI_GUID_AVEncDTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x45fbcaa2_5e6e_4ab0_8893_5903bee93acf);
pub const CODECAPI_GUID_AVEncDTSHD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2052e630_469d_4bfb_80ca_1d656e7e918f);
pub const CODECAPI_GUID_AVEncDV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x09b769c7_3329_44fb_8954_fa30937d3d5a);
pub const CODECAPI_GUID_AVEncDolbyDigitalConsumer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc1a7bf6c_0059_4bfa_94ef_ef747a768d52);
pub const CODECAPI_GUID_AVEncDolbyDigitalPlus: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x698d1b80_f7dd_415c_971c_42492a2056c6);
pub const CODECAPI_GUID_AVEncDolbyDigitalPro: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf5be76cc_0ff8_40eb_9cb1_bba94004d44f);
pub const CODECAPI_GUID_AVEncH264Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x95044eab_31b3_47de_8e75_38a42bb03e28);
pub const CODECAPI_GUID_AVEncMLP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05f73e29_f0d1_431e_a41c_a47432ec5a66);
pub const CODECAPI_GUID_AVEncMPEG1Audio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd4dd1362_cd4a_4cd6_8138_b94db4542b04);
pub const CODECAPI_GUID_AVEncMPEG1Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc8dafefe_da1e_4774_b27d_11830c16b1fe);
pub const CODECAPI_GUID_AVEncMPEG2Audio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xee4cbb1f_9c3f_4770_92b5_fcb7c2a8d381);
pub const CODECAPI_GUID_AVEncMPEG2Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x046dc19a_6677_4aaa_a31d_c1ab716f4560);
pub const CODECAPI_GUID_AVEncPCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x844be7f4_26cf_4779_b386_cc05d187990c);
pub const CODECAPI_GUID_AVEncSDDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1dc1b82f_11c8_4c71_b7b6_ee3eb9bc2b94);
pub const CODECAPI_GUID_AVEncWMALossless: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x55ca7265_23d8_4761_9031_b74fbe12f4c1);
pub const CODECAPI_GUID_AVEncWMAPro: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1955f90c_33f7_4a68_ab81_53f5657125c4);
pub const CODECAPI_GUID_AVEncWMAVoice: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x13ed18cb_50e8_4276_a288_a6aa228382d9);
pub const CODECAPI_GUID_AVEncWMV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4e0fef9b_1d43_41bd_b8bd_4d7bf7457a2a);
pub const CODECAPI_GUID_AVEndMPEG4Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdd37b12a_9503_4f8b_b8d0_324a00c0a1cf);
pub const CODECAPI_GetOPMContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2f036c05_4c14_4689_8839_294c6d73e053);
pub const CODECAPI_SETALLDEFAULTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c5e6a7c_acf8_4f55_a999_1a628109051b);
pub const CODECAPI_SUPPORTSEVENTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0581af97_7693_4dbd_9dca_3f9ebd6585a1);
pub const CODECAPI_SetHDCPManagerContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d2d1fc8_3dc9_47eb_a1a2_471c80cd60d0);
pub const CODECAPI_VIDEO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7112e8e1_3d03_47ef_8e60_03f1cf537301);
pub const CODECAPI_VideoEncoderDisplayContentType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x79b90b27_f4b1_42dc_9dd7_cdaf8135c400);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const COPP_ProtectionType_ACP: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const COPP_ProtectionType_CGMSA: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const COPP_ProtectionType_HDCP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const COPP_ProtectionType_Mask: i32 = -2147483641i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const COPP_ProtectionType_None: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const COPP_ProtectionType_Reserved: i32 = 2147483640i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const COPP_ProtectionType_Unknown: i32 = -2147483648i32;
pub const CPK_DS_AC3Decoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c9c69d6_0ffc_4481_afdb_cdf1c79c6f3e);
pub const CPK_DS_MPEG2Decoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9910c5cd_95c9_4e06_865a_efa1c8016bf4);
pub const CResamplerMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf447b69e_1884_4a7e_8055_346f74d6edb3);
pub const CResizerDMO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1ea1ea14_48f4_4054_ad1a_e8aee10ac805);
pub const CResizerMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd3ec8b8b_7728_4fd8_9fe0_7b67d19f73a3);
pub const CShotDetectorDmo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x56aefacd_110c_4397_9292_b0a0c61b6750);
pub const CSmpteTransformsDmo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbde6388b_da25_485d_ba7f_fabc28b20318);
pub const CThumbnailGeneratorDmo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x559c6bad_1ea8_4963_a087_8a6810f9218b);
pub const CTocGeneratorDmo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4dda1941_77a0_4fb1_a518_e2185041d70c);
pub const CVodafoneAACCCDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7e76bf7f_c993_4e26_8fab_470a70c0d59c);
pub const CVodafoneAACDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7f36f942_dcf3_4d82_9289_5b1820278f7c);
pub const CWMADecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2eeb4adf_4578_4d10_bca7_bb955f56320a);
pub const CWMAEncMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x70f598e9_f4ab_495a_99e2_a7c4d3d89abf);
pub const CWMATransMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xedcad9cb_3127_40df_b527_0152ccb3f6f5);
pub const CWMAudioAEC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x745057c7_f353_4f2d_a7ee_58434477730e);
pub const CWMAudioCAPXGFXAPO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x13ab3ebd_137e_4903_9d89_60be8277fd17);
pub const CWMAudioCAPXLFXAPO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc9453e73_8c5c_4463_9984_af8bab2f5447);
pub const CWMAudioGFXAPO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x637c490d_eee3_4c0a_973f_371958802da2);
pub const CWMAudioLFXAPO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62dc1a93_ae24_464c_a43e_452f824c4250);
pub const CWMAudioSpdTxDMO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5210f8e4_b0bb_47c3_a8d9_7b2282cc79ed);
pub const CWMSPDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x874131cb_4ecc_443b_8948_746b89595d20);
pub const CWMSPEncMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x67841b03_c689_4188_ad3f_4c9ebeec710b);
pub const CWMSPEncMediaObject2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1f1f4e1a_2252_4063_84bb_eee75f8856d5);
pub const CWMTDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf9dbc64e_2dd0_45dd_9b52_66642ef94431);
pub const CWMTEncMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x60b67652_e46b_4e44_8609_f74bffdc083c);
pub const CWMV9EncMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd23b90d0_144f_46bd_841d_59e4eb19dc59);
pub const CWMVDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x82d353df_90bd_4382_8bc2_3f6192b76e34);
pub const CWMVEncMediaObject2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96b57cdd_8966_410c_bb1f_c97eea765c04);
pub const CWMVXEncMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7e320092_596a_41b2_bbeb_175d10504eb6);
pub const CWVC1DecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc9bfbccf_e60e_4588_a3df_5a03b1fd9585);
pub const CWVC1EncMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x44653d0d_8cca_41e7_baca_884337b747ac);
pub const CZuneAACCCDecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa74e98f2_52d6_4b4e_885b_e0a6ca4f187a);
pub const CZuneM4S2DecMediaObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc56fc25c_0fc6_404a_9503_b10bf51a8ab9);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_12BIT_PROFILE2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x17127009_a00f_4ce1_994e_bf4081f6f3f0);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_12BIT_PROFILE2_420: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2d80bed6_9cac_4835_9e91_327bbc4f9ee8);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_PROFILE0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb8be4ccb_cf53_46ba_8d59_d6b8a6da5d2a);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_PROFILE1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6936ff0f_45b1_4163_9cc1_646ef6946108);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_PROFILE2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0c5f2aa1_e541_4089_bb7b_98110a19d7c8);
pub const D3D12_VIDEO_DECODE_PROFILE_H264: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D12_VIDEO_DECODE_PROFILE_H264_MULTIVIEW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const D3D12_VIDEO_DECODE_PROFILE_H264_STEREO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const D3D12_VIDEO_DECODE_PROFILE_H264_STEREO_PROGRESSIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG1_AND_MPEG2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xee27417f_5e28_4e65_beea_1d26b508adc9);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG4PT2_ADVSIMPLE_NOGMC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG4PT2_SIMPLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const D3D12_VIDEO_DECODE_PROFILE_VC1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D12_VIDEO_DECODE_PROFILE_VC1_D2010: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D12_VIDEO_DECODE_PROFILE_VP8: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const D3D12_VIDEO_DECODE_PROFILE_VP9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const D3D12_VIDEO_DECODE_PROFILE_VP9_10BIT_PROFILE2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_IsVirtualCamera: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 3u32 };
pub const DSATTRIB_CAPTURE_STREAMTIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0c1a5614_30cd_4f40_bcbf_d03e52306207);
pub const DSATTRIB_CC_CONTAINER_INFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe7e050fb_dd5d_40dd_9915_35dcb81bdc8a);
pub const DSATTRIB_DSHOW_STREAM_DESC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5fb5673b_0a2a_4565_827b_6853fd75e611);
pub const DSATTRIB_OptionalVideoAttributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5a5f08ca_55c2_4033_92ab_55db8f781226);
pub const DSATTRIB_PBDATAG_ATTRIBUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe0b56679_12b9_43cc_b7df_578caa5a7b63);
pub const DSATTRIB_PicSampleSeq: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2f5bae02_7b8f_4f60_82d6_e4ea2f1f4c99);
pub const DSATTRIB_SAMPLE_LIVE_STREAM_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x892cd111_72f3_411d_8b91_a9e9123ac29a);
pub const DSATTRIB_TRANSPORT_PROPERTIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb622f612_47ad_4671_ad6c_05a98e65de3a);
pub const DSATTRIB_UDCRTag: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeb7836ca_14ff_4919_bce7_3af12319e50c);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DECODE_GET_DRIVER_HANDLE: u32 = 1829u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DECODE_SPECIFY_ENCRYPTED_BLOCKS: u32 = 1828u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_E_NEW_VIDEO_DEVICE: ::windows_sys::core::HRESULT = -2147217407i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_E_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2147217405i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2147217408i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_E_VIDEO_DEVICE_LOCKED: ::windows_sys::core::HRESULT = -2147217406i32;
pub const DXVA2_ModeH264_A: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be64_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_B: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be65_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_C: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be66_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_D: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be67_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_E: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_F: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be69_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_VLD_Multiview_NoFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const DXVA2_ModeH264_VLD_Stereo_NoFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const DXVA2_ModeH264_VLD_Stereo_Progressive_NoFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const DXVA2_ModeH264_VLD_WithFMOASO_NoFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd5f04ff9_3418_45d8_9561_32a76aae2ddd);
pub const DXVA2_ModeHEVC_VLD_Main: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const DXVA2_ModeHEVC_VLD_Main10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const DXVA2_ModeMPEG1_VLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6f3ec719_3735_42cc_8063_65cc3cb36616);
pub const DXVA2_ModeMPEG2_IDCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbf22ad00_03ea_4690_8077_473346209b7e);
pub const DXVA2_ModeMPEG2_MoComp: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe6a9f44b_61b0_4563_9ea4_63d2a3c6fe66);
pub const DXVA2_ModeMPEG2_VLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xee27417f_5e28_4e65_beea_1d26b508adc9);
pub const DXVA2_ModeMPEG2and1_VLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const DXVA2_ModeMPEG4pt2_VLD_AdvSimple_GMC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xab998b5b_4258_44a9_9feb_94e597a6baae);
pub const DXVA2_ModeMPEG4pt2_VLD_AdvSimple_NoGMC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const DXVA2_ModeMPEG4pt2_VLD_Simple: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const DXVA2_ModeVC1_A: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81bea0_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_B: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81bea1_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_C: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81bea2_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_D: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_D2010: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVP8_VLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const DXVA2_ModeVP9_VLD_10bit_Profile2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
pub const DXVA2_ModeVP9_VLD_Profile0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const DXVA2_ModeWMV8_A: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be80_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV8_B: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be81_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV9_A: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be90_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV9_B: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be91_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV9_C: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be94_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_NoEncrypt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81bed0_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_VideoProcBobDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x335aa36e_7884_43a4_9c91_7f87faf3e37e);
pub const DXVA2_VideoProcProgressiveDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5a54a0c9_c7ec_4bd9_8ede_f3c75dc4393b);
pub const DXVA2_VideoProcSoftwareDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4553d47f_ee7e_4e3f_9475_dbf1376c4810);
pub const DXVAHDControlGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0386e75_f70c_464c_a9ce_33c44e091623);
pub const DXVAHDETWGUID_CREATEVIDEOPROCESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x681e3d1e_5674_4fb3_a503_2f2055e91f60);
pub const DXVAHDETWGUID_DESTROYVIDEOPROCESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf943f0a0_3f16_43e0_8093_105a986aa5f1);
pub const DXVAHDETWGUID_VIDEOPROCESSBLTHD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbef3d435_78c7_4de3_9707_cd1b083b160a);
pub const DXVAHDETWGUID_VIDEOPROCESSBLTHD_STREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x27ae473e_a5fc_4be5_b4e3_f24994d3c495);
pub const DXVAHDETWGUID_VIDEOPROCESSBLTSTATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x76c94b5a_193f_4692_9484_a4d999da81a8);
pub const DXVAHDETWGUID_VIDEOPROCESSSTREAMSTATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x262c0b02_209d_47ed_94d8_82ae02b84aa7);
pub const DXVAHD_STREAM_STATE_PRIVATE_IVTC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9c601e3c_0f33_414c_a739_99540ee42da5);
pub const DXVAp_DeinterlaceBobDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x335aa36e_7884_43a4_9c91_7f87faf3e37e);
pub const DXVAp_DeinterlaceContainerDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0e85cb93_3046_4ff0_aecc_d58cb5f035fd);
pub const DXVAp_ModeMPEG2_A: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be0a_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVAp_ModeMPEG2_C: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81be0c_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVAp_NoEncrypt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b81bed0_a0c7_11d3_b984_00c04f2e73c5);
pub const ENCAPIPARAM_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x49cc4c43_ca83_4ad4_a9af_f3696af666df);
pub const ENCAPIPARAM_BITRATE_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xee5fb25c_c713_40d1_9d58_c0d7241e250f);
pub const ENCAPIPARAM_PEAK_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x703f16a9_3d48_44a1_b077_018dff915d19);
pub const ENCAPIPARAM_SAP_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0c0171db_fefc_4af7_9991_a5657c191cd1);
pub const EVRConfig_ForceBatching: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe447df09_10ca_4d17_b17e_6a840f8a3a4c);
pub const EVRConfig_ForceBob: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe447df01_10ca_4d17_b17e_6a840f8a3a4c);
pub const EVRConfig_ForceHalfInterlace: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe447df05_10ca_4d17_b17e_6a840f8a3a4c);
pub const EVRConfig_ForceScaling: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe447df07_10ca_4d17_b17e_6a840f8a3a4c);
pub const EVRConfig_ForceThrottle: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe447df03_10ca_4d17_b17e_6a840f8a3a4c);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const E_TOCPARSER_INVALIDASFFILE: ::windows_sys::core::HRESULT = -1728053247i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const E_TOCPARSER_INVALIDRIFFFILE: ::windows_sys::core::HRESULT = -1728053246i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const FACILITY_MF: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const FACILITY_MF_WIN32: u32 = 7u32;
pub const FORMAT_525WSS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc7ecf04d_4582_4869_9abb_bfb523b62edf);
pub const FORMAT_AnalogVideo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dde0_7817_11cf_8a03_00aa006ecb65);
pub const FORMAT_CAPTIONED_H264VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa4efc024_873e_4da3_898b_474ddbd79fd0);
pub const FORMAT_CAPTIONED_MPEG2VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7ab2ada2_81b6_4f14_b3c8_d0c486393b67);
pub const FORMAT_CC_CONTAINER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50997a4a_e508_4054_a2b2_10ff0ac1a69a);
pub const FORMAT_DvInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05589f84_c356_11ce_bf01_00aa0055595a);
pub const FORMAT_MFVideoFormat: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaed4ab2d_7326_43cb_9464_c879cab9c43d);
pub const FORMAT_MPEGStreams: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05589f83_c356_11ce_bf01_00aa0055595a);
pub const FORMAT_MPEGVideo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05589f82_c356_11ce_bf01_00aa0055595a);
pub const FORMAT_None: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0f6417d6_c318_11d0_a43f_00a0c9223196);
pub const FORMAT_VideoInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const FORMAT_VideoInfo2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf72a76a0_eb0a_11d0_ace4_0000c0cc16ba);
pub const FORMAT_WaveFormatEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const GUID_NativeDeviceService: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xef71e53c_52f4_43c5_b86a_ad6cb216a61e);
pub const GUID_PlayToService: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf6a8ff9d_9e14_41c9_bf0f_120a2b3ce120);
pub const KSPROPSETID_OPMVideoOutput: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x06f414bb_f43a_4fe2_a566_774b4c81f0db);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const LOCAL_D3DFMT_DEFINES: u32 = 1u32;
pub const LOOK_DOWNSTREAM_ONLY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac798be1_98e3_11d1_b3f1_00aa003761c5);
pub const LOOK_UPSTREAM_ONLY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac798be0_98e3_11d1_b3f1_00aa003761c5);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MACROBLOCK_FLAG_DIRTY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MACROBLOCK_FLAG_HAS_MOTION_VECTOR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MACROBLOCK_FLAG_HAS_QP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MACROBLOCK_FLAG_MOTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MACROBLOCK_FLAG_SKIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MACROBLOCK_FLAG_VIDEO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MAX_SUBSTREAMS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEDIASINK_CANNOT_MATCH_CLOCK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEDIASINK_CAN_PREROLL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEDIASINK_CLOCK_REQUIRED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEDIASINK_FIXED_STREAMS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEDIASINK_RATELESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEDIASINK_REQUIRE_REFERENCE_MEDIATYPE: u32 = 32u32;
pub const MEDIASUBTYPE_420O: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4f303234_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_708_608Data: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0af414bc_4ed2_445e_9839_8f095568ab3c);
pub const MEDIASUBTYPE_A2B10G10R10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x576f7893_bdf6_48c4_875f_ae7b81834567);
pub const MEDIASUBTYPE_A2R10G10B10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2f8bb76d_b644_4550_acf3_d30caa65d5c5);
pub const MEDIASUBTYPE_AI44: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34344941_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_AIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb8d_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_ARGB1555: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x297c55af_e209_4cb3_b757_c76d6b9c88a8);
pub const MEDIASUBTYPE_ARGB1555_D3D_DX7_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x35314137_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_ARGB1555_D3D_DX9_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x35314139_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_ARGB32: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x773c9ac0_3274_11d0_b724_00aa006c1a01);
pub const MEDIASUBTYPE_ARGB32_D3D_DX7_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x38384137_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_ARGB32_D3D_DX9_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x38384139_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_ARGB4444: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6e6415e6_5c24_425f_93cd_80102b3d1cca);
pub const MEDIASUBTYPE_ARGB4444_D3D_DX7_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34344137_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_ARGB4444_D3D_DX9_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34344139_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_AU: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb8c_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_AVC1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31435641_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_AYUV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x56555941_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_AnalogVideo_NTSC_M: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dde2_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_PAL_B: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dde5_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_PAL_D: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dde6_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_PAL_G: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dde7_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_PAL_H: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dde8_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_PAL_I: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dde9_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_PAL_M: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddea_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_PAL_N: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddeb_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_PAL_N_COMBO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddec_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_SECAM_B: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddf0_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_SECAM_D: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddf1_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_SECAM_G: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddf2_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_SECAM_H: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddf3_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_SECAM_K: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddf4_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_SECAM_K1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddf5_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_AnalogVideo_SECAM_L: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482ddf6_7817_11cf_8a03_00aa006ecb65);
pub const MEDIASUBTYPE_Asf: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3db80f90_9412_11d1_aded_0000f8754b99);
pub const MEDIASUBTYPE_Avi: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb88_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_CC_CONTAINER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7ea626db_54da_437b_be9f_f73073adfa3c);
pub const MEDIASUBTYPE_CFCC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x43434643_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_CLJR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x524a4c43_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_CLPL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4c504c43_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_CPLA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x414c5043_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_DOLBY_AC3_SPDIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000092_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_DOLBY_DDPLUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa7fb87af_2d02_42fb_a4d4_05cd93843bdd);
pub const MEDIASUBTYPE_DOLBY_TRUEHD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeb27cec4_163e_4ca3_8b74_8e25f91b517e);
pub const MEDIASUBTYPE_DRM_Audio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_DTS2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00002001_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_DTS_HD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa2e58eb7_0fa9_48bb_a40c_fa0e156d0645);
pub const MEDIASUBTYPE_DTS_HD_HRA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa61ac364_ad0e_4744_89ff_213ce0df8804);
pub const MEDIASUBTYPE_DVB_SUBTITLES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34ffcbc3_d5b3_4171_9002_d4c60301697f);
pub const MEDIASUBTYPE_DVCS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x53435644_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_DVM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00002000_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_DVSD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x44535644_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_DssAudio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0af4f82_e163_11d0_bad9_00609744111a);
pub const MEDIASUBTYPE_DssVideo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0af4f81_e163_11d0_bad9_00609744111a);
pub const MEDIASUBTYPE_DtvCcData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf52addaa_36f0_43f5_95ea_6d866484262a);
pub const MEDIASUBTYPE_H264: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34363248_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_I420: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_IA44: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34344149_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_IEEE_FLOAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000003_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_IF09: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x39304649_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_IJPG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47504a49_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_IMC1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31434d49_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_IMC2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32434d49_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_IMC3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33434d49_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_IMC4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34434d49_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_ISDB_CAPTIONS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x059dd67d_2e55_4d41_8d1b_01f5e4f50607);
pub const MEDIASUBTYPE_ISDB_SUPERIMPOSE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36dc6d28_f1a6_4216_9048_9cfcefeb5eba);
pub const MEDIASUBTYPE_IYUV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x56555949_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_Line21_BytePair: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6e8d4a22_310c_11d0_b79a_00aa003767a7);
pub const MEDIASUBTYPE_Line21_GOPPacket: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6e8d4a23_310c_11d0_b79a_00aa003767a7);
pub const MEDIASUBTYPE_Line21_VBIRawData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6e8d4a24_310c_11d0_b79a_00aa003767a7);
pub const MEDIASUBTYPE_M4S2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MDVF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4656444d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MJPG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47504a4d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MP42: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3234504d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MP43: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MP4S: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPEG1Audio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb87_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_MPEG1AudioPayload: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000050_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPEG1Packet: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb80_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_MPEG1Payload: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb81_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_MPEG1System: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb84_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_MPEG1Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb86_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_MPEG1VideoCD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb85_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_MPEG_ADTS_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00001600_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPEG_HEAAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00001610_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPEG_LOAS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00001602_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPEG_RAW_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00001601_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPG4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3447504d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MSAUDIO1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000160_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MSS1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MSS2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_NOKIA_MPEG_ADTS_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00001608_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_NOKIA_MPEG_RAW_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00001609_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_NV11: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3131564e_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_NV12: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3231564e_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_NV24: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3432564e_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_None: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb8e_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_Overlay: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb7f_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_P010: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313050_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_P016: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36313050_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_P208: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x38303250_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_P210: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313250_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_P216: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36313250_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_P408: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x38303450_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_PCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_PCMAudio_Obsolete: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb8a_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_Plum: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d756c50_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_QTJpeg: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6765706a_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_QTMovie: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb89_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_QTRle: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x20656c72_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_QTRpza: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x617a7072_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_QTSmc: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x20636d73_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_RAW_AAC1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x000000ff_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_RAW_SPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000240_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_RGB1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb78_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_RGB16_D3D_DX7_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36315237_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_RGB16_D3D_DX9_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36315239_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_RGB24: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb7d_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_RGB32: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb7e_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_RGB32_D3D_DX7_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32335237_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_RGB32_D3D_DX9_RT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32335239_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_RGB4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb79_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_RGB555: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb7c_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_RGB565: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb7b_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_RGB8: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb7a_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_S340: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30343353_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_S342: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32343353_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_SPDIF_TAG_241h: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000241_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_TELETEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf72a76e3_eb0a_11d0_ace4_0000c0cc16ba);
pub const MEDIASUBTYPE_TVMJ: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4a4d5654_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_UYVY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x59565955_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_V216: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36313256_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_V410: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313456_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_VBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x663da43c_03e8_4e9a_9cd5_bf11ed0def76);
pub const MEDIASUBTYPE_VODAFONE_MPEG_ADTS_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0000160a_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_VODAFONE_MPEG_RAW_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0000160b_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_VPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa1b3f620_9792_4d8d_81a4_86af25772090);
pub const MEDIASUBTYPE_VPVBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5a9b6a41_1a22_11d1_bad9_00609744111a);
pub const MEDIASUBTYPE_VPVideo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5a9b6a40_1a22_11d1_bad9_00609744111a);
pub const MEDIASUBTYPE_WAKE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x454b4157_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WAVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb8b_524f_11ce_9f53_0020af0ba770);
pub const MEDIASUBTYPE_WMASPDIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000164_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMAUDIO2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMAUDIO3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMAUDIO4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000168_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMAUDIO_LOSSLESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMV1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMV2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMV3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMVA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x41564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMVB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x42564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMVP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMVR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x52564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WSS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2791d576_8e7a_466f_9e90_5d3f3083738b);
pub const MEDIASUBTYPE_WVC1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WVP2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32505657_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_X264: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34363258_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_XDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x01ca73e3_dce6_4575_afe1_2bf1c902caf3);
pub const MEDIASUBTYPE_Y210: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313259_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_Y211: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31313259_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_Y216: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36313259_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_Y411: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31313459_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_Y41P: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50313459_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_Y41T: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x54313459_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_Y42T: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x54323459_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_YUY2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32595559_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_YUYV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x56595559_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_YV12: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32315659_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_YVU9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x39555659_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_YVYU: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x55595659_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_dv25: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x35327664_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_dv50: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30357664_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_dvh1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31687664_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_dvhd: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x64687664_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_dvsd: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x64737664_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_dvsl: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c737664_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_h264: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34363268_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_m4s2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3273346d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_mp42: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3234706d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_mp43: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3334706d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_mp4s: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7334706d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_mpg4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3467706d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_v210: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313276_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmv1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmv2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmv3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmva: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x61766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmvb: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmvp: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x70766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmvr: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wvc1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31637677_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wvp2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32707677_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_x264: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34363278_0000_0010_8000_00aa00389b71);
pub const MEDIATYPE_AUXLine21Data: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x670aea80_3a82_11d0_b79b_00aa003767a7);
pub const MEDIATYPE_AUXTeletextPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x11264acb_37de_4eba_8c35_7f04a1a68332);
pub const MEDIATYPE_AnalogAudio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dee1_7817_11cf_8a03_00aa006ecb65);
pub const MEDIATYPE_AnalogVideo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dde1_7817_11cf_8a03_00aa006ecb65);
pub const MEDIATYPE_Audio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const MEDIATYPE_CC_CONTAINER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaeb312e9_3357_43ca_b701_97ec198e2b62);
pub const MEDIATYPE_DTVCCData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb77e152_53b2_499c_b46b_509fc33edfd7);
pub const MEDIATYPE_File: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x656c6966_0000_0010_8000_00aa00389b71);
pub const MEDIATYPE_Interleaved: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73766169_0000_0010_8000_00aa00389b71);
pub const MEDIATYPE_LMRT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x74726c6d_0000_0010_8000_00aa00389b71);
pub const MEDIATYPE_MPEG1SystemStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb82_524f_11ce_9f53_0020af0ba770);
pub const MEDIATYPE_MSTVCaption: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb88b8a89_b049_4c80_adcf_5898985e22c1);
pub const MEDIATYPE_Midi: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7364696d_0000_0010_8000_00aa00389b71);
pub const MEDIATYPE_ScriptCommand: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73636d64_0000_0010_8000_00aa00389b71);
pub const MEDIATYPE_Stream: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb83_524f_11ce_9f53_0020af0ba770);
pub const MEDIATYPE_Text: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73747874_0000_0010_8000_00aa00389b71);
pub const MEDIATYPE_Timecode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0482dee3_7817_11cf_8a03_00aa006ecb65);
pub const MEDIATYPE_URL_STREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x736c7275_0000_0010_8000_00aa00389b71);
pub const MEDIATYPE_VBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf72a76e1_eb0a_11d0_ace4_0000c0cc16ba);
pub const MEDIATYPE_Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
pub const MEDeviceStreamCreated: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0252a1cf_3540_43b4_9164_d72eb405fa40);
pub const MFAMRNBByteStreamHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xefe6208a_0a2c_49fa_8a01_3768b559b6da);
pub const MFAMRNBSinkClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb0271158_70d2_4c5b_9f94_76f549d90fdf);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASFINDEXER_APPROX_SEEK_TIME_UNKNOWN: u64 = 18446744073709551615u64;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASFINDEXER_NO_FIXED_INTERVAL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASFINDEXER_PER_ENTRY_BYTES_DYNAMIC: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASFINDEXER_READ_FOR_REVERSEPLAYBACK_OUTOFDATASEGMENT: u64 = 18446744073709551615u64;
pub const MFASFINDEXER_TYPE_TIMECODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x49815231_6bad_44fd_810a_3f60984ec7fd);
pub const MFASFMutexType_Bitrate: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72178c2c_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFASFMutexType_Language: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72178c2b_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFASFMutexType_Presentation: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72178c2d_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFASFMutexType_Unknown: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72178c2e_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFASFSPLITTER_PACKET_BOUNDARY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfe584a05_e8d6_42e3_b176_f1211705fb6f);
pub const MFASFSampleExtension_ContentType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd590dc20_07bc_436c_9cf7_f3bbfbf1a4dc);
pub const MFASFSampleExtension_Encryption_KeyID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x76376591_795f_4da1_86ed_9d46eca109a9);
pub const MFASFSampleExtension_Encryption_SampleID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6698b84e_0afa_4330_aeb2_1c0a98d7a44d);
pub const MFASFSampleExtension_FileName: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe165ec0e_19ed_45d7_b4a7_25cbd1e28e9b);
pub const MFASFSampleExtension_OutputCleanPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf72a3c6f_6eb4_4ebc_b192_09ad9759e828);
pub const MFASFSampleExtension_PixelAspectRatio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b1ee554_f9ea_4bc8_821a_376b74e4c4b8);
pub const MFASFSampleExtension_SMPTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x399595ec_8667_4e2d_8fdb_98814ce76c1e);
pub const MFASFSampleExtension_SampleDuration: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc6bd9450_867f_4907_83a3_c77921b733ad);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_DEFAULT_BUFFER_WINDOW_MS: u32 = 3000u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_INVALID_STREAM_NUMBER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_MAX_STREAM_NUMBER: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_PAYLOADEXTENSION_MAX_SIZE: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_PAYLOADEXTENSION_VARIABLE_SIZE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_BLOCKING_CALLBACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_CALLBACK_QUEUE_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_CALLBACK_QUEUE_IO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_CALLBACK_QUEUE_LONG_FUNCTION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_CALLBACK_QUEUE_MULTITHREADED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_CALLBACK_QUEUE_PRIVATE_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_CALLBACK_QUEUE_RT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_CALLBACK_QUEUE_STANDARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_CALLBACK_QUEUE_TIMER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_CALLBACK_QUEUE_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_FAST_IO_PROCESSING_CALLBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_LOCALIZE_REMOTE_CALLBACK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_REPLY_CALLBACK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASYNC_SIGNAL_CALLBACK: u32 = 2u32;
pub const MFAudioFormat_AAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00001610_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_AAC_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x419bce76_8b72_400f_adeb_84b57d63484d);
pub const MFAudioFormat_ADTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00001600_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_ADTS_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xda4963a3_14d8_4dcf_92b7_193eb84363db);
pub const MFAudioFormat_ALAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00006c61_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_AMR_NB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00007361_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_AMR_WB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00007362_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_AMR_WP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00007363_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_Base: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_Base_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3884b5bc_e277_43fd_983d_038aa8d9b605);
pub const MFAudioFormat_DRM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_DTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000008_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_DTS_HD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa2e58eb7_0fa9_48bb_a40c_fa0e156d0645);
pub const MFAudioFormat_DTS_LBR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc2fe6f0a_4e3c_4df1_9b60_50863091e4b9);
pub const MFAudioFormat_DTS_RAW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe06d8033_db46_11cf_b4d1_00805f6cbbea);
pub const MFAudioFormat_DTS_UHD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x87020117_ace3_42de_b73e_c656706263f8);
pub const MFAudioFormat_DTS_UHDY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9b9cca00_91b9_4ccc_883a_8f787ac3cc86);
pub const MFAudioFormat_DTS_XLL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x45b37c1b_8c70_4e59_a7be_a1e42c81c80d);
pub const MFAudioFormat_Dolby_AC3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe06d802c_db46_11cf_b4d1_00805f6cbbea);
pub const MFAudioFormat_Dolby_AC3_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x97663a80_8ffb_4445_a6ba_792d908f497f);
pub const MFAudioFormat_Dolby_AC3_SPDIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000092_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_Dolby_AC4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0000ac40_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_Dolby_AC4_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36b7927c_3d87_4a2a_9196_a21ad9e935e6);
pub const MFAudioFormat_Dolby_AC4_V1_ES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d8dccc6_d156_4fb8_979c_a85be7d21dfa);
pub const MFAudioFormat_Dolby_AC4_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7998b2a0_17dd_49b6_8dfa_9b278552a2ac);
pub const MFAudioFormat_Dolby_AC4_V2_ES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7e58c9f9_b070_45f4_8ccd_a99a0417c1ac);
pub const MFAudioFormat_Dolby_DDPlus: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa7fb87af_2d02_42fb_a4d4_05cd93843bdd);
pub const MFAudioFormat_FLAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0000f1ac_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_Float: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000003_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_Float_SpatialObjects: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfa39cd94_bc64_4ab1_9b71_dcd09d5a7e7a);
pub const MFAudioFormat_LPCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe06d8032_db46_11cf_b4d1_00805f6cbbea);
pub const MFAudioFormat_MP3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_MPEG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000050_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_MSP1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0000000a_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_Opus: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0000704f_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_PCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_PCM_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa5e7ff01_8411_4acc_a865_5f4941288d80);
pub const MFAudioFormat_Vorbis: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8d2fd10b_5841_4a6b_8905_588fec1aded9);
pub const MFAudioFormat_WMASPDIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000164_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_WMAudioV8: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_WMAudioV9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const MFAudioFormat_WMAudio_Lossless: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_DOES_NOT_USE_NETWORK: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_HAS_SLOW_SEEK: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_IS_DIRECTORY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_IS_PARTIALLY_DOWNLOADED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_IS_READABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_IS_REMOTE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_IS_SEEKABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_IS_WRITABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_SEEK_FLAG_CANCEL_PENDING_IO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFBYTESTREAM_SHARE_WRITE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCAPTURE_METADATA_SCANLINE_VERTICAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCAPTURE_METADATA_SCAN_BOTTOM_TOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCAPTURE_METADATA_SCAN_RIGHT_LEFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_FREQUENCY_HNS: u32 = 10000000u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_JITTER_DPC: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_JITTER_ISR: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_JITTER_PASSIVE: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_TOLERANCE_UNKNOWN: u32 = 50000u32;
pub const MFCONNECTOR_AGP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac3aef60_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_COMPONENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd596b_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_COMPOSITE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd596a_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_DISPLAYPORT_EMBEDDED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5973_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_DISPLAYPORT_EXTERNAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5972_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_DVI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd596c_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_D_JPN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5970_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_HDMI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd596d_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_LVDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd596e_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_MIRACAST: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5977_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_PCI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac3aef5d_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_PCIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac3aef5e_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_PCI_Express: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac3aef5f_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_SDI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5971_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_SPDIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0b94a712_ad3e_4cee_83ce_ce32e3db6522);
pub const MFCONNECTOR_SVIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5969_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_TRANSPORT_AGNOSTIC_DIGITAL_MODE_A: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5978_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_TRANSPORT_AGNOSTIC_DIGITAL_MODE_B: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5979_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_UDI_EMBEDDED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5975_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_UDI_EXTERNAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5974_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_UNKNOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac3aef5c_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_VGA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57cd5968_ce47_11d9_92db_000bdb28ff98);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCONTENTPROTECTIONDEVICE_FUNCTIONID_START: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA_FUNCTIONID: u32 = 67108864u32;
pub const MFENABLETYPE_MF_RebootRequired: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d4d3d4b_0ece_4652_8b3a_f2d24260d887);
pub const MFENABLETYPE_MF_UpdateRevocationInformation: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe558b0b5_b3c4_44a0_924c_50d178932385);
pub const MFENABLETYPE_MF_UpdateUntrustedComponent: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9879f3d6_cee2_48e6_b573_9767ab172f16);
pub const MFENABLETYPE_WMDRMV1_LicenseAcquisition: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4ff6eeaf_0b43_4797_9b85_abf31815e7b0);
pub const MFENABLETYPE_WMDRMV7_Individualization: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xacd2c84a_b303_4f65_bc2c_2c848d01a989);
pub const MFENABLETYPE_WMDRMV7_LicenseAcquisition: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x003306df_4a06_4884_a097_ef6d22ec84a3);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFEVRDLL: u32 = 0u32;
pub const MFFLACBytestreamHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0e41cfb8_0506_40f4_a516_77cc23642d91);
pub const MFFLACSinkClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7d39c56f_6075_47c9_9bae_8cf9e531b5f5);
pub const MFImageFormat_JPEG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const MFImageFormat_RGB32: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000016_0000_0010_8000_00aa00389b71);
pub const MFMPEG4Format_Base: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000000_767a_494d_b478_f29d25dc9037);
pub const MFMediaType_Audio: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const MFMediaType_Binary: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72178c25_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_Default: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x81a412e6_8103_4b06_857f_1862781024ac);
pub const MFMediaType_FileTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72178c26_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_HTML: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72178c24_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_Image: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72178c23_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_Metadata: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2c8fa20c_82bb_4782_90a0_98a2a5bd8ef8);
pub const MFMediaType_MultiplexedFrames: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ea542b0_281f_4231_a464_fe2f5022501c);
pub const MFMediaType_Perception: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x597ff6f9_6ea2_4670_85b4_ea84073fe940);
pub const MFMediaType_Protected: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b4b6fe6_9d04_4494_be14_7e0bd076c8e4);
pub const MFMediaType_SAMI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe69669a0_3dcd_40cb_9e2e_3708387c0616);
pub const MFMediaType_Script: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72178c22_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_Stream: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe436eb83_524f_11ce_9f53_0020af0ba770);
pub const MFMediaType_Subtitle: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa6d13581_ed50_4e65_ae08_26065576aacc);
pub const MFMediaType_Video: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
pub const MFNETSOURCE_ACCELERATEDSTREAMINGDURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f277_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_AUTORECONNECTLIMIT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f27a_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_AUTORECONNECTPROGRESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f282_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_BROWSERUSERAGENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f28b_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_BROWSERWEBPAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f28c_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_BUFFERINGTIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f276_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_CACHEENABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f279_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_CLIENTGUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x60a2c4a6_f197_4c14_a5bf_88830d2458af);
pub const MFNETSOURCE_CONNECTIONBANDWIDTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f278_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_CREDENTIAL_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f280_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_CROSS_ORIGIN_SUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9842207c_b02c_4271_a2fc_72e49308e5c2);
pub const MFNETSOURCE_DRMNET_LICENSE_REPRESENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47eae1bd_bdfe_42e2_82f3_54a48c17962d);
pub const MFNETSOURCE_ENABLE_DOWNLOAD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f29d_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_HTTP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f299_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_MSB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f296_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_PRIVATEMODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x824779d8_f18b_4405_8cf1_464fb5aa8f71);
pub const MFNETSOURCE_ENABLE_RTSP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f298_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_STREAMING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f29c_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_TCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f295_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_UDP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f294_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_FRIENDLYNAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5b2a7757_bc6b_447e_aa06_0dda1c646e2f);
pub const MFNETSOURCE_HOSTEXE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f28f_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_HOSTVERSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f291_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_HTTP_DOWNLOAD_SESSION_PROVIDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7d55081e_307d_4d6d_a663_a93be97c4b5c);
pub const MFNETSOURCE_LOGPARAMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x64936ae8_9418_453a_8cda_3e0a668b353b);
pub const MFNETSOURCE_LOGURL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f293_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_MAXBUFFERTIMEMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x408b24e6_4038_4401_b5b2_fe701a9ebf10);
pub const MFNETSOURCE_MAXUDPACCELERATEDSTREAMINGDURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4aab2879_bbe1_4994_9ff0_5495bd250129);
pub const MFNETSOURCE_PEERMANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48b29adb_febf_45ee_a9bf_efb81c492efc);
pub const MFNETSOURCE_PLAYERID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f28e_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PLAYERUSERAGENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f292_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PLAYERVERSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f28d_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PPBANDWIDTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f281_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PREVIEWMODEENABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f27f_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROTOCOL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f27d_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYBYPASSFORLOCAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f286_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYEXCEPTIONLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f285_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYHOSTNAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f284_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYINFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f29b_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYLOCATORFACTORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f283_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f288_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYRERUNAUTODETECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f289_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYSETTINGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f287_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_RESENDSENABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f27b_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_RESOURCE_FILTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x815d0ff6_265a_4477_9e46_7b80ad80b5fb);
pub const MFNETSOURCE_SSLCERTIFICATE_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x55e6cb27_e69b_4267_940c_2d7ec5bb8a0f);
pub const MFNETSOURCE_STATISTICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f274_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_STATISTICS_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f275_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_STREAM_LANGUAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9ab44318_f7cd_4f2d_8d6d_fa35b492cecb);
pub const MFNETSOURCE_THINNINGENABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f27c_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_TRANSPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f27e_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_UDP_PORT_RANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cb1f29a_0505_4c5d_ae71_0a556344efa1);
pub const MFNET_SAVEJOB_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb85a587f_3d02_4e52_9565_55d3ec1e7ff7);
pub const MFPROTECTIONATTRIBUTE_BEST_EFFORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc8e06331_75f0_4ec1_8e77_17578f773b46);
pub const MFPROTECTIONATTRIBUTE_CONSTRICTVIDEO_IMAGESIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x008476fc_4b58_4d80_a790_e7297673161d);
pub const MFPROTECTIONATTRIBUTE_FAIL_OVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8536abc5_38f1_4151_9cce_f55d941229ac);
pub const MFPROTECTIONATTRIBUTE_HDCP_SRM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6f302107_3477_4468_8a08_eef9db10e20f);
pub const MFPROTECTION_ACP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc3fd11c6_f8b7_4d20_b008_1db17d61f2da);
pub const MFPROTECTION_CGMSA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe57e69e9_226b_4d31_b4e3_d3db008736dd);
pub const MFPROTECTION_CONSTRICTAUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xffc99b44_df48_4e16_8e66_096892c1578a);
pub const MFPROTECTION_CONSTRICTVIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x193370ce_c5e4_4c3a_8a66_6959b4da4442);
pub const MFPROTECTION_CONSTRICTVIDEO_NOOPM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa580e8cd_c247_4957_b983_3c2eebd1ff59);
pub const MFPROTECTION_DISABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8cc6d81b_fec6_4d8f_964b_cfba0b0dad0d);
pub const MFPROTECTION_DISABLE_SCREEN_SCRAPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa21179a4_b7cd_40d8_9614_8ef2371ba78d);
pub const MFPROTECTION_FFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x462a56b2_2866_4bb6_980d_6d8d9edb1a8c);
pub const MFPROTECTION_GRAPHICS_TRANSFER_AES_ENCRYPTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc873de64_d8a5_49e6_88bb_fb963fd3d4ce);
pub const MFPROTECTION_HARDWARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4ee7f0c1_9ed7_424f_b6be_996b33528856);
pub const MFPROTECTION_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xae7cc03d_c828_4021_acb7_d578d27aaf13);
pub const MFPROTECTION_HDCP_WITH_TYPE_ENFORCEMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa4a585e8_ed60_442d_814d_db4d4220a06d);
pub const MFPROTECTION_PROTECTED_SURFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4f5d9566_e742_4a25_8d1f_d287b5fa0ade);
pub const MFPROTECTION_TRUSTEDAUDIODRIVERS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x65bdf3d2_0168_4816_a533_55d47b027101);
pub const MFPROTECTION_VIDEO_FRAMES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36a59cbc_7401_4a8c_bc20_46a7c9e597f0);
pub const MFPROTECTION_WMDRMOTA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa267a6a1_362e_47d0_8805_4628598a23e4);
pub const MFP_POSITIONTYPE_100NS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFRR_INFO_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSEQUENCER_INVALID_ELEMENT_ID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSIONCAP_DOES_NOT_USE_NETWORK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSIONCAP_PAUSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSIONCAP_RATE_FORWARD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSIONCAP_RATE_REVERSE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSIONCAP_SEEK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSIONCAP_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSTARTUP_FULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSTARTUP_LITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSTARTUP_NOSOCKET: u32 = 1u32;
pub const MFSampleExtension_3DVideo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf86f97a4_dd54_4e2e_9a5e_55fc2d74a005);
pub const MFSampleExtension_3DVideo_SampleFormat: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x08671772_e36f_4cff_97b3_d72e20987a48);
pub const MFSampleExtension_AccumulatedNonRefPicPercent: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x79ea74df_a740_445b_bc98_c9ed1f260eee);
pub const MFSampleExtension_BottomFieldFirst: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x941ce0a3_6ae3_4dda_9a08_a64298340617);
pub const MFSampleExtension_CameraExtrinsics: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6b761658_b7ec_4c3b_8225_8623cabec31d);
pub const MFSampleExtension_CaptureMetadata: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2ebe23a8_faf5_444a_a6a2_eb810880ab5d);
pub const MFSampleExtension_ChromaOnly: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1eb9179c_a01f_4845_8c04_0e65a26eb04f);
pub const MFSampleExtension_CleanPoint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9cdf01d8_a0f0_43ba_b077_eaa06cbd728a);
pub const MFSampleExtension_ClosedCaption_CEA708: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x26f09068_e744_47dc_aa03_dbf20403bde6);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSampleExtension_ClosedCaption_CEA708_MAX_SIZE: u32 = 256u32;
pub const MFSampleExtension_Content_KeyID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc6c7f5b0_acca_415b_87d9_10441469efc6);
pub const MFSampleExtension_DecodeTimestamp: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73a954d4_09e2_4861_befc_94bd97c08e6e);
pub const MFSampleExtension_Depth_MaxReliableDepth: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe45545d1_1f0f_4a32_a8a7_6101a24ea8be);
pub const MFSampleExtension_Depth_MinReliableDepth: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5f8582b2_e36b_47c8_9b87_fee1ca72c5b0);
pub const MFSampleExtension_DerivedFromTopField: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6852465a_ae1c_4553_8e9b_c3420fcb1637);
pub const MFSampleExtension_DescrambleData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x43483be6_4903_4314_b032_2951365936fc);
pub const MFSampleExtension_DeviceReferenceSystemTime: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6523775a_ba2d_405f_b2c5_01ff88e2e8f6);
pub const MFSampleExtension_DeviceTimestamp: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8f3e35e7_2dcd_4887_8622_2a58baa652b0);
pub const MFSampleExtension_DirtyRects: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9ba70225_b342_4e97_9126_0b566ab7ea7e);
pub const MFSampleExtension_Discontinuity: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9cdf01d9_a0f0_43ba_b077_eaa06cbd728a);
pub const MFSampleExtension_Encryption_ClearSliceHeaderData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5509a4f4_320d_4e6c_8d1a_94c66dd20cb0);
pub const MFSampleExtension_Encryption_CryptByteBlock: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d84289b_0c7f_4713_ab95_108ab42ad801);
pub const MFSampleExtension_Encryption_HardwareProtection: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9a2b2d2b_8270_43e3_8448_994f426e8886);
pub const MFSampleExtension_Encryption_HardwareProtection_KeyInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb2372080_455b_4dd7_9989_1a955784b754);
pub const MFSampleExtension_Encryption_HardwareProtection_KeyInfoID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8cbfcceb_94a5_4de1_8231_a85e47cf81e7);
pub const MFSampleExtension_Encryption_HardwareProtection_VideoDecryptorContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x693470c8_e837_47a0_88cb_535b905e3582);
pub const MFSampleExtension_Encryption_KeyID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x76376591_795f_4da1_86ed_9d46eca109a9);
pub const MFSampleExtension_Encryption_NALUTypes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb0f067c7_714c_416c_8d59_5f4ddf8913b6);
pub const MFSampleExtension_Encryption_Opaque_Data: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x224d77e5_1391_4ffb_9f41_b432f68c611d);
pub const MFSampleExtension_Encryption_ProtectionScheme: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd054d096_28bb_45da_87ec_74f351871406);
pub const MFSampleExtension_Encryption_ResumeVideoOutput: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa435aba5_afde_4cf5_bc1c_f6acaf13949d);
pub const MFSampleExtension_Encryption_SEIData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cf0e972_4542_4687_9999_585f565fba7d);
pub const MFSampleExtension_Encryption_SPSPPSData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaede0fa2_0e0c_453c_b7f3_de8693364d11);
pub const MFSampleExtension_Encryption_SampleID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6698b84e_0afa_4330_aeb2_1c0a98d7a44d);
pub const MFSampleExtension_Encryption_SkipByteBlock: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0d550548_8317_4ab1_845f_d06306e293e3);
pub const MFSampleExtension_Encryption_SubSampleMappingSplit: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfe0254b9_2aa5_4edc_99f7_17e89dbf9174);
pub const MFSampleExtension_Encryption_SubSample_Mapping: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8444f27a_69a1_48da_bd08_11cef36830d2);
pub const MFSampleExtension_ExtendedCameraIntrinsics: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x560bc4a5_4de0_4113_9cdc_832db9740f3d);
pub const MFSampleExtension_FeatureMap: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa032d165_46fc_400a_b449_49de53e62a6e);
pub const MFSampleExtension_ForwardedDecodeUnitType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x089e57c7_47d3_4a26_bf9c_4b64fafb5d1e);
pub const MFSampleExtension_ForwardedDecodeUnits: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x424c754c_97c8_48d6_8777_fc41f7b60879);
pub const MFSampleExtension_FrameCorruption: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb4dd4a8c_0beb_44c4_8b75_b02b913b04f0);
pub const MFSampleExtension_GenKeyCtx: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x188120cb_d7da_4b59_9b3e_9252fd37301c);
pub const MFSampleExtension_GenKeyFunc: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x441ca1ee_6b1f_4501_903a_de87df42f6ed);
pub const MFSampleExtension_HDCP_FrameCounter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d389c60_f507_4aa6_a40a_71027a02f3de);
pub const MFSampleExtension_HDCP_OptionalHeader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9a2e7390_121f_455f_8376_c97428e0b540);
pub const MFSampleExtension_HDCP_StreamID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x177e5d74_c370_4a7a_95a2_36833c01d0af);
pub const MFSampleExtension_Interlaced: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb1d5830a_deb8_40e3_90fa_389943716461);
pub const MFSampleExtension_LastSlice: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2b5d5457_5547_4f07_b8c8_b4a3a9a1daac);
pub const MFSampleExtension_LongTermReferenceFrameInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9154733f_e1bd_41bf_81d3_fcd918f71332);
pub const MFSampleExtension_MDLCacheCookie: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5f002af9_d8f9_41a3_b6c3_a2ad43f647ad);
pub const MFSampleExtension_MULTIPLEXED_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8dcdee79_6b5a_4c45_8db9_20b395f02fcf);
pub const MFSampleExtension_MaxDecodeFrameSize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd3cc654f_f9f3_4a13_889f_f04eb2b5b957);
pub const MFSampleExtension_MeanAbsoluteDifference: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1cdbde11_08b4_4311_a6dd_0f9f371907aa);
pub const MFSampleExtension_MoveRegions: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe2a6c693_3a8b_4b8d_95d0_f60281a12fb7);
pub const MFSampleExtension_NALULengthInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x19124e7c_ad4b_465f_bb18_20186287b6af);
pub const MFSampleExtension_PacketCrossOffsets: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2789671d_389f_40bb_90d9_c282f77f9abd);
pub const MFSampleExtension_PhotoThumbnail: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x74bbc85c_c8bb_42dc_b586_da17ffd35dcc);
pub const MFSampleExtension_PhotoThumbnailMediaType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x61ad5420_ebf8_4143_89af_6bf25f672def);
pub const MFSampleExtension_PinholeCameraIntrinsics: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4ee3b6c5_6a15_4e72_9761_70c1db8b9fe3);
pub const MFSampleExtension_ROIRectangle: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3414a438_4998_4d2c_be82_be3ca0b24d43);
pub const MFSampleExtension_RepeatFirstField: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x304d257c_7493_4fbd_b149_9228de8d9a99);
pub const MFSampleExtension_RepeatFrame: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x88be738f_0711_4f42_b458_344aed42ec2f);
pub const MFSampleExtension_SampleKeyID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9ed713c8_9b87_4b26_8297_a93b0c5a8acc);
pub const MFSampleExtension_SingleField: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d85f816_658b_455a_bde0_9fa7e15ab8f9);
pub const MFSampleExtension_Spatial_CameraCoordinateSystem: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d13c82f_2199_4e67_91cd_d1a4181f2534);
pub const MFSampleExtension_Spatial_CameraProjectionTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47f9fcb5_2a02_4f26_a477_792fdf95886a);
pub const MFSampleExtension_Spatial_CameraViewTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4e251fa4_830f_4770_859a_4b8d99aa809b);
pub const MFSampleExtension_TargetGlobalLuminance: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3f60ef36_31ef_4daf_8360_940397e41ef3);
pub const MFSampleExtension_Timestamp: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1e436999_69be_4c7a_9369_70068c0260cb);
pub const MFSampleExtension_Token: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8294da66_f328_4805_b551_00deb4c57a61);
pub const MFSampleExtension_VideoDSPMode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc12d55cb_d7d9_476d_81f3_69117f163ea0);
pub const MFSampleExtension_VideoEncodePictureType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x973704e6_cd14_483c_8f20_c9fc0928bad5);
pub const MFSampleExtension_VideoEncodeQP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb2efe478_f979_4c66_b95e_ee2b82c82f36);
pub const MFStreamExtension_CameraExtrinsics: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x686196d0_13e2_41d9_9638_ef032c272a52);
pub const MFStreamExtension_ExtendedCameraIntrinsics: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaa74b3df_9a2c_48d6_8393_5bd1c1a81e6e);
pub const MFStreamExtension_PinholeCameraIntrinsics: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdbac0455_0ec8_4aef_9c32_7a3ee3456f53);
pub const MFStreamFormat_MPEG2Program: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x263067d1_d330_45dc_b669_34d986e4e3e1);
pub const MFStreamFormat_MPEG2Transport: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe06d8023_db46_11cf_b4d1_00805f6cbbea);
pub const MFSubtitleFormat_ATSC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7fa7faa3_feae_4e16_aedf_36b9acfbb099);
pub const MFSubtitleFormat_CustomUserData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1bb3d849_6614_4d80_8882_ed24aa82da92);
pub const MFSubtitleFormat_PGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x71f40e4a_1278_4442_b30d_39dd1d7722bc);
pub const MFSubtitleFormat_SRT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5e467f2e_77ca_4ca5_8391_d142ed4b76c8);
pub const MFSubtitleFormat_SSA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57176a1b_1a9e_4eea_abef_c61760198ac4);
pub const MFSubtitleFormat_TTML: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73e73992_9a10_4356_9557_7194e91e3e54);
pub const MFSubtitleFormat_VobSub: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6b8e40f4_8d2c_4ced_ad91_5960e45b4433);
pub const MFSubtitleFormat_WebVTT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc886d215_f485_40bb_8db6_fadbc619a45d);
pub const MFSubtitleFormat_XML: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2006f94f_29ca_4195_b8db_00ded8ff0c97);
pub const MFT_AUDIO_DECODER_AUDIO_ENDPOINT_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc7ccdd6e_5398_4695_8be7_51b3e95111bd);
pub const MFT_AUDIO_DECODER_DEGRADATION_INFO_ATTRIBUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c3386ad_ec20_430d_b2a5_505c7178d9c4);
pub const MFT_AUDIO_DECODER_SPATIAL_METADATA_CLIENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05987df4_1270_4999_925f_8e939a7c0af7);
pub const MFT_CATEGORY_AUDIO_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9ea73fb4_ef7a_4559_8d5d_719d8f0426c7);
pub const MFT_CATEGORY_AUDIO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x11064c48_3648_4ed0_932e_05ce8ac811b7);
pub const MFT_CATEGORY_AUDIO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x91c64bd0_f91e_4d8c_9276_db248279d975);
pub const MFT_CATEGORY_DEMULTIPLEXER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa8700a7a_939b_44c5_99d7_76226b23b3f1);
pub const MFT_CATEGORY_ENCRYPTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb0c687be_01cd_44b5_b8b2_7c1d7e058b1f);
pub const MFT_CATEGORY_MULTIPLEXER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x059c561e_05ae_4b61_b69d_55b61ee54a7b);
pub const MFT_CATEGORY_OTHER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x90175d57_b7ea_4901_aeb3_933a8747756f);
pub const MFT_CATEGORY_VIDEO_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd6c02d4b_6833_45b4_971a_05a4b04bab91);
pub const MFT_CATEGORY_VIDEO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x12e17c21_532c_4a6e_8a1c_40825a736397);
pub const MFT_CATEGORY_VIDEO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf79eac7d_e545_4387_bdee_d647d7bde42a);
pub const MFT_CATEGORY_VIDEO_PROCESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x302ea3fc_aa5f_47f9_9f7a_c2188bb16302);
pub const MFT_CATEGORY_VIDEO_RENDERER_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x145cd8b4_92f4_4b23_8ae7_e0df06c2da95);
pub const MFT_CODEC_MERIT_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x88a7cb15_7b07_4a34_9128_e64c6703c4d3);
pub const MFT_CONNECTED_STREAM_ATTRIBUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x71eeb820_a59f_4de2_bcec_38db1dd611a4);
pub const MFT_CONNECTED_TO_HW_STREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34e6e728_06d6_4491_a553_4795650db912);
pub const MFT_DECODER_EXPOSE_OUTPUT_TYPES_IN_NATIVE_ORDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xef80833f_f8fa_44d9_80d8_41ed6232670c);
pub const MFT_DECODER_FINAL_VIDEO_RESOLUTION_HINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdc2f8496_15c4_407a_b6f0_1b66ab5fbf53);
pub const MFT_DECODER_QUALITY_MANAGEMENT_CUSTOM_CONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa24e30d7_de25_4558_bbfb_71070a2d332e);
pub const MFT_DECODER_QUALITY_MANAGEMENT_RECOVERY_WITHOUT_ARTIFACTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd8980deb_0a48_425f_8623_611db41d3810);
pub const MFT_ENCODER_ERROR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc8d1eda4_98e4_41d5_9297_44f53852f90e);
pub const MFT_ENCODER_SUPPORTS_CONFIG_EVENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x86a355ae_3a77_4ec4_9f31_01149a4e92de);
pub const MFT_END_STREAMING_AWARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x70fbc845_b07e_4089_b064_399dc6110f29);
pub const MFT_ENUM_ADAPTER_LUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1d39518c_e220_4da8_a07f_ba172552d6b1);
pub const MFT_ENUM_HARDWARE_URL_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2fb866ac_b078_4942_ab6c_003d05cda674);
pub const MFT_ENUM_HARDWARE_VENDOR_ID_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3aecb0cc_035b_4bcc_8185_2b8d551ef3af);
pub const MFT_ENUM_TRANSCODE_ONLY_ATTRIBUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x111ea8cd_b62a_4bdb_89f6_67ffcdc2458b);
pub const MFT_ENUM_VIDEO_RENDERER_EXTENSION_PROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62c56928_9a4e_443b_b9dc_cac830c24100);
pub const MFT_FIELDOFUSE_UNLOCK_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8ec2e9fd_9148_410d_831e_702439461a8e);
pub const MFT_FRIENDLY_NAME_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x314ffbae_5b41_4c95_9c19_4e7d586face3);
pub const MFT_GFX_DRIVER_VERSION_ID_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf34b9093_05e0_4b16_993d_3e2a2cde6ad3);
pub const MFT_HW_TIMESTAMP_WITH_QPC_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8d030fb8_cc43_4258_a22e_9210bef89be4);
pub const MFT_INPUT_TYPES_Attributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4276c9b1_759d_4bf3_9cd0_0d723d138f96);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_BOUND_UPPER_UNBOUNDED: u64 = 9223372036854775807u64;
pub const MFT_OUTPUT_TYPES_Attributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8eae8cf3_a44f_4306_ba5c_bf5dda242818);
pub const MFT_POLICY_SET_AWARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5a633b19_cc39_4fa8_8ca5_59981b7a0018);
pub const MFT_PREFERRED_ENCODER_PROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x53004909_1ef5_46d7_a18e_5a75f8b5905f);
pub const MFT_PREFERRED_OUTPUTTYPE_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7e700499_396a_49ee_b1b4_f628021e8c9d);
pub const MFT_PROCESS_LOCAL_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x543186e4_4649_4e65_b588_4aa352aff379);
pub const MFT_REMUX_MARK_I_PICTURE_AS_CLEAN_POINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x364e8f85_3f2e_436c_b2a2_4440a012a9e8);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_STREAMS_UNLIMITED: u32 = 4294967295u32;
pub const MFT_SUPPORT_3DVIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x093f81b1_4f2e_4631_8168_7934032a01d3);
pub const MFT_SUPPORT_DYNAMIC_FORMAT_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x53476a11_3f13_49fb_ac42_ee2733c96741);
pub const MFT_TRANSFORM_CLSID_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6821c42b_65a4_4e82_99bc_9a88205ecd0c);
pub const MFT_USING_HARDWARE_DRM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34faa77d_d79e_4957_b8ce_362b2684996c);
pub const MFTranscodeContainerType_3GP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34c50167_4472_4f34_9ea0_c49fbacf037d);
pub const MFTranscodeContainerType_AC3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d8d91c3_8c91_4ed1_8742_8c347d5b44d0);
pub const MFTranscodeContainerType_ADTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x132fd27d_0f02_43de_a301_38fbbbb3834e);
pub const MFTranscodeContainerType_AMR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x025d5ad3_621a_475b_964d_66b1c824f079);
pub const MFTranscodeContainerType_ASF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x430f6f6e_b6bf_4fc1_a0bd_9ee46eee2afb);
pub const MFTranscodeContainerType_AVI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7edfe8af_402f_4d76_a33c_619fd157d0f1);
pub const MFTranscodeContainerType_FLAC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31344aa3_05a9_42b5_901b_8e9d4257f75e);
pub const MFTranscodeContainerType_FMPEG4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9ba876f1_419f_4b77_a1e0_35959d9d4004);
pub const MFTranscodeContainerType_MP3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe438b912_83f1_4de6_9e3a_9ffbc6dd24d1);
pub const MFTranscodeContainerType_MPEG2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbfc2dbf9_7bb4_4f8f_afde_e112c44ba882);
pub const MFTranscodeContainerType_MPEG4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdc6cd05d_b9d0_40ef_bd35_fa622c1ab28a);
pub const MFTranscodeContainerType_WAVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x64c3453c_0f26_4741_be63_87bdf8bb935b);
pub const MFVideoFormat_420O: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4f303234_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_A16B16G16R16F: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000071_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_A2R10G10B10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0000001f_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_AI44: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34344941_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_ARGB32: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000015_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_AV1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31305641_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_AYUV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x56555941_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Base: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Base_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeac3b9d5_bd14_4237_8f1f_bab428e49312);
pub const MFVideoFormat_D16: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000050_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_DV25: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x35327664_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_DV50: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30357664_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_DVH1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31687664_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_DVHD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x64687664_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_DVSD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x64737664_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_DVSL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c737664_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_H263: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33363248_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_H264: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34363248_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_H264_ES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3f40f4f0_5622_4ff8_b6d8_a17a584bee5e);
pub const MFVideoFormat_H264_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5d0ce9dd_9817_49da_bdfd_f5f5b98f18a6);
pub const MFVideoFormat_H265: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x35363248_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_HEVC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x43564548_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_HEVC_ES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x53564548_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_HEVC_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3cfe0fe6_05c4_47dc_9d70_4bdb2959720f);
pub const MFVideoFormat_I420: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_IYUV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x56555949_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_L16: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000051_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_L8: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000032_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_M4S2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_MJPG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47504a4d_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_MP43: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_MP4S: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_MP4V: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5634504d_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_MPEG2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const MFVideoFormat_MPG1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3147504d_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_MSS1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_MSS2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_NV11: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3131564e_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_NV12: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3231564e_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_NV21: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3132564e_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_ORAW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5741524f_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_P010: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313050_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_P016: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36313050_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_P210: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313250_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_P216: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36313250_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_RGB24: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000014_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_RGB32: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000016_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_RGB555: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000018_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_RGB565: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000017_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_RGB8: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000029_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Theora: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6f656874_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_UYVY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x59565955_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_VP10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30315056_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_VP80: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30385056_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_VP90: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30395056_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_WMV1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_WMV2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_WMV3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_WVC1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Y210: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313259_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Y216: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36313259_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Y410: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313459_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Y416: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36313459_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Y41P: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50313459_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Y41T: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x54313459_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_Y42T: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x54323459_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_YUY2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32595559_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_YV12: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32315659_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_YVU9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x39555659_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_YVYU: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x55595659_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_v210: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313276_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_v216: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36313276_0000_0010_8000_00aa00389b71);
pub const MFVideoFormat_v410: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30313476_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_1024_BYTE_ALIGNMENT: u32 = 1023u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_128_BYTE_ALIGNMENT: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_16_BYTE_ALIGNMENT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_1_BYTE_ALIGNMENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_2048_BYTE_ALIGNMENT: u32 = 2047u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_256_BYTE_ALIGNMENT: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_2_BYTE_ALIGNMENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_32_BYTE_ALIGNMENT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_4096_BYTE_ALIGNMENT: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_4_BYTE_ALIGNMENT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_512_BYTE_ALIGNMENT: u32 = 511u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_64_BYTE_ALIGNMENT: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_8192_BYTE_ALIGNMENT: u32 = 8191u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_8_BYTE_ALIGNMENT: u32 = 7u32;
pub const MF_ACCESS_CONTROLLED_MEDIASOURCE_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x014a5031_2f05_4c6a_9f9c_7d0dc4eda5f4);
pub const MF_ACTIVATE_CUSTOM_VIDEO_MIXER_ACTIVATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xba491361_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_MIXER_CLSID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xba491360_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_MIXER_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xba491362_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_ACTIVATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xba491365_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_CLSID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xba491364_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xba491366_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_MFT_LOCKED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc1f6093c_7f65_4fbd_9e39_5faec3c4fbd7);
pub const MF_ACTIVATE_VIDEO_WINDOW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9a2dbbdd_f57e_4162_82b9_6831377682d3);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_API_VERSION: u32 = 112u32;
pub const MF_ASFPROFILE_MAXPACKETSIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x22587627_47de_4168_87f5_b5aa9b12a8f0);
pub const MF_ASFPROFILE_MINPACKETSIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x22587626_47de_4168_87f5_b5aa9b12a8f0);
pub const MF_ASFSTREAMCONFIG_LEAKYBUCKET1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc69b5901_ea1a_4c9b_b692_e2a0d29a8add);
pub const MF_ASFSTREAMCONFIG_LEAKYBUCKET2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc69b5902_ea1a_4c9b_b692_e2a0d29a8add);
pub const MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb10aaec3_ef71_4cc3_b873_05a9a08b9f8e);
pub const MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ROLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ba644ff_27c5_4d02_9887_c28619fdb91b);
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xede4b5e0_f805_4d6c_99b3_db01bf95dfab);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_CROSSPROCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_DONT_ALLOW_FORMAT_CHANGES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_NOPERSIST: u32 = 2u32;
pub const MF_AUDIO_RENDERER_ATTRIBUTE_SESSION_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xede4b5e3_f805_4d6c_99b3_db01bf95dfab);
pub const MF_AUDIO_RENDERER_ATTRIBUTE_STREAM_CATEGORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa9770471_92ec_4df4_94fe_81c36f0c3a7a);
pub const MF_BD_MVC_PLANE_OFFSET_METADATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62a654e4_b76c_4901_9823_2cb615d47318);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_BOOT_DRIVER_VERIFICATION_FAILED: u32 = 1048576u32;
pub const MF_BYTESTREAMHANDLER_ACCEPTS_SHARE_WRITE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa6e1f733_3001_4915_8150_1558a2180ec8);
pub const MF_BYTESTREAM_CONTENT_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc358289_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_DLNA_PROFILE_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc35828d_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_DURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc35828a_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_EFFECTIVE_URL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9afa0209_89d1_42af_8456_1de6b562d691);
pub const MF_BYTESTREAM_IFO_FILE_URI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc35828c_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_LAST_MODIFIED_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc35828b_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_ORIGIN_NAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc358288_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xab025e2b_16d9_4180_a127_ba6c70156161);
pub const MF_BYTESTREAM_TRANSCODED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb6c5c282_4dc9_4db9_ab48_cf3b6d8bc5e0);
pub const MF_CAPTURE_ENGINE_ALL_EFFECTS_REMOVED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfded7521_8ed8_431a_a96b_f3e2565e981c);
pub const MF_CAPTURE_ENGINE_AUDIO_PROCESSING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x10f1be5e_7e11_410b_973d_f4b6109000fe);
pub const MF_CAPTURE_ENGINE_CAMERA_STREAM_BLOCKED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa4209417_8d39_46f3_b759_5912528f4207);
pub const MF_CAPTURE_ENGINE_CAMERA_STREAM_UNBLOCKED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9be9eef0_cdaf_4717_8564_834aae66415c);
pub const MF_CAPTURE_ENGINE_D3D_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x76e25e7b_d595_4283_962c_c594afd78ddf);
pub const MF_CAPTURE_ENGINE_DECODER_MFT_FIELDOFUSE_UNLOCK_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2b8ad2e8_7acb_4321_a606_325c4249f4fc);
pub const MF_CAPTURE_ENGINE_DISABLE_DXVA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf9818862_179d_433f_a32f_74cbcf74466d);
pub const MF_CAPTURE_ENGINE_DISABLE_HARDWARE_TRANSFORMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb7c42a6b_3207_4495_b4e7_81f9c35d5991);
pub const MF_CAPTURE_ENGINE_EFFECT_ADDED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaa8dc7b5_a048_4e13_8ebe_f23c46c830c1);
pub const MF_CAPTURE_ENGINE_EFFECT_REMOVED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc6e8db07_fb09_4a48_89c6_bf92a04222c9);
pub const MF_CAPTURE_ENGINE_ENABLE_CAMERA_STREAMSTATE_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4c808e9d_aaed_4713_90fb_cb24064ab8da);
pub const MF_CAPTURE_ENGINE_ENCODER_MFT_FIELDOFUSE_UNLOCK_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x54c63a00_78d5_422f_aa3e_5e99ac649269);
pub const MF_CAPTURE_ENGINE_ERROR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x46b89fc6_33cc_4399_9dad_784de77d587c);
pub const MF_CAPTURE_ENGINE_EVENT_GENERATOR_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xabfa8ad5_fc6d_4911_87e0_961945f8f7ce);
pub const MF_CAPTURE_ENGINE_EVENT_STREAM_INDEX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x82697f44_b1cf_42eb_9753_f86d649c8865);
pub const MF_CAPTURE_ENGINE_INITIALIZED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x219992bc_cf92_4531_a1ae_96e1e886c8f1);
pub const MF_CAPTURE_ENGINE_MEDIASOURCE_CONFIG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc6989d2_0fc1_46e1_a74f_efd36bc788de);
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8e3f5bd5_dbbf_42f0_8542_d07a3971762a);
pub const MF_CAPTURE_ENGINE_OUTPUT_MEDIA_TYPE_SET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcaaad994_83ec_45e9_a30a_1f20aadb9831);
pub const MF_CAPTURE_ENGINE_PHOTO_TAKEN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3c50c445_7304_48eb_865d_bba19ba3af5c);
pub const MF_CAPTURE_ENGINE_PREVIEW_STARTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa416df21_f9d3_4a74_991b_b817298952c4);
pub const MF_CAPTURE_ENGINE_PREVIEW_STOPPED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x13d5143c_1edd_4e50_a2ef_350a47678060);
pub const MF_CAPTURE_ENGINE_RECORD_SINK_AUDIO_MAX_PROCESSED_SAMPLES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9896e12a_f707_4500_b6bd_db8eb810b50f);
pub const MF_CAPTURE_ENGINE_RECORD_SINK_AUDIO_MAX_UNPROCESSED_SAMPLES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1cddb141_a7f4_4d58_9896_4d15a53c4efe);
pub const MF_CAPTURE_ENGINE_RECORD_SINK_VIDEO_MAX_PROCESSED_SAMPLES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe7b4a49e_382c_4aef_a946_aed5490b7111);
pub const MF_CAPTURE_ENGINE_RECORD_SINK_VIDEO_MAX_UNPROCESSED_SAMPLES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb467f705_7913_4894_9d42_a215fea23da9);
pub const MF_CAPTURE_ENGINE_RECORD_STARTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac2b027b_ddf9_48a0_89be_38ab35ef45c0);
pub const MF_CAPTURE_ENGINE_RECORD_STOPPED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x55e5200a_f98f_4c0d_a9ec_9eb25ed3d773);
pub const MF_CAPTURE_ENGINE_SELECTEDCAMERAPROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x03160b7e_1c6f_4db2_ad56_a7c430f82392);
pub const MF_CAPTURE_ENGINE_SELECTEDCAMERAPROFILE_INDEX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3ce88613_2214_46c3_b417_82f8a313c9c3);
pub const MF_CAPTURE_ENGINE_USE_AUDIO_DEVICE_ONLY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1c8077da_8466_4dc4_8b8e_276b3f85923b);
pub const MF_CAPTURE_ENGINE_USE_VIDEO_DEVICE_ONLY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7e025171_cf32_4f2e_8f19_410577b73a66);
pub const MF_CAPTURE_METADATA_DIGITALWINDOW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x276f72a2_59c8_4f69_97b4_068b8c0ec044);
pub const MF_CAPTURE_METADATA_EXIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2e9575b8_8c31_4a02_8575_42b197b71592);
pub const MF_CAPTURE_METADATA_EXPOSURE_COMPENSATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd198aa75_4b62_4345_abf3_3c31fa12c299);
pub const MF_CAPTURE_METADATA_EXPOSURE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x16b9ae99_cd84_4063_879d_a28c7633729e);
pub const MF_CAPTURE_METADATA_FACEROICHARACTERIZATIONS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb927a1a8_18ef_46d3_b3af_69372f94d9b2);
pub const MF_CAPTURE_METADATA_FACEROIS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x864f25a6_349f_46b1_a30e_54cc22928a47);
pub const MF_CAPTURE_METADATA_FACEROITIMESTAMPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe94d50cc_3da0_44d4_bb34_83198a741868);
pub const MF_CAPTURE_METADATA_FIRST_SCANLINE_START_TIME_QPC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6a2c49f1_e052_46b6_b2d9_73c1558709af);
pub const MF_CAPTURE_METADATA_FLASH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4a51520b_fb36_446c_9df2_68171b9a0389);
pub const MF_CAPTURE_METADATA_FLASH_POWER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9c0e0d49_0205_491a_bc9d_2d6e1f4d5684);
pub const MF_CAPTURE_METADATA_FOCUSSTATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa87ee154_997f_465d_b91f_29d53b982b88);
pub const MF_CAPTURE_METADATA_FRAME_BACKGROUND_MASK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x03f14dd3_75dd_433a_a8e2_1e3f5f2a50a0);
pub const MF_CAPTURE_METADATA_FRAME_ILLUMINATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d688ffc_63d3_46fe_bada_5b947db0d080);
pub const MF_CAPTURE_METADATA_FRAME_RAWSTREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9252077b_2680_49b9_ae02_b19075973b70);
pub const MF_CAPTURE_METADATA_HISTOGRAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x85358432_2ef6_4ba9_a3fb_06d82974b895);
pub const MF_CAPTURE_METADATA_ISO_GAINS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05802ac9_0e1d_41c7_a8c8_7e7369f84e1e);
pub const MF_CAPTURE_METADATA_ISO_SPEED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe528a68f_b2e3_44fe_8b65_07bf4b5a13ff);
pub const MF_CAPTURE_METADATA_LAST_SCANLINE_END_TIME_QPC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdccadecb_c4d4_400d_b418_10e88525e1f6);
pub const MF_CAPTURE_METADATA_LENS_POSITION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb5fc8e86_11d1_4e70_819b_723a89fa4520);
pub const MF_CAPTURE_METADATA_PHOTO_FRAME_FLASH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0f9dd6c6_6003_45d8_bd59_f1f53e3d04e8);
pub const MF_CAPTURE_METADATA_REQUESTED_FRAME_SETTING_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbb3716d9_8a61_47a4_8197_459c7ff174d5);
pub const MF_CAPTURE_METADATA_SCANLINE_DIRECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6496a3ba_1907_49e6_b0c3_123795f380a9);
pub const MF_CAPTURE_METADATA_SCANLINE_TIME_QPC_ACCURACY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4cd79c51_f765_4b09_b1e1_27d1f7ebea09);
pub const MF_CAPTURE_METADATA_SCENE_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9cc3b54d_5ed3_4bae_b388_7670aef59e13);
pub const MF_CAPTURE_METADATA_SENSORFRAMERATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdb51357e_9d3d_4962_b06d_07ce650d9a0a);
pub const MF_CAPTURE_METADATA_UVC_PAYLOADHEADER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf9f88a87_e1dd_441e_95cb_42e21a64f1d9);
pub const MF_CAPTURE_METADATA_WHITEBALANCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc736fd77_0fb9_4e2e_97a2_fcd490739ee9);
pub const MF_CAPTURE_METADATA_WHITEBALANCE_GAINS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe7570c8f_2dcb_4c7c_aace_22ece7cce647);
pub const MF_CAPTURE_METADATA_ZOOMFACTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe50b0b81_e501_42c2_abf2_857ecb13fa5c);
pub const MF_CAPTURE_SINK_PREPARED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7bfce257_12b1_4409_8c34_d445daab7578);
pub const MF_CAPTURE_SOURCE_CURRENT_DEVICE_MEDIA_TYPE_SET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe7e75e4c_039c_4410_815b_8741307b63aa);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_COMPONENT_CERT_REVOKED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_COMPONENT_HS_CERT_REVOKED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_COMPONENT_INVALID_EKU: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_COMPONENT_INVALID_ROOT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_COMPONENT_LS_CERT_REVOKED: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_COMPONENT_REVOKED: u32 = 8192u32;
pub const MF_CONTENTDECRYPTIONMODULE_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x15320c45_ff80_484a_9dcb_0df894e69a01);
pub const MF_CONTENT_DECRYPTOR_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x68a72927_fc7b_44ee_85f4_7c51bd55a659);
pub const MF_CONTENT_PROTECTION_DEVICE_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xff58436f_76a0_41fe_b566_10cc53962edd);
pub const MF_D3D12_SYNCHRONIZATION_OBJECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a7c8d6a_85a6_494d_a046_06ea1a138f4b);
pub const MF_DECODER_FWD_CUSTOM_SEI_DECODE_ORDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf13bbe3c_36d4_410a_b985_7a951a1e6294);
pub const MF_DEVICEMFT_CONNECTED_FILTER_KSCONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6a2c4fa6_d179_41cd_9523_822371ea40e5);
pub const MF_DEVICEMFT_CONNECTED_PIN_KSCONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe63310f7_b244_4ef8_9a7d_24c74e32ebd0);
pub const MF_DEVICEMFT_EXTENSION_PLUGIN_CLSID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0844dbae_34fa_48a0_a783_8e696fb1c9a8);
pub const MF_DEVICEMFT_SENSORPROFILE_COLLECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36ebdc44_b12c_441b_89f4_08b2f41a9cfc);
pub const MF_DEVICESTREAM_ATTRIBUTE_FACEAUTH_CAPABILITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcb6fd12a_2248_4e41_ad46_e78bb90ab9fc);
pub const MF_DEVICESTREAM_ATTRIBUTE_FRAMESOURCE_TYPES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x17145fd1_1b2b_423c_8001_2b6833ed3588);
pub const MF_DEVICESTREAM_ATTRIBUTE_SECURE_CAPABILITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x940fd626_ea6e_4684_9840_36bd6ec9fbef);
pub const MF_DEVICESTREAM_EXTENSION_PLUGIN_CLSID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x048e6558_60c4_4173_bd5b_6a3ca2896aee);
pub const MF_DEVICESTREAM_EXTENSION_PLUGIN_CONNECTION_POINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x37f9375c_e664_4ea4_aae4_cb6d1daca1f4);
pub const MF_DEVICESTREAM_FILTER_KSCONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x46783cca_3df5_4923_a9ef_36b7223edde0);
pub const MF_DEVICESTREAM_FRAMESERVER_HIDDEN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf402567b_4d91_4179_96d1_74c8480c2034);
pub const MF_DEVICESTREAM_FRAMESERVER_SHARED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1cb378e9_b279_41d4_af97_34a243e68320);
pub const MF_DEVICESTREAM_IMAGE_STREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa7ffb865_e7b2_43b0_9f6f_9af2a0e50fc0);
pub const MF_DEVICESTREAM_INDEPENDENT_IMAGE_STREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x03eeec7e_d605_4576_8b29_6580b490d7d3);
pub const MF_DEVICESTREAM_MAX_FRAME_BUFFERS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1684cebe_3175_4985_882c_0efd3e8ac11e);
pub const MF_DEVICESTREAM_MULTIPLEXED_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ea542b0_281f_4231_a464_fe2f5022501c);
pub const MF_DEVICESTREAM_PIN_KSCONTROL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xef3ef9a7_87f2_48ca_be02_674878918e98);
pub const MF_DEVICESTREAM_REQUIRED_CAPABILITIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d8b957e_7cf6_43f4_af56_9c0e1e4fcbe1);
pub const MF_DEVICESTREAM_REQUIRED_SDDL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x331ae85d_c0d3_49ba_83ba_82a12d63cdd6);
pub const MF_DEVICESTREAM_SENSORSTREAM_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe35b9fe4_0659_4cad_bb51_33160be7e413);
pub const MF_DEVICESTREAM_SOURCE_ATTRIBUTES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2f8cb617_361b_434f_85ea_99a03e1ce4e0);
pub const MF_DEVICESTREAM_STREAM_CATEGORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2939e7b8_a62e_4579_b674_d4073dfabbba);
pub const MF_DEVICESTREAM_STREAM_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x11bd5120_d124_446b_88e6_17060257fff9);
pub const MF_DEVICESTREAM_TAKEPHOTO_TRIGGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1d180e34_538c_4fbb_a75a_859af7d261a6);
pub const MF_DEVICESTREAM_TRANSFORM_STREAM_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe63937b7_daaf_4d49_815f_d826f8ad31e7);
pub const MF_DEVICE_THERMAL_STATE_CHANGED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x70ccd0af_fc9f_4deb_a875_9fecd16c5bd4);
pub const MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x60d0e559_52f8_4fa2_bbce_acdb34a8ec01);
pub const MF_DEVSOURCE_ATTRIBUTE_MEDIA_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x56a819ca_0c78_4de4_a0a7_3ddaba0f24d4);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_PASSWORD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0fd7e16_42d9_49df_84c0_e82c5eab8874);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_STREAM_URL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d7b40d2_3617_4043_93e3_8d6da9bb3492);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc60ac5fe_252a_478f_a0ef_bc8fa5f7cad3);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ENDPOINT_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30da9258_feb9_47a7_a453_763a7a8e1c5f);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x14dd9a1c_7cff_41be_b1b9_ba1ac6ecb571);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ROLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbc9d118e_8c67_4a18_85d4_12d300400552);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_SYMBOLIC_LINK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x98d24b5e_5930_4614_b5a1_f600f9355a78);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_CATEGORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x77f0ae69_c3bd_4509_941d_467e4d24899e);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8ac3587a_4ae7_42d8_99e0_0a6013eef90f);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_HW_SOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xde7046ba_54d6_4487_a2a4_ec7c0d1bd163);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_MAX_BUFFERS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7dd9b730_4f2d_41d5_8f95_0cc9a912ba26);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_PROVIDER_DEVICE_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36689d42_a06c_40ae_84cf_f5a034067cc4);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_SYMBOLIC_LINK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x58f0aad8_22bf_4f8a_bb3d_d2c4978c6e2f);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_USERNAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x05d01add_949f_46eb_bc8e_8b0d2b32d79d);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_XADDRESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbca0be52_c327_44c7_9b7d_7fa8d9b5bcda);
pub const MF_DISABLE_FRAME_CORRUPTION_INFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7086e16c_49c5_4201_882a_8538f38cf13a);
pub const MF_DISABLE_LOCALLY_REGISTERED_PLUGINS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x66b16da9_add4_47e0_a16b_5af1fb483634);
pub const MF_DMFT_FRAME_BUFFER_INFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x396ce1c9_67a9_454c_8797_95a45799d804);
pub const MF_ENABLE_3DVIDEO_OUTPUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbdad7bca_0e5f_4b10_ab16_26de381b6293);
pub const MF_EVENT_DO_THINNING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x321ea6fb_dad9_46e4_b31d_d2eae7090e30);
pub const MF_EVENT_MFT_CONTEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb7cd31f1_899e_4b41_80c9_26a896d32977);
pub const MF_EVENT_MFT_INPUT_STREAM_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf29c2cca_7ae6_42d2_b284_bf837cc874e2);
pub const MF_EVENT_OUTPUT_NODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x830f1a8b_c060_46dd_a801_1c95dec9b107);
pub const MF_EVENT_PRESENTATION_TIME_OFFSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5ad914d1_9b45_4a8d_a2c0_81d1e50bfb07);
pub const MF_EVENT_SCRUBSAMPLE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9ac712b3_dcb8_44d5_8d0c_37455a2782e3);
pub const MF_EVENT_SESSIONCAPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7e5ebcd0_11b8_4abe_afad_10f6599a7f42);
pub const MF_EVENT_SESSIONCAPS_DELTA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7e5ebcd1_11b8_4abe_afad_10f6599a7f42);
pub const MF_EVENT_SOURCE_ACTUAL_START: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa8cc55a9_6b31_419f_845d_ffb351a2434b);
pub const MF_EVENT_SOURCE_CHARACTERISTICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47db8490_8b22_4f52_afda_9ce1b2d3cfa8);
pub const MF_EVENT_SOURCE_CHARACTERISTICS_OLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47db8491_8b22_4f52_afda_9ce1b2d3cfa8);
pub const MF_EVENT_SOURCE_FAKE_START: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa8cc55a7_6b31_419f_845d_ffb351a2434b);
pub const MF_EVENT_SOURCE_PROJECTSTART: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa8cc55a8_6b31_419f_845d_ffb351a2434b);
pub const MF_EVENT_SOURCE_TOPOLOGY_CANCELED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdb62f650_9a5e_4704_acf3_563bc6a73364);
pub const MF_EVENT_START_PRESENTATION_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5ad914d0_9b45_4a8d_a2c0_81d1e50bfb07);
pub const MF_EVENT_START_PRESENTATION_TIME_AT_OUTPUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5ad914d2_9b45_4a8d_a2c0_81d1e50bfb07);
pub const MF_EVENT_STREAM_METADATA_CONTENT_KEYIDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5063449d_cc29_4fc6_a75a_d247b35af85c);
pub const MF_EVENT_STREAM_METADATA_KEYDATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcd59a4a1_4a3b_4bbd_8665_72a40fbea776);
pub const MF_EVENT_STREAM_METADATA_SYSTEMID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1ea2ef64_ba16_4a36_8719_fe7560ba32ad);
pub const MF_EVENT_TOPOLOGY_STATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30c5018d_9a53_454b_ad9e_6d5f8fa7c43b);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ALLOCATOR_ALREADY_COMMITED: ::windows_sys::core::HRESULT = -1072846854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ALLOCATOR_NOT_COMMITED: ::windows_sys::core::HRESULT = -1072846855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ALLOCATOR_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -1072846856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ALL_PROCESS_RESTART_REQUIRED: ::windows_sys::core::HRESULT = -1072860820i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = -1072871856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_DROPPED_PACKET: ::windows_sys::core::HRESULT = -1072874847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_FILESINK_BITRATE_UNKNOWN: ::windows_sys::core::HRESULT = -1072870848i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_INDEXNOTLOADED: ::windows_sys::core::HRESULT = -1072874850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_INVALIDDATA: ::windows_sys::core::HRESULT = -1072874854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_MISSINGDATA: ::windows_sys::core::HRESULT = -1072874855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_NOINDEX: ::windows_sys::core::HRESULT = -1072874852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_OPAQUEPACKET: ::windows_sys::core::HRESULT = -1072874853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_OUTOFRANGE: ::windows_sys::core::HRESULT = -1072874851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_PARSINGINCOMPLETE: ::windows_sys::core::HRESULT = -1072874856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_TOO_MANY_PAYLOADS: ::windows_sys::core::HRESULT = -1072874849i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ASF_UNSUPPORTED_STREAM_TYPE: ::windows_sys::core::HRESULT = -1072874848i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ATTRIBUTENOTFOUND: ::windows_sys::core::HRESULT = -1072875802i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_AUDIO_BUFFER_SIZE_ERROR: ::windows_sys::core::HRESULT = -1072869752i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_AUDIO_CLIENT_WRAPPER_SPOOF_ERROR: ::windows_sys::core::HRESULT = -1072869751i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_AUDIO_PLAYBACK_DEVICE_INVALIDATED: ::windows_sys::core::HRESULT = -1072869754i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_AUDIO_PLAYBACK_DEVICE_IN_USE: ::windows_sys::core::HRESULT = -1072869755i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_AUDIO_RECORDING_DEVICE_INVALIDATED: ::windows_sys::core::HRESULT = -1072873823i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_AUDIO_RECORDING_DEVICE_IN_USE: ::windows_sys::core::HRESULT = -1072873824i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_AUDIO_SERVICE_NOT_RUNNING: ::windows_sys::core::HRESULT = -1072869753i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_BACKUP_RESTRICTED_LICENSE: ::windows_sys::core::HRESULT = -1072860850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_BAD_OPL_STRUCTURE_FORMAT: ::windows_sys::core::HRESULT = -1072860803i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_BAD_STARTUP_VERSION: ::windows_sys::core::HRESULT = -1072875805i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_BANDWIDTH_OVERRUN: ::windows_sys::core::HRESULT = -1072871855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_BUFFERTOOSMALL: ::windows_sys::core::HRESULT = -1072875855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_BYTESTREAM_NOT_SEEKABLE: ::windows_sys::core::HRESULT = -1072875794i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_BYTESTREAM_UNKNOWN_LENGTH: ::windows_sys::core::HRESULT = -1072875781i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CANNOT_CREATE_SINK: ::windows_sys::core::HRESULT = -1072875782i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CANNOT_FIND_KEYFRAME_SAMPLE: ::windows_sys::core::HRESULT = -1072873827i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CANNOT_INDEX_IN_PLACE: ::windows_sys::core::HRESULT = -1072871849i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CANNOT_PARSE_BYTESTREAM: ::windows_sys::core::HRESULT = -1072875792i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_ENGINE_ALL_EFFECTS_REMOVED: ::windows_sys::core::HRESULT = -1072845851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_ENGINE_INVALID_OP: ::windows_sys::core::HRESULT = -1072845852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_NO_SAMPLES_IN_QUEUE: ::windows_sys::core::HRESULT = -1072845845i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_PROPERTY_SET_DURING_PHOTO: ::windows_sys::core::HRESULT = -1072845846i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_SINK_MIRROR_ERROR: ::windows_sys::core::HRESULT = -1072845854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_SINK_OUTPUT_NOT_SET: ::windows_sys::core::HRESULT = -1072845855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_SINK_ROTATE_ERROR: ::windows_sys::core::HRESULT = -1072845853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_SOURCE_DEVICE_EXTENDEDPROP_OP_IN_PROGRESS: ::windows_sys::core::HRESULT = -1072845847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_SOURCE_NO_AUDIO_STREAM_PRESENT: ::windows_sys::core::HRESULT = -1072845848i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_SOURCE_NO_INDEPENDENT_PHOTO_STREAM_PRESENT: ::windows_sys::core::HRESULT = -1072845850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CAPTURE_SOURCE_NO_VIDEO_STREAM_PRESENT: ::windows_sys::core::HRESULT = -1072845849i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CLOCK_AUDIO_DEVICE_POSITION_UNEXPECTED: ::windows_sys::core::HRESULT = 891973i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CLOCK_AUDIO_RENDER_POSITION_UNEXPECTED: ::windows_sys::core::HRESULT = 891974i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CLOCK_AUDIO_RENDER_TIME_UNEXPECTED: ::windows_sys::core::HRESULT = 891975i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CLOCK_INVALID_CONTINUITY_KEY: ::windows_sys::core::HRESULT = -1072849856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CLOCK_NOT_SIMPLE: ::windows_sys::core::HRESULT = -1072849853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CLOCK_NO_TIME_SOURCE: ::windows_sys::core::HRESULT = -1072849855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CLOCK_STATE_ALREADY_SET: ::windows_sys::core::HRESULT = -1072849854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CODE_EXPIRED: ::windows_sys::core::HRESULT = -1072860834i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_COMPONENT_REVOKED: ::windows_sys::core::HRESULT = -1072860847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_CONTENT_PROTECTION_SYSTEM_NOT_ENABLED: ::windows_sys::core::HRESULT = -1072860795i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DEBUGGING_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1072860835i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DISABLED_IN_SAFEMODE: ::windows_sys::core::HRESULT = -1072875793i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DRM_HARDWARE_INCONSISTENT: ::windows_sys::core::HRESULT = -1072860853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DRM_MIGRATION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072860793i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DRM_UNSUPPORTED: ::windows_sys::core::HRESULT = -1072875776i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DROPTIME_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072848854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DURATION_TOO_LONG: ::windows_sys::core::HRESULT = -1072875769i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DXGI_DEVICE_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2147217408i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DXGI_NEW_VIDEO_DEVICE: ::windows_sys::core::HRESULT = -2147217407i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_DXGI_VIDEO_DEVICE_LOCKED: ::windows_sys::core::HRESULT = -2147217406i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_END_OF_STREAM: ::windows_sys::core::HRESULT = -1072873852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_FLUSH_NEEDED: ::windows_sys::core::HRESULT = -1072871853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_FORMAT_CHANGE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072875778i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_GRL_ABSENT: ::windows_sys::core::HRESULT = -1072860814i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_GRL_EXTENSIBLE_ENTRY_NOT_FOUND: ::windows_sys::core::HRESULT = -1072860831i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_GRL_INVALID_FORMAT: ::windows_sys::core::HRESULT = -1072860822i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_GRL_RENEWAL_NOT_FOUND: ::windows_sys::core::HRESULT = -1072860832i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_GRL_UNRECOGNIZED_FORMAT: ::windows_sys::core::HRESULT = -1072860821i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_GRL_VERSION_TOO_LOW: ::windows_sys::core::HRESULT = -1072860833i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_HARDWARE_DRM_UNSUPPORTED: ::windows_sys::core::HRESULT = -1072875770i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_HDCP_AUTHENTICATION_FAILURE: ::windows_sys::core::HRESULT = -1072860792i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_HDCP_LINK_FAILURE: ::windows_sys::core::HRESULT = -1072860791i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_HIGH_SECURITY_LEVEL_CONTENT_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1072860808i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_HW_ACCELERATED_THUMBNAIL_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072845844i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_HW_MFT_FAILED_START_STREAMING: ::windows_sys::core::HRESULT = -1072875772i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_HW_STREAM_NOT_CONNECTED: ::windows_sys::core::HRESULT = -1072846851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INCOMPATIBLE_SAMPLE_PROTECTION: ::windows_sys::core::HRESULT = -1072860810i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INDEX_NOT_COMMITTED: ::windows_sys::core::HRESULT = -1072871851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INSUFFICIENT_BUFFER: ::windows_sys::core::HRESULT = -1072860816i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALIDINDEX: ::windows_sys::core::HRESULT = -1072875841i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALIDMEDIATYPE: ::windows_sys::core::HRESULT = -1072875852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALIDNAME: ::windows_sys::core::HRESULT = -1072875844i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALIDREQUEST: ::windows_sys::core::HRESULT = -1072875854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALIDSTREAMNUMBER: ::windows_sys::core::HRESULT = -1072875853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALIDTYPE: ::windows_sys::core::HRESULT = -1072875843i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_AKE_CHANNEL_PARAMETERS: ::windows_sys::core::HRESULT = -1072860796i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_ASF_STREAMID: ::windows_sys::core::HRESULT = -1072871847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_CODEC_MERIT: ::windows_sys::core::HRESULT = -1072875773i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_FILE_FORMAT: ::windows_sys::core::HRESULT = -1072875842i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_FORMAT: ::windows_sys::core::HRESULT = -1072873844i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_KEY: ::windows_sys::core::HRESULT = -1072875806i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_POSITION: ::windows_sys::core::HRESULT = -1072875803i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_PROFILE: ::windows_sys::core::HRESULT = -1072871852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_STATE_TRANSITION: ::windows_sys::core::HRESULT = -1072873854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_STREAM_DATA: ::windows_sys::core::HRESULT = -1072875829i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_STREAM_STATE: ::windows_sys::core::HRESULT = -1072846852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_TIMESTAMP: ::windows_sys::core::HRESULT = -1072875840i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_INVALID_WORKQUEUE: ::windows_sys::core::HRESULT = -1072875777i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ITA_ERROR_PARSING_SAP_PARAMETERS: ::windows_sys::core::HRESULT = -1072860805i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ITA_OPL_DATA_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -1072860800i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ITA_UNRECOGNIZED_ANALOG_VIDEO_OUTPUT: ::windows_sys::core::HRESULT = -1072860799i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ITA_UNRECOGNIZED_ANALOG_VIDEO_PROTECTION_GUID: ::windows_sys::core::HRESULT = -1072860802i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ITA_UNRECOGNIZED_DIGITAL_VIDEO_OUTPUT: ::windows_sys::core::HRESULT = -1072860798i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_ITA_UNSUPPORTED_ACTION: ::windows_sys::core::HRESULT = -1072860806i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_KERNEL_UNTRUSTED: ::windows_sys::core::HRESULT = -1072860830i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_LATE_SAMPLE: ::windows_sys::core::HRESULT = -1072871854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_LICENSE_INCORRECT_RIGHTS: ::windows_sys::core::HRESULT = -1072860856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_LICENSE_OUTOFDATE: ::windows_sys::core::HRESULT = -1072860855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_LICENSE_REQUIRED: ::windows_sys::core::HRESULT = -1072860854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_LICENSE_RESTORE_NEEDS_INDIVIDUALIZATION: ::windows_sys::core::HRESULT = -1072860849i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_LICENSE_RESTORE_NO_RIGHTS: ::windows_sys::core::HRESULT = -1072860851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MEDIAPROC_WRONGSTATE: ::windows_sys::core::HRESULT = -1072875790i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MEDIA_EXTENSION_APPSERVICE_CONNECTION_FAILED: ::windows_sys::core::HRESULT = -1072843856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MEDIA_EXTENSION_APPSERVICE_REQUEST_FAILED: ::windows_sys::core::HRESULT = -1072843855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MEDIA_EXTENSION_PACKAGE_INTEGRITY_CHECK_FAILED: ::windows_sys::core::HRESULT = -1072843854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MEDIA_EXTENSION_PACKAGE_LICENSE_INVALID: ::windows_sys::core::HRESULT = -1072843853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MEDIA_SOURCE_NOT_STARTED: ::windows_sys::core::HRESULT = -1072873839i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MEDIA_SOURCE_NO_STREAMS_SELECTED: ::windows_sys::core::HRESULT = -1072873828i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MEDIA_SOURCE_WRONGSTATE: ::windows_sys::core::HRESULT = -1072873829i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_METADATA_TOO_LONG: ::windows_sys::core::HRESULT = -1072870845i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MISSING_ASF_LEAKYBUCKET: ::windows_sys::core::HRESULT = -1072871848i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MP3_BAD_CRC: ::windows_sys::core::HRESULT = -1072873831i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MP3_NOTFOUND: ::windows_sys::core::HRESULT = -1072873850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MP3_NOTMP3: ::windows_sys::core::HRESULT = -1072873848i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MP3_NOTSUPPORTED: ::windows_sys::core::HRESULT = -1072873847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MP3_OUTOFDATA: ::windows_sys::core::HRESULT = -1072873849i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MULTIPLE_BEGIN: ::windows_sys::core::HRESULT = -1072875815i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_MULTIPLE_SUBSCRIBERS: ::windows_sys::core::HRESULT = -1072875814i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NETWORK_RESOURCE_FAILURE: ::windows_sys::core::HRESULT = -1072872856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_BAD_CONTROL_DATA: ::windows_sys::core::HRESULT = -1072872838i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_BAD_REQUEST: ::windows_sys::core::HRESULT = -1072872833i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_BUSY: ::windows_sys::core::HRESULT = -1072872822i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_BWLEVEL_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072872851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_CACHESTREAM_NOT_FOUND: ::windows_sys::core::HRESULT = -1072872847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_CACHE_NO_DATA: ::windows_sys::core::HRESULT = -1072872835i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_CANNOTCONNECT: ::windows_sys::core::HRESULT = -1072872825i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_CLIENT_CLOSE: ::windows_sys::core::HRESULT = -1072872839i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_COMPANION_DRIVER_DISCONNECT: ::windows_sys::core::HRESULT = -1072872811i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_CONNECTION_FAILURE: ::windows_sys::core::HRESULT = -1072872829i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_EOL: ::windows_sys::core::HRESULT = -1072872834i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_ERROR_FROM_PROXY: ::windows_sys::core::HRESULT = -1072872820i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_INCOMPATIBLE_PUSHSERVER: ::windows_sys::core::HRESULT = -1072872828i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_INCOMPATIBLE_SERVER: ::windows_sys::core::HRESULT = -1072872837i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_INTERNAL_SERVER_ERROR: ::windows_sys::core::HRESULT = -1072872832i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_INVALID_PRESENTATION_DESCRIPTOR: ::windows_sys::core::HRESULT = -1072872848i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_INVALID_PUSH_PUBLISHING_POINT: ::windows_sys::core::HRESULT = -1072872823i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_INVALID_PUSH_TEMPLATE: ::windows_sys::core::HRESULT = -1072872824i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_MANUALSS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072872849i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_NOCONNECTION: ::windows_sys::core::HRESULT = -1072872830i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_PROTOCOL_DISABLED: ::windows_sys::core::HRESULT = -1072872812i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_PROXY_ACCESSDENIED: ::windows_sys::core::HRESULT = -1072872826i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_PROXY_TIMEOUT: ::windows_sys::core::HRESULT = -1072872819i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_READ: ::windows_sys::core::HRESULT = -1072872854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_REDIRECT: ::windows_sys::core::HRESULT = -1072872843i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_REDIRECT_TO_PROXY: ::windows_sys::core::HRESULT = -1072872842i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_REQUIRE_ASYNC: ::windows_sys::core::HRESULT = -1072872852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_REQUIRE_INPUT: ::windows_sys::core::HRESULT = -1072872844i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_REQUIRE_NETWORK: ::windows_sys::core::HRESULT = -1072872853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_RESOURCE_GONE: ::windows_sys::core::HRESULT = -1072872821i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_SERVER_ACCESSDENIED: ::windows_sys::core::HRESULT = -1072872827i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_SERVER_UNAVAILABLE: ::windows_sys::core::HRESULT = -1072872818i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_SESSION_INVALID: ::windows_sys::core::HRESULT = -1072872816i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_SESSION_NOT_FOUND: ::windows_sys::core::HRESULT = -1072872831i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_STREAMGROUPS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072872850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_TIMEOUT: ::windows_sys::core::HRESULT = -1072872840i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_TOO_MANY_REDIRECTS: ::windows_sys::core::HRESULT = -1072872841i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_TOO_MUCH_DATA: ::windows_sys::core::HRESULT = -1072872817i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_UDP_BLOCKED: ::windows_sys::core::HRESULT = -1072872814i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_UNSAFE_URL: ::windows_sys::core::HRESULT = -1072872836i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_UNSUPPORTED_CONFIGURATION: ::windows_sys::core::HRESULT = -1072872813i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NET_WRITE: ::windows_sys::core::HRESULT = -1072872855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NEW_VIDEO_DEVICE: ::windows_sys::core::HRESULT = -1072869851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NON_PE_PROCESS: ::windows_sys::core::HRESULT = -1072860827i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NOTACCEPTING: ::windows_sys::core::HRESULT = -1072875851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -1072875818i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NOT_FOUND: ::windows_sys::core::HRESULT = -1072875819i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -1072875850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NOT_PROTECTED: ::windows_sys::core::HRESULT = -1072873830i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_AUDIO_PLAYBACK_DEVICE: ::windows_sys::core::HRESULT = -1072869756i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_AUDIO_RECORDING_DEVICE: ::windows_sys::core::HRESULT = -1072873825i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_BITPUMP: ::windows_sys::core::HRESULT = -1072875786i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_CAPTURE_DEVICES_AVAILABLE: ::windows_sys::core::HRESULT = -1072845856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_CLOCK: ::windows_sys::core::HRESULT = -1072875817i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_CONTENT_PROTECTION_MANAGER: ::windows_sys::core::HRESULT = -1072860852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_DURATION: ::windows_sys::core::HRESULT = -1072873846i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_EVENTS_AVAILABLE: ::windows_sys::core::HRESULT = -1072873856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_INDEX: ::windows_sys::core::HRESULT = -1072871850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_MORE_DROP_MODES: ::windows_sys::core::HRESULT = -1072848856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_MORE_QUALITY_LEVELS: ::windows_sys::core::HRESULT = -1072848855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_MORE_TYPES: ::windows_sys::core::HRESULT = -1072875847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_PMP_HOST: ::windows_sys::core::HRESULT = -1072860801i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_SAMPLE_DURATION: ::windows_sys::core::HRESULT = -1072875831i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_SAMPLE_TIMESTAMP: ::windows_sys::core::HRESULT = -1072875832i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_SOURCE_IN_CACHE: ::windows_sys::core::HRESULT = -1072864850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_NO_VIDEO_SAMPLE_AVAILABLE: ::windows_sys::core::HRESULT = -1072869850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_OFFLINE_MODE: ::windows_sys::core::HRESULT = -1072872815i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_OPERATION_CANCELLED: ::windows_sys::core::HRESULT = -1072875795i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_OPERATION_IN_PROGRESS: ::windows_sys::core::HRESULT = -1072875771i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_OPERATION_UNSUPPORTED_AT_D3D_FEATURE_LEVEL: ::windows_sys::core::HRESULT = -1072875768i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_OPL_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072860838i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_OUT_OF_RANGE: ::windows_sys::core::HRESULT = -1072875774i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PEAUTH_NOT_STARTED: ::windows_sys::core::HRESULT = -1072860811i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PEAUTH_PUBLICKEY_REVOKED: ::windows_sys::core::HRESULT = -1072860815i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PEAUTH_SESSION_NOT_STARTED: ::windows_sys::core::HRESULT = -1072860817i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PEAUTH_UNTRUSTED: ::windows_sys::core::HRESULT = -1072860829i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PE_SESSIONS_MAXED: ::windows_sys::core::HRESULT = -1072860809i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PE_UNTRUSTED: ::windows_sys::core::HRESULT = -1072860812i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PLATFORM_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -1072875856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_POLICY_MGR_ACTION_OUTOFBOUNDS: ::windows_sys::core::HRESULT = -1072860804i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_POLICY_UNSUPPORTED: ::windows_sys::core::HRESULT = -1072860839i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROCESS_RESTART_REQUIRED: ::windows_sys::core::HRESULT = -1072860819i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROPERTY_EMPTY: ::windows_sys::core::HRESULT = -1072875799i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROPERTY_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1072873841i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROPERTY_NOT_EMPTY: ::windows_sys::core::HRESULT = -1072875798i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROPERTY_NOT_FOUND: ::windows_sys::core::HRESULT = -1072873843i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROPERTY_READ_ONLY: ::windows_sys::core::HRESULT = -1072873842i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROPERTY_TYPE_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1072875801i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROPERTY_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072875800i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROPERTY_VECTOR_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1072875797i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_PROPERTY_VECTOR_REQUIRED: ::windows_sys::core::HRESULT = -1072875796i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_QM_INVALIDSTATE: ::windows_sys::core::HRESULT = -1072848852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_QUALITYKNOB_WAIT_LONGER: ::windows_sys::core::HRESULT = -1072848853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_RATE_CHANGE_PREEMPTED: ::windows_sys::core::HRESULT = -1072875820i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_REBOOT_REQUIRED: ::windows_sys::core::HRESULT = -1072860825i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_RESOLUTION_REQUIRES_PMP_CREATION_CALLBACK: ::windows_sys::core::HRESULT = -1072860797i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_REVERSE_UNSUPPORTED: ::windows_sys::core::HRESULT = -1072875822i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_RT_OUTOFMEMORY: ::windows_sys::core::HRESULT = -1072875785i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_RT_THROUGHPUT_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -1072875789i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_RT_TOO_MANY_CLASSES: ::windows_sys::core::HRESULT = -1072875788i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_RT_UNAVAILABLE: ::windows_sys::core::HRESULT = -1072875825i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_RT_WORKQUEUE_CLASS_NOT_SPECIFIED: ::windows_sys::core::HRESULT = -1072875784i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_RT_WOULDBLOCK: ::windows_sys::core::HRESULT = -1072875787i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SAMPLEALLOCATOR_CANCELED: ::windows_sys::core::HRESULT = -1072870851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SAMPLEALLOCATOR_EMPTY: ::windows_sys::core::HRESULT = -1072870850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SAMPLE_HAS_TOO_MANY_BUFFERS: ::windows_sys::core::HRESULT = -1072875809i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SAMPLE_NOT_WRITABLE: ::windows_sys::core::HRESULT = -1072875808i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SEQUENCER_UNKNOWN_SEGMENT_ID: ::windows_sys::core::HRESULT = -1072864852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SESSION_PAUSEWHILESTOPPED: ::windows_sys::core::HRESULT = -1072875780i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SHUTDOWN: ::windows_sys::core::HRESULT = -1072873851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SIGNATURE_VERIFICATION_FAILED: ::windows_sys::core::HRESULT = -1072860836i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SINK_ALREADYSTOPPED: ::windows_sys::core::HRESULT = -1072870849i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SINK_HEADERS_NOT_FOUND: ::windows_sys::core::HRESULT = -1072870843i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SINK_NO_SAMPLES_PROCESSED: ::windows_sys::core::HRESULT = -1072870844i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SINK_NO_STREAMS: ::windows_sys::core::HRESULT = -1072870847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_SOURCERESOLVER_MUTUALLY_EXCLUSIVE_FLAGS: ::windows_sys::core::HRESULT = -1072875791i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_STATE_TRANSITION_PENDING: ::windows_sys::core::HRESULT = -1072875812i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_STREAMSINKS_FIXED: ::windows_sys::core::HRESULT = -1072870853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_STREAMSINKS_OUT_OF_SYNC: ::windows_sys::core::HRESULT = -1072870854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_STREAMSINK_EXISTS: ::windows_sys::core::HRESULT = -1072870852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_STREAMSINK_REMOVED: ::windows_sys::core::HRESULT = -1072870856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_STREAM_ERROR: ::windows_sys::core::HRESULT = -1072846853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TEST_SIGNED_COMPONENTS_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1072860807i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_THINNING_UNSUPPORTED: ::windows_sys::core::HRESULT = -1072875823i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TIMELINECONTROLLER_CANNOT_ATTACH: ::windows_sys::core::HRESULT = -1072844854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TIMELINECONTROLLER_NOT_ALLOWED: ::windows_sys::core::HRESULT = -1072844855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TIMELINECONTROLLER_UNSUPPORTED_SOURCE_TYPE: ::windows_sys::core::HRESULT = -1072844856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TIMER_ORPHANED: ::windows_sys::core::HRESULT = -1072875813i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPOLOGY_VERIFICATION_FAILED: ::windows_sys::core::HRESULT = -1072860837i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_CANNOT_CONNECT: ::windows_sys::core::HRESULT = -1072868845i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_CANNOT_FIND_DECRYPTOR: ::windows_sys::core::HRESULT = -1072868847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_CODEC_NOT_FOUND: ::windows_sys::core::HRESULT = -1072868846i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_INVALID_OPTIONAL_NODE: ::windows_sys::core::HRESULT = -1072868850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_INVALID_TIME_ATTRIBUTES: ::windows_sys::core::HRESULT = -1072868843i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_LOOPS_IN_TOPOLOGY: ::windows_sys::core::HRESULT = -1072868842i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_MISSING_PRESENTATION_DESCRIPTOR: ::windows_sys::core::HRESULT = -1072868841i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_MISSING_SOURCE: ::windows_sys::core::HRESULT = -1072868838i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_MISSING_STREAM_DESCRIPTOR: ::windows_sys::core::HRESULT = -1072868840i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_SINK_ACTIVATES_UNSUPPORTED: ::windows_sys::core::HRESULT = -1072868837i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_STREAM_DESCRIPTOR_NOT_SELECTED: ::windows_sys::core::HRESULT = -1072868839i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TOPO_UNSUPPORTED: ::windows_sys::core::HRESULT = -1072868844i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSCODE_INVALID_PROFILE: ::windows_sys::core::HRESULT = -1072847853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSCODE_NO_CONTAINERTYPE: ::windows_sys::core::HRESULT = -1072847856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSCODE_NO_MATCHING_ENCODER: ::windows_sys::core::HRESULT = -1072847854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSCODE_PROFILE_NO_MATCHING_STREAMS: ::windows_sys::core::HRESULT = -1072847855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_ASYNC_LOCKED: ::windows_sys::core::HRESULT = -1072861833i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_ASYNC_MFT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072861830i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_CANNOT_CHANGE_MEDIATYPE_WHILE_PROCESSING: ::windows_sys::core::HRESULT = -1072861836i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_CANNOT_INITIALIZE_ACM_DRIVER: ::windows_sys::core::HRESULT = -1072861832i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_CONFLICTS_WITH_OTHER_CURRENTLY_ENABLED_FEATURES: ::windows_sys::core::HRESULT = -1072861840i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_EXATTRIBUTE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072861828i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_INPUT_REMAINING: ::windows_sys::core::HRESULT = -1072861854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_NEED_MORE_INPUT: ::windows_sys::core::HRESULT = -1072861838i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_NOT_POSSIBLE_FOR_CURRENT_INPUT_MEDIATYPE: ::windows_sys::core::HRESULT = -1072861842i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_NOT_POSSIBLE_FOR_CURRENT_MEDIATYPE_COMBINATION: ::windows_sys::core::HRESULT = -1072861841i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_NOT_POSSIBLE_FOR_CURRENT_OUTPUT_MEDIATYPE: ::windows_sys::core::HRESULT = -1072861843i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_NOT_POSSIBLE_FOR_CURRENT_SPKR_CONFIG: ::windows_sys::core::HRESULT = -1072861837i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROFILE_INVALID_OR_CORRUPT: ::windows_sys::core::HRESULT = -1072861852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROFILE_MISSING: ::windows_sys::core::HRESULT = -1072861853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROFILE_TRUNCATED: ::windows_sys::core::HRESULT = -1072861851i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROPERTY_ARRAY_VALUE_WRONG_NUM_DIM: ::windows_sys::core::HRESULT = -1072861847i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROPERTY_NOT_WRITEABLE: ::windows_sys::core::HRESULT = -1072861848i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROPERTY_PID_NOT_RECOGNIZED: ::windows_sys::core::HRESULT = -1072861850i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROPERTY_VALUE_INCOMPATIBLE: ::windows_sys::core::HRESULT = -1072861844i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROPERTY_VALUE_OUT_OF_RANGE: ::windows_sys::core::HRESULT = -1072861845i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROPERTY_VALUE_SIZE_WRONG: ::windows_sys::core::HRESULT = -1072861846i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_PROPERTY_VARIANT_TYPE_WRONG: ::windows_sys::core::HRESULT = -1072861849i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_STREAM_CHANGE: ::windows_sys::core::HRESULT = -1072861855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_STREAM_INVALID_RESOLUTION: ::windows_sys::core::HRESULT = -1072861831i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRANSFORM_TYPE_NOT_SET: ::windows_sys::core::HRESULT = -1072861856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_TRUST_DISABLED: ::windows_sys::core::HRESULT = -1072860846i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNAUTHORIZED: ::windows_sys::core::HRESULT = -1072875775i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNEXPECTED: ::windows_sys::core::HRESULT = -1072875845i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNRECOVERABLE_ERROR_OCCURRED: ::windows_sys::core::HRESULT = -1072875810i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_BYTESTREAM_TYPE: ::windows_sys::core::HRESULT = -1072875836i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_CAPTION: ::windows_sys::core::HRESULT = -1072875804i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_CAPTURE_DEVICE_PRESENT: ::windows_sys::core::HRESULT = -1072845843i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_CHARACTERISTICS: ::windows_sys::core::HRESULT = -1072873826i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_CONTENT_PROTECTION_SYSTEM: ::windows_sys::core::HRESULT = -1072860794i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_D3D_TYPE: ::windows_sys::core::HRESULT = -1072861834i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_FORMAT: ::windows_sys::core::HRESULT = -1072873832i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_MEDIATYPE_AT_D3D_FEATURE_LEVEL: ::windows_sys::core::HRESULT = -1072875767i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_RATE: ::windows_sys::core::HRESULT = -1072875824i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_RATE_TRANSITION: ::windows_sys::core::HRESULT = -1072875821i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_REPRESENTATION: ::windows_sys::core::HRESULT = -1072875849i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_SCHEME: ::windows_sys::core::HRESULT = -1072875837i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_SERVICE: ::windows_sys::core::HRESULT = -1072875846i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_STATE_TRANSITION: ::windows_sys::core::HRESULT = -1072875811i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_UNSUPPORTED_TIME_FORMAT: ::windows_sys::core::HRESULT = -1072875835i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_USERMODE_UNTRUSTED: ::windows_sys::core::HRESULT = -1072860818i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_VIDEO_DEVICE_LOCKED: ::windows_sys::core::HRESULT = -1072869852i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_VIDEO_RECORDING_DEVICE_INVALIDATED: ::windows_sys::core::HRESULT = -1072873822i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_VIDEO_RECORDING_DEVICE_PREEMPTED: ::windows_sys::core::HRESULT = -1072873821i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_VIDEO_REN_COPYPROT_FAILED: ::windows_sys::core::HRESULT = -1072869854i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_VIDEO_REN_NO_DEINTERLACE_HW: ::windows_sys::core::HRESULT = -1072869855i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_VIDEO_REN_NO_PROCAMP_HW: ::windows_sys::core::HRESULT = -1072869856i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_VIDEO_REN_SURFACE_NOT_SHARED: ::windows_sys::core::HRESULT = -1072869853i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_WMDRMOTA_ACTION_ALREADY_SET: ::windows_sys::core::HRESULT = -1072860844i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_WMDRMOTA_ACTION_MISMATCH: ::windows_sys::core::HRESULT = -1072860841i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_WMDRMOTA_DRM_ENCRYPTION_SCHEME_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -1072860842i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_WMDRMOTA_DRM_HEADER_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -1072860843i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_WMDRMOTA_INVALID_POLICY: ::windows_sys::core::HRESULT = -1072860840i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_E_WMDRMOTA_NO_ACTION: ::windows_sys::core::HRESULT = -1072860845i32;
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_CUSTOM_EVENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6e59489c_47d3_4467_83ef_12d34e871665);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_PIPELINE_SHUTDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x45a81b31_43f8_4e5d_8ce2_22dce026996d);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_INITIALIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe52c4dff_e46d_4d0b_bc75_ddd4c8723f96);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_START: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb1eeb989_b456_4f4a_ae40_079c28e24af8);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_STOP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb7fe7a61_fe91_415e_8608_d37dedb1a58b);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_UNINITIALIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0ebaba7_a422_4e33_8401_b37d2800aa67);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_GRL_ABSENT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_GRL_LOAD_FAILED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_HISTOGRAM_CHANNEL_B: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_HISTOGRAM_CHANNEL_Cb: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_HISTOGRAM_CHANNEL_Cr: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_HISTOGRAM_CHANNEL_G: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_HISTOGRAM_CHANNEL_R: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_HISTOGRAM_CHANNEL_Y: u32 = 1u32;
pub const MF_INDEPENDENT_STILL_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xea12af41_0710_42c9_a127_daa3e78483a5);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_INDEX_SIZE_ERR: u32 = 2154823681u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_INVALID_ACCESS_ERR: u32 = 2154823695u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_INVALID_GRL_SIGNATURE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_INVALID_PRESENTATION_TIME: u64 = 9223372036854775808u64;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_INVALID_STATE_ERR: u32 = 2154823691u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_I_MANUAL_PROXY: ::windows_sys::core::HRESULT = 1074610802i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_KERNEL_MODE_COMPONENT_LOAD: u32 = 2u32;
pub const MF_LOCAL_MFT_REGISTRATION_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xddf5cf9c_4506_45aa_abf0_6d5d94dd1b4a);
pub const MF_LOCAL_PLUGIN_CONTROL_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd91b0085_c86d_4f81_8822_8c68e1d7fa04);
pub const MF_LOW_LATENCY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9c27891a_ed7a_40e1_88e8_b22727a024ee);
pub const MF_LUMA_KEY_ENABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7369820f_76de_43ca_9284_47b8f37e0649);
pub const MF_LUMA_KEY_LOWER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x93d7b8d5_0b81_4715_aea0_8725871621e9);
pub const MF_LUMA_KEY_UPPER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd09f39bb_4602_4c31_a706_a12171a5110a);
pub const MF_MEDIASINK_AUTOFINALIZE_SUPPORTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48c131be_135a_41cb_8290_03652509c999);
pub const MF_MEDIASINK_ENABLE_AUTOFINALIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x34014265_cb7e_4cde_ac7c_effd3b3c2530);
pub const MF_MEDIASOURCE_EXPOSE_ALL_STREAMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe7f250b8_8fd9_4a09_b6c1_6a315c7c720e);
pub const MF_MEDIASOURCE_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf09992f7_9fba_4c4a_a37f_8c47b4e1dfe7);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIATYPE_EQUAL_MAJOR_TYPES: u32 = 1u32;
pub const MF_MEDIATYPE_MULTIPLEXED_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x13c78fb5_f275_4ea0_bb5f_0249832b0d6e);
pub const MF_MEDIA_ENGINE_AUDIO_CATEGORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc8d4c51d_350e_41f2_ba46_faebbb0857f6);
pub const MF_MEDIA_ENGINE_AUDIO_ENDPOINT_ROLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd2cb93d1_116a_44f2_9385_f7d0fda2fb46);
pub const MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4e0212e2_e18f_41e1_95e5_c0e7e9235bc3);
pub const MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE_IE10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x11a47afd_6589_4124_b312_6158ec517fc3);
pub const MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE_IE11: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1cf1315f_ce3f_4035_9391_16142f775189);
pub const MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE_IE9: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x052c2d39_40c0_4188_ab86_f828273b7522);
pub const MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE_IE_EDGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa6f3e465_3aca_442c_a3f0_ad6ddad839ae);
pub const MF_MEDIA_ENGINE_CALLBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc60381b8_83a4_41f8_a3d0_de05076849a9);
pub const MF_MEDIA_ENGINE_COMPATIBILITY_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3ef26ad4_dc54_45de_b9af_76c8c66bfa8e);
pub const MF_MEDIA_ENGINE_COMPATIBILITY_MODE_WIN10: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5b25e089_6ca7_4139_a2cb_fcaab39552a3);
pub const MF_MEDIA_ENGINE_COMPATIBILITY_MODE_WWA_EDGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x15b29098_9f01_4e4d_b65a_c06c6c89da2a);
pub const MF_MEDIA_ENGINE_CONTENT_PROTECTION_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe0350223_5aaf_4d76_a7c3_06de70894db4);
pub const MF_MEDIA_ENGINE_CONTENT_PROTECTION_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfdd6dfaa_bd85_4af3_9e0f_a01d539d876a);
pub const MF_MEDIA_ENGINE_CONTINUE_ON_CODEC_ERROR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdbcdb7f9_48e4_4295_b70d_d518234eeb38);
pub const MF_MEDIA_ENGINE_COREWINDOW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfccae4dc_0b7f_41c2_9f96_4659948acddc);
pub const MF_MEDIA_ENGINE_DXGI_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x065702da_1094_486d_8617_ee7cc4ee4648);
pub const MF_MEDIA_ENGINE_EME_CALLBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494553a7_a481_4cb7_bec5_380903513731);
pub const MF_MEDIA_ENGINE_EXTENSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3109fd46_060d_4b62_8dcf_faff811318d2);
pub const MF_MEDIA_ENGINE_MEDIA_PLAYER_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3ddd8d45_5aa1_4112_82e5_36f6a2197e6e);
pub const MF_MEDIA_ENGINE_NEEDKEY_CALLBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7ea80843_b6e4_432c_8ea4_7848ffe4220e);
pub const MF_MEDIA_ENGINE_OPM_HWND: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0be8ee7_0572_4f2c_a801_2a151bd3e726);
pub const MF_MEDIA_ENGINE_PLAYBACK_HWND: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd988879b_67c9_4d92_baa7_6eadd446039d);
pub const MF_MEDIA_ENGINE_PLAYBACK_VISUAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6debd26f_6ab9_4d7e_b0ee_c61a73ffad15);
pub const MF_MEDIA_ENGINE_SOURCE_RESOLVER_CONFIG_STORE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0ac0c497_b3c4_48c9_9cde_bb8ca2442ca3);
pub const MF_MEDIA_ENGINE_STREAM_CONTAINS_ALPHA_CHANNEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5cbfaf44_d2b2_4cfb_80a7_d429c74c789d);
pub const MF_MEDIA_ENGINE_SYNCHRONOUS_CLOSE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc3c2e12f_7e0e_4e43_b91c_dc992ccdfa5e);
pub const MF_MEDIA_ENGINE_TELEMETRY_APPLICATION_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1e7b273b_a7e4_402a_8f51_c48e88a2cabc);
pub const MF_MEDIA_ENGINE_TIMEDTEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x805ea411_92e0_4e59_9b6e_5c7d7915e64f);
pub const MF_MEDIA_ENGINE_TRACK_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x65bea312_4043_4815_8eab_44dce2ef8f2a);
pub const MF_MEDIA_ENGINE_VIDEO_OUTPUT_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5066893c_8cf9_42bc_8b8a_472212e52726);
pub const MF_MEDIA_PROTECTION_MANAGER_PROPERTIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x38bd81a9_acea_4c73_89b2_5532c0aeca79);
pub const MF_MEDIA_SHARING_ENGINE_DEVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb461c58a_7a08_4b98_99a8_70fd5f3badfd);
pub const MF_MEDIA_SHARING_ENGINE_DEVICE_NAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x771e05d1_862f_4299_95ac_ae81fd14f3e7);
pub const MF_MEDIA_SHARING_ENGINE_INITIAL_SEEK_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6f3497f5_d528_4a4f_8dd7_db36657ec4c9);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_METADATAFACIALEXPRESSION_SMILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_METADATATIMESTAMPS_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_METADATATIMESTAMPS_PRESENTATION: u32 = 2u32;
pub const MF_METADATA_PROVIDER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdb214084_58a4_4d2e_b84f_6f755b2f7a0d);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MINCRYPT_FAILURE: u32 = 268435456u32;
pub const MF_MP2DLNA_AUDIO_BIT_RATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2d1c070e_2b5f_4ab3_a7e6_8d943ba8d00a);
pub const MF_MP2DLNA_ENCODE_QUALITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb52379d7_1d46_4fb6_a317_a4a5f60959f8);
pub const MF_MP2DLNA_STATISTICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x75e488a3_d5ad_4898_85e0_bcce24a722d7);
pub const MF_MP2DLNA_USE_MMCSS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x54f3e2ee_a2a2_497d_9834_973afde521eb);
pub const MF_MP2DLNA_VIDEO_BIT_RATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe88548de_73b4_42d7_9c75_adfa0a2a6e4c);
pub const MF_MPEG4SINK_MAX_CODED_SEQUENCES_PER_FRAGMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc1b3bd6_692d_4ce5_9299_738aa5463e9a);
pub const MF_MPEG4SINK_MINIMUM_PROPERTIES_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdca1ed52_450e_4a22_8c62_4ed452f7a187);
pub const MF_MPEG4SINK_MIN_FRAGMENT_DURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa30b570c_8efd_45e8_94fe_27c84b5bdff6);
pub const MF_MPEG4SINK_MOOV_BEFORE_MDAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf672e3ac_e1e6_4f10_b5ec_5f3b30828816);
pub const MF_MPEG4SINK_SPSPPS_PASSTHROUGH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5601a134_2005_4ad2_b37d_22a6c554deb2);
pub const MF_MSE_ACTIVELIST_CALLBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x949bda0f_4549_46d5_ad7f_b846e1ab1652);
pub const MF_MSE_BUFFERLIST_CALLBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x42e669b0_d60e_4afb_a85b_d8e5fe6bdab5);
pub const MF_MSE_CALLBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9063a7c0_42c5_4ffd_a8a8_6fcf9ea3d00c);
pub const MF_MSE_OPUS_SUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4d224cc1_8cc4_48a3_a7a7_e4c16ce6388a);
pub const MF_MSE_VP9_SUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x92d78429_d88b_4ff0_8322_803efa6e9626);
pub const MF_MT_AAC_AUDIO_PROFILE_LEVEL_INDICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7632f0e6_9538_4d61_acda_ea29c8c14456);
pub const MF_MT_AAC_PAYLOAD_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbfbabe79_7434_4d1c_94f0_72a3b9e17188);
pub const MF_MT_ALL_SAMPLES_INDEPENDENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc9173739_5e56_461c_b713_46fb995cb95f);
pub const MF_MT_ALPHA_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5d959b0d_4cbf_4d04_919f_3f5f7f284211);
pub const MF_MT_AM_FORMAT_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73d1072d_1870_4174_a063_29ff4ff6c11e);
pub const MF_MT_ARBITRARY_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5a75b249_0d7d_49a1_a1c3_e0d87f0cade5);
pub const MF_MT_ARBITRARY_HEADER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9e6bd6f5_0109_4f95_84ac_9309153a19fc);
pub const MF_MT_AUDIO_AVG_BYTES_PER_SECOND: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1aab75c8_cfef_451c_ab95_ac034b8e1731);
pub const MF_MT_AUDIO_BITS_PER_SAMPLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2deb57f_40fa_4764_aa33_ed4f2d1ff669);
pub const MF_MT_AUDIO_BLOCK_ALIGNMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x322de230_9eeb_43bd_ab7a_ff412251541d);
pub const MF_MT_AUDIO_CHANNEL_MASK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x55fb5765_644a_4caf_8479_938983bb1588);
pub const MF_MT_AUDIO_FLAC_MAX_BLOCK_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8b81adae_4b5a_4d40_8022_f38d09ca3c5c);
pub const MF_MT_AUDIO_FLOAT_SAMPLES_PER_SECOND: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb3b724a_cfb5_4319_aefe_6e42b2406132);
pub const MF_MT_AUDIO_FOLDDOWN_MATRIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d62927c_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AUDIO_NUM_CHANNELS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x37e48bf5_645e_4c5b_89de_ada9e29b696a);
pub const MF_MT_AUDIO_PREFER_WAVEFORMATEX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa901aaba_e037_458a_bdf6_545be2074042);
pub const MF_MT_AUDIO_SAMPLES_PER_BLOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaab15aac_e13a_4995_9222_501ea15c6877);
pub const MF_MT_AUDIO_SAMPLES_PER_SECOND: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5faeeae7_0290_4c31_9e8a_c534f68d9dba);
pub const MF_MT_AUDIO_VALID_BITS_PER_SAMPLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd9bf8d6a_9530_4b7c_9ddf_ff6fd58bbd06);
pub const MF_MT_AUDIO_WMADRC_AVGREF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d62927f_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AUDIO_WMADRC_AVGTARGET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d629280_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AUDIO_WMADRC_PEAKREF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d62927d_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AUDIO_WMADRC_PEAKTARGET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d62927e_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AVG_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x20332624_fb0d_4d9e_bd0d_cbf6786c102e);
pub const MF_MT_AVG_BIT_ERROR_RATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x799cabd6_3508_4db4_a3c7_569cd533deb1);
pub const MF_MT_COMPRESSED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3afd0cee_18f2_4ba5_a110_8bea502e1f92);
pub const MF_MT_CONTAINER_RATE_SCALING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x83877f5e_0444_4e28_8479_6db0989b8c09);
pub const MF_MT_CUSTOM_VIDEO_PRIMARIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47537213_8cfb_4722_aa34_fbc9e24d77b8);
pub const MF_MT_D3D12_CPU_READBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x28ee9fe3_d481_46a6_b98a_7f69d5280e82);
pub const MF_MT_D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa6a1e439_2f96_4ab5_98dc_adf74973505d);
pub const MF_MT_D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb1138dc3_01d5_4c14_9bdc_cdc9336f55b9);
pub const MF_MT_D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeeac2585_3430_498c_84a2_77b1bba570f6);
pub const MF_MT_D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0a4940b2_cfd6_4738_9d02_98113734015a);
pub const MF_MT_D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x82c85647_5057_4960_9559_f45b8e271427);
pub const MF_MT_D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xba06bfac_ffe3_474a_ab55_161ee4417a2e);
pub const MF_MT_D3D12_TEXTURE_LAYOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x97c85caa_0beb_4ee1_9715_f22fad8c10f5);
pub const MF_MT_D3D_RESOURCE_VERSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x174f1e85_fe26_453d_b52e_5bdd4e55b944);
pub const MF_MT_DECODER_MAX_DPB_COUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x67be144c_88b7_4ca9_9628_c808d5262217);
pub const MF_MT_DECODER_USE_MAX_RESOLUTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4c547c24_af9a_4f38_96ad_978773cf53e7);
pub const MF_MT_DEFAULT_STRIDE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x644b4e48_1e02_4516_b0eb_c01ca9d49ac6);
pub const MF_MT_DEPTH_MEASUREMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfd5ac489_0917_4bb6_9d54_3122bf70144b);
pub const MF_MT_DEPTH_VALUE_UNIT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x21a800f5_3189_4797_beba_f13cd9a31a5e);
pub const MF_MT_DRM_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8772f323_355a_4cc7_bb78_6d61a048ae82);
pub const MF_MT_DV_AAUX_CTRL_PACK_0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf731004e_1dd1_4515_aabe_f0c06aa536ac);
pub const MF_MT_DV_AAUX_CTRL_PACK_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcd1f470d_1f04_4fe0_bfb9_d07ae0386ad8);
pub const MF_MT_DV_AAUX_SRC_PACK_0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x84bd5d88_0fb8_4ac8_be4b_a8848bef98f3);
pub const MF_MT_DV_AAUX_SRC_PACK_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x720e6544_0225_4003_a651_0196563a958e);
pub const MF_MT_DV_VAUX_CTRL_PACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2f84e1c4_0da1_4788_938e_0dfbfbb34b48);
pub const MF_MT_DV_VAUX_SRC_PACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x41402d9d_7b57_43c6_b129_2cb997f15009);
pub const MF_MT_FIXED_SIZE_SAMPLES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb8ebefaf_b718_4e04_b0a9_116775e3321b);
pub const MF_MT_FORWARD_CUSTOM_NALU: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xed336efd_244f_428d_9153_28f399458890);
pub const MF_MT_FORWARD_CUSTOM_SEI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe27362f1_b136_41d1_9594_3a7e4febf2d1);
pub const MF_MT_FRAME_RATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc459a2e8_3d2c_4e44_b132_fee5156c7bb0);
pub const MF_MT_FRAME_RATE_RANGE_MAX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe3371d41_b4cf_4a05_bd4e_20b88bb2c4d6);
pub const MF_MT_FRAME_RATE_RANGE_MIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd2e7558c_dc1f_403f_9a72_d28bb1eb3b5e);
pub const MF_MT_FRAME_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1652c33d_d6b2_4012_b834_72030849a37d);
pub const MF_MT_GEOMETRIC_APERTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x66758743_7e5f_400d_980a_aa8596c85696);
pub const MF_MT_H264_CAPABILITIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbb3bd508_490a_11e0_99e4_1316dfd72085);
pub const MF_MT_H264_LAYOUT_PER_STREAM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x85e299b2_90e3_4fe8_b2f5_c067e0bfe57a);
pub const MF_MT_H264_MAX_CODEC_CONFIG_DELAY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf5929986_4c45_4fbb_bb49_6cc534d05b9b);
pub const MF_MT_H264_MAX_MB_PER_SEC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x45256d30_7215_4576_9336_b0f1bcd59bb2);
pub const MF_MT_H264_RATE_CONTROL_MODES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x705177d8_45cb_11e0_ac7d_b91ce0d72085);
pub const MF_MT_H264_RESOLUTION_SCALING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe3854272_f715_4757_ba90_1b696c773457);
pub const MF_MT_H264_SIMULCAST_SUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9ea2d63d_53f0_4a34_b94e_9de49a078cb3);
pub const MF_MT_H264_SUPPORTED_RATE_CONTROL_MODES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6a8ac47e_519c_4f18_9bb3_7eeaaea5594d);
pub const MF_MT_H264_SUPPORTED_SLICE_MODES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc8be1937_4d64_4549_8343_a8086c0bfda5);
pub const MF_MT_H264_SUPPORTED_SYNC_FRAME_TYPES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x89a52c01_f282_48d2_b522_22e6ae633199);
pub const MF_MT_H264_SUPPORTED_USAGES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x60b1a998_dc01_40ce_9736_aba845a2dbdc);
pub const MF_MT_H264_SVC_CAPABILITIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf8993abe_d937_4a8f_bbca_6966fe9e1152);
pub const MF_MT_H264_USAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x359ce3a5_af00_49ca_a2f4_2ac94ca82b61);
pub const MF_MT_IMAGE_LOSS_TOLERANT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xed062cf4_e34e_4922_be99_934032133d7c);
pub const MF_MT_INTERLACE_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe2724bb8_e676_4806_b4b2_a8d6efb44ccd);
pub const MF_MT_IN_BAND_PARAMETER_SET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x75da5090_910b_4a03_896c_7b898feea5af);
pub const MF_MT_MAJOR_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48eba18e_f8c9_4687_bf11_0a74c9f96a8f);
pub const MF_MT_MAX_FRAME_AVERAGE_LUMINANCE_LEVEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x58d4bf57_6f52_4733_a195_a9e29ecf9e27);
pub const MF_MT_MAX_KEYFRAME_SPACING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc16eb52b_73a1_476f_8d62_839d6a020652);
pub const MF_MT_MAX_LUMINANCE_LEVEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50253128_c110_4de4_98ae_46a324fae6da);
pub const MF_MT_MAX_MASTERING_LUMINANCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd6c6b997_272f_4ca1_8d00_8042111a0ff6);
pub const MF_MT_MINIMUM_DISPLAY_APERTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd7388766_18fe_48c6_a177_ee894867c8c4);
pub const MF_MT_MIN_MASTERING_LUMINANCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x839a4460_4e7e_4b4f_ae79_cc08905c7b27);
pub const MF_MT_MPEG2_CONTENT_PACKET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x825d55e4_4f12_4197_9eb3_59b6e4710f06);
pub const MF_MT_MPEG2_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31e3991d_f701_4b2f_b426_8ae3bda9e04b);
pub const MF_MT_MPEG2_HDCP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x168f1b4a_3e91_450f_aea7_e4baeadae5ba);
pub const MF_MT_MPEG2_LEVEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96f66574_11c5_4015_8666_bff516436da7);
pub const MF_MT_MPEG2_ONE_FRAME_PER_PACKET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x91a49eb5_1d20_4b42_ace8_804269bf95ed);
pub const MF_MT_MPEG2_PROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xad76a80b_2d5c_4e0b_b375_64e520137036);
pub const MF_MT_MPEG2_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa20af9e8_928a_4b26_aaa9_f05c74cac47c);
pub const MF_MT_MPEG2_TIMECODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5229ba10_e29d_4f80_a59c_df4f180207d2);
pub const MF_MT_MPEG4_CURRENT_SAMPLE_ENTRY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9aa7e155_b64a_4c1d_a500_455d600b6560);
pub const MF_MT_MPEG4_SAMPLE_DESCRIPTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x261e9d83_9529_4b8f_a111_8b9c950a81a9);
pub const MF_MT_MPEG4_TRACK_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x54f486dd_9327_4f6d_80ab_6f709ebb4cce);
pub const MF_MT_MPEG_SEQUENCE_HEADER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3c036de7_3ad0_4c9e_9216_ee6d6ac21cb3);
pub const MF_MT_MPEG_START_TIME_CODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x91f67885_4333_4280_97cd_bd5a6c03a06e);
pub const MF_MT_ORIGINAL_4CC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd7be3fe0_2bc7_492d_b843_61a1919b70c3);
pub const MF_MT_ORIGINAL_WAVE_FORMAT_TAG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8cbbc843_9fd9_49c2_882f_a72586c408ad);
pub const MF_MT_OUTPUT_BUFFER_NUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa505d3ac_f930_436e_8ede_93a509ce23b2);
pub const MF_MT_PAD_CONTROL_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4d0e73e5_80ea_4354_a9d0_1176ceb028ea);
pub const MF_MT_PALETTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d283f42_9846_4410_afd9_654d503b1a54);
pub const MF_MT_PAN_SCAN_APERTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x79614dde_9187_48fb_b8c7_4d52689de649);
pub const MF_MT_PAN_SCAN_ENABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4b7f6bc3_8b13_40b2_a993_abf630b8204e);
pub const MF_MT_PIXEL_ASPECT_RATIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc6376a1e_8d0a_4027_be45_6d9a0ad39bb6);
pub const MF_MT_REALTIME_CONTENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbb12d222_2bdb_425e_91ec_2308e189a58f);
pub const MF_MT_SAMPLE_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdad3ab78_1990_408b_bce2_eba673dacc10);
pub const MF_MT_SECURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc5acc4fd_0304_4ecf_809f_47bc97ff63bd);
pub const MF_MT_SOURCE_CONTENT_HINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x68aca3cc_22d0_44e6_85f8_28167197fa38);
pub const MF_MT_SPATIAL_AUDIO_DATA_PRESENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6842f6e7_d43e_4ebb_9c9c_c96f41784863);
pub const MF_MT_SPATIAL_AUDIO_MAX_DYNAMIC_OBJECTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdcfba24a_2609_4240_a721_3faea76a4df9);
pub const MF_MT_SPATIAL_AUDIO_MAX_METADATA_ITEMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x11aa80b4_e0da_47c6_8060_96c1259ae50d);
pub const MF_MT_SPATIAL_AUDIO_MIN_METADATA_ITEM_OFFSET_SPACING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x83e96ec9_1184_417e_8254_9f269158fc06);
pub const MF_MT_SPATIAL_AUDIO_OBJECT_METADATA_FORMAT_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2ab71bc0_6223_4ba7_ad64_7b94b47ae792);
pub const MF_MT_SPATIAL_AUDIO_OBJECT_METADATA_LENGTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x094ba8be_d723_489f_92fa_766777b34726);
pub const MF_MT_SUBTYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf7e34c9a_42e8_4714_b74b_cb29d72c35e5);
pub const MF_MT_TIMESTAMP_CAN_BE_DTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x24974215_1b7b_41e4_8625_ac469f2dedaa);
pub const MF_MT_TRANSFER_FUNCTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5fb0fce9_be5c_4935_a811_ec838f8eed93);
pub const MF_MT_USER_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb6bc765f_4c3b_40a4_bd51_2535b66fe09d);
pub const MF_MT_VIDEO_3D: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcb5e88cf_7b5b_476b_85aa_1ca5ae187555);
pub const MF_MT_VIDEO_3D_FIRST_IS_LEFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xec298493_0ada_4ea1_a4fe_cbbd36ce9331);
pub const MF_MT_VIDEO_3D_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5315d8a0_87c5_4697_b793_6606c67c049b);
pub const MF_MT_VIDEO_3D_LEFT_IS_BASE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d4b7bff_5629_4404_948c_c634f4ce26d4);
pub const MF_MT_VIDEO_3D_NUM_VIEWS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbb077e8a_dcbf_42eb_af60_418df98aa495);
pub const MF_MT_VIDEO_CHROMA_SITING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x65df2370_c773_4c33_aa64_843e068efb0c);
pub const MF_MT_VIDEO_H264_NO_FMOASO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xed461cd6_ec9f_416a_a8a3_26d7d31018d7);
pub const MF_MT_VIDEO_LEVEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96f66574_11c5_4015_8666_bff516436da7);
pub const MF_MT_VIDEO_LIGHTING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x53a0529c_890b_4216_8bf9_599367ad6d20);
pub const MF_MT_VIDEO_NOMINAL_RANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc21b8ee5_b956_4071_8daf_325edf5cab11);
pub const MF_MT_VIDEO_NO_FRAME_ORDERING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3f5b106f_6bc2_4ee3_b7ed_8902c18f5351);
pub const MF_MT_VIDEO_PRIMARIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdbfbe4d7_0740_4ee0_8192_850ab0e21935);
pub const MF_MT_VIDEO_PROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xad76a80b_2d5c_4e0b_b375_64e520137036);
pub const MF_MT_VIDEO_RENDERER_EXTENSION_PROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8437d4b9_d448_4fcd_9b6b_839bf96c7798);
pub const MF_MT_VIDEO_ROTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc380465d_2271_428c_9b83_ecea3b4a85c1);
pub const MF_MT_WRAPPED_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4d3f7b23_d02f_4e6c_9bee_e4bf2c6c695d);
pub const MF_MT_YUV_MATRIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3e23d450_2c75_4d25_a00e_b91670d12327);
pub const MF_NALU_LENGTH_INFORMATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x19124e7c_ad4b_465f_bb18_20186287b6af);
pub const MF_NALU_LENGTH_SET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa7911d53_12a4_4965_ae70_6eadd6ff0551);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_NOT_FOUND_ERR: u32 = 2154823688u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_NOT_SUPPORTED_ERR: u32 = 2154823689u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_PARSE_ERR: u32 = 2154823761u32;
pub const MF_PD_ADAPTIVE_STREAMING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xea0d5d97_29f9_488b_ae6b_7d6b4136112b);
pub const MF_PD_APP_CONTEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d32_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_ASF_CODECLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe4bb3509_c18d_4df1_bb99_7a36b3cc4119);
pub const MF_PD_ASF_CONTENTENCRYPTIONEX_ENCRYPTION_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62508be5_ecdf_4924_a359_72bab3397b9d);
pub const MF_PD_ASF_CONTENTENCRYPTION_KEYID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8520fe3e_277e_46ea_99e4_e30a86db12be);
pub const MF_PD_ASF_CONTENTENCRYPTION_LICENSE_URL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8520fe40_277e_46ea_99e4_e30a86db12be);
pub const MF_PD_ASF_CONTENTENCRYPTION_SECRET_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8520fe3f_277e_46ea_99e4_e30a86db12be);
pub const MF_PD_ASF_CONTENTENCRYPTION_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8520fe3d_277e_46ea_99e4_e30a86db12be);
pub const MF_PD_ASF_DATA_LENGTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe7d5b3e8_1f29_45d3_8822_3e78fae272ed);
pub const MF_PD_ASF_DATA_START_OFFSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe7d5b3e7_1f29_45d3_8822_3e78fae272ed);
pub const MF_PD_ASF_FILEPROPERTIES_CREATION_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649b6_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_FILEPROPERTIES_FILE_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649b4_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_FILEPROPERTIES_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649bb_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_FILEPROPERTIES_MAX_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649be_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_FILEPROPERTIES_MAX_PACKET_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649bd_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_FILEPROPERTIES_MIN_PACKET_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649bc_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_FILEPROPERTIES_PACKETS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649b7_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_FILEPROPERTIES_PLAY_DURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649b8_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_FILEPROPERTIES_PREROLL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649ba_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_FILEPROPERTIES_SEND_DURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3de649b9_d76d_4e66_9ec9_78120fb4c7e3);
pub const MF_PD_ASF_INFO_HAS_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x80e62295_2296_4a44_b31c_d103c6fed23c);
pub const MF_PD_ASF_INFO_HAS_NON_AUDIO_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x80e62297_2296_4a44_b31c_d103c6fed23c);
pub const MF_PD_ASF_INFO_HAS_VIDEO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x80e62296_2296_4a44_b31c_d103c6fed23c);
pub const MF_PD_ASF_LANGLIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf23de43c_9977_460d_a6ec_32937f160f7d);
pub const MF_PD_ASF_LANGLIST_LEGACYORDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf23de43d_9977_460d_a6ec_32937f160f7d);
pub const MF_PD_ASF_MARKER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5134330e_83a6_475e_a9d5_4fb875fb2e31);
pub const MF_PD_ASF_METADATA_IS_VBR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5fc6947a_ef60_445d_b449_442ecc78b4c1);
pub const MF_PD_ASF_METADATA_LEAKY_BUCKET_PAIRS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5fc6947d_ef60_445d_b449_442ecc78b4c1);
pub const MF_PD_ASF_METADATA_V8_BUFFERAVERAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5fc6947c_ef60_445d_b449_442ecc78b4c1);
pub const MF_PD_ASF_METADATA_V8_VBRPEAK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5fc6947b_ef60_445d_b449_442ecc78b4c1);
pub const MF_PD_ASF_SCRIPT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe29cd0d7_d602_4923_a7fe_73fd97ecc650);
pub const MF_PD_AUDIO_ENCODING_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d35_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_AUDIO_ISVARIABLEBITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33026ee0_e387_4582_ae0a_34a2ad3baa18);
pub const MF_PD_DURATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d33_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_LAST_MODIFIED_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d38_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_MIME_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d37_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_PLAYBACK_BOUNDARY_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d3b_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_PLAYBACK_ELEMENT_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d39_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_PMPHOST_CONTEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d31_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_PREFERRED_LANGUAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d3a_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_SAMI_STYLELIST: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe0b73c7f_486d_484e_9872_4de5192a7bf8);
pub const MF_PD_TOTAL_FILE_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d34_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_VIDEO_ENCODING_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c990d36_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PMP_SERVER_CONTEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2f00c910_d2cf_4278_8b6a_d077fac3a25f);
pub const MF_POLICY_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb160c24d_c059_48f1_a901_9ee298a9a8c3);
pub const MF_PREFERRED_SOURCE_URI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5fc85488_436a_4db8_90af_4db402ae5c57);
pub const MF_PROGRESSIVE_CODING_CONTENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8f020eea_1508_471f_9da6_507d7cfa40db);
pub const MF_PROPERTY_HANDLER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa3face02_32b8_41dd_90e7_5fef7c8991b5);
pub const MF_QUALITY_NOTIFY_PROCESSING_LATENCY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf6b44af8_604d_46fe_a95d_45479b10c9bc);
pub const MF_QUALITY_NOTIFY_SAMPLE_LAG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x30d15206_ed2a_4760_be17_eb4a9f12295c);
pub const MF_QUALITY_SERVICES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb7e2be11_2f96_4640_b52c_282365bdf16c);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_QUOTA_EXCEEDED_ERR: u32 = 2154823702u32;
pub const MF_RATE_CONTROL_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x866fa297_b802_4bf8_9dc9_5e3b6a9f53c9);
pub const MF_READWRITE_D3D_OPTIONAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x216479d9_3071_42ca_bb6c_4c22102e1d18);
pub const MF_READWRITE_DISABLE_CONVERTERS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x98d5b065_1374_4847_8d5d_31520fee7156);
pub const MF_READWRITE_ENABLE_AUTOFINALIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdd7ca129_8cd1_4dc5_9dde_ce168675de61);
pub const MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa634a91c_822b_41b9_a494_4de4643612b0);
pub const MF_READWRITE_MMCSS_CLASS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x39384300_d0eb_40b1_87a0_3318871b5a53);
pub const MF_READWRITE_MMCSS_CLASS_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x430847da_0890_4b0e_938c_054332c547e1);
pub const MF_READWRITE_MMCSS_PRIORITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x43ad19ce_f33f_4ba9_a580_e4cd12f2d144);
pub const MF_READWRITE_MMCSS_PRIORITY_AUDIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x273db885_2de2_4db2_a6a7_fdb66fb40b61);
pub const MF_REMOTE_PROXY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2f00c90e_d2cf_4278_8b6a_d077fac3a25f);
pub const MF_SAMI_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x49a89ae7_b4d9_4ef2_aa5c_f65a3e05ae4e);
pub const MF_SAMPLEGRABBERSINK_IGNORE_CLOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0efda2c0_2b69_4e2e_ab8d_46dcbff7d25d);
pub const MF_SAMPLEGRABBERSINK_SAMPLE_TIME_OFFSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x62e3d776_8100_4e03_a6e8_bd3857ac9c47);
pub const MF_SA_AUDIO_ENDPOINT_AWARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc0381701_805c_42b2_ac8d_e2b4bf21f4f8);
pub const MF_SA_BUFFERS_PER_SAMPLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x873c5171_1e3d_4e25_988d_b433ce041983);
pub const MF_SA_D3D11_ALLOCATE_DISPLAYABLE_RESOURCES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeeface6d_2ea9_4adf_bbdf_7bbc482a1b6d);
pub const MF_SA_D3D11_ALLOW_DYNAMIC_YUV_TEXTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xce06d49f_0613_4b9d_86a6_d8c4f9c10075);
pub const MF_SA_D3D11_AWARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x206b4fc8_fcf9_4c51_afe3_9764369e33a0);
pub const MF_SA_D3D11_BINDFLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeacf97ad_065c_4408_bee3_fdcbfd128be2);
pub const MF_SA_D3D11_HW_PROTECTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3a8ba9d9_92ca_4307_a391_6999dbf3b6ce);
pub const MF_SA_D3D11_SHARED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b8f32c3_6d96_4b89_9203_dd38b61414f3);
pub const MF_SA_D3D11_SHARED_WITHOUT_MUTEX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x39dbd44d_2e44_4931_a4c8_352d3dc42115);
pub const MF_SA_D3D11_USAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe85fe442_2ca3_486e_a9c7_109dda609880);
pub const MF_SA_D3D12_CLEAR_VALUE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x86ba9a39_0526_495d_9ab5_54ec9fad6fc3);
pub const MF_SA_D3D12_HEAP_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x496b3266_d28f_4f8c_93a7_4a596b1a31a1);
pub const MF_SA_D3D12_HEAP_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x56f26a76_bbc1_4ce0_bb11_e22368d874ed);
pub const MF_SA_D3D_AWARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeaa35c29_775e_488e_9b61_b3283e49583b);
pub const MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x851745d5_c3d6_476d_9527_498ef2d10d18);
pub const MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT_PROGRESSIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0f5523a5_1cb2_47c5_a550_2eeb84b4d14a);
pub const MF_SA_REQUIRED_SAMPLE_COUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x18802c61_324b_4952_abd0_176ff5c696ff);
pub const MF_SA_REQUIRED_SAMPLE_COUNT_PROGRESSIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb172d58e_fa77_4e48_8d2a_1df2d850eac2);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SDK_VERSION: u32 = 2u32;
pub const MF_SD_AMBISONICS_SAMPLE3D_DESCRIPTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf715cf3e_a964_4c3f_94ae_9d6ba7264641);
pub const MF_SD_ASF_EXTSTRMPROP_AVG_BUFFERSIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48f8a524_305d_422d_8524_2502dda33680);
pub const MF_SD_ASF_EXTSTRMPROP_AVG_DATA_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48f8a523_305d_422d_8524_2502dda33680);
pub const MF_SD_ASF_EXTSTRMPROP_LANGUAGE_ID_INDEX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48f8a522_305d_422d_8524_2502dda33680);
pub const MF_SD_ASF_EXTSTRMPROP_MAX_BUFFERSIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48f8a526_305d_422d_8524_2502dda33680);
pub const MF_SD_ASF_EXTSTRMPROP_MAX_DATA_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48f8a525_305d_422d_8524_2502dda33680);
pub const MF_SD_ASF_METADATA_DEVICE_CONFORMANCE_TEMPLATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x245e929d_c44e_4f7e_bb3c_77d4dfd27f8a);
pub const MF_SD_ASF_STREAMBITRATES_BITRATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa8e182ed_afc8_43d0_b0d1_f65bad9da558);
pub const MF_SD_AUDIO_ENCODER_DELAY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8e85422c_73de_403f_9a35_550ad6e8b951);
pub const MF_SD_AUDIO_ENCODER_PADDING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x529c7f2c_ac4b_4e3f_bfc3_0902194982cb);
pub const MF_SD_LANGUAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00af2180_bdc2_423c_abca_f503593bc121);
pub const MF_SD_MEDIASOURCE_STATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1913678b_fc0f_44da_8f43_1ba3b526f4ae);
pub const MF_SD_MUTUALLY_EXCLUSIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x023ef79c_388d_487f_ac17_696cd6e3c6f5);
pub const MF_SD_PROTECTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00af2181_bdc2_423c_abca_f503593bc121);
pub const MF_SD_SAMI_LANGUAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36fcb98a_6cd0_44cb_acb9_a8f5600dd0bb);
pub const MF_SD_STREAM_NAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4f1b099d_d314_41e5_a781_7fefaa4c501f);
pub const MF_SD_VIDEO_SPHERICAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa51da449_3fdc_478c_bcb5_30be76595f55);
pub const MF_SD_VIDEO_SPHERICAL_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4a8fc407_6ea1_46c8_b567_6971d4a139c3);
pub const MF_SD_VIDEO_SPHERICAL_INITIAL_VIEWDIRECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x11d25a49_bb62_467f_9db1_c17165716c49);
pub const MF_SESSION_APPROX_EVENT_OCCURRENCE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x190e852f_6238_42d1_b5af_69ea338ef850);
pub const MF_SESSION_CONTENT_PROTECTION_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1e83d482_1f1c_4571_8405_88f4b2181f74);
pub const MF_SESSION_GLOBAL_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1e83d482_1f1c_4571_8405_88f4b2181f72);
pub const MF_SESSION_QUALITY_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1e83d482_1f1c_4571_8405_88f4b2181f73);
pub const MF_SESSION_REMOTE_SOURCE_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf4033ef4_9bb3_4378_941f_85a0856bc244);
pub const MF_SESSION_SERVER_CONTEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xafe5b291_50fa_46e8_b9be_0c0c3ce4b3a5);
pub const MF_SESSION_TOPOLOADER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1e83d482_1f1c_4571_8405_88f4b2181f71);
pub const MF_SHARING_ENGINE_CALLBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57dc1e95_d252_43fa_9bbc_180070eefe6d);
pub const MF_SHARING_ENGINE_SHAREDRENDERER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xefa446a0_73e7_404e_8ae2_fef60af5a32b);
pub const MF_SHUTDOWN_RENDERER_ON_ENGINE_SHUTDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc112d94d_6b9c_48f8_b6f9_7950ff9ab71e);
pub const MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_DENOMINATOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6ea1eb97_1fe0_4f10_a6e4_1f4f661564e0);
pub const MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_NUMERATOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd0f33b22_b78a_4879_b455_f03ef3fa82cd);
pub const MF_SINK_VIDEO_NATIVE_HEIGHT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf0ca6705_490c_43e8_941c_c0b3206b9a65);
pub const MF_SINK_VIDEO_NATIVE_WIDTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe6d6a707_1505_4747_9b10_72d2d158cb3a);
pub const MF_SINK_VIDEO_PTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2162bde7_421e_4b90_9b33_e58fbf1d58b6);
pub const MF_SINK_WRITER_ASYNC_CALLBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48cb183e_7b0b_46f4_822e_5e1d2dda4354);
pub const MF_SINK_WRITER_D3D_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xec822da2_e1e9_4b29_a0d8_563c719f5269);
pub const MF_SINK_WRITER_DISABLE_THROTTLING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x08b845d8_2b74_4afe_9d53_be16d2d5ae4f);
pub const MF_SINK_WRITER_ENCODER_CONFIG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xad91cd04_a7cc_4ac7_99b6_a57b9a4a7c70);
pub const MF_SOURCE_PRESENTATION_PROVIDER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe002aadc_f4af_4ee5_9847_053edf840426);
pub const MF_SOURCE_READER_ASYNC_CALLBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1e3dbeac_bb43_4c35_b507_cd644464c965);
pub const MF_SOURCE_READER_D3D11_BIND_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x33f3197b_f73a_4e14_8d85_0e4c4368788d);
pub const MF_SOURCE_READER_D3D_MANAGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xec822da2_e1e9_4b29_a0d8_563c719f5269);
pub const MF_SOURCE_READER_DISABLE_CAMERA_PLUGINS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d3365dd_058f_4cfb_9f97_b314cc99c8ad);
pub const MF_SOURCE_READER_DISABLE_DXVA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaa456cfd_3943_4a1e_a77d_1838c0ea2e35);
pub const MF_SOURCE_READER_DISCONNECT_MEDIASOURCE_ON_SHUTDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x56b67165_219e_456d_a22e_2d3004c7fe56);
pub const MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0f81da2c_b537_4672_a8b2_a681b17307a3);
pub const MF_SOURCE_READER_ENABLE_TRANSCODE_ONLY_TRANSFORMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdfd4f008_b5fd_4e78_ae44_62a1e67bbe27);
pub const MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb394f3d_ccf1_42ee_bbb3_f9b845d5681d);
pub const MF_SOURCE_READER_MEDIASOURCE_CHARACTERISTICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d23f5c8_c5d7_4a9b_9971_5d11f8bca880);
pub const MF_SOURCE_READER_MEDIASOURCE_CONFIG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9085abeb_0354_48f9_abb5_200df838c68e);
pub const MF_SOURCE_STREAM_SUPPORTS_HW_CONNECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa38253aa_6314_42fd_a3ce_bb27b6859946);
pub const MF_STF_VERSION_DATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31a165d5_df67_4095_8e44_8868fc20dbfd);
pub const MF_STF_VERSION_INFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6770bd39_ef82_44ee_a49b_934beb24aef7);
pub const MF_STREAM_SINK_SUPPORTS_HW_CONNECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9b465cbf_0597_4f9e_9f3c_b97eeef90359);
pub const MF_STREAM_SINK_SUPPORTS_ROTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb3e96280_bd05_41a5_97ad_8a7fee24b912);
pub const MF_ST_MEDIASOURCE_COLLECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x616de972_83ad_4950_8170_630d19cbe307);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SYNTAX_ERR: u32 = 2154823692u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_ACTIVATE_REPLACED: ::windows_sys::core::HRESULT = 866045i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_ASF_PARSEINPROGRESS: ::windows_sys::core::HRESULT = 1074608792i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_CLOCK_STOPPED: ::windows_sys::core::HRESULT = 891972i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_MULTIPLE_BEGIN: ::windows_sys::core::HRESULT = 866008i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_PE_TRUSTED: ::windows_sys::core::HRESULT = 881011i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_PROTECTION_NOT_REQUIRED: ::windows_sys::core::HRESULT = 880976i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_SEQUENCER_CONTEXT_CANCELED: ::windows_sys::core::HRESULT = 876973i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_SEQUENCER_SEGMENT_AT_END_OF_STREAM: ::windows_sys::core::HRESULT = 876975i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_SINK_NOT_FINALIZED: ::windows_sys::core::HRESULT = 870978i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_TRANSFORM_DO_NOT_PROPAGATE_EVENT: ::windows_sys::core::HRESULT = 879989i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_VIDEO_DISABLED_WITH_UNKNOWN_SOFTWARE_OUTPUT: ::windows_sys::core::HRESULT = 881001i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_S_WAIT_FOR_POLICY_SET: ::windows_sys::core::HRESULT = 881000i32;
pub const MF_SampleProtectionSalt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5403deee_b9ee_438f_aa83_3804997e569d);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TEST_SIGNED_COMPONENT_LOADING: u32 = 16777216u32;
pub const MF_TIMECODE_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0d502a7_0eb3_4885_b1b9_9feb0d083454);
pub const MF_TIME_FORMAT_ENTRY_RELATIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4399f178_46d3_4504_afda_20d32e9ba360);
pub const MF_TIME_FORMAT_SEGMENT_OFFSET: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc8b8be77_869c_431d_812e_169693f65a39);
pub const MF_TOPOLOGY_DXVA_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1e8d34f6_f5ab_4e23_bb88_874aa3a1a74d);
pub const MF_TOPOLOGY_DYNAMIC_CHANGE_NOT_ALLOWED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd529950b_d484_4527_a9cd_b1909532b5b0);
pub const MF_TOPOLOGY_ENABLE_XVP_FOR_PLAYBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1967731f_cd78_42fc_b026_0992a56e5693);
pub const MF_TOPOLOGY_ENUMERATE_SOURCE_TYPES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6248c36d_5d0b_4f40_a0bb_b0b305f77698);
pub const MF_TOPOLOGY_HARDWARE_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd2d362fd_4e4f_4191_a579_c618b66706af);
pub const MF_TOPOLOGY_NO_MARKIN_MARKOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7ed3f804_86bb_4b3f_b7e4_7cb43afd4b80);
pub const MF_TOPOLOGY_PLAYBACK_FRAMERATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc164737a_c2b1_4553_83bb_5a526072448f);
pub const MF_TOPOLOGY_PLAYBACK_MAX_DIMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5715cf19_5768_44aa_ad6e_8721f1b0f9bb);
pub const MF_TOPOLOGY_PROJECTSTART: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7ed3f802_86bb_4b3f_b7e4_7cb43afd4b80);
pub const MF_TOPOLOGY_PROJECTSTOP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7ed3f803_86bb_4b3f_b7e4_7cb43afd4b80);
pub const MF_TOPOLOGY_RESOLUTION_STATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcde_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPOLOGY_START_TIME_ON_PRESENTATION_SWITCH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc8cc113f_7951_4548_aad6_9ed6202e62b3);
pub const MF_TOPOLOGY_STATIC_PLAYBACK_OPTIMIZATIONS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb86cac42_41a6_4b79_897a_1ab0e52b4a1b);
pub const MF_TOPONODE_ATTRIBUTE_EDITOR_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x65656e1a_077f_4472_83ef_316f11d5087a);
pub const MF_TOPONODE_CONNECT_METHOD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcf1_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_D3DAWARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbced_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbd02_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_DECRYPTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcfa_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_DISABLE_PREROLL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x14932f9e_9087_4bb4_8412_5167145cbe04);
pub const MF_TOPONODE_DISCARDABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcfb_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_DRAIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbce9_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_ERRORCODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcee_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_ERROR_MAJORTYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcfd_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_ERROR_SUBTYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcfe_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_FLUSH: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbce8_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_LOCKED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcf7_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_MARKIN_HERE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbd00_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_MARKOUT_HERE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbd01_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_MEDIASTART: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x835c58ea_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_MEDIASTOP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x835c58eb_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_NOSHUTDOWN_ON_REMOVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x14932f9c_9087_4bb4_8412_5167145cbe04);
pub const MF_TOPONODE_PRESENTATION_DESCRIPTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x835c58ed_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_PRIMARYOUTPUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6304ef99_16b2_4ebe_9d67_e4c539b3a259);
pub const MF_TOPONODE_RATELESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x14932f9d_9087_4bb4_8412_5167145cbe04);
pub const MF_TOPONODE_SEQUENCE_ELEMENTID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x835c58ef_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_SOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x835c58ec_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_STREAMID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x14932f9b_9087_4bb4_8412_5167145cbe04);
pub const MF_TOPONODE_STREAM_DESCRIPTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x835c58ee_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_TRANSFORM_OBJECTID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x88dcc0c9_293e_4e8b_9aeb_0ad64cc016b0);
pub const MF_TOPONODE_WORKQUEUE_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcf8_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_WORKQUEUE_ITEM_PRIORITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa1ff99be_5e97_4a53_b494_568c642c0ff3);
pub const MF_TOPONODE_WORKQUEUE_MMCSS_CLASS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcf9_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_WORKQUEUE_MMCSS_PRIORITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5001f840_2816_48f4_9364_ad1ef661a123);
pub const MF_TOPONODE_WORKQUEUE_MMCSS_TASKID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x494bbcff_b031_4e38_97c4_d5422dd618dc);
pub const MF_TRANSCODE_ADJUST_PROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9c37c21b_060f_487c_a690_80d7f50d1c72);
pub const MF_TRANSCODE_CONTAINERTYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x150ff23f_4abc_478b_ac4f_e1916fba1cca);
pub const MF_TRANSCODE_DONOT_INSERT_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf45aa7ce_ab24_4012_a11b_dc8220201410);
pub const MF_TRANSCODE_ENCODINGPROFILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6947787c_f508_4ea9_b1e9_a1fe3a49fbc9);
pub const MF_TRANSCODE_QUALITYVSSPEED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x98332df8_03cd_476b_89fa_3f9e442dec9f);
pub const MF_TRANSCODE_SKIP_METADATA_TRANSFER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4e4469ef_b571_4959_8f83_3dcfba33a393);
pub const MF_TRANSCODE_TOPOLOGYMODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3e3df610_394a_40b2_9dea_3bab650bebf2);
pub const MF_TRANSFORM_ASYNC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf81a699a_649a_497d_8c73_29f8fed6ad7a);
pub const MF_TRANSFORM_ASYNC_UNLOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe5666d6b_3422_4eb6_a421_da7db1f8e207);
pub const MF_TRANSFORM_CATEGORY_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xceabba49_506d_4757_a6ff_66c184987e4e);
pub const MF_TRANSFORM_FLAGS_Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9359bb7e_6275_46c4_a025_1c01e45f1a86);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TYPE_ERR: u32 = 2154840069u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_UNKNOWN_DURATION: u32 = 0u32;
pub const MF_USER_DATA_PAYLOAD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd1d4985d_dc92_457a_b3a0_651a33a31047);
pub const MF_USER_EXTENDED_ATTRIBUTES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc02abac6_feb2_4541_922f_920b43702722);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_USER_MODE_COMPONENT_LOAD: u32 = 1u32;
pub const MF_VIDEODSP_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x16d720f0_768c_11de_8a39_0800200c9a66);
pub const MF_VIDEO_MAX_MB_PER_SEC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe3f2e203_d445_4b8c_9211_ae390d3ba017);
pub const MF_VIDEO_PROCESSOR_ALGORITHM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4a0a1e1f_272c_4fb6_9eb1_db330cbc97ca);
pub const MF_VIDEO_RENDERER_EFFECT_APP_SERVICE_NAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc6052a80_6d9c_40a3_9db8_f027a25c9ab9);
pub const MF_VIRTUALCAMERA_CONFIGURATION_APP_PACKAGE_FAMILY_NAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x658abe51_8044_462e_97ea_e676fd72055f);
pub const MF_WORKQUEUE_SERVICES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8e37d489_41e0_413a_9068_287c886d8dda);
pub const MF_WRAPPED_BUFFER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xab544072_c269_4ebc_a552_1c3b32bed5ca);
pub const MF_WRAPPED_OBJECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2b182c4c_d6ac_49f4_8915_f71887db70cd);
pub const MF_WRAPPED_SAMPLE_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x31f52bf2_d03e_4048_80d0_9c1046d87c61);
pub const MF_WVC1_PROG_SINGLE_SLICE_CONTENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x67ec2559_0f2f_4420_a4dd_2f8ee7a5738b);
pub const MF_XVP_CALLER_ALLOCATES_OUTPUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x04a2cabc_0cab_40b1_a1b9_75bc3658f000);
pub const MF_XVP_DISABLE_FRC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2c0afa19_7a97_4d5a_9ee8_16d4fc518d8c);
pub const MF_XVP_SAMPLE_LOCK_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaa4ddb29_5134_4363_ac72_83ec4bc10426);
pub const MP3ACMCodecWrapper: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x11103421_354c_4cca_a7a3_1aff9a5b6701);
pub const MR_AUDIO_POLICY_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x911fd737_6775_4ab0_a614_297862fdac88);
pub const MR_BUFFER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa562248c_9ac6_4ffc_9fba_3af8f8ad1a4d);
pub const MR_CAPTURE_POLICY_VOLUME_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x24030acd_107a_4265_975c_414e33e65f2a);
pub const MR_POLICY_VOLUME_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1abaa2ac_9d3b_47c6_ab48_c59506de784d);
pub const MR_STREAM_VOLUME_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf8b5fa2f_32ef_46f5_b172_1321212fb2c4);
pub const MR_VIDEO_ACCELERATION_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xefef5175_5c7d_4ce2_bbbd_34ff8bca6554);
pub const MR_VIDEO_MIXER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x073cd2fc_6cf4_40b7_8859_e89552c841f8);
pub const MR_VIDEO_RENDER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1092a86c_ab1a_459a_a336_831fbc4d11ff);
pub const MSAMRNBDecoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x265011ae_5481_4f77_a295_abb6ffe8d63e);
pub const MSAMRNBEncoder: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2fae8afe_04a3_423a_a814_85db454712b0);
pub const MULawCodecWrapper: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x92b66080_5e2d_449e_90c4_c41f268e5514);
pub const OPM_GET_ACP_AND_CGMSA_SIGNALING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6629a591_3b79_4cf3_924a_11e8e7811671);
pub const OPM_GET_ACTUAL_OUTPUT_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd7bf1ba3_ad13_4f8e_af98_0dcb3ca204cc);
pub const OPM_GET_ACTUAL_PROTECTION_LEVEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1957210a_7766_452a_b99a_d27aed54f03a);
pub const OPM_GET_ADAPTER_BUS_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc6f4d673_6174_4184_8e35_f6db5200bcba);
pub const OPM_GET_CODEC_INFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4f374491_8f5f_4445_9dba_95588f6b58b4);
pub const OPM_GET_CONNECTED_HDCP_DEVICE_INFORMATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0db59d74_a992_492e_a0bd_c23fda564e00);
pub const OPM_GET_CONNECTOR_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x81d0bfd5_6afe_48c2_99c0_95a08f97c5da);
pub const OPM_GET_CURRENT_HDCP_SRM_VERSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x99c5ceff_5f1d_4879_81c1_c52443c9482b);
pub const OPM_GET_DVI_CHARACTERISTICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa470b3bb_5dd7_4172_839c_3d3776e0ebf5);
pub const OPM_GET_OUTPUT_HARDWARE_PROTECTION_SUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3b129589_2af8_4ef0_96a2_704a845a218e);
pub const OPM_GET_OUTPUT_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x72cb6df3_244f_40ce_b09e_20506af6302f);
pub const OPM_GET_SUPPORTED_PROTECTION_TYPES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x38f2a801_9a6c_48bb_9107_b6696e6f1797);
pub const OPM_GET_VIRTUAL_PROTECTION_LEVEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb2075857_3eda_4d5d_88db_748f8c1a0549);
pub const OPM_SET_ACP_AND_CGMSA_SIGNALING: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x09a631a5_d684_4c60_8e4d_d3bb0f0be3ee);
pub const OPM_SET_HDCP_SRM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8b5ef5d1_c30d_44ff_84a5_ea71dce78f13);
pub const OPM_SET_PROTECTION_LEVEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9bb9327c_4eb5_4727_9f00_b42b0919c0da);
pub const OPM_SET_PROTECTION_LEVEL_ACCORDING_TO_CSS_DVD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x39ce333e_4cc0_44ae_bfcc_da50b5f82e72);
pub const PIN_CATEGORY_ANALOGVIDEOIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c4283_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c4281_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_CC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c4289_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_EDS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c4287_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_NABTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c4286_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_PREVIEW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c4282_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_STILL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c428a_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_TELETEXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c4288_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_TIMECODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c428b_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_VBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c4284_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_VIDEOPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c4285_0353_11d1_905f_0000c0cc16ba);
pub const PIN_CATEGORY_VIDEOPORT_VBI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb6c428c_0353_11d1_905f_0000c0cc16ba);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PRESENTATION_CURRENT_POSITION: u64 = 9223372036854775807u64;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SHA_HASH_LEN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SYSFXUI_DONOTSHOW_BASSBOOST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SYSFXUI_DONOTSHOW_BASSMANAGEMENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SYSFXUI_DONOTSHOW_CHANNELPHANTOMING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SYSFXUI_DONOTSHOW_HEADPHONEVIRTUALIZATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SYSFXUI_DONOTSHOW_LOUDNESSEQUALIZATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SYSFXUI_DONOTSHOW_ROOMCORRECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SYSFXUI_DONOTSHOW_SPEAKERFILLING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SYSFXUI_DONOTSHOW_VIRTUALSURROUND: u32 = 32u32;
pub const TIME_FORMAT_BYTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b785571_8c82_11cf_bc0c_00aa00ac74f6);
pub const TIME_FORMAT_FIELD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b785573_8c82_11cf_bc0c_00aa00ac74f6);
pub const TIME_FORMAT_FRAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b785570_8c82_11cf_bc0c_00aa00ac74f6);
pub const TIME_FORMAT_MEDIA_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b785574_8c82_11cf_bc0c_00aa00ac74f6);
pub const TIME_FORMAT_NONE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const TIME_FORMAT_SAMPLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b785572_8c82_11cf_bc0c_00aa00ac74f6);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const TOC_ENTRY_MAX_TITLE_SIZE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const TOC_MAX_DESCRIPTION_SIZE: u32 = 65535u32;
pub const UUID_UdriTagTables: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe1b98d74_9778_4878_b664_eb2020364d88);
pub const UUID_WMDRMTagTables: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5dcd1101_9263_45bb_a4d5_c415ab8c589c);
pub const VIDEO_ZOOM_RECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7aaa1638_1b7f_4c93_bd89_5b9c9fb6fcf0);
pub const VorbisDecoderMFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1a198ef2_60e5_4ea8_90d8_da1f2832c288);
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMAAECMA_E_NO_ACTIVE_RENDER_STREAM: u32 = 2278293514u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WM_CODEC_ONEPASS_CBR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WM_CODEC_ONEPASS_VBR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WM_CODEC_TWOPASS_CBR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WM_CODEC_TWOPASS_VBR_PEAKCONSTRAINED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WM_CODEC_TWOPASS_VBR_UNCONSTRAINED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const g_wszSpeechFormatCaps: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SpeechFormatCap");
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const g_wszWMCPAudioVBRQuality: ::windows_sys::core::PCWSTR = ::windows_sys::w!("_VBRQUALITY");
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const g_wszWMCPAudioVBRSupported: ::windows_sys::core::PCWSTR = ::windows_sys::w!("_VBRENABLED");
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const g_wszWMCPCodecName: ::windows_sys::core::PCWSTR = ::windows_sys::w!("_CODECNAME");
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const g_wszWMCPDefaultCrisp: ::windows_sys::core::PCWSTR = ::windows_sys::w!("_DEFAULTCRISP");
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const g_wszWMCPMaxPasses: ::windows_sys::core::PCWSTR = ::windows_sys::w!("_PASSESRECOMMENDED");
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const g_wszWMCPSupportedVBRModes: ::windows_sys::core::PCWSTR = ::windows_sys::w!("_SUPPORTEDVBRMODES");
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type AEC_INPUT_STREAM = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AEC_CAPTURE_STREAM: AEC_INPUT_STREAM = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AEC_REFERENCE_STREAM: AEC_INPUT_STREAM = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type AEC_SYSTEM_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SINGLE_CHANNEL_AEC: AEC_SYSTEM_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ADAPTIVE_ARRAY_ONLY: AEC_SYSTEM_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPTIBEAM_ARRAY_ONLY: AEC_SYSTEM_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ADAPTIVE_ARRAY_AND_AEC: AEC_SYSTEM_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPTIBEAM_ARRAY_AND_AEC: AEC_SYSTEM_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SINGLE_CHANNEL_NSAGC: AEC_SYSTEM_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MODE_NOT_SET: AEC_SYSTEM_MODE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type AEC_VAD_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AEC_VAD_DISABLED: AEC_VAD_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AEC_VAD_NORMAL: AEC_VAD_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AEC_VAD_FOR_AGC: AEC_VAD_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AEC_VAD_FOR_SILENCE_SUPPRESSION: AEC_VAD_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type ASF_SELECTION_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ASF_STATUS_NOTSELECTED: ASF_SELECTION_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ASF_STATUS_CLEANPOINTSONLY: ASF_SELECTION_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ASF_STATUS_ALLDATAUNITS: ASF_SELECTION_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type ASF_STATUSFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ASF_STATUSFLAGS_INCOMPLETE: ASF_STATUSFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ASF_STATUSFLAGS_NONFATAL_ERROR: ASF_STATUSFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_BITSTREAM_ENCRYPTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_BITSTREAM_ENCRYPTION_TYPE_NONE: D3D12_BITSTREAM_ENCRYPTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_FEATURE_VIDEO = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODE_SUPPORT: D3D12_FEATURE_VIDEO = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODE_PROFILES: D3D12_FEATURE_VIDEO = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODE_FORMATS: D3D12_FEATURE_VIDEO = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODE_CONVERSION_SUPPORT: D3D12_FEATURE_VIDEO = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_PROCESS_SUPPORT: D3D12_FEATURE_VIDEO = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_PROCESS_MAX_INPUT_STREAMS: D3D12_FEATURE_VIDEO = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_PROCESS_REFERENCE_INFO: D3D12_FEATURE_VIDEO = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODER_HEAP_SIZE: D3D12_FEATURE_VIDEO = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_PROCESSOR_SIZE: D3D12_FEATURE_VIDEO = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODE_PROFILE_COUNT: D3D12_FEATURE_VIDEO = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODE_FORMAT_COUNT: D3D12_FEATURE_VIDEO = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_ARCHITECTURE: D3D12_FEATURE_VIDEO = 17i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODE_HISTOGRAM: D3D12_FEATURE_VIDEO = 18i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_FEATURE_AREA_SUPPORT: D3D12_FEATURE_VIDEO = 19i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_MOTION_ESTIMATOR: D3D12_FEATURE_VIDEO = 20i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_MOTION_ESTIMATOR_SIZE: D3D12_FEATURE_VIDEO = 21i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_COUNT: D3D12_FEATURE_VIDEO = 22i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMANDS: D3D12_FEATURE_VIDEO = 23i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT: D3D12_FEATURE_VIDEO = 24i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_PARAMETERS: D3D12_FEATURE_VIDEO = 25i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_SUPPORT: D3D12_FEATURE_VIDEO = 26i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_SIZE: D3D12_FEATURE_VIDEO = 27i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODE_PROTECTED_RESOURCES: D3D12_FEATURE_VIDEO = 28i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_PROCESS_PROTECTED_RESOURCES: D3D12_FEATURE_VIDEO = 29i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES: D3D12_FEATURE_VIDEO = 30i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_DECODER_HEAP_SIZE1: D3D12_FEATURE_VIDEO = 31i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_FEATURE_VIDEO_PROCESSOR_SIZE1: D3D12_FEATURE_VIDEO = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_DECODE_ARGUMENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_ARGUMENT_TYPE_PICTURE_PARAMETERS: D3D12_VIDEO_DECODE_ARGUMENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_ARGUMENT_TYPE_INVERSE_QUANTIZATION_MATRIX: D3D12_VIDEO_DECODE_ARGUMENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_ARGUMENT_TYPE_SLICE_CONTROL: D3D12_VIDEO_DECODE_ARGUMENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_ARGUMENT_TYPE_MAX_VALID: D3D12_VIDEO_DECODE_ARGUMENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_NONE: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_HEIGHT_ALIGNMENT_MULTIPLE_32_REQUIRED: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_POST_PROCESSING_SUPPORTED: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_REFERENCE_ONLY_ALLOCATIONS_REQUIRED: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_ALLOW_RESOLUTION_CHANGE_ON_NON_KEY_FRAME: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAG_NONE: D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_Y: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_U: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_V: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_R: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_G: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_B: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_A: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_NONE: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_Y: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_U: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_V: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_R: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_G: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_B: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_A: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_DECODE_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_STATUS_OK: D3D12_VIDEO_DECODE_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_STATUS_CONTINUE: D3D12_VIDEO_DECODE_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_STATUS_CONTINUE_SKIP_DISPLAY: D3D12_VIDEO_DECODE_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_STATUS_RESTART: D3D12_VIDEO_DECODE_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_STATUS_RATE_EXCEEDED: D3D12_VIDEO_DECODE_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_DECODE_SUPPORT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_SUPPORT_FLAG_NONE: D3D12_VIDEO_DECODE_SUPPORT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_DECODE_SUPPORT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_DECODE_TIER = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_TIER_NOT_SUPPORTED: D3D12_VIDEO_DECODE_TIER = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_TIER_1: D3D12_VIDEO_DECODE_TIER = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_TIER_2: D3D12_VIDEO_DECODE_TIER = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_DECODE_TIER_3: D3D12_VIDEO_DECODE_TIER = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_H264: D3D12_VIDEO_ENCODER_CODEC = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_HEVC: D3D12_VIDEO_ENCODER_CODEC = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES_DISABLED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES_TEMPORAL: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES_SPATIAL: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_USE_CONSTRAINED_INTRAPREDICTION: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_USE_ADAPTIVE_8x8_TRANSFORM: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_ENABLE_CABAC_ENCODING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_ALLOW_REQUEST_INTRA_CONSTRAINED_SLICES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_0_ALL_LUMA_CHROMA_SLICE_BLOCK_EDGES_ALWAYS_FILTERED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_1_DISABLE_ALL_SLICE_BLOCK_EDGES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_2_DISABLE_SLICE_BOUNDARIES_BLOCKS: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_3_USE_TWO_STAGE_DEBLOCKING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_4_DISABLE_CHROMA_BLOCK_EDGES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_5_DISABLE_CHROMA_BLOCK_EDGES_AND_LUMA_BOUNDARIES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_6_DISABLE_CHROMA_BLOCK_EDGES_AND_USE_LUMA_TWO_STAGE_DEBLOCKING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_0_ALL_LUMA_CHROMA_SLICE_BLOCK_EDGES_ALWAYS_FILTERED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_1_DISABLE_ALL_SLICE_BLOCK_EDGES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_2_DISABLE_SLICE_BOUNDARIES_BLOCKS: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_3_USE_TWO_STAGE_DEBLOCKING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_4_DISABLE_CHROMA_BLOCK_EDGES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_5_DISABLE_CHROMA_BLOCK_EDGES_AND_LUMA_BOUNDARIES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_6_DISABLE_CHROMA_BLOCK_EDGES_AND_USE_LUMA_TWO_STAGE_DEBLOCKING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE_8x8: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE_16x16: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE_32x32: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE_64x64: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_DISABLE_LOOP_FILTER_ACROSS_SLICES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_ALLOW_REQUEST_INTRA_CONSTRAINED_SLICES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_ENABLE_SAO_FILTER: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_ENABLE_LONG_TERM_REFERENCES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_USE_ASYMETRIC_MOTION_PARTITION: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_ENABLE_TRANSFORM_SKIPPING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_USE_CONSTRAINED_INTRAPREDICTION: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE_4x4: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE_8x8: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE_16x16: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE_32x32: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_CABAC_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_INTRA_SLICE_CONSTRAINED_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_BFRAME_LTR_COMBINED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_ADAPTIVE_8x8_TRANSFORM_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_DIRECT_SPATIAL_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_DIRECT_TEMPORAL_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_CONSTRAINED_INTRAPREDICTION_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_BFRAME_LTR_COMBINED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_INTRA_SLICE_CONSTRAINED_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_CONSTRAINED_INTRAPREDICTION_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_SAO_FILTER_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_ASYMETRIC_MOTION_PARTITION_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_ASYMETRIC_MOTION_PARTITION_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_TRANSFORM_SKIP_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_DISABLING_LOOP_FILTER_ACROSS_SLICES_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_P_FRAMES_IMPLEMENTED_AS_LOW_DELAY_B_FRAMES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_NO_ERROR: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_CODEC_PICTURE_CONTROL_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_SUBREGION_LAYOUT_CONFIGURATION_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_INVALID_REFERENCE_PICTURES: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_RECONFIGURATION_REQUEST_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_INVALID_METADATA_BUFFER_SOURCE: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FLAG_NONE: D3D12_VIDEO_ENCODER_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_FULL_FRAME: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_BYTES_PER_SUBREGION: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_SQUARE_UNITS_PER_SUBREGION_ROW_UNALIGNED: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_UNIFORM_PARTITIONING_ROWS_PER_SUBREGION: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_UNIFORM_PARTITIONING_SUBREGIONS_PER_FRAME: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_H264_I_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_H264_P_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_H264_B_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_H264_IDR_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC_I_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC_P_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC_B_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC_IDR_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_HEAP_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_HEAP_FLAG_NONE: D3D12_VIDEO_ENCODER_HEAP_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE_NONE: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE_ROW_BASED: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_LEVELS_H264 = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_1: D3D12_VIDEO_ENCODER_LEVELS_H264 = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_1b: D3D12_VIDEO_ENCODER_LEVELS_H264 = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_11: D3D12_VIDEO_ENCODER_LEVELS_H264 = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_12: D3D12_VIDEO_ENCODER_LEVELS_H264 = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_13: D3D12_VIDEO_ENCODER_LEVELS_H264 = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_2: D3D12_VIDEO_ENCODER_LEVELS_H264 = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_21: D3D12_VIDEO_ENCODER_LEVELS_H264 = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_22: D3D12_VIDEO_ENCODER_LEVELS_H264 = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_3: D3D12_VIDEO_ENCODER_LEVELS_H264 = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_31: D3D12_VIDEO_ENCODER_LEVELS_H264 = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_32: D3D12_VIDEO_ENCODER_LEVELS_H264 = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_4: D3D12_VIDEO_ENCODER_LEVELS_H264 = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_41: D3D12_VIDEO_ENCODER_LEVELS_H264 = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_42: D3D12_VIDEO_ENCODER_LEVELS_H264 = 13i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_5: D3D12_VIDEO_ENCODER_LEVELS_H264 = 14i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_51: D3D12_VIDEO_ENCODER_LEVELS_H264 = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_52: D3D12_VIDEO_ENCODER_LEVELS_H264 = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_6: D3D12_VIDEO_ENCODER_LEVELS_H264 = 17i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_61: D3D12_VIDEO_ENCODER_LEVELS_H264 = 18i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_62: D3D12_VIDEO_ENCODER_LEVELS_H264 = 19i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_LEVELS_HEVC = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_1: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_2: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_21: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_3: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_31: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_4: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_41: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_5: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_51: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_52: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_6: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_61: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_62: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE_MAXIMUM: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE_FULL_PIXEL: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE_HALF_PIXEL: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE_QUARTER_PIXEL: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAG_NONE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAG_REQUEST_INTRA_CONSTRAINED_SLICES: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAG_NONE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAG_REQUEST_INTRA_CONSTRAINED_SLICES: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAG_NONE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAG_USED_AS_REFERENCE_PICTURE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_PROFILE_H264 = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PROFILE_H264_MAIN: D3D12_VIDEO_ENCODER_PROFILE_H264 = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PROFILE_H264_HIGH: D3D12_VIDEO_ENCODER_PROFILE_H264 = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PROFILE_H264_HIGH_10: D3D12_VIDEO_ENCODER_PROFILE_H264 = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_PROFILE_HEVC = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN10: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_NONE: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_DELTA_QP: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_FRAME_ANALYSIS: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_QP_RANGE: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_INITIAL_QP: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_MAX_FRAME_SIZE: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_VBV_SIZES: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_ABSOLUTE_QP_MAP: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_CQP: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_CBR: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_VBR: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_QVBR: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_NONE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_RESOLUTION_CHANGE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_RATE_CONTROL_CHANGE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_SUBREGION_LAYOUT_CHANGE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_REQUEST_INTRA_REFRESH: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_GOP_SEQUENCE_CHANGE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_NONE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_GENERAL_SUPPORT_OK: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_RECONFIGURATION_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RESOLUTION_RECONFIGURATION_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_VBV_SIZE_CONFIG_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_FRAME_ANALYSIS_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RECONSTRUCTED_FRAMES_REQUIRE_TEXTURE_ARRAYS: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_DELTA_QP_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_SUBREGION_LAYOUT_RECONFIGURATION_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_ADJUSTABLE_QP_RANGE_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_INITIAL_QP_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_MAX_FRAME_SIZE_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_SEQUENCE_GOP_RECONFIGURATION_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_MOTION_ESTIMATION_PRECISION_MODE_LIMIT_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_TIER_HEVC = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_TIER_HEVC_MAIN: D3D12_VIDEO_ENCODER_TIER_HEVC = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_TIER_HEVC_HIGH: D3D12_VIDEO_ENCODER_TIER_HEVC = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_NONE: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_CODEC_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_INPUT_FORMAT_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_CODEC_CONFIGURATION_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_RATE_CONTROL_MODE_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_RATE_CONTROL_CONFIGURATION_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_INTRA_REFRESH_MODE_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_SUBREGION_LAYOUT_MODE_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_RESOLUTION_NOT_SUPPORTED_IN_LIST: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_GOP_STRUCTURE_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAG_NONE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAG_READ: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAG_WRITE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_CREATION: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_INITIALIZATION: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_EXECUTION: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_CAPS_INPUT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_CAPS_OUTPUT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_DEVICE_EXECUTE_INPUT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_DEVICE_EXECUTE_OUTPUT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_UINT8: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_UINT16: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_UINT32: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_UINT64: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_SINT8: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_SINT16: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_SINT32: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_SINT64: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_FLOAT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_DOUBLE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_RESOURCE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_FIELD_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FIELD_TYPE_NONE: D3D12_VIDEO_FIELD_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FIELD_TYPE_INTERLACED_TOP_FIELD_FIRST: D3D12_VIDEO_FIELD_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FIELD_TYPE_INTERLACED_BOTTOM_FIELD_FIRST: D3D12_VIDEO_FIELD_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE_NONE: D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE_FIELD_BASED: D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_FRAME_STEREO_FORMAT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_NONE: D3D12_VIDEO_FRAME_STEREO_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_MONO: D3D12_VIDEO_FRAME_STEREO_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_HORIZONTAL: D3D12_VIDEO_FRAME_STEREO_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_VERTICAL: D3D12_VIDEO_FRAME_STEREO_FORMAT = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_SEPARATE: D3D12_VIDEO_FRAME_STEREO_FORMAT = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_8X8: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_16X16: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAG_NONE: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAG_8X8: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAG_16X16: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_QUARTER_PEL: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAG_NONE: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAG_QUARTER_PEL: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE_OPAQUE: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE_BACKGROUND: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE_DESTINATION: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE_SOURCE_STREAM: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_NONE: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_DENOISE: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_DERINGING: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_EDGE_ENHANCEMENT: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_COLOR_CORRECTION: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_FLESH_TONE_MAPPING: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_IMAGE_STABILIZATION: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_SUPER_RESOLUTION: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_ANAMORPHIC_SCALING: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_CUSTOM: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_DEINTERLACE_FLAG_NONE: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_DEINTERLACE_FLAG_BOB: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_DEINTERLACE_FLAG_CUSTOM: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROCESS_FEATURE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_NONE: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_ALPHA_FILL: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_LUMA_KEY: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_STEREO: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_ROTATION: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_FLIP: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_ALPHA_BLENDING: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_PIXEL_ASPECT_RATIO: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROCESS_FILTER = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_BRIGHTNESS: D3D12_VIDEO_PROCESS_FILTER = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_CONTRAST: D3D12_VIDEO_PROCESS_FILTER = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_HUE: D3D12_VIDEO_PROCESS_FILTER = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_SATURATION: D3D12_VIDEO_PROCESS_FILTER = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_NOISE_REDUCTION: D3D12_VIDEO_PROCESS_FILTER = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_EDGE_ENHANCEMENT: D3D12_VIDEO_PROCESS_FILTER = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_ANAMORPHIC_SCALING: D3D12_VIDEO_PROCESS_FILTER = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_STEREO_ADJUSTMENT: D3D12_VIDEO_PROCESS_FILTER = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROCESS_FILTER_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_NONE: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_BRIGHTNESS: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_CONTRAST: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_HUE: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_SATURATION: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_NOISE_REDUCTION: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_EDGE_ENHANCEMENT: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_ANAMORPHIC_SCALING: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_STEREO_ADJUSTMENT: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAG_NONE: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAG_FRAME_DISCONTINUITY: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAG_FRAME_REPEAT: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROCESS_ORIENTATION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ORIENTATION_DEFAULT: D3D12_VIDEO_PROCESS_ORIENTATION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ORIENTATION_FLIP_HORIZONTAL: D3D12_VIDEO_PROCESS_ORIENTATION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_90: D3D12_VIDEO_PROCESS_ORIENTATION = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_90_FLIP_HORIZONTAL: D3D12_VIDEO_PROCESS_ORIENTATION = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_180: D3D12_VIDEO_PROCESS_ORIENTATION = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ORIENTATION_FLIP_VERTICAL: D3D12_VIDEO_PROCESS_ORIENTATION = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_270: D3D12_VIDEO_PROCESS_ORIENTATION = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_270_FLIP_HORIZONTAL: D3D12_VIDEO_PROCESS_ORIENTATION = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROCESS_SUPPORT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_SUPPORT_FLAG_NONE: D3D12_VIDEO_PROCESS_SUPPORT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROCESS_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_PROCESS_SUPPORT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAG_NONE: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type D3D12_VIDEO_SCALE_SUPPORT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_SCALE_SUPPORT_FLAG_NONE: D3D12_VIDEO_SCALE_SUPPORT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_SCALE_SUPPORT_FLAG_POW2_ONLY: D3D12_VIDEO_SCALE_SUPPORT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const D3D12_VIDEO_SCALE_SUPPORT_FLAG_EVEN_DIMENSIONS_ONLY: D3D12_VIDEO_SCALE_SUPPORT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_BufferfType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_PictureParametersBufferType: DXVA2_BufferfType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_MacroBlockControlBufferType: DXVA2_BufferfType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_ResidualDifferenceBufferType: DXVA2_BufferfType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeblockingControlBufferType: DXVA2_BufferfType = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_InverseQuantizationMatrixBufferType: DXVA2_BufferfType = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SliceControlBufferType: DXVA2_BufferfType = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_BitStreamDateBufferType: DXVA2_BufferfType = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_MotionVectorBuffer: DXVA2_BufferfType = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_FilmGrainBuffer: DXVA2_BufferfType = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_DeinterlaceTech = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_Unknown: DXVA2_DeinterlaceTech = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_BOBLineReplicate: DXVA2_DeinterlaceTech = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_BOBVerticalStretch: DXVA2_DeinterlaceTech = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_BOBVerticalStretch4Tap: DXVA2_DeinterlaceTech = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_MedianFiltering: DXVA2_DeinterlaceTech = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_EdgeFiltering: DXVA2_DeinterlaceTech = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_FieldAdaptive: DXVA2_DeinterlaceTech = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_PixelAdaptive: DXVA2_DeinterlaceTech = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_MotionVectorSteered: DXVA2_DeinterlaceTech = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_InverseTelecine: DXVA2_DeinterlaceTech = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DeinterlaceTech_Mask: DXVA2_DeinterlaceTech = 511i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_DestData = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DestData_RFF: DXVA2_DestData = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DestData_TFF: DXVA2_DestData = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DestData_RFF_TFF_Present: DXVA2_DestData = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DestData_Mask: DXVA2_DestData = 65535i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_DetailFilterTech = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterTech_Unsupported: DXVA2_DetailFilterTech = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterTech_Unknown: DXVA2_DetailFilterTech = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterTech_Edge: DXVA2_DetailFilterTech = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterTech_Sharpening: DXVA2_DetailFilterTech = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterTech_Mask: DXVA2_DetailFilterTech = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_FilterType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterLumaLevel: DXVA2_FilterType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterLumaThreshold: DXVA2_FilterType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterLumaRadius: DXVA2_FilterType = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterChromaLevel: DXVA2_FilterType = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterChromaThreshold: DXVA2_FilterType = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterChromaRadius: DXVA2_FilterType = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterLumaLevel: DXVA2_FilterType = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterLumaThreshold: DXVA2_FilterType = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterLumaRadius: DXVA2_FilterType = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterChromaLevel: DXVA2_FilterType = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterChromaThreshold: DXVA2_FilterType = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_DetailFilterChromaRadius: DXVA2_FilterType = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_NoiseFilterTech = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterTech_Unsupported: DXVA2_NoiseFilterTech = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterTech_Unknown: DXVA2_NoiseFilterTech = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterTech_Median: DXVA2_NoiseFilterTech = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterTech_Temporal: DXVA2_NoiseFilterTech = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterTech_BlockNoise: DXVA2_NoiseFilterTech = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterTech_MosquitoNoise: DXVA2_NoiseFilterTech = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NoiseFilterTech_Mask: DXVA2_NoiseFilterTech = 31i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_NominalRange = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NominalRangeMask: DXVA2_NominalRange = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NominalRange_Unknown: DXVA2_NominalRange = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NominalRange_Normal: DXVA2_NominalRange = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NominalRange_Wide: DXVA2_NominalRange = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NominalRange_0_255: DXVA2_NominalRange = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NominalRange_16_235: DXVA2_NominalRange = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_NominalRange_48_208: DXVA2_NominalRange = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_ProcAmp = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_ProcAmp_None: DXVA2_ProcAmp = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_ProcAmp_Brightness: DXVA2_ProcAmp = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_ProcAmp_Contrast: DXVA2_ProcAmp = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_ProcAmp_Hue: DXVA2_ProcAmp = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_ProcAmp_Saturation: DXVA2_ProcAmp = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_ProcAmp_Mask: DXVA2_ProcAmp = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_SampleData = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleData_RFF: DXVA2_SampleData = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleData_TFF: DXVA2_SampleData = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleData_RFF_TFF_Present: DXVA2_SampleData = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleData_Mask: DXVA2_SampleData = 65535i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_SampleFormat = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleFormatMask: DXVA2_SampleFormat = 255i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleUnknown: DXVA2_SampleFormat = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleProgressiveFrame: DXVA2_SampleFormat = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleFieldInterleavedEvenFirst: DXVA2_SampleFormat = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleFieldInterleavedOddFirst: DXVA2_SampleFormat = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleFieldSingleEven: DXVA2_SampleFormat = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleFieldSingleOdd: DXVA2_SampleFormat = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SampleSubStream: DXVA2_SampleFormat = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_SurfaceType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SurfaceType_DecoderRenderTarget: DXVA2_SurfaceType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SurfaceType_ProcessorRenderTarget: DXVA2_SurfaceType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_SurfaceType_D3DRenderTargetTexture: DXVA2_SurfaceType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_VPDev = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VPDev_HardwareDevice: DXVA2_VPDev = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VPDev_EmulatedDXVA1: DXVA2_VPDev = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VPDev_SoftwareDevice: DXVA2_VPDev = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VPDev_Mask: DXVA2_VPDev = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_VideoChromaSubSampling = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsamplingMask: DXVA2_VideoChromaSubSampling = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsampling_Unknown: DXVA2_VideoChromaSubSampling = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsampling_ProgressiveChroma: DXVA2_VideoChromaSubSampling = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsampling_Horizontally_Cosited: DXVA2_VideoChromaSubSampling = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsampling_Vertically_Cosited: DXVA2_VideoChromaSubSampling = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsampling_Vertically_AlignedChromaPlanes: DXVA2_VideoChromaSubSampling = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsampling_MPEG2: DXVA2_VideoChromaSubSampling = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsampling_MPEG1: DXVA2_VideoChromaSubSampling = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsampling_DV_PAL: DXVA2_VideoChromaSubSampling = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoChromaSubsampling_Cosited: DXVA2_VideoChromaSubSampling = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_VideoLighting = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoLightingMask: DXVA2_VideoLighting = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoLighting_Unknown: DXVA2_VideoLighting = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoLighting_bright: DXVA2_VideoLighting = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoLighting_office: DXVA2_VideoLighting = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoLighting_dim: DXVA2_VideoLighting = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoLighting_dark: DXVA2_VideoLighting = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_VideoPrimaries = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimariesMask: DXVA2_VideoPrimaries = 31i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimaries_Unknown: DXVA2_VideoPrimaries = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimaries_reserved: DXVA2_VideoPrimaries = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimaries_BT709: DXVA2_VideoPrimaries = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimaries_BT470_2_SysM: DXVA2_VideoPrimaries = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimaries_BT470_2_SysBG: DXVA2_VideoPrimaries = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimaries_SMPTE170M: DXVA2_VideoPrimaries = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimaries_SMPTE240M: DXVA2_VideoPrimaries = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimaries_EBU3213: DXVA2_VideoPrimaries = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoPrimaries_SMPTE_C: DXVA2_VideoPrimaries = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_VideoProcess = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_None: DXVA2_VideoProcess = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_YUV2RGB: DXVA2_VideoProcess = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_StretchX: DXVA2_VideoProcess = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_StretchY: DXVA2_VideoProcess = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_AlphaBlend: DXVA2_VideoProcess = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_SubRects: DXVA2_VideoProcess = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_SubStreams: DXVA2_VideoProcess = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_SubStreamsExtended: DXVA2_VideoProcess = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_YUV2RGBExtended: DXVA2_VideoProcess = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_AlphaBlendExtended: DXVA2_VideoProcess = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_Constriction: DXVA2_VideoProcess = 512i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_NoiseFilter: DXVA2_VideoProcess = 1024i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_DetailFilter: DXVA2_VideoProcess = 2048i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_PlanarAlpha: DXVA2_VideoProcess = 4096i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_LinearScaling: DXVA2_VideoProcess = 8192i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_GammaCompensated: DXVA2_VideoProcess = 16384i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_MaintainsOriginalFieldData: DXVA2_VideoProcess = 32768i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcess_Mask: DXVA2_VideoProcess = 65535i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_VideoRenderTargetType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoDecoderRenderTarget: DXVA2_VideoRenderTargetType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoProcessorRenderTarget: DXVA2_VideoRenderTargetType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoSoftwareRenderTarget: DXVA2_VideoRenderTargetType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_VideoTransferFunction = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFuncMask: DXVA2_VideoTransferFunction = 31i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFunc_Unknown: DXVA2_VideoTransferFunction = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFunc_10: DXVA2_VideoTransferFunction = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFunc_18: DXVA2_VideoTransferFunction = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFunc_20: DXVA2_VideoTransferFunction = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFunc_22: DXVA2_VideoTransferFunction = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFunc_709: DXVA2_VideoTransferFunction = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFunc_240M: DXVA2_VideoTransferFunction = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFunc_sRGB: DXVA2_VideoTransferFunction = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransFunc_28: DXVA2_VideoTransferFunction = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA2_VideoTransferMatrix = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransferMatrixMask: DXVA2_VideoTransferMatrix = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransferMatrix_Unknown: DXVA2_VideoTransferMatrix = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransferMatrix_BT709: DXVA2_VideoTransferMatrix = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransferMatrix_BT601: DXVA2_VideoTransferMatrix = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA2_VideoTransferMatrix_SMPTE240M: DXVA2_VideoTransferMatrix = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_ALPHA_FILL_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ALPHA_FILL_MODE_OPAQUE: DXVAHD_ALPHA_FILL_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ALPHA_FILL_MODE_BACKGROUND: DXVAHD_ALPHA_FILL_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ALPHA_FILL_MODE_DESTINATION: DXVAHD_ALPHA_FILL_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ALPHA_FILL_MODE_SOURCE_STREAM: DXVAHD_ALPHA_FILL_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_BLT_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_BLT_STATE_TARGET_RECT: DXVAHD_BLT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_BLT_STATE_BACKGROUND_COLOR: DXVAHD_BLT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE: DXVAHD_BLT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_BLT_STATE_ALPHA_FILL: DXVAHD_BLT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_BLT_STATE_CONSTRICTION: DXVAHD_BLT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_BLT_STATE_PRIVATE: DXVAHD_BLT_STATE = 1000i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_DEVICE_CAPS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_CAPS_LINEAR_SPACE: DXVAHD_DEVICE_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_CAPS_xvYCC: DXVAHD_DEVICE_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_CAPS_RGB_RANGE_CONVERSION: DXVAHD_DEVICE_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_CAPS_YCbCr_MATRIX_CONVERSION: DXVAHD_DEVICE_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_DEVICE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_TYPE_HARDWARE: DXVAHD_DEVICE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_TYPE_SOFTWARE: DXVAHD_DEVICE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_TYPE_REFERENCE: DXVAHD_DEVICE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_TYPE_OTHER: DXVAHD_DEVICE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_DEVICE_USAGE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_USAGE_PLAYBACK_NORMAL: DXVAHD_DEVICE_USAGE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_USAGE_OPTIMAL_SPEED: DXVAHD_DEVICE_USAGE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_DEVICE_USAGE_OPTIMAL_QUALITY: DXVAHD_DEVICE_USAGE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_FEATURE_CAPS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FEATURE_CAPS_ALPHA_FILL: DXVAHD_FEATURE_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FEATURE_CAPS_CONSTRICTION: DXVAHD_FEATURE_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FEATURE_CAPS_LUMA_KEY: DXVAHD_FEATURE_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FEATURE_CAPS_ALPHA_PALETTE: DXVAHD_FEATURE_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_FILTER = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_BRIGHTNESS: DXVAHD_FILTER = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_CONTRAST: DXVAHD_FILTER = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_HUE: DXVAHD_FILTER = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_SATURATION: DXVAHD_FILTER = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_NOISE_REDUCTION: DXVAHD_FILTER = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_EDGE_ENHANCEMENT: DXVAHD_FILTER = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_ANAMORPHIC_SCALING: DXVAHD_FILTER = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_FILTER_CAPS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_CAPS_BRIGHTNESS: DXVAHD_FILTER_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_CAPS_CONTRAST: DXVAHD_FILTER_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_CAPS_HUE: DXVAHD_FILTER_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_CAPS_SATURATION: DXVAHD_FILTER_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_CAPS_NOISE_REDUCTION: DXVAHD_FILTER_CAPS = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_CAPS_EDGE_ENHANCEMENT: DXVAHD_FILTER_CAPS = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FILTER_CAPS_ANAMORPHIC_SCALING: DXVAHD_FILTER_CAPS = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_FRAME_FORMAT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FRAME_FORMAT_PROGRESSIVE: DXVAHD_FRAME_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FRAME_FORMAT_INTERLACED_TOP_FIELD_FIRST: DXVAHD_FRAME_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_FRAME_FORMAT_INTERLACED_BOTTOM_FIELD_FIRST: DXVAHD_FRAME_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_INPUT_FORMAT_CAPS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_INPUT_FORMAT_CAPS_RGB_INTERLACED: DXVAHD_INPUT_FORMAT_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_INPUT_FORMAT_CAPS_RGB_PROCAMP: DXVAHD_INPUT_FORMAT_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_INPUT_FORMAT_CAPS_RGB_LUMA_KEY: DXVAHD_INPUT_FORMAT_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_INPUT_FORMAT_CAPS_PALETTE_INTERLACED: DXVAHD_INPUT_FORMAT_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_ITELECINE_CAPS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_32: DXVAHD_ITELECINE_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_22: DXVAHD_ITELECINE_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_2224: DXVAHD_ITELECINE_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_2332: DXVAHD_ITELECINE_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_32322: DXVAHD_ITELECINE_CAPS = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_55: DXVAHD_ITELECINE_CAPS = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_64: DXVAHD_ITELECINE_CAPS = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_87: DXVAHD_ITELECINE_CAPS = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_222222222223: DXVAHD_ITELECINE_CAPS = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_ITELECINE_CAPS_OTHER: DXVAHD_ITELECINE_CAPS = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_OUTPUT_RATE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_OUTPUT_RATE_NORMAL: DXVAHD_OUTPUT_RATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_OUTPUT_RATE_HALF: DXVAHD_OUTPUT_RATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_OUTPUT_RATE_CUSTOM: DXVAHD_OUTPUT_RATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_PROCESSOR_CAPS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_PROCESSOR_CAPS_DEINTERLACE_BLEND: DXVAHD_PROCESSOR_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_PROCESSOR_CAPS_DEINTERLACE_BOB: DXVAHD_PROCESSOR_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_PROCESSOR_CAPS_DEINTERLACE_ADAPTIVE: DXVAHD_PROCESSOR_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_PROCESSOR_CAPS_DEINTERLACE_MOTION_COMPENSATION: DXVAHD_PROCESSOR_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_PROCESSOR_CAPS_INVERSE_TELECINE: DXVAHD_PROCESSOR_CAPS = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_PROCESSOR_CAPS_FRAME_RATE_CONVERSION: DXVAHD_PROCESSOR_CAPS = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_STREAM_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_D3DFORMAT: DXVAHD_STREAM_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_FRAME_FORMAT: DXVAHD_STREAM_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE: DXVAHD_STREAM_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_OUTPUT_RATE: DXVAHD_STREAM_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_SOURCE_RECT: DXVAHD_STREAM_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_DESTINATION_RECT: DXVAHD_STREAM_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_ALPHA: DXVAHD_STREAM_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_PALETTE: DXVAHD_STREAM_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_LUMA_KEY: DXVAHD_STREAM_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_ASPECT_RATIO: DXVAHD_STREAM_STATE = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_FILTER_BRIGHTNESS: DXVAHD_STREAM_STATE = 100i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_FILTER_CONTRAST: DXVAHD_STREAM_STATE = 101i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_FILTER_HUE: DXVAHD_STREAM_STATE = 102i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_FILTER_SATURATION: DXVAHD_STREAM_STATE = 103i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_FILTER_NOISE_REDUCTION: DXVAHD_STREAM_STATE = 104i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_FILTER_EDGE_ENHANCEMENT: DXVAHD_STREAM_STATE = 105i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_FILTER_ANAMORPHIC_SCALING: DXVAHD_STREAM_STATE = 106i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_STREAM_STATE_PRIVATE: DXVAHD_STREAM_STATE = 1000i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVAHD_SURFACE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_SURFACE_TYPE_VIDEO_INPUT: DXVAHD_SURFACE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_SURFACE_TYPE_VIDEO_INPUT_PRIVATE: DXVAHD_SURFACE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVAHD_SURFACE_TYPE_VIDEO_OUTPUT: DXVAHD_SURFACE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_DeinterlaceTech = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DeinterlaceTech_Unknown: DXVA_DeinterlaceTech = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DeinterlaceTech_BOBLineReplicate: DXVA_DeinterlaceTech = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DeinterlaceTech_BOBVerticalStretch: DXVA_DeinterlaceTech = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DeinterlaceTech_BOBVerticalStretch4Tap: DXVA_DeinterlaceTech = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DeinterlaceTech_MedianFiltering: DXVA_DeinterlaceTech = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DeinterlaceTech_EdgeFiltering: DXVA_DeinterlaceTech = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DeinterlaceTech_FieldAdaptive: DXVA_DeinterlaceTech = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DeinterlaceTech_PixelAdaptive: DXVA_DeinterlaceTech = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DeinterlaceTech_MotionVectorSteered: DXVA_DeinterlaceTech = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_DestinationFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DestinationFlagMask: DXVA_DestinationFlags = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DestinationFlag_Background_Changed: DXVA_DestinationFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DestinationFlag_TargetRect_Changed: DXVA_DestinationFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DestinationFlag_ColorData_Changed: DXVA_DestinationFlags = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_DestinationFlag_Alpha_Changed: DXVA_DestinationFlags = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_NominalRange = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_NominalRangeShift: DXVA_NominalRange = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_NominalRangeMask: DXVA_NominalRange = 28672i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_NominalRange_Unknown: DXVA_NominalRange = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_NominalRange_Normal: DXVA_NominalRange = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_NominalRange_Wide: DXVA_NominalRange = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_NominalRange_0_255: DXVA_NominalRange = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_NominalRange_16_235: DXVA_NominalRange = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_NominalRange_48_208: DXVA_NominalRange = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_ProcAmpControlProp = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_ProcAmp_None: DXVA_ProcAmpControlProp = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_ProcAmp_Brightness: DXVA_ProcAmpControlProp = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_ProcAmp_Contrast: DXVA_ProcAmpControlProp = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_ProcAmp_Hue: DXVA_ProcAmpControlProp = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_ProcAmp_Saturation: DXVA_ProcAmpControlProp = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_SampleFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFlagsMask: DXVA_SampleFlags = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFlag_Palette_Changed: DXVA_SampleFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFlag_SrcRect_Changed: DXVA_SampleFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFlag_DstRect_Changed: DXVA_SampleFlags = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFlag_ColorData_Changed: DXVA_SampleFlags = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_SampleFormat = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFormatMask: DXVA_SampleFormat = 255i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleUnknown: DXVA_SampleFormat = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SamplePreviousFrame: DXVA_SampleFormat = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleProgressiveFrame: DXVA_SampleFormat = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFieldInterleavedEvenFirst: DXVA_SampleFormat = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFieldInterleavedOddFirst: DXVA_SampleFormat = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFieldSingleEven: DXVA_SampleFormat = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleFieldSingleOdd: DXVA_SampleFormat = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_SampleSubStream: DXVA_SampleFormat = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_VideoChromaSubsampling = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsamplingShift: DXVA_VideoChromaSubsampling = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsamplingMask: DXVA_VideoChromaSubsampling = 3840i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsampling_Unknown: DXVA_VideoChromaSubsampling = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsampling_ProgressiveChroma: DXVA_VideoChromaSubsampling = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsampling_Horizontally_Cosited: DXVA_VideoChromaSubsampling = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsampling_Vertically_Cosited: DXVA_VideoChromaSubsampling = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsampling_Vertically_AlignedChromaPlanes: DXVA_VideoChromaSubsampling = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsampling_MPEG2: DXVA_VideoChromaSubsampling = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsampling_MPEG1: DXVA_VideoChromaSubsampling = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsampling_DV_PAL: DXVA_VideoChromaSubsampling = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoChromaSubsampling_Cosited: DXVA_VideoChromaSubsampling = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_VideoLighting = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoLightingShift: DXVA_VideoLighting = 18i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoLightingMask: DXVA_VideoLighting = 3932160i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoLighting_Unknown: DXVA_VideoLighting = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoLighting_bright: DXVA_VideoLighting = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoLighting_office: DXVA_VideoLighting = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoLighting_dim: DXVA_VideoLighting = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoLighting_dark: DXVA_VideoLighting = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_VideoPrimaries = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimariesShift: DXVA_VideoPrimaries = 22i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimariesMask: DXVA_VideoPrimaries = 130023424i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimaries_Unknown: DXVA_VideoPrimaries = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimaries_reserved: DXVA_VideoPrimaries = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimaries_BT709: DXVA_VideoPrimaries = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimaries_BT470_2_SysM: DXVA_VideoPrimaries = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimaries_BT470_2_SysBG: DXVA_VideoPrimaries = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimaries_SMPTE170M: DXVA_VideoPrimaries = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimaries_SMPTE240M: DXVA_VideoPrimaries = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimaries_EBU3213: DXVA_VideoPrimaries = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoPrimaries_SMPTE_C: DXVA_VideoPrimaries = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_VideoProcessCaps = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_None: DXVA_VideoProcessCaps = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_YUV2RGB: DXVA_VideoProcessCaps = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_StretchX: DXVA_VideoProcessCaps = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_StretchY: DXVA_VideoProcessCaps = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_AlphaBlend: DXVA_VideoProcessCaps = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_SubRects: DXVA_VideoProcessCaps = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_SubStreams: DXVA_VideoProcessCaps = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_SubStreamsExtended: DXVA_VideoProcessCaps = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_YUV2RGBExtended: DXVA_VideoProcessCaps = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoProcess_AlphaBlendExtended: DXVA_VideoProcessCaps = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_VideoTransferFunction = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFuncShift: DXVA_VideoTransferFunction = 27i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFuncMask: DXVA_VideoTransferFunction = -134217728i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFunc_Unknown: DXVA_VideoTransferFunction = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFunc_10: DXVA_VideoTransferFunction = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFunc_18: DXVA_VideoTransferFunction = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFunc_20: DXVA_VideoTransferFunction = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFunc_22: DXVA_VideoTransferFunction = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFunc_22_709: DXVA_VideoTransferFunction = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFunc_22_240M: DXVA_VideoTransferFunction = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFunc_22_8bit_sRGB: DXVA_VideoTransferFunction = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransFunc_28: DXVA_VideoTransferFunction = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DXVA_VideoTransferMatrix = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransferMatrixShift: DXVA_VideoTransferMatrix = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransferMatrixMask: DXVA_VideoTransferMatrix = 229376i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransferMatrix_Unknown: DXVA_VideoTransferMatrix = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransferMatrix_BT709: DXVA_VideoTransferMatrix = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransferMatrix_BT601: DXVA_VideoTransferMatrix = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DXVA_VideoTransferMatrix_SMPTE240M: DXVA_VideoTransferMatrix = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type DeviceStreamState = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DeviceStreamState_Stop: DeviceStreamState = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DeviceStreamState_Pause: DeviceStreamState = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DeviceStreamState_Run: DeviceStreamState = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DeviceStreamState_Disabled: DeviceStreamState = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type EAllocationType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAllocationTypeDynamic: EAllocationType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAllocationTypeRT: EAllocationType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAllocationTypePageable: EAllocationType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAllocationTypeIgnore: EAllocationType = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type EVRFilterConfigPrefs = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const EVRFilterConfigPrefs_EnableQoS: EVRFilterConfigPrefs = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const EVRFilterConfigPrefs_Mask: EVRFilterConfigPrefs = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type FILE_ACCESSMODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ACCESSMODE_READ: FILE_ACCESSMODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ACCESSMODE_WRITE: FILE_ACCESSMODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ACCESSMODE_READWRITE: FILE_ACCESSMODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ACCESSMODE_WRITE_EXCLUSIVE: FILE_ACCESSMODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type FILE_OPENMODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPENMODE_FAIL_IF_NOT_EXIST: FILE_OPENMODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPENMODE_FAIL_IF_EXIST: FILE_OPENMODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPENMODE_RESET_IF_EXIST: FILE_OPENMODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPENMODE_APPEND_IF_EXIST: FILE_OPENMODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPENMODE_DELETE_IF_EXIST: FILE_OPENMODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type KSMETHOD_OPMVIDEOOUTPUT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const KSMETHOD_OPMVIDEOOUTPUT_STARTINITIALIZATION: KSMETHOD_OPMVIDEOOUTPUT = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const KSMETHOD_OPMVIDEOOUTPUT_FINISHINITIALIZATION: KSMETHOD_OPMVIDEOOUTPUT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const KSMETHOD_OPMVIDEOOUTPUT_GETINFORMATION: KSMETHOD_OPMVIDEOOUTPUT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_EVENT_FLAG_NONE: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_EVENT_FLAG_NO_WAIT: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF2DBuffer_LockFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF2DBuffer_LockFlags_LockTypeMask: MF2DBuffer_LockFlags = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF2DBuffer_LockFlags_Read: MF2DBuffer_LockFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF2DBuffer_LockFlags_Write: MF2DBuffer_LockFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF2DBuffer_LockFlags_ReadWrite: MF2DBuffer_LockFlags = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF2DBuffer_LockFlags_ForceDWORD: MF2DBuffer_LockFlags = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF3DVideoOutputType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF3DVideoOutputType_BaseView: MF3DVideoOutputType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF3DVideoOutputType_Stereo: MF3DVideoOutputType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFASF_INDEXER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_INDEXER_WRITE_NEW_INDEX: MFASF_INDEXER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_INDEXER_READ_FOR_REVERSEPLAYBACK: MFASF_INDEXER_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_INDEXER_WRITE_FOR_LIVEREAD: MFASF_INDEXER_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFASF_MULTIPLEXERFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_MULTIPLEXER_AUTOADJUST_BITRATE: MFASF_MULTIPLEXERFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFASF_SPLITTERFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_SPLITTER_REVERSE: MFASF_SPLITTERFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_SPLITTER_WMDRM: MFASF_SPLITTERFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFASF_STREAMSELECTOR_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_STREAMSELECTOR_DISABLE_THINNING: MFASF_STREAMSELECTOR_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFASF_STREAMSELECTOR_USE_AVERAGE_BITRATE: MFASF_STREAMSELECTOR_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFASYNC_WORKQUEUE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_STANDARD_WORKQUEUE: MFASYNC_WORKQUEUE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_WINDOW_WORKQUEUE: MFASYNC_WORKQUEUE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MULTITHREADED_WORKQUEUE: MFASYNC_WORKQUEUE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFAudioConstriction = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFaudioConstrictionOff: MFAudioConstriction = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFaudioConstriction48_16: MFAudioConstriction = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFaudioConstriction44_16: MFAudioConstriction = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFaudioConstriction14_14: MFAudioConstriction = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFaudioConstrictionMute: MFAudioConstriction = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFBYTESTREAM_SEEK_ORIGIN = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const msoBegin: MFBYTESTREAM_SEEK_ORIGIN = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const msoCurrent: MFBYTESTREAM_SEEK_ORIGIN = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFCLOCK_CHARACTERISTICS_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_CHARACTERISTICS_FLAG_FREQUENCY_10MHZ: MFCLOCK_CHARACTERISTICS_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_CHARACTERISTICS_FLAG_ALWAYS_RUNNING: MFCLOCK_CHARACTERISTICS_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_CHARACTERISTICS_FLAG_IS_SYSTEM_CLOCK: MFCLOCK_CHARACTERISTICS_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFCLOCK_RELATIONAL_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_RELATIONAL_FLAG_JITTER_NEVER_AHEAD: MFCLOCK_RELATIONAL_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFCLOCK_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_STATE_INVALID: MFCLOCK_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_STATE_RUNNING: MFCLOCK_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_STATE_STOPPED: MFCLOCK_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCLOCK_STATE_PAUSED: MFCLOCK_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFCameraIntrinsic_DistortionModelType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCameraIntrinsic_DistortionModelType_6KT: MFCameraIntrinsic_DistortionModelType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCameraIntrinsic_DistortionModelType_ArcTan: MFCameraIntrinsic_DistortionModelType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFCameraOcclusionState = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCameraOcclusionState_Open: MFCameraOcclusionState = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCameraOcclusionState_OccludedByLid: MFCameraOcclusionState = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFCameraOcclusionState_OccludedByCameraHardware: MFCameraOcclusionState = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFDepthMeasurement = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DistanceToFocalPlane: MFDepthMeasurement = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const DistanceToOpticalCenter: MFDepthMeasurement = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFFrameSourceTypes = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFFrameSourceTypes_Color: MFFrameSourceTypes = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFFrameSourceTypes_Infrared: MFFrameSourceTypes = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFFrameSourceTypes_Depth: MFFrameSourceTypes = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFFrameSourceTypes_Image: MFFrameSourceTypes = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFFrameSourceTypes_Custom: MFFrameSourceTypes = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFMEDIASOURCE_CHARACTERISTICS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFMEDIASOURCE_IS_LIVE: MFMEDIASOURCE_CHARACTERISTICS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFMEDIASOURCE_CAN_SEEK: MFMEDIASOURCE_CHARACTERISTICS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFMEDIASOURCE_CAN_PAUSE: MFMEDIASOURCE_CHARACTERISTICS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFMEDIASOURCE_HAS_SLOW_SEEK: MFMEDIASOURCE_CHARACTERISTICS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFMEDIASOURCE_HAS_MULTIPLE_PRESENTATIONS: MFMEDIASOURCE_CHARACTERISTICS = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFMEDIASOURCE_CAN_SKIPFORWARD: MFMEDIASOURCE_CHARACTERISTICS = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFMEDIASOURCE_CAN_SKIPBACKWARD: MFMEDIASOURCE_CHARACTERISTICS = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFMEDIASOURCE_DOES_NOT_USE_NETWORK: MFMEDIASOURCE_CHARACTERISTICS = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFNETSOURCE_CACHE_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_CACHE_UNAVAILABLE: MFNETSOURCE_CACHE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_CACHE_ACTIVE_WRITING: MFNETSOURCE_CACHE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_CACHE_ACTIVE_COMPLETE: MFNETSOURCE_CACHE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFNETSOURCE_PROTOCOL_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_UNDEFINED: MFNETSOURCE_PROTOCOL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_HTTP: MFNETSOURCE_PROTOCOL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_RTSP: MFNETSOURCE_PROTOCOL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_FILE: MFNETSOURCE_PROTOCOL_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_MULTICAST: MFNETSOURCE_PROTOCOL_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFNETSOURCE_STATISTICS_IDS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_RECVPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_LOSTPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_RESENDSREQUESTED_ID: MFNETSOURCE_STATISTICS_IDS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_RESENDSRECEIVED_ID: MFNETSOURCE_STATISTICS_IDS = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_RECOVEREDBYECCPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_RECOVEREDBYRTXPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_OUTPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_RECVRATE_ID: MFNETSOURCE_STATISTICS_IDS = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_AVGBANDWIDTHBPS_ID: MFNETSOURCE_STATISTICS_IDS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_BYTESRECEIVED_ID: MFNETSOURCE_STATISTICS_IDS = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_PROTOCOL_ID: MFNETSOURCE_STATISTICS_IDS = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_TRANSPORT_ID: MFNETSOURCE_STATISTICS_IDS = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_CACHE_STATE_ID: MFNETSOURCE_STATISTICS_IDS = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_LINKBANDWIDTH_ID: MFNETSOURCE_STATISTICS_IDS = 13i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_CONTENTBITRATE_ID: MFNETSOURCE_STATISTICS_IDS = 14i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_SPEEDFACTOR_ID: MFNETSOURCE_STATISTICS_IDS = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_BUFFERSIZE_ID: MFNETSOURCE_STATISTICS_IDS = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_BUFFERPROGRESS_ID: MFNETSOURCE_STATISTICS_IDS = 17i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_LASTBWSWITCHTS_ID: MFNETSOURCE_STATISTICS_IDS = 18i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_SEEKRANGESTART_ID: MFNETSOURCE_STATISTICS_IDS = 19i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_SEEKRANGEEND_ID: MFNETSOURCE_STATISTICS_IDS = 20i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_BUFFERINGCOUNT_ID: MFNETSOURCE_STATISTICS_IDS = 21i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_INCORRECTLYSIGNEDPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 22i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_SIGNEDSESSION_ID: MFNETSOURCE_STATISTICS_IDS = 23i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_MAXBITRATE_ID: MFNETSOURCE_STATISTICS_IDS = 24i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_RECEPTION_QUALITY_ID: MFNETSOURCE_STATISTICS_IDS = 25i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_RECOVEREDPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 26i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_VBR_ID: MFNETSOURCE_STATISTICS_IDS = 27i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_DOWNLOADPROGRESS_ID: MFNETSOURCE_STATISTICS_IDS = 28i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_UNPREDEFINEDPROTOCOLNAME_ID: MFNETSOURCE_STATISTICS_IDS = 29i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFNETSOURCE_TRANSPORT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_UDP: MFNETSOURCE_TRANSPORT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNETSOURCE_TCP: MFNETSOURCE_TRANSPORT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFNET_PROXYSETTINGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_PROXYSETTING_NONE: MFNET_PROXYSETTINGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_PROXYSETTING_MANUAL: MFNET_PROXYSETTINGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_PROXYSETTING_AUTO: MFNET_PROXYSETTINGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_PROXYSETTING_BROWSER: MFNET_PROXYSETTINGS = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFNetAuthenticationFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_AUTHENTICATION_PROXY: MFNetAuthenticationFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_AUTHENTICATION_CLEAR_TEXT: MFNetAuthenticationFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_AUTHENTICATION_LOGGED_ON_USER: MFNetAuthenticationFlags = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFNetCredentialOptions = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_CREDENTIAL_SAVE: MFNetCredentialOptions = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_CREDENTIAL_DONT_CACHE: MFNetCredentialOptions = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNET_CREDENTIAL_ALLOW_CLEAR_TEXT: MFNetCredentialOptions = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFNetCredentialRequirements = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const REQUIRE_PROMPT: MFNetCredentialRequirements = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const REQUIRE_SAVE_SELECTED: MFNetCredentialRequirements = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFNominalRange = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNominalRange_Unknown: MFNominalRange = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNominalRange_Normal: MFNominalRange = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNominalRange_Wide: MFNominalRange = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNominalRange_0_255: MFNominalRange = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNominalRange_16_235: MFNominalRange = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNominalRange_48_208: MFNominalRange = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNominalRange_64_127: MFNominalRange = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNominalRange_Last: MFNominalRange = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFNominalRange_ForceDWORD: MFNominalRange = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFPMPSESSION_CREATION_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFPMPSESSION_UNPROTECTED_PROCESS: MFPMPSESSION_CREATION_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFPMPSESSION_IN_PROCESS: MFPMPSESSION_CREATION_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFPOLICYMANAGER_ACTION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PEACTION_NO: MFPOLICYMANAGER_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PEACTION_PLAY: MFPOLICYMANAGER_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PEACTION_COPY: MFPOLICYMANAGER_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PEACTION_EXPORT: MFPOLICYMANAGER_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PEACTION_EXTRACT: MFPOLICYMANAGER_ACTION = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PEACTION_RESERVED1: MFPOLICYMANAGER_ACTION = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PEACTION_RESERVED2: MFPOLICYMANAGER_ACTION = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PEACTION_RESERVED3: MFPOLICYMANAGER_ACTION = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PEACTION_LAST: MFPOLICYMANAGER_ACTION = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFP_CREATION_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_OPTION_NONE: MFP_CREATION_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_OPTION_FREE_THREADED_CALLBACK: MFP_CREATION_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_OPTION_NO_MMCSS: MFP_CREATION_OPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_OPTION_NO_REMOTE_DESKTOP_OPTIMIZATION: MFP_CREATION_OPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFP_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_PLAY: MFP_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_PAUSE: MFP_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_STOP: MFP_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_POSITION_SET: MFP_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_RATE_SET: MFP_EVENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_MEDIAITEM_CREATED: MFP_EVENT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_MEDIAITEM_SET: MFP_EVENT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_FRAME_STEP: MFP_EVENT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_MEDIAITEM_CLEARED: MFP_EVENT_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_MF: MFP_EVENT_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_ERROR: MFP_EVENT_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_PLAYBACK_ENDED: MFP_EVENT_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_EVENT_TYPE_ACQUIRE_USER_CREDENTIAL: MFP_EVENT_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFP_MEDIAPLAYER_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_MEDIAPLAYER_STATE_EMPTY: MFP_MEDIAPLAYER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_MEDIAPLAYER_STATE_STOPPED: MFP_MEDIAPLAYER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_MEDIAPLAYER_STATE_PLAYING: MFP_MEDIAPLAYER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_MEDIAPLAYER_STATE_PAUSED: MFP_MEDIAPLAYER_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_MEDIAPLAYER_STATE_SHUTDOWN: MFP_MEDIAPLAYER_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFRATE_DIRECTION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFRATE_FORWARD: MFRATE_DIRECTION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFRATE_REVERSE: MFRATE_DIRECTION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSESSION_GETFULLTOPOLOGY_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSION_GETFULLTOPOLOGY_CURRENT: MFSESSION_GETFULLTOPOLOGY_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSESSION_SETTOPOLOGY_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSION_SETTOPOLOGY_IMMEDIATE: MFSESSION_SETTOPOLOGY_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSION_SETTOPOLOGY_NORESOLUTION: MFSESSION_SETTOPOLOGY_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSESSION_SETTOPOLOGY_CLEAR_CURRENT: MFSESSION_SETTOPOLOGY_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSHUTDOWN_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSHUTDOWN_INITIATED: MFSHUTDOWN_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSHUTDOWN_COMPLETED: MFSHUTDOWN_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSINK_WMDRMACTION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSINK_WMDRMACTION_UNDEFINED: MFSINK_WMDRMACTION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSINK_WMDRMACTION_ENCODE: MFSINK_WMDRMACTION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSINK_WMDRMACTION_TRANSCODE: MFSINK_WMDRMACTION = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSINK_WMDRMACTION_TRANSCRYPT: MFSINK_WMDRMACTION = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSINK_WMDRMACTION_LAST: MFSINK_WMDRMACTION = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSTREAMSINK_MARKER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSTREAMSINK_MARKER_DEFAULT: MFSTREAMSINK_MARKER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSTREAMSINK_MARKER_ENDOFSEGMENT: MFSTREAMSINK_MARKER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSTREAMSINK_MARKER_TICK: MFSTREAMSINK_MARKER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSTREAMSINK_MARKER_EVENT: MFSTREAMSINK_MARKER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSampleAllocatorUsage = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSampleAllocatorUsage_UsesProvidedAllocator: MFSampleAllocatorUsage = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSampleAllocatorUsage_UsesCustomAllocator: MFSampleAllocatorUsage = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSampleAllocatorUsage_DoesNotAllocate: MFSampleAllocatorUsage = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSampleEncryptionProtectionScheme = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_NONE: MFSampleEncryptionProtectionScheme = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_AES_CTR: MFSampleEncryptionProtectionScheme = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_AES_CBC: MFSampleEncryptionProtectionScheme = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSensorDeviceMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorDeviceMode_Controller: MFSensorDeviceMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorDeviceMode_Shared: MFSensorDeviceMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSensorDeviceType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorDeviceType_Unknown: MFSensorDeviceType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorDeviceType_Device: MFSensorDeviceType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorDeviceType_MediaSource: MFSensorDeviceType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorDeviceType_FrameProvider: MFSensorDeviceType = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorDeviceType_SensorTransform: MFSensorDeviceType = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSensorStreamType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorStreamType_Unknown: MFSensorStreamType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorStreamType_Input: MFSensorStreamType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSensorStreamType_Output: MFSensorStreamType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFSequencerTopologyFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SequencerTopologyFlags_Last: MFSequencerTopologyFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFStandardVideoFormat = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_reserved: MFStandardVideoFormat = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_NTSC: MFStandardVideoFormat = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_PAL: MFStandardVideoFormat = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_DVD_NTSC: MFStandardVideoFormat = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_DVD_PAL: MFStandardVideoFormat = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_DV_PAL: MFStandardVideoFormat = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_DV_NTSC: MFStandardVideoFormat = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_ATSC_SD480i: MFStandardVideoFormat = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_ATSC_HD1080i: MFStandardVideoFormat = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFStdVideoFormat_ATSC_HD720p: MFStandardVideoFormat = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFTIMER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFTIMER_RELATIVE: MFTIMER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFTOPOLOGY_DXVA_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFTOPOLOGY_DXVA_DEFAULT: MFTOPOLOGY_DXVA_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFTOPOLOGY_DXVA_NONE: MFTOPOLOGY_DXVA_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFTOPOLOGY_DXVA_FULL: MFTOPOLOGY_DXVA_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFTOPOLOGY_HARDWARE_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFTOPOLOGY_HWMODE_SOFTWARE_ONLY: MFTOPOLOGY_HARDWARE_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFTOPOLOGY_HWMODE_USE_HARDWARE: MFTOPOLOGY_HARDWARE_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFTOPOLOGY_HWMODE_USE_ONLY_HARDWARE: MFTOPOLOGY_HARDWARE_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFT_AUDIO_DECODER_DEGRADATION_REASON = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_AUDIO_DECODER_DEGRADATION_REASON_NONE: MFT_AUDIO_DECODER_DEGRADATION_REASON = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_AUDIO_DECODER_DEGRADATION_REASON_LICENSING_REQUIREMENT: MFT_AUDIO_DECODER_DEGRADATION_REASON = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFT_AUDIO_DECODER_DEGRADATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_NONE: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX2CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX6CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX8CHANNEL: MFT_AUDIO_DECODER_DEGRADATION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFT_DRAIN_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_DRAIN_PRODUCE_TAILS: MFT_DRAIN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_DRAIN_NO_TAILS: MFT_DRAIN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFT_ENUM_FLAG = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_SYNCMFT: MFT_ENUM_FLAG = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_ASYNCMFT: MFT_ENUM_FLAG = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_HARDWARE: MFT_ENUM_FLAG = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_FIELDOFUSE: MFT_ENUM_FLAG = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_LOCALMFT: MFT_ENUM_FLAG = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_TRANSCODE_ONLY: MFT_ENUM_FLAG = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_SORTANDFILTER: MFT_ENUM_FLAG = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_SORTANDFILTER_APPROVED_ONLY: MFT_ENUM_FLAG = 192u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_SORTANDFILTER_WEB_ONLY: MFT_ENUM_FLAG = 320u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_SORTANDFILTER_WEB_ONLY_EDGEMODE: MFT_ENUM_FLAG = 576u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_UNTRUSTED_STOREMFT: MFT_ENUM_FLAG = 1024u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_ENUM_FLAG_ALL: MFT_ENUM_FLAG = 63u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFT_MESSAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_COMMAND_FLUSH: MFT_MESSAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_COMMAND_DRAIN: MFT_MESSAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_SET_D3D_MANAGER: MFT_MESSAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_DROP_SAMPLES: MFT_MESSAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_COMMAND_TICK: MFT_MESSAGE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_NOTIFY_BEGIN_STREAMING: MFT_MESSAGE_TYPE = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_NOTIFY_END_STREAMING: MFT_MESSAGE_TYPE = 268435457i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_NOTIFY_END_OF_STREAM: MFT_MESSAGE_TYPE = 268435458i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_NOTIFY_START_OF_STREAM: MFT_MESSAGE_TYPE = 268435459i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_NOTIFY_RELEASE_RESOURCES: MFT_MESSAGE_TYPE = 268435460i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_NOTIFY_REACQUIRE_RESOURCES: MFT_MESSAGE_TYPE = 268435461i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_NOTIFY_EVENT: MFT_MESSAGE_TYPE = 268435462i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_COMMAND_SET_OUTPUT_STREAM_STATE: MFT_MESSAGE_TYPE = 268435463i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_COMMAND_FLUSH_OUTPUT_STREAM: MFT_MESSAGE_TYPE = 268435464i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_MESSAGE_COMMAND_MARKER: MFT_MESSAGE_TYPE = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVP_MESSAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVP_MESSAGE_FLUSH: MFVP_MESSAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVP_MESSAGE_INVALIDATEMEDIATYPE: MFVP_MESSAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVP_MESSAGE_PROCESSINPUTNOTIFY: MFVP_MESSAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVP_MESSAGE_BEGINSTREAMING: MFVP_MESSAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVP_MESSAGE_ENDSTREAMING: MFVP_MESSAGE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVP_MESSAGE_ENDOFSTREAM: MFVP_MESSAGE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVP_MESSAGE_STEP: MFVP_MESSAGE_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVP_MESSAGE_CANCELSTEP: MFVP_MESSAGE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideo3DFormat = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideo3DSampleFormat_BaseView: MFVideo3DFormat = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideo3DSampleFormat_MultiView: MFVideo3DFormat = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideo3DSampleFormat_Packed_LeftRight: MFVideo3DFormat = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideo3DSampleFormat_Packed_TopBottom: MFVideo3DFormat = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideo3DSampleFormat = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSampleExtension_3DVideo_MultiView: MFVideo3DSampleFormat = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFSampleExtension_3DVideo_Packed: MFVideo3DSampleFormat = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoAlphaBitmapFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoAlphaBitmap_EntireDDS: MFVideoAlphaBitmapFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoAlphaBitmap_SrcColorKey: MFVideoAlphaBitmapFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoAlphaBitmap_SrcRect: MFVideoAlphaBitmapFlags = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoAlphaBitmap_DestRect: MFVideoAlphaBitmapFlags = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoAlphaBitmap_FilterMode: MFVideoAlphaBitmapFlags = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoAlphaBitmap_Alpha: MFVideoAlphaBitmapFlags = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoAlphaBitmap_BitMask: MFVideoAlphaBitmapFlags = 63i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoAspectRatioMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoARMode_None: MFVideoAspectRatioMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoARMode_PreservePicture: MFVideoAspectRatioMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoARMode_PreservePixel: MFVideoAspectRatioMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoARMode_NonLinearStretch: MFVideoAspectRatioMode = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoARMode_Mask: MFVideoAspectRatioMode = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoChromaSubsampling = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_Unknown: MFVideoChromaSubsampling = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_ProgressiveChroma: MFVideoChromaSubsampling = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_Horizontally_Cosited: MFVideoChromaSubsampling = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_Vertically_Cosited: MFVideoChromaSubsampling = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes: MFVideoChromaSubsampling = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_MPEG2: MFVideoChromaSubsampling = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_MPEG1: MFVideoChromaSubsampling = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_DV_PAL: MFVideoChromaSubsampling = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_Cosited: MFVideoChromaSubsampling = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_Last: MFVideoChromaSubsampling = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoChromaSubsampling_ForceDWORD: MFVideoChromaSubsampling = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoDRMFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoDRMFlag_None: MFVideoDRMFlags = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoDRMFlag_AnalogProtected: MFVideoDRMFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoDRMFlag_DigitallyProtected: MFVideoDRMFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoDSPMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoDSPMode_Passthrough: MFVideoDSPMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoDSPMode_Stabilization: MFVideoDSPMode = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_PAD_TO_Mask: MFVideoFlags = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_PAD_TO_None: MFVideoFlags = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_PAD_TO_4x3: MFVideoFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_PAD_TO_16x9: MFVideoFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_SrcContentHintMask: MFVideoFlags = 28i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_SrcContentHintNone: MFVideoFlags = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_SrcContentHint16x9: MFVideoFlags = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_SrcContentHint235_1: MFVideoFlags = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_AnalogProtected: MFVideoFlags = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_DigitallyProtected: MFVideoFlags = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_ProgressiveContent: MFVideoFlags = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_FieldRepeatCountMask: MFVideoFlags = 1792i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_FieldRepeatCountShift: MFVideoFlags = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_ProgressiveSeqReset: MFVideoFlags = 2048i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_PanScanEnabled: MFVideoFlags = 131072i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_LowerFieldFirst: MFVideoFlags = 262144i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlag_BottomUpLinearRep: MFVideoFlags = 524288i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlags_DXVASurface: MFVideoFlags = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlags_RenderTargetSurface: MFVideoFlags = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoFlags_ForceQWORD: MFVideoFlags = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoInterlaceMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoInterlace_Unknown: MFVideoInterlaceMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoInterlace_Progressive: MFVideoInterlaceMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoInterlace_FieldInterleavedUpperFirst: MFVideoInterlaceMode = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoInterlace_FieldInterleavedLowerFirst: MFVideoInterlaceMode = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoInterlace_FieldSingleUpper: MFVideoInterlaceMode = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoInterlace_FieldSingleLower: MFVideoInterlaceMode = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoInterlace_MixedInterlaceOrProgressive: MFVideoInterlaceMode = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoInterlace_Last: MFVideoInterlaceMode = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoInterlace_ForceDWORD: MFVideoInterlaceMode = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoLighting = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoLighting_Unknown: MFVideoLighting = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoLighting_bright: MFVideoLighting = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoLighting_office: MFVideoLighting = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoLighting_dim: MFVideoLighting = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoLighting_dark: MFVideoLighting = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoLighting_Last: MFVideoLighting = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoLighting_ForceDWORD: MFVideoLighting = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoMixPrefs = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoMixPrefs_ForceHalfInterlace: MFVideoMixPrefs = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoMixPrefs_AllowDropToHalfInterlace: MFVideoMixPrefs = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoMixPrefs_AllowDropToBob: MFVideoMixPrefs = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoMixPrefs_ForceBob: MFVideoMixPrefs = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoMixPrefs_EnableRotation: MFVideoMixPrefs = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoMixPrefs_Mask: MFVideoMixPrefs = 31i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoPadFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPadFlag_PAD_TO_None: MFVideoPadFlags = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPadFlag_PAD_TO_4x3: MFVideoPadFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPadFlag_PAD_TO_16x9: MFVideoPadFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoPrimaries = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_Unknown: MFVideoPrimaries = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_reserved: MFVideoPrimaries = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_BT709: MFVideoPrimaries = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_BT470_2_SysM: MFVideoPrimaries = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_BT470_2_SysBG: MFVideoPrimaries = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_SMPTE170M: MFVideoPrimaries = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_SMPTE240M: MFVideoPrimaries = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_EBU3213: MFVideoPrimaries = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_SMPTE_C: MFVideoPrimaries = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_BT2020: MFVideoPrimaries = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_XYZ: MFVideoPrimaries = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_DCI_P3: MFVideoPrimaries = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_ACES: MFVideoPrimaries = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_Last: MFVideoPrimaries = 13i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoPrimaries_ForceDWORD: MFVideoPrimaries = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoRenderPrefs = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_DoNotRenderBorder: MFVideoRenderPrefs = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_DoNotClipToDevice: MFVideoRenderPrefs = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_AllowOutputThrottling: MFVideoRenderPrefs = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_ForceOutputThrottling: MFVideoRenderPrefs = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_ForceBatching: MFVideoRenderPrefs = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_AllowBatching: MFVideoRenderPrefs = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_ForceScaling: MFVideoRenderPrefs = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_AllowScaling: MFVideoRenderPrefs = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_DoNotRepaintOnStop: MFVideoRenderPrefs = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRenderPrefs_Mask: MFVideoRenderPrefs = 511i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoRotationFormat = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRotationFormat_0: MFVideoRotationFormat = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRotationFormat_90: MFVideoRotationFormat = 90i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRotationFormat_180: MFVideoRotationFormat = 180i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoRotationFormat_270: MFVideoRotationFormat = 270i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoSphericalFormat = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoSphericalFormat_Unsupported: MFVideoSphericalFormat = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoSphericalFormat_Equirectangular: MFVideoSphericalFormat = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoSphericalFormat_CubeMap: MFVideoSphericalFormat = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoSphericalFormat_3DMesh: MFVideoSphericalFormat = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoSphericalProjectionMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoSphericalProjectionMode_Spherical: MFVideoSphericalProjectionMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoSphericalProjectionMode_Flat: MFVideoSphericalProjectionMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoSrcContentHintFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoSrcContentHintFlag_None: MFVideoSrcContentHintFlags = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoSrcContentHintFlag_16x9: MFVideoSrcContentHintFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoSrcContentHintFlag_235_1: MFVideoSrcContentHintFlags = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoTransferFunction = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_Unknown: MFVideoTransferFunction = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_10: MFVideoTransferFunction = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_18: MFVideoTransferFunction = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_20: MFVideoTransferFunction = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_22: MFVideoTransferFunction = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_709: MFVideoTransferFunction = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_240M: MFVideoTransferFunction = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_sRGB: MFVideoTransferFunction = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_28: MFVideoTransferFunction = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_Log_100: MFVideoTransferFunction = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_Log_316: MFVideoTransferFunction = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_709_sym: MFVideoTransferFunction = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_2020_const: MFVideoTransferFunction = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_2020: MFVideoTransferFunction = 13i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_26: MFVideoTransferFunction = 14i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_2084: MFVideoTransferFunction = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_HLG: MFVideoTransferFunction = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_10_rel: MFVideoTransferFunction = 17i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_Last: MFVideoTransferFunction = 18i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransFunc_ForceDWORD: MFVideoTransferFunction = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVideoTransferMatrix = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransferMatrix_Unknown: MFVideoTransferMatrix = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransferMatrix_BT709: MFVideoTransferMatrix = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransferMatrix_BT601: MFVideoTransferMatrix = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransferMatrix_SMPTE240M: MFVideoTransferMatrix = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransferMatrix_BT2020_10: MFVideoTransferMatrix = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransferMatrix_BT2020_12: MFVideoTransferMatrix = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransferMatrix_Last: MFVideoTransferMatrix = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVideoTransferMatrix_ForceDWORD: MFVideoTransferMatrix = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVirtualCameraAccess = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVirtualCameraAccess_CurrentUser: MFVirtualCameraAccess = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVirtualCameraAccess_AllUsers: MFVirtualCameraAccess = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVirtualCameraLifetime = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVirtualCameraLifetime_Session: MFVirtualCameraLifetime = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVirtualCameraLifetime_System: MFVirtualCameraLifetime = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFVirtualCameraType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFVirtualCameraType_SoftwareCameraSource: MFVirtualCameraType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFWaveFormatExConvertFlags = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFWaveFormatExConvertFlag_Normal: MFWaveFormatExConvertFlags = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFWaveFormatExConvertFlag_ForceExtensible: MFWaveFormatExConvertFlags = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_ACTIVATE_CUSTOM_MIXER = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ACTIVATE_CUSTOM_MIXER_ALLOWFAIL: MF_ACTIVATE_CUSTOM_MIXER = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_ACTIVATE_CUSTOM_PRESENTER = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ACTIVATE_CUSTOM_PRESENTER_ALLOWFAIL: MF_ACTIVATE_CUSTOM_PRESENTER = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_ATTRIBUTES_MATCH_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTES_MATCH_OUR_ITEMS: MF_ATTRIBUTES_MATCH_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTES_MATCH_THEIR_ITEMS: MF_ATTRIBUTES_MATCH_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTES_MATCH_ALL_ITEMS: MF_ATTRIBUTES_MATCH_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTES_MATCH_INTERSECTION: MF_ATTRIBUTES_MATCH_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTES_MATCH_SMALLER: MF_ATTRIBUTES_MATCH_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_ATTRIBUTE_SERIALIZE_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTE_SERIALIZE_UNKNOWN_BYREF: MF_ATTRIBUTE_SERIALIZE_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_ATTRIBUTE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTE_UINT32: MF_ATTRIBUTE_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTE_UINT64: MF_ATTRIBUTE_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTE_DOUBLE: MF_ATTRIBUTE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTE_GUID: MF_ATTRIBUTE_TYPE = 72i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTE_STRING: MF_ATTRIBUTE_TYPE = 31i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTE_BLOB: MF_ATTRIBUTE_TYPE = 4113i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ATTRIBUTE_IUNKNOWN: MF_ATTRIBUTE_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_AUVRHP_ROOMMODEL = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const VRHP_SMALLROOM: MF_AUVRHP_ROOMMODEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const VRHP_MEDIUMROOM: MF_AUVRHP_ROOMMODEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const VRHP_BIGROOM: MF_AUVRHP_ROOMMODEL = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const VRHP_CUSTUMIZEDROOM: MF_AUVRHP_ROOMMODEL = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_AUDIO_PROCESSING_DEFAULT: MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_AUDIO_PROCESSING_RAW: MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_CAPTURE_ENGINE_DEVICE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_DEVICE_TYPE_AUDIO: MF_CAPTURE_ENGINE_DEVICE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_DEVICE_TYPE_VIDEO: MF_CAPTURE_ENGINE_DEVICE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_OTHER: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_COMMUNICATIONS: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_MEDIA: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_GAMECHAT: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_SPEECH: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_FARFIELDSPEECH: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_UNIFORMSPEECH: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_VOICETYPING: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_CAPTURE_ENGINE_SINK_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_SINK_TYPE_RECORD: MF_CAPTURE_ENGINE_SINK_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_SINK_TYPE_PREVIEW: MF_CAPTURE_ENGINE_SINK_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_SINK_TYPE_PHOTO: MF_CAPTURE_ENGINE_SINK_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_CAPTURE_ENGINE_SOURCE = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_VIDEO_PREVIEW: MF_CAPTURE_ENGINE_SOURCE = 4294967290u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_VIDEO_RECORD: MF_CAPTURE_ENGINE_SOURCE = 4294967289u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_PHOTO: MF_CAPTURE_ENGINE_SOURCE = 4294967288u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_AUDIO: MF_CAPTURE_ENGINE_SOURCE = 4294967287u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_METADATA: MF_CAPTURE_ENGINE_SOURCE = 4294967286u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_MEDIASOURCE: MF_CAPTURE_ENGINE_SOURCE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_CAPTURE_ENGINE_STREAM_CATEGORY = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_VIDEO_PREVIEW: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_VIDEO_CAPTURE: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_PHOTO_INDEPENDENT: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_PHOTO_DEPENDENT: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_AUDIO: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_UNSUPPORTED: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_METADATA: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_CONNECT_METHOD = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CONNECT_DIRECT: MF_CONNECT_METHOD = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CONNECT_ALLOW_CONVERTER: MF_CONNECT_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CONNECT_ALLOW_DECODER: MF_CONNECT_METHOD = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CONNECT_RESOLVE_INDEPENDENT_OUTPUTTYPES: MF_CONNECT_METHOD = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CONNECT_AS_OPTIONAL: MF_CONNECT_METHOD = 65536i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CONNECT_AS_OPTIONAL_BRANCH: MF_CONNECT_METHOD = 131072i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_CROSS_ORIGIN_POLICY = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CROSS_ORIGIN_POLICY_NONE: MF_CROSS_ORIGIN_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CROSS_ORIGIN_POLICY_ANONYMOUS: MF_CROSS_ORIGIN_POLICY = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_CROSS_ORIGIN_POLICY_USE_CREDENTIALS: MF_CROSS_ORIGIN_POLICY = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_CUSTOM_DECODE_UNIT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_DECODE_UNIT_NAL: MF_CUSTOM_DECODE_UNIT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_DECODE_UNIT_SEI: MF_CUSTOM_DECODE_UNIT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEUnknown: MF_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEError: MF_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEExtendedType: MF_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MENonFatalError: MF_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEGenericV1Anchor: MF_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionUnknown: MF_EVENT_TYPE = 100i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionTopologySet: MF_EVENT_TYPE = 101i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionTopologiesCleared: MF_EVENT_TYPE = 102i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionStarted: MF_EVENT_TYPE = 103i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionPaused: MF_EVENT_TYPE = 104i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionStopped: MF_EVENT_TYPE = 105i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionClosed: MF_EVENT_TYPE = 106i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionEnded: MF_EVENT_TYPE = 107i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionRateChanged: MF_EVENT_TYPE = 108i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionScrubSampleComplete: MF_EVENT_TYPE = 109i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionCapabilitiesChanged: MF_EVENT_TYPE = 110i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionTopologyStatus: MF_EVENT_TYPE = 111i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionNotifyPresentationTime: MF_EVENT_TYPE = 112i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MENewPresentation: MF_EVENT_TYPE = 113i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MELicenseAcquisitionStart: MF_EVENT_TYPE = 114i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MELicenseAcquisitionCompleted: MF_EVENT_TYPE = 115i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEIndividualizationStart: MF_EVENT_TYPE = 116i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEIndividualizationCompleted: MF_EVENT_TYPE = 117i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEEnablerProgress: MF_EVENT_TYPE = 118i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEEnablerCompleted: MF_EVENT_TYPE = 119i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEPolicyError: MF_EVENT_TYPE = 120i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEPolicyReport: MF_EVENT_TYPE = 121i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEBufferingStarted: MF_EVENT_TYPE = 122i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEBufferingStopped: MF_EVENT_TYPE = 123i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEConnectStart: MF_EVENT_TYPE = 124i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEConnectEnd: MF_EVENT_TYPE = 125i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEReconnectStart: MF_EVENT_TYPE = 126i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEReconnectEnd: MF_EVENT_TYPE = 127i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MERendererEvent: MF_EVENT_TYPE = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionStreamSinkFormatChanged: MF_EVENT_TYPE = 129i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESessionV1Anchor: MF_EVENT_TYPE = 129i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourceUnknown: MF_EVENT_TYPE = 200i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourceStarted: MF_EVENT_TYPE = 201i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamStarted: MF_EVENT_TYPE = 202i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourceSeeked: MF_EVENT_TYPE = 203i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSeeked: MF_EVENT_TYPE = 204i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MENewStream: MF_EVENT_TYPE = 205i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEUpdatedStream: MF_EVENT_TYPE = 206i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourceStopped: MF_EVENT_TYPE = 207i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamStopped: MF_EVENT_TYPE = 208i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourcePaused: MF_EVENT_TYPE = 209i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamPaused: MF_EVENT_TYPE = 210i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEEndOfPresentation: MF_EVENT_TYPE = 211i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEEndOfStream: MF_EVENT_TYPE = 212i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEMediaSample: MF_EVENT_TYPE = 213i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamTick: MF_EVENT_TYPE = 214i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamThinMode: MF_EVENT_TYPE = 215i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamFormatChanged: MF_EVENT_TYPE = 216i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourceRateChanged: MF_EVENT_TYPE = 217i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEEndOfPresentationSegment: MF_EVENT_TYPE = 218i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourceCharacteristicsChanged: MF_EVENT_TYPE = 219i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourceRateChangeRequested: MF_EVENT_TYPE = 220i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourceMetadataChanged: MF_EVENT_TYPE = 221i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESequencerSourceTopologyUpdated: MF_EVENT_TYPE = 222i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESourceV1Anchor: MF_EVENT_TYPE = 222i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESinkUnknown: MF_EVENT_TYPE = 300i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkStarted: MF_EVENT_TYPE = 301i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkStopped: MF_EVENT_TYPE = 302i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkPaused: MF_EVENT_TYPE = 303i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkRateChanged: MF_EVENT_TYPE = 304i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkRequestSample: MF_EVENT_TYPE = 305i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkMarker: MF_EVENT_TYPE = 306i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkPrerolled: MF_EVENT_TYPE = 307i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkScrubSampleComplete: MF_EVENT_TYPE = 308i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkFormatChanged: MF_EVENT_TYPE = 309i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkDeviceChanged: MF_EVENT_TYPE = 310i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEQualityNotify: MF_EVENT_TYPE = 311i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESinkInvalidated: MF_EVENT_TYPE = 312i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEAudioSessionNameChanged: MF_EVENT_TYPE = 313i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEAudioSessionVolumeChanged: MF_EVENT_TYPE = 314i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEAudioSessionDeviceRemoved: MF_EVENT_TYPE = 315i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEAudioSessionServerShutdown: MF_EVENT_TYPE = 316i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEAudioSessionGroupingParamChanged: MF_EVENT_TYPE = 317i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEAudioSessionIconChanged: MF_EVENT_TYPE = 318i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEAudioSessionFormatChanged: MF_EVENT_TYPE = 319i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEAudioSessionDisconnected: MF_EVENT_TYPE = 320i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEAudioSessionExclusiveModeOverride: MF_EVENT_TYPE = 321i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESinkV1Anchor: MF_EVENT_TYPE = 321i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MECaptureAudioSessionVolumeChanged: MF_EVENT_TYPE = 322i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MECaptureAudioSessionDeviceRemoved: MF_EVENT_TYPE = 323i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MECaptureAudioSessionFormatChanged: MF_EVENT_TYPE = 324i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MECaptureAudioSessionDisconnected: MF_EVENT_TYPE = 325i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MECaptureAudioSessionExclusiveModeOverride: MF_EVENT_TYPE = 326i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MECaptureAudioSessionServerShutdown: MF_EVENT_TYPE = 327i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MESinkV2Anchor: MF_EVENT_TYPE = 327i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const METrustUnknown: MF_EVENT_TYPE = 400i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEPolicyChanged: MF_EVENT_TYPE = 401i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEContentProtectionMessage: MF_EVENT_TYPE = 402i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEPolicySet: MF_EVENT_TYPE = 403i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const METrustV1Anchor: MF_EVENT_TYPE = 403i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMLicenseBackupCompleted: MF_EVENT_TYPE = 500i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMLicenseBackupProgress: MF_EVENT_TYPE = 501i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMLicenseRestoreCompleted: MF_EVENT_TYPE = 502i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMLicenseRestoreProgress: MF_EVENT_TYPE = 503i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMLicenseAcquisitionCompleted: MF_EVENT_TYPE = 506i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMIndividualizationCompleted: MF_EVENT_TYPE = 508i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMIndividualizationProgress: MF_EVENT_TYPE = 513i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMProximityCompleted: MF_EVENT_TYPE = 514i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMLicenseStoreCleaned: MF_EVENT_TYPE = 515i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMRevocationDownloadCompleted: MF_EVENT_TYPE = 516i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEWMDRMV1Anchor: MF_EVENT_TYPE = 516i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const METransformUnknown: MF_EVENT_TYPE = 600i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const METransformNeedInput: MF_EVENT_TYPE = 601i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const METransformHaveOutput: MF_EVENT_TYPE = 602i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const METransformDrainComplete: MF_EVENT_TYPE = 603i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const METransformMarker: MF_EVENT_TYPE = 604i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const METransformInputStreamStateChanged: MF_EVENT_TYPE = 605i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEByteStreamCharacteristicsChanged: MF_EVENT_TYPE = 700i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEVideoCaptureDeviceRemoved: MF_EVENT_TYPE = 800i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEVideoCaptureDevicePreempted: MF_EVENT_TYPE = 801i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEStreamSinkFormatInvalidated: MF_EVENT_TYPE = 802i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEEncodingParameters: MF_EVENT_TYPE = 803i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEContentProtectionMetadata: MF_EVENT_TYPE = 900i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEDeviceThermalStateChanged: MF_EVENT_TYPE = 950i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MEReservedMax: MF_EVENT_TYPE = 10000i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_FILE_ACCESSMODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ACCESSMODE_READ: MF_FILE_ACCESSMODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ACCESSMODE_WRITE: MF_FILE_ACCESSMODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_ACCESSMODE_READWRITE: MF_FILE_ACCESSMODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_FILE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_FILEFLAGS_NONE: MF_FILE_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_FILEFLAGS_NOBUFFERING: MF_FILE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_FILEFLAGS_ALLOW_WRITE_SHARING: MF_FILE_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_FILE_OPENMODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPENMODE_FAIL_IF_NOT_EXIST: MF_FILE_OPENMODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPENMODE_FAIL_IF_EXIST: MF_FILE_OPENMODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPENMODE_RESET_IF_EXIST: MF_FILE_OPENMODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPENMODE_APPEND_IF_EXIST: MF_FILE_OPENMODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPENMODE_DELETE_IF_EXIST: MF_FILE_OPENMODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_HDCP_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_HDCP_STATUS_ON: MF_HDCP_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_HDCP_STATUS_OFF: MF_HDCP_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_HDCP_STATUS_ON_WITH_TYPE_ENFORCEMENT: MF_HDCP_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIAKEYSESSION_MESSAGETYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_REQUEST: MF_MEDIAKEYSESSION_MESSAGETYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_RENEWAL: MF_MEDIAKEYSESSION_MESSAGETYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_RELEASE: MF_MEDIAKEYSESSION_MESSAGETYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYSESSION_MESSAGETYPE_INDIVIDUALIZATION_REQUEST: MF_MEDIAKEYSESSION_MESSAGETYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIAKEYSESSION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYSESSION_TYPE_TEMPORARY: MF_MEDIAKEYSESSION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYSESSION_TYPE_PERSISTENT_LICENSE: MF_MEDIAKEYSESSION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYSESSION_TYPE_PERSISTENT_RELEASE_MESSAGE: MF_MEDIAKEYSESSION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYSESSION_TYPE_PERSISTENT_USAGE_RECORD: MF_MEDIAKEYSESSION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIAKEYS_REQUIREMENT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYS_REQUIREMENT_REQUIRED: MF_MEDIAKEYS_REQUIREMENT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYS_REQUIREMENT_OPTIONAL: MF_MEDIAKEYS_REQUIREMENT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEYS_REQUIREMENT_NOT_ALLOWED: MF_MEDIAKEYS_REQUIREMENT = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIAKEY_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEY_STATUS_USABLE: MF_MEDIAKEY_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEY_STATUS_EXPIRED: MF_MEDIAKEY_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEY_STATUS_OUTPUT_DOWNSCALED: MF_MEDIAKEY_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEY_STATUS_OUTPUT_NOT_ALLOWED: MF_MEDIAKEY_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEY_STATUS_STATUS_PENDING: MF_MEDIAKEY_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEY_STATUS_INTERNAL_ERROR: MF_MEDIAKEY_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEY_STATUS_RELEASED: MF_MEDIAKEY_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAKEY_STATUS_OUTPUT_RESTRICTED: MF_MEDIAKEY_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_CANPLAY = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_CANPLAY_NOT_SUPPORTED: MF_MEDIA_ENGINE_CANPLAY = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_CANPLAY_MAYBE: MF_MEDIA_ENGINE_CANPLAY = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_CANPLAY_PROBABLY: MF_MEDIA_ENGINE_CANPLAY = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_CREATEFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_AUDIOONLY: MF_MEDIA_ENGINE_CREATEFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_WAITFORSTABLE_STATE: MF_MEDIA_ENGINE_CREATEFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_FORCEMUTE: MF_MEDIA_ENGINE_CREATEFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_REAL_TIME_MODE: MF_MEDIA_ENGINE_CREATEFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_DISABLE_LOCAL_PLUGINS: MF_MEDIA_ENGINE_CREATEFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_CREATEFLAGS_MASK: MF_MEDIA_ENGINE_CREATEFLAGS = 31i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_ERR = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_ERR_NOERROR: MF_MEDIA_ENGINE_ERR = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_ERR_ABORTED: MF_MEDIA_ENGINE_ERR = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_ERR_NETWORK: MF_MEDIA_ENGINE_ERR = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_ERR_DECODE: MF_MEDIA_ENGINE_ERR = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_ERR_SRC_NOT_SUPPORTED: MF_MEDIA_ENGINE_ERR = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_ERR_ENCRYPTED: MF_MEDIA_ENGINE_ERR = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_EVENT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_LOADSTART: MF_MEDIA_ENGINE_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_PROGRESS: MF_MEDIA_ENGINE_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_SUSPEND: MF_MEDIA_ENGINE_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_ABORT: MF_MEDIA_ENGINE_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_ERROR: MF_MEDIA_ENGINE_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_EMPTIED: MF_MEDIA_ENGINE_EVENT = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_STALLED: MF_MEDIA_ENGINE_EVENT = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_PLAY: MF_MEDIA_ENGINE_EVENT = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_PAUSE: MF_MEDIA_ENGINE_EVENT = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_LOADEDMETADATA: MF_MEDIA_ENGINE_EVENT = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_LOADEDDATA: MF_MEDIA_ENGINE_EVENT = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_WAITING: MF_MEDIA_ENGINE_EVENT = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_PLAYING: MF_MEDIA_ENGINE_EVENT = 13i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_CANPLAY: MF_MEDIA_ENGINE_EVENT = 14i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_CANPLAYTHROUGH: MF_MEDIA_ENGINE_EVENT = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_SEEKING: MF_MEDIA_ENGINE_EVENT = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_SEEKED: MF_MEDIA_ENGINE_EVENT = 17i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_TIMEUPDATE: MF_MEDIA_ENGINE_EVENT = 18i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_ENDED: MF_MEDIA_ENGINE_EVENT = 19i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_RATECHANGE: MF_MEDIA_ENGINE_EVENT = 20i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_DURATIONCHANGE: MF_MEDIA_ENGINE_EVENT = 21i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_VOLUMECHANGE: MF_MEDIA_ENGINE_EVENT = 22i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_FORMATCHANGE: MF_MEDIA_ENGINE_EVENT = 1000i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_PURGEQUEUEDEVENTS: MF_MEDIA_ENGINE_EVENT = 1001i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_TIMELINE_MARKER: MF_MEDIA_ENGINE_EVENT = 1002i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_BALANCECHANGE: MF_MEDIA_ENGINE_EVENT = 1003i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_DOWNLOADCOMPLETE: MF_MEDIA_ENGINE_EVENT = 1004i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_BUFFERINGSTARTED: MF_MEDIA_ENGINE_EVENT = 1005i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_BUFFERINGENDED: MF_MEDIA_ENGINE_EVENT = 1006i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_FRAMESTEPCOMPLETED: MF_MEDIA_ENGINE_EVENT = 1007i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_NOTIFYSTABLESTATE: MF_MEDIA_ENGINE_EVENT = 1008i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_FIRSTFRAMEREADY: MF_MEDIA_ENGINE_EVENT = 1009i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_TRACKSCHANGE: MF_MEDIA_ENGINE_EVENT = 1010i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_OPMINFO: MF_MEDIA_ENGINE_EVENT = 1011i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_RESOURCELOST: MF_MEDIA_ENGINE_EVENT = 1012i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_DELAYLOADEVENT_CHANGED: MF_MEDIA_ENGINE_EVENT = 1013i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_STREAMRENDERINGERROR: MF_MEDIA_ENGINE_EVENT = 1014i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_SUPPORTEDRATES_CHANGED: MF_MEDIA_ENGINE_EVENT = 1015i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EVENT_AUDIOENDPOINTCHANGE: MF_MEDIA_ENGINE_EVENT = 1016i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_EXTENSION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EXTENSION_TYPE_MEDIASOURCE: MF_MEDIA_ENGINE_EXTENSION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_EXTENSION_TYPE_BYTESTREAM: MF_MEDIA_ENGINE_EXTENSION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAG_PROTECTED: MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAG_REQUIRES_SURFACE_PROTECTION: MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAG_REQUIRES_ANTI_SCREEN_SCRAPE_PROTECTION: MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_KEYERR = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAENGINE_KEYERR_UNKNOWN: MF_MEDIA_ENGINE_KEYERR = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAENGINE_KEYERR_CLIENT: MF_MEDIA_ENGINE_KEYERR = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAENGINE_KEYERR_SERVICE: MF_MEDIA_ENGINE_KEYERR = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAENGINE_KEYERR_OUTPUT: MF_MEDIA_ENGINE_KEYERR = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAENGINE_KEYERR_HARDWARECHANGE: MF_MEDIA_ENGINE_KEYERR = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIAENGINE_KEYERR_DOMAIN: MF_MEDIA_ENGINE_KEYERR = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_NETWORK = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_NETWORK_EMPTY: MF_MEDIA_ENGINE_NETWORK = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_NETWORK_IDLE: MF_MEDIA_ENGINE_NETWORK = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_NETWORK_LOADING: MF_MEDIA_ENGINE_NETWORK = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_NETWORK_NO_SOURCE: MF_MEDIA_ENGINE_NETWORK = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_OPM_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_OPM_NOT_REQUESTED: MF_MEDIA_ENGINE_OPM_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_OPM_ESTABLISHED: MF_MEDIA_ENGINE_OPM_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_OPM_FAILED_VM: MF_MEDIA_ENGINE_OPM_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_OPM_FAILED_BDA: MF_MEDIA_ENGINE_OPM_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_OPM_FAILED_UNSIGNED_DRIVER: MF_MEDIA_ENGINE_OPM_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_OPM_FAILED: MF_MEDIA_ENGINE_OPM_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_PRELOAD = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_PRELOAD_MISSING: MF_MEDIA_ENGINE_PRELOAD = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_PRELOAD_EMPTY: MF_MEDIA_ENGINE_PRELOAD = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_PRELOAD_NONE: MF_MEDIA_ENGINE_PRELOAD = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_PRELOAD_METADATA: MF_MEDIA_ENGINE_PRELOAD = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_PRELOAD_AUTOMATIC: MF_MEDIA_ENGINE_PRELOAD = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_PROTECTION_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_ENABLE_PROTECTED_CONTENT: MF_MEDIA_ENGINE_PROTECTION_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_USE_PMP_FOR_ALL_CONTENT: MF_MEDIA_ENGINE_PROTECTION_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_USE_UNPROTECTED_PMP: MF_MEDIA_ENGINE_PROTECTION_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_READY = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_READY_HAVE_NOTHING: MF_MEDIA_ENGINE_READY = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_READY_HAVE_METADATA: MF_MEDIA_ENGINE_READY = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_READY_HAVE_CURRENT_DATA: MF_MEDIA_ENGINE_READY = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_READY_HAVE_FUTURE_DATA: MF_MEDIA_ENGINE_READY = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_READY_HAVE_ENOUGH_DATA: MF_MEDIA_ENGINE_READY = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_S3D_PACKING_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_S3D_PACKING_MODE_NONE: MF_MEDIA_ENGINE_S3D_PACKING_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_S3D_PACKING_MODE_SIDE_BY_SIDE: MF_MEDIA_ENGINE_S3D_PACKING_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_S3D_PACKING_MODE_TOP_BOTTOM: MF_MEDIA_ENGINE_S3D_PACKING_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_SEEK_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_SEEK_MODE_NORMAL: MF_MEDIA_ENGINE_SEEK_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_SEEK_MODE_APPROXIMATE: MF_MEDIA_ENGINE_SEEK_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_STATISTIC = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STATISTIC_FRAMES_RENDERED: MF_MEDIA_ENGINE_STATISTIC = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STATISTIC_FRAMES_DROPPED: MF_MEDIA_ENGINE_STATISTIC = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STATISTIC_BYTES_DOWNLOADED: MF_MEDIA_ENGINE_STATISTIC = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STATISTIC_BUFFER_PROGRESS: MF_MEDIA_ENGINE_STATISTIC = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STATISTIC_FRAMES_PER_SECOND: MF_MEDIA_ENGINE_STATISTIC = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STATISTIC_PLAYBACK_JITTER: MF_MEDIA_ENGINE_STATISTIC = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STATISTIC_FRAMES_CORRUPTED: MF_MEDIA_ENGINE_STATISTIC = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STATISTIC_TOTAL_FRAME_DELAY: MF_MEDIA_ENGINE_STATISTIC = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_ENGINE_STREAMTYPE_FAILED = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STREAMTYPE_FAILED_UNKNOWN: MF_MEDIA_ENGINE_STREAMTYPE_FAILED = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STREAMTYPE_FAILED_AUDIO: MF_MEDIA_ENGINE_STREAMTYPE_FAILED = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_ENGINE_STREAMTYPE_FAILED_VIDEO: MF_MEDIA_ENGINE_STREAMTYPE_FAILED = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MEDIA_SHARING_ENGINE_EVENT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MEDIA_SHARING_ENGINE_EVENT_DISCONNECT: MF_MEDIA_SHARING_ENGINE_EVENT = 2000i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MSE_APPEND_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_APPEND_MODE_SEGMENTS: MF_MSE_APPEND_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_APPEND_MODE_SEQUENCE: MF_MSE_APPEND_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MSE_ERROR = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_ERROR_NOERROR: MF_MSE_ERROR = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_ERROR_NETWORK: MF_MSE_ERROR = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_ERROR_DECODE: MF_MSE_ERROR = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_ERROR_UNKNOWN_ERROR: MF_MSE_ERROR = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MSE_OPUS_SUPPORT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_OPUS_SUPPORT_ON: MF_MSE_OPUS_SUPPORT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_OPUS_SUPPORT_OFF: MF_MSE_OPUS_SUPPORT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MSE_READY = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_READY_CLOSED: MF_MSE_READY = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_READY_OPEN: MF_MSE_READY = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_READY_ENDED: MF_MSE_READY = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MSE_VP9_SUPPORT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_VP9_SUPPORT_DEFAULT: MF_MSE_VP9_SUPPORT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_VP9_SUPPORT_ON: MF_MSE_VP9_SUPPORT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_MSE_VP9_SUPPORT_OFF: MF_MSE_VP9_SUPPORT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_MT_D3D_RESOURCE_VERSION_ENUM = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_D3D11_RESOURCE: MF_MT_D3D_RESOURCE_VERSION_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_D3D12_RESOURCE: MF_MT_D3D_RESOURCE_VERSION_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OBJECT_MEDIASOURCE: MF_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OBJECT_BYTESTREAM: MF_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OBJECT_INVALID: MF_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_OPM_ACP_PROTECTION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_ACP_OFF: MF_OPM_ACP_PROTECTION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_ACP_LEVEL_ONE: MF_OPM_ACP_PROTECTION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_ACP_LEVEL_TWO: MF_OPM_ACP_PROTECTION_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_ACP_LEVEL_THREE: MF_OPM_ACP_PROTECTION_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_ACP_FORCE_ULONG: MF_OPM_ACP_PROTECTION_LEVEL = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_OPM_CGMSA_PROTECTION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_CGMSA_OFF: MF_OPM_CGMSA_PROTECTION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_CGMSA_COPY_FREELY: MF_OPM_CGMSA_PROTECTION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_CGMSA_COPY_NO_MORE: MF_OPM_CGMSA_PROTECTION_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_CGMSA_COPY_ONE_GENERATION: MF_OPM_CGMSA_PROTECTION_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_CGMSA_COPY_NEVER: MF_OPM_CGMSA_PROTECTION_LEVEL = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPM_CGMSA_REDISTRIBUTION_CONTROL_REQUIRED: MF_OPM_CGMSA_PROTECTION_LEVEL = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_PLUGIN_CONTROL_POLICY = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_PLUGIN_CONTROL_POLICY_USE_ALL_PLUGINS: MF_PLUGIN_CONTROL_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_PLUGIN_CONTROL_POLICY_USE_APPROVED_PLUGINS: MF_PLUGIN_CONTROL_POLICY = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS: MF_PLUGIN_CONTROL_POLICY = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS_EDGEMODE: MF_PLUGIN_CONTROL_POLICY = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_Plugin_Type = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_Plugin_Type_MFT: MF_Plugin_Type = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_Plugin_Type_MediaSource: MF_Plugin_Type = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_Plugin_Type_MFT_MatchOutputType: MF_Plugin_Type = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_Plugin_Type_Other: MF_Plugin_Type = -1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_QUALITY_ADVISE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_QUALITY_CANNOT_KEEP_UP: MF_QUALITY_ADVISE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_QUALITY_DROP_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_DROP_MODE_NONE: MF_QUALITY_DROP_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_DROP_MODE_1: MF_QUALITY_DROP_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_DROP_MODE_2: MF_QUALITY_DROP_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_DROP_MODE_3: MF_QUALITY_DROP_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_DROP_MODE_4: MF_QUALITY_DROP_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_DROP_MODE_5: MF_QUALITY_DROP_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_NUM_DROP_MODES: MF_QUALITY_DROP_MODE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_QUALITY_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_QUALITY_NORMAL: MF_QUALITY_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_QUALITY_NORMAL_MINUS_1: MF_QUALITY_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_QUALITY_NORMAL_MINUS_2: MF_QUALITY_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_QUALITY_NORMAL_MINUS_3: MF_QUALITY_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_QUALITY_NORMAL_MINUS_4: MF_QUALITY_LEVEL = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_QUALITY_NORMAL_MINUS_5: MF_QUALITY_LEVEL = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_NUM_QUALITY_LEVELS: MF_QUALITY_LEVEL = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_RESOLUTION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_MEDIASOURCE: MF_RESOLUTION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_BYTESTREAM: MF_RESOLUTION_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_CONTENT_DOES_NOT_HAVE_TO_MATCH_EXTENSION_OR_MIME_TYPE: MF_RESOLUTION_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_KEEP_BYTE_STREAM_ALIVE_ON_FAIL: MF_RESOLUTION_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_DISABLE_LOCAL_PLUGINS: MF_RESOLUTION_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_PLUGIN_CONTROL_POLICY_APPROVED_ONLY: MF_RESOLUTION_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY: MF_RESOLUTION_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY_EDGEMODE: MF_RESOLUTION_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_ENABLE_STORE_PLUGINS: MF_RESOLUTION_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_READ: MF_RESOLUTION_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_RESOLUTION_WRITE: MF_RESOLUTION_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_SERVICE_LOOKUP_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SERVICE_LOOKUP_UPSTREAM: MF_SERVICE_LOOKUP_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SERVICE_LOOKUP_UPSTREAM_DIRECT: MF_SERVICE_LOOKUP_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SERVICE_LOOKUP_DOWNSTREAM: MF_SERVICE_LOOKUP_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SERVICE_LOOKUP_DOWNSTREAM_DIRECT: MF_SERVICE_LOOKUP_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SERVICE_LOOKUP_ALL: MF_SERVICE_LOOKUP_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SERVICE_LOOKUP_GLOBAL: MF_SERVICE_LOOKUP_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_SHARING_ENGINE_EVENT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SHARING_ENGINE_EVENT_DISCONNECT: MF_SHARING_ENGINE_EVENT = 2000i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SHARING_ENGINE_EVENT_LOCALRENDERINGSTARTED: MF_SHARING_ENGINE_EVENT = 2001i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SHARING_ENGINE_EVENT_LOCALRENDERINGENDED: MF_SHARING_ENGINE_EVENT = 2002i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SHARING_ENGINE_EVENT_STOPPED: MF_SHARING_ENGINE_EVENT = 2003i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SHARING_ENGINE_EVENT_ERROR: MF_SHARING_ENGINE_EVENT = 2501i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_SINK_WRITER_CONSTANTS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SINK_WRITER_INVALID_STREAM_INDEX: MF_SINK_WRITER_CONSTANTS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SINK_WRITER_ALL_STREAMS: MF_SINK_WRITER_CONSTANTS = 4294967294u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SINK_WRITER_MEDIASINK: MF_SINK_WRITER_CONSTANTS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_SOURCE_READER_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READER_INVALID_STREAM_INDEX: MF_SOURCE_READER_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READER_ALL_STREAMS: MF_SOURCE_READER_CONSTANTS = -2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READER_ANY_STREAM: MF_SOURCE_READER_CONSTANTS = -2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READER_FIRST_AUDIO_STREAM: MF_SOURCE_READER_CONSTANTS = -3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READER_FIRST_VIDEO_STREAM: MF_SOURCE_READER_CONSTANTS = -4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READER_MEDIASOURCE: MF_SOURCE_READER_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_SOURCE_READER_CONTROL_FLAG = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READER_CONTROLF_DRAIN: MF_SOURCE_READER_CONTROL_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_SOURCE_READER_CURRENT_TYPE_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READER_CURRENT_TYPE_INDEX: MF_SOURCE_READER_CURRENT_TYPE_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_SOURCE_READER_FLAG = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READERF_ERROR: MF_SOURCE_READER_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READERF_ENDOFSTREAM: MF_SOURCE_READER_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READERF_NEWSTREAM: MF_SOURCE_READER_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED: MF_SOURCE_READER_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED: MF_SOURCE_READER_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READERF_STREAMTICK: MF_SOURCE_READER_FLAG = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_SOURCE_READERF_ALLEFFECTSREMOVED: MF_SOURCE_READER_FLAG = 512i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_STREAM_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_STREAM_STATE_STOPPED: MF_STREAM_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_STREAM_STATE_PAUSED: MF_STREAM_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_STREAM_STATE_RUNNING: MF_STREAM_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_ALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_ALIGNMENT_START: MF_TIMED_TEXT_ALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_ALIGNMENT_END: MF_TIMED_TEXT_ALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_ALIGNMENT_CENTER: MF_TIMED_TEXT_ALIGNMENT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_BOUTEN_POSITION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_POSITION_BEFORE: MF_TIMED_TEXT_BOUTEN_POSITION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_POSITION_AFTER: MF_TIMED_TEXT_BOUTEN_POSITION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_POSITION_OUTSIDE: MF_TIMED_TEXT_BOUTEN_POSITION = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_BOUTEN_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_TYPE_NONE: MF_TIMED_TEXT_BOUTEN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_TYPE_AUTO: MF_TIMED_TEXT_BOUTEN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_TYPE_FILLEDCIRCLE: MF_TIMED_TEXT_BOUTEN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_TYPE_OPENCIRCLE: MF_TIMED_TEXT_BOUTEN_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_TYPE_FILLEDDOT: MF_TIMED_TEXT_BOUTEN_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_TYPE_OPENDOT: MF_TIMED_TEXT_BOUTEN_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_TYPE_FILLEDSESAME: MF_TIMED_TEXT_BOUTEN_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_BOUTEN_TYPE_OPENSESAME: MF_TIMED_TEXT_BOUTEN_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_CUE_EVENT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_CUE_EVENT_ACTIVE: MF_TIMED_TEXT_CUE_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_CUE_EVENT_INACTIVE: MF_TIMED_TEXT_CUE_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_CUE_EVENT_CLEAR: MF_TIMED_TEXT_CUE_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_DECORATION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_DECORATION_NONE: MF_TIMED_TEXT_DECORATION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_DECORATION_UNDERLINE: MF_TIMED_TEXT_DECORATION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_DECORATION_LINE_THROUGH: MF_TIMED_TEXT_DECORATION = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_DECORATION_OVERLINE: MF_TIMED_TEXT_DECORATION = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_DISPLAY_ALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_DISPLAY_ALIGNMENT_BEFORE: MF_TIMED_TEXT_DISPLAY_ALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_DISPLAY_ALIGNMENT_AFTER: MF_TIMED_TEXT_DISPLAY_ALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_DISPLAY_ALIGNMENT_CENTER: MF_TIMED_TEXT_DISPLAY_ALIGNMENT = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_ERROR_CODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_ERROR_CODE_NOERROR: MF_TIMED_TEXT_ERROR_CODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_ERROR_CODE_FATAL: MF_TIMED_TEXT_ERROR_CODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_ERROR_CODE_DATA_FORMAT: MF_TIMED_TEXT_ERROR_CODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_ERROR_CODE_NETWORK: MF_TIMED_TEXT_ERROR_CODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_ERROR_CODE_INTERNAL: MF_TIMED_TEXT_ERROR_CODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_FONT_STYLE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_FONT_STYLE_NORMAL: MF_TIMED_TEXT_FONT_STYLE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_FONT_STYLE_OBLIQUE: MF_TIMED_TEXT_FONT_STYLE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_FONT_STYLE_ITALIC: MF_TIMED_TEXT_FONT_STYLE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_RUBY_ALIGN = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_ALIGN_CENTER: MF_TIMED_TEXT_RUBY_ALIGN = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_ALIGN_START: MF_TIMED_TEXT_RUBY_ALIGN = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_ALIGN_END: MF_TIMED_TEXT_RUBY_ALIGN = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_ALIGN_SPACEAROUND: MF_TIMED_TEXT_RUBY_ALIGN = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_ALIGN_SPACEBETWEEN: MF_TIMED_TEXT_RUBY_ALIGN = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_ALIGN_WITHBASE: MF_TIMED_TEXT_RUBY_ALIGN = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_RUBY_POSITION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_POSITION_BEFORE: MF_TIMED_TEXT_RUBY_POSITION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_POSITION_AFTER: MF_TIMED_TEXT_RUBY_POSITION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_POSITION_OUTSIDE: MF_TIMED_TEXT_RUBY_POSITION = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_RUBY_RESERVE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_RESERVE_NONE: MF_TIMED_TEXT_RUBY_RESERVE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_RESERVE_BEFORE: MF_TIMED_TEXT_RUBY_RESERVE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_RESERVE_AFTER: MF_TIMED_TEXT_RUBY_RESERVE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_RESERVE_BOTH: MF_TIMED_TEXT_RUBY_RESERVE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_RUBY_RESERVE_OUTSIDE: MF_TIMED_TEXT_RUBY_RESERVE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_SCROLL_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_SCROLL_MODE_POP_ON: MF_TIMED_TEXT_SCROLL_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_SCROLL_MODE_ROLL_UP: MF_TIMED_TEXT_SCROLL_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_TRACK_KIND = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_TRACK_KIND_UNKNOWN: MF_TIMED_TEXT_TRACK_KIND = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_TRACK_KIND_SUBTITLES: MF_TIMED_TEXT_TRACK_KIND = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_TRACK_KIND_CAPTIONS: MF_TIMED_TEXT_TRACK_KIND = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_TRACK_KIND_METADATA: MF_TIMED_TEXT_TRACK_KIND = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_TRACK_READY_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_TRACK_READY_STATE_NONE: MF_TIMED_TEXT_TRACK_READY_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_TRACK_READY_STATE_LOADING: MF_TIMED_TEXT_TRACK_READY_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_TRACK_READY_STATE_LOADED: MF_TIMED_TEXT_TRACK_READY_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_TRACK_READY_STATE_ERROR: MF_TIMED_TEXT_TRACK_READY_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_UNIT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_UNIT_TYPE_PIXELS: MF_TIMED_TEXT_UNIT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_UNIT_TYPE_PERCENTAGE: MF_TIMED_TEXT_UNIT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TIMED_TEXT_WRITING_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_WRITING_MODE_LRTB: MF_TIMED_TEXT_WRITING_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_WRITING_MODE_RLTB: MF_TIMED_TEXT_WRITING_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_WRITING_MODE_TBRL: MF_TIMED_TEXT_WRITING_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_WRITING_MODE_TBLR: MF_TIMED_TEXT_WRITING_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_WRITING_MODE_LR: MF_TIMED_TEXT_WRITING_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_WRITING_MODE_RL: MF_TIMED_TEXT_WRITING_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TIMED_TEXT_WRITING_MODE_TB: MF_TIMED_TEXT_WRITING_MODE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOLOGY_RESOLUTION_SUCCEEDED: MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPTIONAL_NODE_REJECTED_MEDIA_TYPE: MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_OPTIONAL_NODE_REJECTED_PROTECTED_PROCESS: MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TOPOLOGY_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOLOGY_OUTPUT_NODE: MF_TOPOLOGY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOLOGY_SOURCESTREAM_NODE: MF_TOPOLOGY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOLOGY_TRANSFORM_NODE: MF_TOPOLOGY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOLOGY_TEE_NODE: MF_TOPOLOGY_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOLOGY_MAX: MF_TOPOLOGY_TYPE = -1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TOPONODE_DRAIN_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPONODE_DRAIN_DEFAULT: MF_TOPONODE_DRAIN_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPONODE_DRAIN_ALWAYS: MF_TOPONODE_DRAIN_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPONODE_DRAIN_NEVER: MF_TOPONODE_DRAIN_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TOPONODE_FLUSH_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPONODE_FLUSH_ALWAYS: MF_TOPONODE_FLUSH_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPONODE_FLUSH_SEEK: MF_TOPONODE_FLUSH_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPONODE_FLUSH_NEVER: MF_TOPONODE_FLUSH_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TOPOSTATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOSTATUS_INVALID: MF_TOPOSTATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOSTATUS_READY: MF_TOPOSTATUS = 100i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOSTATUS_STARTED_SOURCE: MF_TOPOSTATUS = 200i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOSTATUS_DYNAMIC_CHANGED: MF_TOPOSTATUS = 210i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOSTATUS_SINK_SWITCHED: MF_TOPOSTATUS = 300i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TOPOSTATUS_ENDED: MF_TOPOSTATUS = 400i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TRANSCODE_ADJUST_PROFILE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TRANSCODE_ADJUST_PROFILE_DEFAULT: MF_TRANSCODE_ADJUST_PROFILE_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TRANSCODE_ADJUST_PROFILE_USE_SOURCE_ATTRIBUTES: MF_TRANSCODE_ADJUST_PROFILE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_TRANSCODE_TOPOLOGYMODE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TRANSCODE_TOPOLOGYMODE_SOFTWARE_ONLY: MF_TRANSCODE_TOPOLOGYMODE_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_TRANSCODE_TOPOLOGYMODE_HARDWARE_ALLOWED: MF_TRANSCODE_TOPOLOGYMODE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_URL_TRUST_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_LICENSE_URL_UNTRUSTED: MF_URL_TRUST_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_LICENSE_URL_TRUSTED: MF_URL_TRUST_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_LICENSE_URL_TAMPERED: MF_URL_TRUST_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_VIDEO_PROCESSOR_ALGORITHM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_VIDEO_PROCESSOR_ALGORITHM_DEFAULT: MF_VIDEO_PROCESSOR_ALGORITHM_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MF_VIDEO_PROCESSOR_ALGORITHM_MRF_CRF_444: MF_VIDEO_PROCESSOR_ALGORITHM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_VIDEO_PROCESSOR_MIRROR = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MIRROR_NONE: MF_VIDEO_PROCESSOR_MIRROR = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MIRROR_HORIZONTAL: MF_VIDEO_PROCESSOR_MIRROR = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MIRROR_VERTICAL: MF_VIDEO_PROCESSOR_MIRROR = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MF_VIDEO_PROCESSOR_ROTATION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ROTATION_NONE: MF_VIDEO_PROCESSOR_ROTATION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const ROTATION_NORMAL: MF_VIDEO_PROCESSOR_ROTATION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MIC_ARRAY_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MICARRAY_SINGLE_CHAN: MIC_ARRAY_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MICARRAY_SIMPLE_SUM: MIC_ARRAY_MODE = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MICARRAY_SINGLE_BEAM: MIC_ARRAY_MODE = 512i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MICARRAY_FIXED_BEAM: MIC_ARRAY_MODE = 1024i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MICARRAY_EXTERN_BEAM: MIC_ARRAY_MODE = 2048i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MPEG2VIDEOINFO_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_DoPanScan: MPEG2VIDEOINFO_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_DVDLine21Field1: MPEG2VIDEOINFO_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_DVDLine21Field2: MPEG2VIDEOINFO_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_SourceIsLetterboxed: MPEG2VIDEOINFO_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_FilmCameraMode: MPEG2VIDEOINFO_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_LetterboxAnalogOut: MPEG2VIDEOINFO_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_DSS_UserData: MPEG2VIDEOINFO_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_DVB_UserData: MPEG2VIDEOINFO_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_27MhzTimebase: MPEG2VIDEOINFO_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AMMPEG2_WidescreenAnalogOut: MPEG2VIDEOINFO_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_ACP_PROTECTION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ACP_OFF: OPM_ACP_PROTECTION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ACP_LEVEL_ONE: OPM_ACP_PROTECTION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ACP_LEVEL_TWO: OPM_ACP_PROTECTION_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ACP_LEVEL_THREE: OPM_ACP_PROTECTION_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ACP_FORCE_ULONG: OPM_ACP_PROTECTION_LEVEL = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_BUS_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_TYPE_OTHER: OPM_BUS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_TYPE_PCI: OPM_BUS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_TYPE_PCIX: OPM_BUS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_TYPE_PCIEXPRESS: OPM_BUS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_TYPE_AGP: OPM_BUS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_IMPLEMENTATION_MODIFIER_INSIDE_OF_CHIPSET: OPM_BUS_TYPE = 65536i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_IMPLEMENTATION_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: OPM_BUS_TYPE = 131072i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_IMPLEMENTATION_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: OPM_BUS_TYPE = 196608i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_IMPLEMENTATION_MODIFIER_DAUGHTER_BOARD_CONNECTOR: OPM_BUS_TYPE = 262144i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_IMPLEMENTATION_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: OPM_BUS_TYPE = 327680i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_IMPLEMENTATION_MODIFIER_NON_STANDARD: OPM_BUS_TYPE = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_COPP_COMPATIBLE_BUS_TYPE_INTEGRATED: OPM_BUS_TYPE = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_CGMSA = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CGMSA_OFF: OPM_CGMSA = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CGMSA_COPY_FREELY: OPM_CGMSA = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CGMSA_COPY_NO_MORE: OPM_CGMSA = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CGMSA_COPY_ONE_GENERATION: OPM_CGMSA = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CGMSA_COPY_NEVER: OPM_CGMSA = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CGMSA_REDISTRIBUTION_CONTROL_REQUIRED: OPM_CGMSA = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_CONNECTOR_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_OTHER: OPM_CONNECTOR_TYPE = -1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_VGA: OPM_CONNECTOR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_SVIDEO: OPM_CONNECTOR_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_COMPOSITE_VIDEO: OPM_CONNECTOR_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_COMPONENT_VIDEO: OPM_CONNECTOR_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_DVI: OPM_CONNECTOR_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_HDMI: OPM_CONNECTOR_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_LVDS: OPM_CONNECTOR_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_D_JPN: OPM_CONNECTOR_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_SDI: OPM_CONNECTOR_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_DISPLAYPORT_EXTERNAL: OPM_CONNECTOR_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_DISPLAYPORT_EMBEDDED: OPM_CONNECTOR_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_UDI_EXTERNAL: OPM_CONNECTOR_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_UDI_EMBEDDED: OPM_CONNECTOR_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_RESERVED: OPM_CONNECTOR_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_MIRACAST: OPM_CONNECTOR_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_TRANSPORT_AGNOSTIC_DIGITAL_MODE_A: OPM_CONNECTOR_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONNECTOR_TYPE_TRANSPORT_AGNOSTIC_DIGITAL_MODE_B: OPM_CONNECTOR_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_COPP_COMPATIBLE_CONNECTOR_TYPE_INTERNAL: OPM_CONNECTOR_TYPE = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_DPCP_PROTECTION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_DPCP_OFF: OPM_DPCP_PROTECTION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_DPCP_ON: OPM_DPCP_PROTECTION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_DPCP_FORCE_ULONG: OPM_DPCP_PROTECTION_LEVEL = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_DVI_CHARACTERISTIC = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_DVI_CHARACTERISTIC_1_0: OPM_DVI_CHARACTERISTIC = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_DVI_CHARACTERISTIC_1_1_OR_ABOVE: OPM_DVI_CHARACTERISTIC = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_HDCP_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_FLAG_NONE: OPM_HDCP_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_FLAG_REPEATER: OPM_HDCP_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_HDCP_PROTECTION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_OFF: OPM_HDCP_PROTECTION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_ON: OPM_HDCP_PROTECTION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_FORCE_ULONG: OPM_HDCP_PROTECTION_LEVEL = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_HDCP_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_STATUS_ON: OPM_HDCP_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_STATUS_OFF: OPM_HDCP_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_HDCP_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_TYPE_0: OPM_HDCP_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_TYPE_1: OPM_HDCP_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_IMAGE_ASPECT_RATIO_EN300294 = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_4_BY_3: OPM_IMAGE_ASPECT_RATIO_EN300294 = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ASPECT_RATIO_EN300294_BOX_14_BY_9_CENTER: OPM_IMAGE_ASPECT_RATIO_EN300294 = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ASPECT_RATIO_EN300294_BOX_14_BY_9_TOP: OPM_IMAGE_ASPECT_RATIO_EN300294 = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ASPECT_RATIO_EN300294_BOX_16_BY_9_CENTER: OPM_IMAGE_ASPECT_RATIO_EN300294 = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ASPECT_RATIO_EN300294_BOX_16_BY_9_TOP: OPM_IMAGE_ASPECT_RATIO_EN300294 = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ASPECT_RATIO_EN300294_BOX_GT_16_BY_9_CENTER: OPM_IMAGE_ASPECT_RATIO_EN300294 = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_4_BY_3_PROTECTED_CENTER: OPM_IMAGE_ASPECT_RATIO_EN300294 = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_16_BY_9_ANAMORPHIC: OPM_IMAGE_ASPECT_RATIO_EN300294 = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ASPECT_RATIO_FORCE_ULONG: OPM_IMAGE_ASPECT_RATIO_EN300294 = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_OUTPUT_HARDWARE_PROTECTION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_OUTPUT_HARDWARE_PROTECTION_NOT_SUPPORTED: OPM_OUTPUT_HARDWARE_PROTECTION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_OUTPUT_HARDWARE_PROTECTION_SUPPORTED: OPM_OUTPUT_HARDWARE_PROTECTION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_PROTECTION_STANDARD_TYPE = u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_OTHER: OPM_PROTECTION_STANDARD_TYPE = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_NONE: OPM_PROTECTION_STANDARD_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_IEC61880_525I: OPM_PROTECTION_STANDARD_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_IEC61880_2_525I: OPM_PROTECTION_STANDARD_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_IEC62375_625P: OPM_PROTECTION_STANDARD_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_EIA608B_525: OPM_PROTECTION_STANDARD_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_EN300294_625I: OPM_PROTECTION_STANDARD_TYPE = 16u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_CEA805A_TYPEA_525P: OPM_PROTECTION_STANDARD_TYPE = 32u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_CEA805A_TYPEA_750P: OPM_PROTECTION_STANDARD_TYPE = 64u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_CEA805A_TYPEA_1125I: OPM_PROTECTION_STANDARD_TYPE = 128u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_CEA805A_TYPEB_525P: OPM_PROTECTION_STANDARD_TYPE = 256u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_CEA805A_TYPEB_750P: OPM_PROTECTION_STANDARD_TYPE = 512u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_CEA805A_TYPEB_1125I: OPM_PROTECTION_STANDARD_TYPE = 1024u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_ARIBTRB15_525I: OPM_PROTECTION_STANDARD_TYPE = 2048u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_ARIBTRB15_525P: OPM_PROTECTION_STANDARD_TYPE = 4096u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_ARIBTRB15_750P: OPM_PROTECTION_STANDARD_TYPE = 8192u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_STANDARD_ARIBTRB15_1125I: OPM_PROTECTION_STANDARD_TYPE = 16384u32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_PROTECTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_TYPE_OTHER: OPM_PROTECTION_TYPE = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_TYPE_NONE: OPM_PROTECTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_TYPE_COPP_COMPATIBLE_HDCP: OPM_PROTECTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_TYPE_ACP: OPM_PROTECTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_TYPE_CGMSA: OPM_PROTECTION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_TYPE_HDCP: OPM_PROTECTION_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_TYPE_DPCP: OPM_PROTECTION_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_TYPE_TYPE_ENFORCEMENT_HDCP: OPM_PROTECTION_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_STATUS_NORMAL: OPM_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_STATUS_LINK_LOST: OPM_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_STATUS_RENEGOTIATION_REQUIRED: OPM_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_STATUS_TAMPERING_DETECTED: OPM_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_STATUS_REVOKED_HDCP_DEVICE_ATTACHED: OPM_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_OMAC_SIZE: OPM_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_128_BIT_RANDOM_NUMBER_SIZE: OPM_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_ENCRYPTED_INITIALIZATION_PARAMETERS_SIZE: OPM_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_CONFIGURE_SETTING_DATA_SIZE: OPM_TYPE = 4056i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_GET_INFORMATION_PARAMETERS_SIZE: OPM_TYPE = 4056i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_REQUESTED_INFORMATION_SIZE: OPM_TYPE = 4076i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_HDCP_KEY_SELECTION_VECTOR_SIZE: OPM_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_PROTECTION_TYPE_SIZE: OPM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_TYPE_MASK: OPM_TYPE = 65535i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_BUS_IMPLEMENTATION_MODIFIER_MASK: OPM_TYPE = 32767i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_TYPE_ENFORCEMENT_HDCP_OFF: OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_TYPE_ENFORCEMENT_HDCP_ON_WITH_NO_TYPE_RESTRICTION: OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_TYPE_ENFORCEMENT_HDCP_ON_WITH_TYPE1_RESTRICTION: OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_TYPE_ENFORCEMENT_HDCP_FORCE_ULONG: OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 2147483647i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type OPM_VIDEO_OUTPUT_SEMANTICS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_VOS_COPP_SEMANTICS: OPM_VIDEO_OUTPUT_SEMANTICS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_VOS_OPM_SEMANTICS: OPM_VIDEO_OUTPUT_SEMANTICS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const OPM_VOS_OPM_INDIRECT_DISPLAY: OPM_VIDEO_OUTPUT_SEMANTICS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type PLAYTO_SOURCE_CREATEFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PLAYTO_SOURCE_NONE: PLAYTO_SOURCE_CREATEFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PLAYTO_SOURCE_IMAGE: PLAYTO_SOURCE_CREATEFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PLAYTO_SOURCE_AUDIO: PLAYTO_SOURCE_CREATEFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PLAYTO_SOURCE_VIDEO: PLAYTO_SOURCE_CREATEFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const PLAYTO_SOURCE_PROTECTED: PLAYTO_SOURCE_CREATEFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type SAMPLE_PROTECTION_VERSION = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SAMPLE_PROTECTION_VERSION_NO: SAMPLE_PROTECTION_VERSION = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SAMPLE_PROTECTION_VERSION_BASIC_LOKI: SAMPLE_PROTECTION_VERSION = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SAMPLE_PROTECTION_VERSION_SCATTER: SAMPLE_PROTECTION_VERSION = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SAMPLE_PROTECTION_VERSION_RC4: SAMPLE_PROTECTION_VERSION = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const SAMPLE_PROTECTION_VERSION_AES128CTR: SAMPLE_PROTECTION_VERSION = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type SEEK_ORIGIN = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const _msoBegin: SEEK_ORIGIN = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const _msoCurrent: SEEK_ORIGIN = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type TOC_POS_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const TOC_POS_INHEADER: TOC_POS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const TOC_POS_TOPLEVELOBJECT: TOC_POS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type WMT_PROP_DATATYPE = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMT_PROP_TYPE_DWORD: WMT_PROP_DATATYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMT_PROP_TYPE_STRING: WMT_PROP_DATATYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMT_PROP_TYPE_BINARY: WMT_PROP_DATATYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMT_PROP_TYPE_BOOL: WMT_PROP_DATATYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMT_PROP_TYPE_QWORD: WMT_PROP_DATATYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMT_PROP_TYPE_WORD: WMT_PROP_DATATYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMT_PROP_TYPE_GUID: WMT_PROP_DATATYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type WMV_DYNAMIC_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMV_DYNAMIC_BITRATE: WMV_DYNAMIC_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMV_DYNAMIC_RESOLUTION: WMV_DYNAMIC_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const WMV_DYNAMIC_COMPLEXITY: WMV_DYNAMIC_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFP_CREDENTIAL_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_CREDENTIAL_PROMPT: _MFP_CREDENTIAL_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_CREDENTIAL_SAVE: _MFP_CREDENTIAL_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_CREDENTIAL_DO_NOT_CACHE: _MFP_CREDENTIAL_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_CREDENTIAL_CLEAR_TEXT: _MFP_CREDENTIAL_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_CREDENTIAL_PROXY: _MFP_CREDENTIAL_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_CREDENTIAL_LOGGED_ON_USER: _MFP_CREDENTIAL_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFP_MEDIAITEM_CHARACTERISTICS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_MEDIAITEM_IS_LIVE: _MFP_MEDIAITEM_CHARACTERISTICS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_MEDIAITEM_CAN_SEEK: _MFP_MEDIAITEM_CHARACTERISTICS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_MEDIAITEM_CAN_PAUSE: _MFP_MEDIAITEM_CHARACTERISTICS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFP_MEDIAITEM_HAS_SLOW_SEEK: _MFP_MEDIAITEM_CHARACTERISTICS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFT_INPUT_DATA_BUFFER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_DATA_BUFFER_PLACEHOLDER: _MFT_INPUT_DATA_BUFFER_FLAGS = -1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFT_INPUT_STATUS_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_STATUS_ACCEPT_DATA: _MFT_INPUT_STATUS_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFT_INPUT_STREAM_INFO_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_STREAM_WHOLE_SAMPLES: _MFT_INPUT_STREAM_INFO_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER: _MFT_INPUT_STREAM_INFO_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_STREAM_FIXED_SAMPLE_SIZE: _MFT_INPUT_STREAM_INFO_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_STREAM_HOLDS_BUFFERS: _MFT_INPUT_STREAM_INFO_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_STREAM_DOES_NOT_ADDREF: _MFT_INPUT_STREAM_INFO_FLAGS = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_STREAM_REMOVABLE: _MFT_INPUT_STREAM_INFO_FLAGS = 512i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_STREAM_OPTIONAL: _MFT_INPUT_STREAM_INFO_FLAGS = 1024i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_INPUT_STREAM_PROCESSES_IN_PLACE: _MFT_INPUT_STREAM_INFO_FLAGS = 2048i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFT_OUTPUT_DATA_BUFFER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_DATA_BUFFER_INCOMPLETE: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_DATA_BUFFER_FORMAT_CHANGE: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_DATA_BUFFER_STREAM_END: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 512i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_DATA_BUFFER_NO_SAMPLE: _MFT_OUTPUT_DATA_BUFFER_FLAGS = 768i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFT_OUTPUT_STATUS_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STATUS_SAMPLE_READY: _MFT_OUTPUT_STATUS_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFT_OUTPUT_STREAM_INFO_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STREAM_WHOLE_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER: _MFT_OUTPUT_STREAM_INFO_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STREAM_FIXED_SAMPLE_SIZE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STREAM_DISCARDABLE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STREAM_OPTIONAL: _MFT_OUTPUT_STREAM_INFO_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STREAM_PROVIDES_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STREAM_CAN_PROVIDE_SAMPLES: _MFT_OUTPUT_STREAM_INFO_FLAGS = 512i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STREAM_LAZY_READ: _MFT_OUTPUT_STREAM_INFO_FLAGS = 1024i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_OUTPUT_STREAM_REMOVABLE: _MFT_OUTPUT_STREAM_INFO_FLAGS = 2048i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFT_PROCESS_OUTPUT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _MFT_PROCESS_OUTPUT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_PROCESS_OUTPUT_REGENERATE_LAST_OUTPUT: _MFT_PROCESS_OUTPUT_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFT_PROCESS_OUTPUT_STATUS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_PROCESS_OUTPUT_STATUS_NEW_STREAMS: _MFT_PROCESS_OUTPUT_STATUS = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type _MFT_SET_TYPE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const MFT_SET_TYPE_TEST_ONLY: _MFT_SET_TYPE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVAudioChannelConfig = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_FRONT_LEFT: eAVAudioChannelConfig = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_FRONT_RIGHT: eAVAudioChannelConfig = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_FRONT_CENTER: eAVAudioChannelConfig = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_LOW_FREQUENCY: eAVAudioChannelConfig = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_BACK_LEFT: eAVAudioChannelConfig = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_BACK_RIGHT: eAVAudioChannelConfig = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_FRONT_LEFT_OF_CENTER: eAVAudioChannelConfig = 64i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_FRONT_RIGHT_OF_CENTER: eAVAudioChannelConfig = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_BACK_CENTER: eAVAudioChannelConfig = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_SIDE_LEFT: eAVAudioChannelConfig = 512i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_SIDE_RIGHT: eAVAudioChannelConfig = 1024i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_TOP_CENTER: eAVAudioChannelConfig = 2048i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_TOP_FRONT_LEFT: eAVAudioChannelConfig = 4096i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_TOP_FRONT_CENTER: eAVAudioChannelConfig = 8192i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_TOP_FRONT_RIGHT: eAVAudioChannelConfig = 16384i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_TOP_BACK_LEFT: eAVAudioChannelConfig = 32768i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_TOP_BACK_CENTER: eAVAudioChannelConfig = 65536i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVAudioChannelConfig_TOP_BACK_RIGHT: eAVAudioChannelConfig = 131072i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDDSurroundMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDDSurroundMode_NotIndicated: eAVDDSurroundMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDDSurroundMode_No: eAVDDSurroundMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDDSurroundMode_Yes: eAVDDSurroundMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDSPLoudnessEqualization = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDSPLoudnessEqualization_OFF: eAVDSPLoudnessEqualization = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDSPLoudnessEqualization_ON: eAVDSPLoudnessEqualization = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDSPLoudnessEqualization_AUTO: eAVDSPLoudnessEqualization = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDSPSpeakerFill = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDSPSpeakerFill_OFF: eAVDSPSpeakerFill = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDSPSpeakerFill_ON: eAVDSPSpeakerFill = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDSPSpeakerFill_AUTO: eAVDSPSpeakerFill = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecAACDownmixMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecAACUseISODownmix: eAVDecAACDownmixMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecAACUseARIBDownmix: eAVDecAACDownmixMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecAudioDualMono = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecAudioDualMono_IsNotDualMono: eAVDecAudioDualMono = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecAudioDualMono_IsDualMono: eAVDecAudioDualMono = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecAudioDualMono_UnSpecified: eAVDecAudioDualMono = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecAudioDualMonoReproMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecAudioDualMonoReproMode_STEREO: eAVDecAudioDualMonoReproMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecAudioDualMonoReproMode_LEFT_MONO: eAVDecAudioDualMonoReproMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecAudioDualMonoReproMode_RIGHT_MONO: eAVDecAudioDualMonoReproMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecAudioDualMonoReproMode_MIX_MONO: eAVDecAudioDualMonoReproMode = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecDDMatrixDecodingMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDMatrixDecodingMode_OFF: eAVDecDDMatrixDecodingMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDMatrixDecodingMode_ON: eAVDecDDMatrixDecodingMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDMatrixDecodingMode_AUTO: eAVDecDDMatrixDecodingMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecDDOperationalMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDOperationalMode_NONE: eAVDecDDOperationalMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDOperationalMode_LINE: eAVDecDDOperationalMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDOperationalMode_RF: eAVDecDDOperationalMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDOperationalMode_CUSTOM0: eAVDecDDOperationalMode = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDOperationalMode_CUSTOM1: eAVDecDDOperationalMode = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDOperationalMode_PORTABLE8: eAVDecDDOperationalMode = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDOperationalMode_PORTABLE11: eAVDecDDOperationalMode = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDOperationalMode_PORTABLE14: eAVDecDDOperationalMode = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecDDStereoDownMixMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDStereoDownMixMode_Auto: eAVDecDDStereoDownMixMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDStereoDownMixMode_LtRt: eAVDecDDStereoDownMixMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecDDStereoDownMixMode_LoRo: eAVDecDDStereoDownMixMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecHEAACDynamicRangeControl = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecHEAACDynamicRangeControl_OFF: eAVDecHEAACDynamicRangeControl = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecHEAACDynamicRangeControl_ON: eAVDecHEAACDynamicRangeControl = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecVideoCodecType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoCodecType_NOTPLAYING: eAVDecVideoCodecType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoCodecType_MPEG2: eAVDecVideoCodecType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoCodecType_H264: eAVDecVideoCodecType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecVideoDXVABusEncryption = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoDXVABusEncryption_NONE: eAVDecVideoDXVABusEncryption = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoDXVABusEncryption_PRIVATE: eAVDecVideoDXVABusEncryption = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoDXVABusEncryption_AES: eAVDecVideoDXVABusEncryption = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecVideoDXVAMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoDXVAMode_NOTPLAYING: eAVDecVideoDXVAMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoDXVAMode_SW: eAVDecVideoDXVAMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoDXVAMode_MC: eAVDecVideoDXVAMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoDXVAMode_IDCT: eAVDecVideoDXVAMode = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoDXVAMode_VLD: eAVDecVideoDXVAMode = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecVideoH264ErrorConcealment = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eErrorConcealmentTypeDrop: eAVDecVideoH264ErrorConcealment = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eErrorConcealmentTypeBasic: eAVDecVideoH264ErrorConcealment = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eErrorConcealmentTypeAdvanced: eAVDecVideoH264ErrorConcealment = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eErrorConcealmentTypeDXVASetBlack: eAVDecVideoH264ErrorConcealment = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecVideoInputScanType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoInputScan_Unknown: eAVDecVideoInputScanType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoInputScan_Progressive: eAVDecVideoInputScanType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoInputScan_Interlaced_UpperFieldFirst: eAVDecVideoInputScanType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoInputScan_Interlaced_LowerFieldFirst: eAVDecVideoInputScanType = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecVideoMPEG2ErrorConcealment = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eErrorConcealmentOff: eAVDecVideoMPEG2ErrorConcealment = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eErrorConcealmentOn: eAVDecVideoMPEG2ErrorConcealment = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecVideoSWPowerLevel = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoSWPowerLevel_BatteryLife: eAVDecVideoSWPowerLevel = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoSWPowerLevel_Balanced: eAVDecVideoSWPowerLevel = 50i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoSWPowerLevel_VideoQuality: eAVDecVideoSWPowerLevel = 100i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVDecVideoSoftwareDeinterlaceMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoSoftwareDeinterlaceMode_NoDeinterlacing: eAVDecVideoSoftwareDeinterlaceMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoSoftwareDeinterlaceMode_ProgressiveDeinterlacing: eAVDecVideoSoftwareDeinterlaceMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoSoftwareDeinterlaceMode_BOBDeinterlacing: eAVDecVideoSoftwareDeinterlaceMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVDecVideoSoftwareDeinterlaceMode_SmartBOBDeinterlacing: eAVDecVideoSoftwareDeinterlaceMode = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncAdaptiveMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncAdaptiveMode_None: eAVEncAdaptiveMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncAdaptiveMode_Resolution: eAVEncAdaptiveMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncAdaptiveMode_FrameRate: eAVEncAdaptiveMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncAudioDualMono = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncAudioDualMono_SameAsInput: eAVEncAudioDualMono = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncAudioDualMono_Off: eAVEncAudioDualMono = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncAudioDualMono_On: eAVEncAudioDualMono = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncAudioInputContent = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AVEncAudioInputContent_Unknown: eAVEncAudioInputContent = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AVEncAudioInputContent_Voice: eAVEncAudioInputContent = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const AVEncAudioInputContent_Music: eAVEncAudioInputContent = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncChromaEncodeMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncChromaEncodeMode_420: eAVEncChromaEncodeMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncChromaEncodeMode_444: eAVEncChromaEncodeMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncChromaEncodeMode_444_v2: eAVEncChromaEncodeMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncCommonRateControlMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncCommonRateControlMode_CBR: eAVEncCommonRateControlMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncCommonRateControlMode_PeakConstrainedVBR: eAVEncCommonRateControlMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncCommonRateControlMode_UnconstrainedVBR: eAVEncCommonRateControlMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncCommonRateControlMode_Quality: eAVEncCommonRateControlMode = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncCommonRateControlMode_LowDelayVBR: eAVEncCommonRateControlMode = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncCommonRateControlMode_GlobalVBR: eAVEncCommonRateControlMode = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncCommonRateControlMode_GlobalLowDelayVBR: eAVEncCommonRateControlMode = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncCommonStreamEndHandling = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncCommonStreamEndHandling_DiscardPartial: eAVEncCommonStreamEndHandling = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncCommonStreamEndHandling_EnsureComplete: eAVEncCommonStreamEndHandling = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncDDAtoDConverterType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDAtoDConverterType_Standard: eAVEncDDAtoDConverterType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDAtoDConverterType_HDCD: eAVEncDDAtoDConverterType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncDDDynamicRangeCompressionControl = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDDynamicRangeCompressionControl_None: eAVEncDDDynamicRangeCompressionControl = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDDynamicRangeCompressionControl_FilmStandard: eAVEncDDDynamicRangeCompressionControl = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDDynamicRangeCompressionControl_FilmLight: eAVEncDDDynamicRangeCompressionControl = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDDynamicRangeCompressionControl_MusicStandard: eAVEncDDDynamicRangeCompressionControl = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDDynamicRangeCompressionControl_MusicLight: eAVEncDDDynamicRangeCompressionControl = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDDynamicRangeCompressionControl_Speech: eAVEncDDDynamicRangeCompressionControl = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncDDHeadphoneMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDHeadphoneMode_NotIndicated: eAVEncDDHeadphoneMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDHeadphoneMode_NotEncoded: eAVEncDDHeadphoneMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDHeadphoneMode_Encoded: eAVEncDDHeadphoneMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncDDPreferredStereoDownMixMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDPreferredStereoDownMixMode_LtRt: eAVEncDDPreferredStereoDownMixMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDPreferredStereoDownMixMode_LoRo: eAVEncDDPreferredStereoDownMixMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncDDProductionRoomType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDProductionRoomType_NotIndicated: eAVEncDDProductionRoomType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDProductionRoomType_Large: eAVEncDDProductionRoomType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDProductionRoomType_Small: eAVEncDDProductionRoomType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncDDService = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDService_CM: eAVEncDDService = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDService_ME: eAVEncDDService = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDService_VI: eAVEncDDService = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDService_HI: eAVEncDDService = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDService_D: eAVEncDDService = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDService_C: eAVEncDDService = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDService_E: eAVEncDDService = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDService_VO: eAVEncDDService = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncDDSurroundExMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDSurroundExMode_NotIndicated: eAVEncDDSurroundExMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDSurroundExMode_No: eAVEncDDSurroundExMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncDDSurroundExMode_Yes: eAVEncDDSurroundExMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncH263PictureType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263PictureType_I: eAVEncH263PictureType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263PictureType_P: eAVEncH263PictureType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263PictureType_B: eAVEncH263PictureType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncH263VLevel = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VLevel1: eAVEncH263VLevel = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VLevel2: eAVEncH263VLevel = 20i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VLevel3: eAVEncH263VLevel = 30i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VLevel4: eAVEncH263VLevel = 40i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VLevel4_5: eAVEncH263VLevel = 45i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VLevel5: eAVEncH263VLevel = 50i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VLevel6: eAVEncH263VLevel = 60i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VLevel7: eAVEncH263VLevel = 70i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncH263VProfile = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VProfile_Base: eAVEncH263VProfile = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VProfile_CompatibilityV2: eAVEncH263VProfile = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VProfile_CompatibilityV1: eAVEncH263VProfile = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VProfile_WirelessV2: eAVEncH263VProfile = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VProfile_WirelessV3: eAVEncH263VProfile = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VProfile_HighCompression: eAVEncH263VProfile = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VProfile_Internet: eAVEncH263VProfile = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VProfile_Interlace: eAVEncH263VProfile = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH263VProfile_HighLatency: eAVEncH263VProfile = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncH264PictureType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264PictureType_IDR: eAVEncH264PictureType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264PictureType_P: eAVEncH264PictureType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264PictureType_B: eAVEncH264PictureType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncH264VLevel = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel1: eAVEncH264VLevel = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel1_b: eAVEncH264VLevel = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel1_1: eAVEncH264VLevel = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel1_2: eAVEncH264VLevel = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel1_3: eAVEncH264VLevel = 13i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel2: eAVEncH264VLevel = 20i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel2_1: eAVEncH264VLevel = 21i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel2_2: eAVEncH264VLevel = 22i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel3: eAVEncH264VLevel = 30i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel3_1: eAVEncH264VLevel = 31i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel3_2: eAVEncH264VLevel = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel4: eAVEncH264VLevel = 40i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel4_1: eAVEncH264VLevel = 41i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel4_2: eAVEncH264VLevel = 42i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel5: eAVEncH264VLevel = 50i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel5_1: eAVEncH264VLevel = 51i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VLevel5_2: eAVEncH264VLevel = 52i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncH264VProfile = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_unknown: eAVEncH264VProfile = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_Simple: eAVEncH264VProfile = 66i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_Base: eAVEncH264VProfile = 66i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_Main: eAVEncH264VProfile = 77i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_High: eAVEncH264VProfile = 100i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_422: eAVEncH264VProfile = 122i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_High10: eAVEncH264VProfile = 110i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_444: eAVEncH264VProfile = 244i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_Extended: eAVEncH264VProfile = 88i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_ScalableBase: eAVEncH264VProfile = 83i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_ScalableHigh: eAVEncH264VProfile = 86i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_MultiviewHigh: eAVEncH264VProfile = 118i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_StereoHigh: eAVEncH264VProfile = 128i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_ConstrainedBase: eAVEncH264VProfile = 256i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_UCConstrainedHigh: eAVEncH264VProfile = 257i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_UCScalableConstrainedBase: eAVEncH264VProfile = 258i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH264VProfile_UCScalableConstrainedHigh: eAVEncH264VProfile = 259i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncH265VLevel = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel1: eAVEncH265VLevel = 30i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel2: eAVEncH265VLevel = 60i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel2_1: eAVEncH265VLevel = 63i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel3: eAVEncH265VLevel = 90i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel3_1: eAVEncH265VLevel = 93i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel4: eAVEncH265VLevel = 120i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel4_1: eAVEncH265VLevel = 123i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel5: eAVEncH265VLevel = 150i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel5_1: eAVEncH265VLevel = 153i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel5_2: eAVEncH265VLevel = 156i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel6: eAVEncH265VLevel = 180i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel6_1: eAVEncH265VLevel = 183i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VLevel6_2: eAVEncH265VLevel = 186i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncH265VProfile = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_unknown: eAVEncH265VProfile = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Main_420_8: eAVEncH265VProfile = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Main_420_10: eAVEncH265VProfile = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Main_420_12: eAVEncH265VProfile = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Main_422_10: eAVEncH265VProfile = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Main_422_12: eAVEncH265VProfile = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Main_444_8: eAVEncH265VProfile = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Main_444_10: eAVEncH265VProfile = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Main_444_12: eAVEncH265VProfile = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Monochrome_12: eAVEncH265VProfile = 9i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_Monochrome_16: eAVEncH265VProfile = 10i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainIntra_420_8: eAVEncH265VProfile = 11i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainIntra_420_10: eAVEncH265VProfile = 12i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainIntra_420_12: eAVEncH265VProfile = 13i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainIntra_422_10: eAVEncH265VProfile = 14i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainIntra_422_12: eAVEncH265VProfile = 15i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainIntra_444_8: eAVEncH265VProfile = 16i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainIntra_444_10: eAVEncH265VProfile = 17i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainIntra_444_12: eAVEncH265VProfile = 18i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainIntra_444_16: eAVEncH265VProfile = 19i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainStill_420_8: eAVEncH265VProfile = 20i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainStill_444_8: eAVEncH265VProfile = 21i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncH265VProfile_MainStill_444_16: eAVEncH265VProfile = 22i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncInputVideoSystem = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncInputVideoSystem_Unspecified: eAVEncInputVideoSystem = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncInputVideoSystem_PAL: eAVEncInputVideoSystem = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncInputVideoSystem_NTSC: eAVEncInputVideoSystem = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncInputVideoSystem_SECAM: eAVEncInputVideoSystem = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncInputVideoSystem_MAC: eAVEncInputVideoSystem = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncInputVideoSystem_HDV: eAVEncInputVideoSystem = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncInputVideoSystem_Component: eAVEncInputVideoSystem = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPACodingMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPACodingMode_Mono: eAVEncMPACodingMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPACodingMode_Stereo: eAVEncMPACodingMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPACodingMode_DualChannel: eAVEncMPACodingMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPACodingMode_JointStereo: eAVEncMPACodingMode = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPACodingMode_Surround: eAVEncMPACodingMode = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPAEmphasisType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPAEmphasisType_None: eAVEncMPAEmphasisType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPAEmphasisType_50_15: eAVEncMPAEmphasisType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPAEmphasisType_Reserved: eAVEncMPAEmphasisType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPAEmphasisType_CCITT_J17: eAVEncMPAEmphasisType = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPALayer = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPALayer_1: eAVEncMPALayer = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPALayer_2: eAVEncMPALayer = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPALayer_3: eAVEncMPALayer = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPVFrameFieldMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVFrameFieldMode_FieldMode: eAVEncMPVFrameFieldMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVFrameFieldMode_FrameMode: eAVEncMPVFrameFieldMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPVIntraVLCTable = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVIntraVLCTable_Auto: eAVEncMPVIntraVLCTable = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVIntraVLCTable_MPEG1: eAVEncMPVIntraVLCTable = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVIntraVLCTable_Alternate: eAVEncMPVIntraVLCTable = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPVLevel = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVLevel_Low: eAVEncMPVLevel = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVLevel_Main: eAVEncMPVLevel = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVLevel_High1440: eAVEncMPVLevel = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVLevel_High: eAVEncMPVLevel = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPVProfile = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVProfile_unknown: eAVEncMPVProfile = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVProfile_Simple: eAVEncMPVProfile = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVProfile_Main: eAVEncMPVProfile = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVProfile_High: eAVEncMPVProfile = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVProfile_422: eAVEncMPVProfile = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPVQScaleType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVQScaleType_Auto: eAVEncMPVQScaleType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVQScaleType_Linear: eAVEncMPVQScaleType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVQScaleType_NonLinear: eAVEncMPVQScaleType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPVScanPattern = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVScanPattern_Auto: eAVEncMPVScanPattern = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVScanPattern_ZigZagScan: eAVEncMPVScanPattern = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVScanPattern_AlternateScan: eAVEncMPVScanPattern = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMPVSceneDetection = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVSceneDetection_None: eAVEncMPVSceneDetection = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVSceneDetection_InsertIPicture: eAVEncMPVSceneDetection = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVSceneDetection_StartNewGOP: eAVEncMPVSceneDetection = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMPVSceneDetection_StartNewLocatableGOP: eAVEncMPVSceneDetection = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncMuxOutput = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMuxOutputAuto: eAVEncMuxOutput = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMuxOutputPS: eAVEncMuxOutput = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncMuxOutputTS: eAVEncMuxOutput = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVP9VProfile = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVP9VProfile_unknown: eAVEncVP9VProfile = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVP9VProfile_420_8: eAVEncVP9VProfile = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVP9VProfile_420_10: eAVEncVP9VProfile = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVP9VProfile_420_12: eAVEncVP9VProfile = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoChromaResolution = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaResolution_SameAsSource: eAVEncVideoChromaResolution = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaResolution_444: eAVEncVideoChromaResolution = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaResolution_422: eAVEncVideoChromaResolution = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaResolution_420: eAVEncVideoChromaResolution = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaResolution_411: eAVEncVideoChromaResolution = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoChromaSubsampling = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaSubsamplingFormat_SameAsSource: eAVEncVideoChromaSubsampling = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaSubsamplingFormat_ProgressiveChroma: eAVEncVideoChromaSubsampling = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaSubsamplingFormat_Horizontally_Cosited: eAVEncVideoChromaSubsampling = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaSubsamplingFormat_Vertically_Cosited: eAVEncVideoChromaSubsampling = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoChromaSubsamplingFormat_Vertically_AlignedChromaPlanes: eAVEncVideoChromaSubsampling = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoColorLighting = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorLighting_SameAsSource: eAVEncVideoColorLighting = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorLighting_Unknown: eAVEncVideoColorLighting = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorLighting_Bright: eAVEncVideoColorLighting = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorLighting_Office: eAVEncVideoColorLighting = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorLighting_Dim: eAVEncVideoColorLighting = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorLighting_Dark: eAVEncVideoColorLighting = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoColorNominalRange = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorNominalRange_SameAsSource: eAVEncVideoColorNominalRange = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorNominalRange_0_255: eAVEncVideoColorNominalRange = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorNominalRange_16_235: eAVEncVideoColorNominalRange = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorNominalRange_48_208: eAVEncVideoColorNominalRange = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoColorPrimaries = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorPrimaries_SameAsSource: eAVEncVideoColorPrimaries = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorPrimaries_Reserved: eAVEncVideoColorPrimaries = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorPrimaries_BT709: eAVEncVideoColorPrimaries = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorPrimaries_BT470_2_SysM: eAVEncVideoColorPrimaries = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorPrimaries_BT470_2_SysBG: eAVEncVideoColorPrimaries = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorPrimaries_SMPTE170M: eAVEncVideoColorPrimaries = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorPrimaries_SMPTE240M: eAVEncVideoColorPrimaries = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorPrimaries_EBU3231: eAVEncVideoColorPrimaries = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorPrimaries_SMPTE_C: eAVEncVideoColorPrimaries = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoColorTransferFunction = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferFunction_SameAsSource: eAVEncVideoColorTransferFunction = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferFunction_10: eAVEncVideoColorTransferFunction = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferFunction_18: eAVEncVideoColorTransferFunction = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferFunction_20: eAVEncVideoColorTransferFunction = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferFunction_22: eAVEncVideoColorTransferFunction = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferFunction_22_709: eAVEncVideoColorTransferFunction = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferFunction_22_240M: eAVEncVideoColorTransferFunction = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferFunction_22_8bit_sRGB: eAVEncVideoColorTransferFunction = 7i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferFunction_28: eAVEncVideoColorTransferFunction = 8i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoColorTransferMatrix = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferMatrix_SameAsSource: eAVEncVideoColorTransferMatrix = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferMatrix_BT709: eAVEncVideoColorTransferMatrix = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferMatrix_BT601: eAVEncVideoColorTransferMatrix = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoColorTransferMatrix_SMPTE240M: eAVEncVideoColorTransferMatrix = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoContentType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoContentType_Unknown: eAVEncVideoContentType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoContentType_FixedCameraAngle: eAVEncVideoContentType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoFilmContent = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoFilmContent_VideoOnly: eAVEncVideoFilmContent = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoFilmContent_FilmOnly: eAVEncVideoFilmContent = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoFilmContent_Mixed: eAVEncVideoFilmContent = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoOutputFrameRateConversion = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoOutputFrameRateConversion_Disable: eAVEncVideoOutputFrameRateConversion = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoOutputFrameRateConversion_Enable: eAVEncVideoOutputFrameRateConversion = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoOutputFrameRateConversion_Alias: eAVEncVideoOutputFrameRateConversion = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoOutputScanType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoOutputScan_Progressive: eAVEncVideoOutputScanType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoOutputScan_Interlaced: eAVEncVideoOutputScanType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoOutputScan_SameAsInput: eAVEncVideoOutputScanType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoOutputScan_Automatic: eAVEncVideoOutputScanType = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVEncVideoSourceScanType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoSourceScan_Automatic: eAVEncVideoSourceScanType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoSourceScan_Interlaced: eAVEncVideoSourceScanType = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVEncVideoSourceScan_Progressive: eAVEncVideoSourceScanType = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVFastDecodeMode = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eVideoDecodeCompliant: eAVFastDecodeMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eVideoDecodeOptimalLF: eAVFastDecodeMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eVideoDecodeDisableLF: eAVFastDecodeMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eVideoDecodeFastest: eAVFastDecodeMode = 32i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eAVScenarioInfo = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVScenarioInfo_Unknown: eAVScenarioInfo = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVScenarioInfo_DisplayRemoting: eAVScenarioInfo = 1i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVScenarioInfo_VideoConference: eAVScenarioInfo = 2i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVScenarioInfo_Archive: eAVScenarioInfo = 3i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVScenarioInfo_LiveStreaming: eAVScenarioInfo = 4i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVScenarioInfo_CameraRecord: eAVScenarioInfo = 5i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eAVScenarioInfo_DisplayRemotingWithFeatureMap: eAVScenarioInfo = 6i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type eVideoEncoderDisplayContentType = i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eVideoEncoderDisplayContent_Unknown: eVideoEncoderDisplayContentType = 0i32;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub const eVideoEncoderDisplayContent_FullScreenVideo: eVideoEncoderDisplayContentType = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AM_MEDIA_TYPE {
    pub majortype: ::windows_sys::core::GUID,
    pub subtype: ::windows_sys::core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows_sys::core::GUID,
    pub pUnk: ::windows_sys::core::IUnknown,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AM_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct ASF_FLAT_PICTURE {
    pub bPictureType: u8,
    pub dwDataLen: u32,
}
impl ::core::marker::Copy for ASF_FLAT_PICTURE {}
impl ::core::clone::Clone for ASF_FLAT_PICTURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct ASF_FLAT_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub dwLyricsLen: u32,
}
impl ::core::marker::Copy for ASF_FLAT_SYNCHRONISED_LYRICS {}
impl ::core::clone::Clone for ASF_FLAT_SYNCHRONISED_LYRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct ASF_INDEX_DESCRIPTOR {
    pub Identifier: ASF_INDEX_IDENTIFIER,
    pub cPerEntryBytes: u16,
    pub szDescription: [u16; 32],
    pub dwInterval: u32,
}
impl ::core::marker::Copy for ASF_INDEX_DESCRIPTOR {}
impl ::core::clone::Clone for ASF_INDEX_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct ASF_INDEX_IDENTIFIER {
    pub guidIndexType: ::windows_sys::core::GUID,
    pub wStreamNumber: u16,
}
impl ::core::marker::Copy for ASF_INDEX_IDENTIFIER {}
impl ::core::clone::Clone for ASF_INDEX_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct ASF_MUX_STATISTICS {
    pub cFramesWritten: u32,
    pub cFramesDropped: u32,
}
impl ::core::marker::Copy for ASF_MUX_STATISTICS {}
impl ::core::clone::Clone for ASF_MUX_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct AecQualityMetrics_Struct {
    pub i64Timestamp: i64,
    pub ConvergenceFlag: u8,
    pub MicClippedFlag: u8,
    pub MicSilenceFlag: u8,
    pub PstvFeadbackFlag: u8,
    pub SpkClippedFlag: u8,
    pub SpkMuteFlag: u8,
    pub GlitchFlag: u8,
    pub DoubleTalkFlag: u8,
    pub uGlitchCount: u32,
    pub uMicClipCount: u32,
    pub fDuration: f32,
    pub fTSVariance: f32,
    pub fTSDriftRate: f32,
    pub fVoiceLevel: f32,
    pub fNoiseLevel: f32,
    pub fERLE: f32,
    pub fAvgERLE: f32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for AecQualityMetrics_Struct {}
impl ::core::clone::Clone for AecQualityMetrics_Struct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct CodecAPIEventData {
    pub guid: ::windows_sys::core::GUID,
    pub dataLength: u32,
    pub reserved: [u32; 3],
}
impl ::core::marker::Copy for CodecAPIEventData {}
impl ::core::clone::Clone for CodecAPIEventData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE {
    pub IOCoherent: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE {
    pub VideoDecoderHeapDesc: D3D12_VIDEO_DECODER_HEAP_DESC,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1 {
    pub VideoDecoderHeapDesc: D3D12_VIDEO_DECODER_HEAP_DESC,
    pub Protected: super::super::Foundation::BOOL,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_CONVERSION_SUPPORT {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub DecodeSample: D3D12_VIDEO_SAMPLE,
    pub OutputFormat: D3D12_VIDEO_FORMAT,
    pub FrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub BitRate: u32,
    pub SupportFlags: D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS,
    pub ScaleSupport: D3D12_VIDEO_SCALE_SUPPORT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODE_CONVERSION_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODE_CONVERSION_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub FormatCount: u32,
    pub pOutputFormats: *mut super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub FormatCount: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM {
    pub NodeIndex: u32,
    pub DecodeProfile: ::windows_sys::core::GUID,
    pub Width: u32,
    pub Height: u32,
    pub DecodeFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub Components: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS,
    pub BinCount: u32,
    pub CounterBitDepth: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {
    pub NodeIndex: u32,
    pub ProfileCount: u32,
    pub pProfiles: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT {
    pub NodeIndex: u32,
    pub ProfileCount: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub SupportFlags: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_SUPPORT {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub Width: u32,
    pub Height: u32,
    pub DecodeFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub FrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub BitRate: u32,
    pub SupportFlags: D3D12_VIDEO_DECODE_SUPPORT_FLAGS,
    pub ConfigurationFlags: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS,
    pub DecodeTier: D3D12_VIDEO_DECODE_TIER,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_DECODE_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_DECODE_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub IsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub IsSupported: super::super::Foundation::BOOL,
    pub CodecSupportLimits: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub IsSupported: super::super::Foundation::BOOL,
    pub PictureSupport: D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub Level: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub SubregionMode: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub IsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_HEAP_SIZE {
    pub HeapDesc: D3D12_VIDEO_ENCODER_HEAP_DESC,
    pub IsSupported: super::super::Foundation::BOOL,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_HEAP_SIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_HEAP_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_INPUT_FORMAT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub Format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub IsSupported: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_INPUT_FORMAT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_INPUT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_INTRA_REFRESH_MODE {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub Level: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub IntraRefreshMode: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE,
    pub IsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_INTRA_REFRESH_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_INTRA_REFRESH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub ResolutionRatiosCount: u32,
    pub IsSupported: super::super::Foundation::BOOL,
    pub MinResolutionSupported: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub MaxResolutionSupported: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub ResolutionWidthMultipleRequirement: u32,
    pub ResolutionHeightMultipleRequirement: u32,
    pub pResolutionRatios: *mut D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub ResolutionRatiosCount: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_PROFILE_LEVEL {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub IsSupported: super::super::Foundation::BOOL,
    pub MinSupportedLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub MaxSupportedLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_PROFILE_LEVEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_PROFILE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub RateControlMode: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE,
    pub IsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS {
    pub MaxSubregionsNumber: u32,
    pub MaxIntraRefreshFrameDuration: u32,
    pub SubregionBlockPixelsSize: u32,
    pub QPMapRegionPixelsSize: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOURCE_REQUIREMENTS {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub InputFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub PictureTargetResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub IsSupported: super::super::Foundation::BOOL,
    pub CompressedBitstreamBufferAccessAlignment: u32,
    pub EncoderMetadataBufferAccessAlignment: u32,
    pub MaxEncoderOutputMetadataBufferSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOURCE_REQUIREMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOURCE_REQUIREMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub InputFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
    pub CodecGopSequence: D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE,
    pub RateControl: D3D12_VIDEO_ENCODER_RATE_CONTROL,
    pub IntraRefresh: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE,
    pub SubregionFrameEncoding: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub ResolutionsListCount: u32,
    pub pResolutionList: *const D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub MaxReferenceFramesInDPB: u32,
    pub ValidationFlags: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS,
    pub SupportFlags: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS,
    pub SuggestedProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub SuggestedLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub pResolutionDependentSupport: *mut D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS {
    pub NodeIndex: u32,
    pub CommandCount: u32,
    pub pCommandInfos: *mut D3D12_VIDEO_EXTENSION_COMMAND_INFO,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT {
    pub NodeIndex: u32,
    pub CommandCount: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {
    pub CommandId: ::windows_sys::core::GUID,
    pub Stage: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE,
    pub ParameterCount: u32,
    pub pParameterInfos: *mut D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT {
    pub CommandId: ::windows_sys::core::GUID,
    pub Stage: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE,
    pub ParameterCount: u32,
    pub ParameterPacking: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {
    pub NodeIndex: u32,
    pub CommandId: ::windows_sys::core::GUID,
    pub pCreationParameters: *const ::core::ffi::c_void,
    pub CreationParametersSizeInBytes: usize,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {
    pub NodeIndex: u32,
    pub CommandId: ::windows_sys::core::GUID,
    pub pInputData: *const ::core::ffi::c_void,
    pub InputDataSizeInBytes: usize,
    pub pOutputData: *mut ::core::ffi::c_void,
    pub OutputDataSizeInBytes: usize,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT {
    pub NodeIndex: u32,
    pub VideoDecodeSupport: super::super::Foundation::BOOL,
    pub VideoProcessSupport: super::super::Foundation::BOOL,
    pub VideoEncodeSupport: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR {
    pub NodeIndex: u32,
    pub InputFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub BlockSizeFlags: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS,
    pub PrecisionFlags: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES {
    pub NodeIndex: u32,
    pub SupportFlags: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE {
    pub NodeIndex: u32,
    pub InputFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub BlockSize: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE,
    pub Precision: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub Protected: super::super::Foundation::BOOL,
    pub MotionVectorHeapMemoryPoolL0Size: u64,
    pub MotionVectorHeapMemoryPoolL1Size: u64,
    pub MotionEstimatorMemoryPoolL0Size: u64,
    pub MotionEstimatorMemoryPoolL1Size: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE {
    pub NodeMask: u32,
    pub pOutputStreamDesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC,
    pub NumInputStreamDescs: u32,
    pub pInputStreamDescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {
    pub NodeMask: u32,
    pub pOutputStreamDesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC,
    pub NumInputStreamDescs: u32,
    pub pInputStreamDescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC,
    pub Protected: super::super::Foundation::BOOL,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS {
    pub NodeIndex: u32,
    pub MaxInputStreams: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES {
    pub NodeIndex: u32,
    pub SupportFlags: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO {
    pub NodeIndex: u32,
    pub DeinterlaceMode: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS,
    pub Filters: D3D12_VIDEO_PROCESS_FILTER_FLAGS,
    pub FeatureSupport: D3D12_VIDEO_PROCESS_FEATURE_FLAGS,
    pub InputFrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub OutputFrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub EnableAutoProcessing: super::super::Foundation::BOOL,
    pub PastFrames: u32,
    pub FutureFrames: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT {
    pub NodeIndex: u32,
    pub InputSample: D3D12_VIDEO_SAMPLE,
    pub InputFieldType: D3D12_VIDEO_FIELD_TYPE,
    pub InputStereoFormat: D3D12_VIDEO_FRAME_STEREO_FORMAT,
    pub InputFrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub OutputFormat: D3D12_VIDEO_FORMAT,
    pub OutputStereoFormat: D3D12_VIDEO_FRAME_STEREO_FORMAT,
    pub OutputFrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub SupportFlags: D3D12_VIDEO_PROCESS_SUPPORT_FLAGS,
    pub ScaleSupport: D3D12_VIDEO_SCALE_SUPPORT,
    pub FeatureSupport: D3D12_VIDEO_PROCESS_FEATURE_FLAGS,
    pub DeinterlaceSupport: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS,
    pub AutoProcessingSupport: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS,
    pub FilterSupport: D3D12_VIDEO_PROCESS_FILTER_FLAGS,
    pub FilterRangeSupport: [D3D12_VIDEO_PROCESS_FILTER_RANGE; 32],
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS {
    pub Status: u64,
    pub NumMacroblocksAffected: u64,
    pub FrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub BitRate: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {
    pub pMotionVectorHeap: ID3D12VideoMotionVectorHeap,
    pub PixelWidth: u32,
    pub PixelHeight: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {
    pub pMotionVectorTexture2D: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub MotionVectorCoordinate: D3D12_RESOURCE_COORDINATE,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_RESOURCE_COORDINATE {
    pub X: u64,
    pub Y: u32,
    pub Z: u32,
    pub SubresourceIndex: u32,
}
impl ::core::marker::Copy for D3D12_RESOURCE_COORDINATE {}
impl ::core::clone::Clone for D3D12_RESOURCE_COORDINATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_DECODER_DESC {
    pub NodeMask: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
}
impl ::core::marker::Copy for D3D12_VIDEO_DECODER_DESC {}
impl ::core::clone::Clone for D3D12_VIDEO_DECODER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_VIDEO_DECODER_HEAP_DESC {
    pub NodeMask: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub DecodeWidth: u32,
    pub DecodeHeight: u32,
    pub Format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub FrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub BitRate: u32,
    pub MaxDecodePictureBufferCount: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_VIDEO_DECODER_HEAP_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_VIDEO_DECODER_HEAP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {
    pub pBuffer: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub Offset: u64,
    pub Size: u64,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_DECODE_CONFIGURATION {
    pub DecodeProfile: ::windows_sys::core::GUID,
    pub BitstreamEncryption: D3D12_BITSTREAM_ENCRYPTION_TYPE,
    pub InterlaceType: D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE,
}
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_CONFIGURATION {}
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {
    pub Enable: super::super::Foundation::BOOL,
    pub pReferenceTexture2D: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub ReferenceSubresource: u32,
    pub OutputColorSpace: super::super::Graphics::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
    pub DecodeColorSpace: super::super::Graphics::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {
    pub Enable: super::super::Foundation::BOOL,
    pub pReferenceTexture2D: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub ReferenceSubresource: u32,
    pub OutputColorSpace: super::super::Graphics::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
    pub DecodeColorSpace: super::super::Graphics::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
    pub OutputWidth: u32,
    pub OutputHeight: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_DECODE_FRAME_ARGUMENT {
    pub Type: D3D12_VIDEO_DECODE_ARGUMENT_TYPE,
    pub Size: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_FRAME_ARGUMENT {}
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_FRAME_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {
    pub NumFrameArguments: u32,
    pub FrameArguments: [D3D12_VIDEO_DECODE_FRAME_ARGUMENT; 10],
    pub ReferenceFrames: D3D12_VIDEO_DECODE_REFERENCE_FRAMES,
    pub CompressedBitstream: D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM,
    pub pHeap: ID3D12VideoDecoderHeap,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {
    pub Offset: u64,
    pub pBuffer: super::super::Graphics::Direct3D12::ID3D12Resource,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {
    pub pOutputTexture2D: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub OutputSubresource: u32,
    pub ConversionArguments: D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {
    pub pOutputTexture2D: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub OutputSubresource: u32,
    pub ConversionArguments: D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1,
    pub Histograms: [D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM; 4],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_DECODE_REFERENCE_FRAMES {
    pub NumTexture2Ds: u32,
    pub ppTexture2Ds: *mut super::super::Graphics::Direct3D12::ID3D12Resource,
    pub pSubresources: *mut u32,
    pub ppHeaps: *mut ID3D12VideoDecoderHeap,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_DECODE_REFERENCE_FRAMES {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_DECODE_REFERENCE_FRAMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_0,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_0 {
    pub pH264Config: *mut D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264,
    pub pHEVCConfig: *mut D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_0 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264 {
    pub ConfigurationFlags: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS,
    pub DirectModeConfig: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES,
    pub DisableDeblockingFilterConfig: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC {
    pub ConfigurationFlags: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS,
    pub MinLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MaxLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MinLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub MaxLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_0,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_0 {
    pub pH264Support: *mut D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264,
    pub pHEVCSupport: *mut D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_0 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264 {
    pub SupportFlags: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS,
    pub DisableDeblockingFilterSupportedModes: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC {
    pub SupportFlags: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS,
    pub MinLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MaxLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MinLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub MaxLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_0,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_0 {
    pub pH264Support: *mut D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264,
    pub pHEVCSupport: *mut D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_0 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264 {
    pub MaxL0ReferencesForP: u32,
    pub MaxL0ReferencesForB: u32,
    pub MaxL1ReferencesForB: u32,
    pub MaxLongTermReferences: u32,
    pub MaxDPBCapacity: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC {
    pub MaxL0ReferencesForP: u32,
    pub MaxL0ReferencesForB: u32,
    pub MaxL1ReferencesForB: u32,
    pub MaxLongTermReferences: u32,
    pub MaxDPBCapacity: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {
    pub pBuffer: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub FrameStartOffset: u64,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_VIDEO_ENCODER_DESC {
    pub NodeMask: u32,
    pub Flags: D3D12_VIDEO_ENCODER_FLAGS,
    pub EncodeCodec: D3D12_VIDEO_ENCODER_CODEC,
    pub EncodeProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub InputFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
    pub MaxMotionEstimationPrecision: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS {
    pub SequenceControlDesc: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC,
    pub PictureControlDesc: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC,
    pub pInputFrame: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub InputFrameSubresource: u32,
    pub CurrentFrameBitstreamMetadataSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS {
    pub Bitstream: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM,
    pub ReconstructedPicture: D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE,
    pub EncoderOutputMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {
    pub pBuffer: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub Offset: u64,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_FRAME_SUBREGION_METADATA {
    pub bSize: u64,
    pub bStartOffset: u64,
    pub bHeaderSize: u64,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_METADATA {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_HEAP_DESC {
    pub NodeMask: u32,
    pub Flags: D3D12_VIDEO_ENCODER_HEAP_FLAGS,
    pub EncodeCodec: D3D12_VIDEO_ENCODER_CODEC,
    pub EncodeProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub EncodeLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub ResolutionsListCount: u32,
    pub pResolutionList: *const D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_HEAP_DESC {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_HEAP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_INTRA_REFRESH {
    pub Mode: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE,
    pub IntraRefreshDuration: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_INTRA_REFRESH {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_INTRA_REFRESH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_LEVEL_SETTING {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_LEVEL_SETTING_0,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_LEVEL_SETTING {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_LEVEL_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union D3D12_VIDEO_ENCODER_LEVEL_SETTING_0 {
    pub pH264LevelSetting: *mut D3D12_VIDEO_ENCODER_LEVELS_H264,
    pub pHEVCLevelSetting: *mut D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_LEVEL_SETTING_0 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_LEVEL_SETTING_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC {
    pub Level: D3D12_VIDEO_ENCODER_LEVELS_HEVC,
    pub Tier: D3D12_VIDEO_ENCODER_TIER_HEVC,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_OUTPUT_METADATA {
    pub EncodeErrorFlags: u64,
    pub EncodeStats: D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS,
    pub EncodedBitstreamWrittenBytesCount: u64,
    pub WrittenSubregionsCount: u64,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_OUTPUT_METADATA {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_OUTPUT_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS {
    pub AverageQP: u64,
    pub IntraCodingUnitsCount: u64,
    pub InterCodingUnitsCount: u64,
    pub SkipCodingUnitsCount: u64,
    pub AverageMotionEstimationXDirection: u64,
    pub AverageMotionEstimationYDirection: u64,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_0 {
    pub pH264PicData: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264,
    pub pHEVCPicData: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264 {
    pub Flags: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS,
    pub FrameType: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264,
    pub pic_parameter_set_id: u32,
    pub idr_pic_id: u32,
    pub PictureOrderCountNumber: u32,
    pub FrameDecodingOrderNumber: u32,
    pub TemporalLayerIndex: u32,
    pub List0ReferenceFramesCount: u32,
    pub pList0ReferenceFrames: *mut u32,
    pub List1ReferenceFramesCount: u32,
    pub pList1ReferenceFrames: *mut u32,
    pub ReferenceFramesReconPictureDescriptorsCount: u32,
    pub pReferenceFramesReconPictureDescriptors: *mut D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264,
    pub adaptive_ref_pic_marking_mode_flag: u8,
    pub RefPicMarkingOperationsCommandsCount: u32,
    pub pRefPicMarkingOperationsCommands: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION,
    pub List0RefPicModificationsCount: u32,
    pub pList0RefPicModifications: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION,
    pub List1RefPicModificationsCount: u32,
    pub pList1RefPicModifications: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION,
    pub QPMapValuesCount: u32,
    pub pRateControlQPMap: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION {
    pub modification_of_pic_nums_idc: u8,
    pub abs_diff_pic_num_minus1: u32,
    pub long_term_pic_num: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION {
    pub memory_management_control_operation: u8,
    pub difference_of_pic_nums_minus1: u32,
    pub long_term_pic_num: u32,
    pub long_term_frame_idx: u32,
    pub max_long_term_frame_idx_plus1: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC {
    pub Flags: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS,
    pub FrameType: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC,
    pub slice_pic_parameter_set_id: u32,
    pub PictureOrderCountNumber: u32,
    pub TemporalLayerIndex: u32,
    pub List0ReferenceFramesCount: u32,
    pub pList0ReferenceFrames: *mut u32,
    pub List1ReferenceFramesCount: u32,
    pub pList1ReferenceFrames: *mut u32,
    pub ReferenceFramesReconPictureDescriptorsCount: u32,
    pub pReferenceFramesReconPictureDescriptors: *mut D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC,
    pub List0RefPicModificationsCount: u32,
    pub pList0RefPicModifications: *mut u32,
    pub List1RefPicModificationsCount: u32,
    pub pList1RefPicModifications: *mut u32,
    pub QPMapValuesCount: u32,
    pub pRateControlQPMap: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC {
    pub IntraRefreshFrameIndex: u32,
    pub Flags: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS,
    pub PictureControlCodecData: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA,
    pub ReferenceFrames: D3D12_VIDEO_ENCODE_REFERENCE_FRAMES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_0,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_0 {
    pub pSlicesPartition_H264: *const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES,
    pub pSlicesPartition_HEVC: *const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_0 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES {
    pub Anonymous: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES_0,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES_0 {
    pub MaxBytesPerSlice: u32,
    pub NumberOfCodingUnitsPerSlice: u32,
    pub NumberOfRowsPerSlice: u32,
    pub NumberOfSlicesPerFrame: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES_0 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC {
    pub WidthRatio: u32,
    pub HeightRatio: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_PROFILE_DESC {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_PROFILE_DESC_0,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PROFILE_DESC {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PROFILE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union D3D12_VIDEO_ENCODER_PROFILE_DESC_0 {
    pub pH264Profile: *mut D3D12_VIDEO_ENCODER_PROFILE_H264,
    pub pHEVCProfile: *mut D3D12_VIDEO_ENCODER_PROFILE_HEVC,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_PROFILE_DESC_0 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_PROFILE_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL {
    pub Mode: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE,
    pub Flags: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS,
    pub ConfigParams: D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS,
    pub TargetFrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RATE_CONTROL {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RATE_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR {
    pub InitialQP: u32,
    pub MinQP: u32,
    pub MaxQP: u32,
    pub MaxFrameBitSize: u64,
    pub TargetBitRate: u64,
    pub VBVCapacity: u64,
    pub InitialVBVFullness: u64,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS_0,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS_0 {
    pub pConfiguration_CQP: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP,
    pub pConfiguration_CBR: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR,
    pub pConfiguration_VBR: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR,
    pub pConfiguration_QVBR: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS_0 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP {
    pub ConstantQP_FullIntracodedFrame: u32,
    pub ConstantQP_InterPredictedFrame_PrevRefOnly: u32,
    pub ConstantQP_InterPredictedFrame_BiDirectionalRef: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR {
    pub InitialQP: u32,
    pub MinQP: u32,
    pub MaxQP: u32,
    pub MaxFrameBitSize: u64,
    pub TargetAvgBitRate: u64,
    pub PeakBitRate: u64,
    pub ConstantQualityTarget: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR {
    pub InitialQP: u32,
    pub MinQP: u32,
    pub MaxQP: u32,
    pub MaxFrameBitSize: u64,
    pub TargetAvgBitRate: u64,
    pub PeakBitRate: u64,
    pub VBVCapacity: u64,
    pub InitialVBVFullness: u64,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {
    pub pReconstructedPicture: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub ReconstructedPictureSubresource: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264 {
    pub ReconstructedPictureResourceIndex: u32,
    pub IsLongTermReference: super::super::Foundation::BOOL,
    pub LongTermPictureIdx: u32,
    pub PictureOrderCountNumber: u32,
    pub FrameDecodingOrderNumber: u32,
    pub TemporalLayerIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC {
    pub ReconstructedPictureResourceIndex: u32,
    pub IsRefUsedByCurrentPic: super::super::Foundation::BOOL,
    pub IsLongTermReference: super::super::Foundation::BOOL,
    pub PictureOrderCountNumber: u32,
    pub TemporalLayerIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS {
    pub EncoderCodec: D3D12_VIDEO_ENCODER_CODEC,
    pub EncoderProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub EncoderInputFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub EncodedPictureEffectiveResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub HWLayoutMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS {}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS {
    pub ResolvedLayoutMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC {
    pub Flags: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS,
    pub IntraRefreshConfig: D3D12_VIDEO_ENCODER_INTRA_REFRESH,
    pub RateControl: D3D12_VIDEO_ENCODER_RATE_CONTROL,
    pub PictureTargetResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub SelectedLayoutMode: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub FrameSubregionsLayoutData: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA,
    pub CodecGopSequence: D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_0,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_0 {
    pub pH264GroupOfPictures: *mut D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264,
    pub pHEVCGroupOfPictures: *mut D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_0 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264 {
    pub GOPLength: u32,
    pub PPicturePeriod: u32,
    pub pic_order_cnt_type: u8,
    pub log2_max_frame_num_minus4: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264 {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC {
    pub GOPLength: u32,
    pub PPicturePeriod: u32,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
}
impl ::core::marker::Copy for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC {}
impl ::core::clone::Clone for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {
    pub NumTexture2Ds: u32,
    pub ppTexture2Ds: *mut super::super::Graphics::Direct3D12::ID3D12Resource,
    pub pSubresources: *mut u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_EXTENSION_COMMAND_DESC {
    pub NodeMask: u32,
    pub CommandId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for D3D12_VIDEO_EXTENSION_COMMAND_DESC {}
impl ::core::clone::Clone for D3D12_VIDEO_EXTENSION_COMMAND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_EXTENSION_COMMAND_INFO {
    pub CommandId: ::windows_sys::core::GUID,
    pub Name: ::windows_sys::core::PCWSTR,
    pub CommandListSupportFlags: super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_SUPPORT_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_EXTENSION_COMMAND_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_EXTENSION_COMMAND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {
    pub Name: ::windows_sys::core::PCWSTR,
    pub Type: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE,
    pub Flags: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS,
}
impl ::core::marker::Copy for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {}
impl ::core::clone::Clone for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_VIDEO_FORMAT {
    pub Format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub ColorSpace: super::super::Graphics::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_VIDEO_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_VIDEO_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_VIDEO_MOTION_ESTIMATOR_DESC {
    pub NodeMask: u32,
    pub InputFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub BlockSize: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE,
    pub Precision: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_VIDEO_MOTION_ESTIMATOR_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_VIDEO_MOTION_ESTIMATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {
    pub pInputTexture2D: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub InputSubresourceIndex: u32,
    pub pReferenceTexture2D: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub ReferenceSubresourceIndex: u32,
    pub pHintMotionVectorHeap: ID3D12VideoMotionVectorHeap,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {
    pub pMotionVectorHeap: ID3D12VideoMotionVectorHeap,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {
    pub NodeMask: u32,
    pub InputFormat: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub BlockSize: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE,
    pub Precision: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_VIDEO_PROCESS_ALPHA_BLENDING {
    pub Enable: super::super::Foundation::BOOL,
    pub Alpha: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_ALPHA_BLENDING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_ALPHA_BLENDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_PROCESS_FILTER_RANGE {
    pub Minimum: i32,
    pub Maximum: i32,
    pub Default: i32,
    pub Multiplier: f32,
}
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_FILTER_RANGE {}
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_FILTER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM {
    pub pTexture2D: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub Subresource: u32,
    pub ReferenceSet: D3D12_VIDEO_PROCESS_REFERENCE_SET,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_INPUT_STREAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_INPUT_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS {
    pub InputStream: [D3D12_VIDEO_PROCESS_INPUT_STREAM; 2],
    pub Transform: D3D12_VIDEO_PROCESS_TRANSFORM,
    pub Flags: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS,
    pub RateInfo: D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE,
    pub FilterLevels: [i32; 32],
    pub AlphaBlending: D3D12_VIDEO_PROCESS_ALPHA_BLENDING,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1 {
    pub InputStream: [D3D12_VIDEO_PROCESS_INPUT_STREAM; 2],
    pub Transform: D3D12_VIDEO_PROCESS_TRANSFORM,
    pub Flags: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS,
    pub RateInfo: D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE,
    pub FilterLevels: [i32; 32],
    pub AlphaBlending: D3D12_VIDEO_PROCESS_ALPHA_BLENDING,
    pub FieldType: D3D12_VIDEO_FIELD_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC {
    pub Format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub ColorSpace: super::super::Graphics::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
    pub SourceAspectRatio: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub DestinationAspectRatio: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub FrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub SourceSizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub DestinationSizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub EnableOrientation: super::super::Foundation::BOOL,
    pub FilterFlags: D3D12_VIDEO_PROCESS_FILTER_FLAGS,
    pub StereoFormat: D3D12_VIDEO_FRAME_STEREO_FORMAT,
    pub FieldType: D3D12_VIDEO_FIELD_TYPE,
    pub DeinterlaceMode: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS,
    pub EnableAlphaBlending: super::super::Foundation::BOOL,
    pub LumaKey: D3D12_VIDEO_PROCESS_LUMA_KEY,
    pub NumPastFrames: u32,
    pub NumFutureFrames: u32,
    pub EnableAutoProcessing: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE {
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE {}
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_VIDEO_PROCESS_LUMA_KEY {
    pub Enable: super::super::Foundation::BOOL,
    pub Lower: f32,
    pub Upper: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_LUMA_KEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_LUMA_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_PROCESS_OUTPUT_STREAM {
    pub pTexture2D: super::super::Graphics::Direct3D12::ID3D12Resource,
    pub Subresource: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_OUTPUT_STREAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_OUTPUT_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
pub struct D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS {
    pub OutputStream: [D3D12_VIDEO_PROCESS_OUTPUT_STREAM; 2],
    pub TargetRectangle: super::super::Foundation::RECT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
    pub Format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub ColorSpace: super::super::Graphics::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
    pub AlphaFillMode: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE,
    pub AlphaFillModeSourceStreamIndex: u32,
    pub BackgroundColor: [f32; 4],
    pub FrameRate: super::super::Graphics::Dxgi::Common::DXGI_RATIONAL,
    pub EnableStereo: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct D3D12_VIDEO_PROCESS_REFERENCE_SET {
    pub NumPastFrames: u32,
    pub ppPastFrames: *mut super::super::Graphics::Direct3D12::ID3D12Resource,
    pub pPastSubresources: *mut u32,
    pub NumFutureFrames: u32,
    pub ppFutureFrames: *mut super::super::Graphics::Direct3D12::ID3D12Resource,
    pub pFutureSubresources: *mut u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_REFERENCE_SET {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_REFERENCE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_VIDEO_PROCESS_TRANSFORM {
    pub SourceRectangle: super::super::Foundation::RECT,
    pub DestinationRectangle: super::super::Foundation::RECT,
    pub Orientation: D3D12_VIDEO_PROCESS_ORIENTATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VIDEO_PROCESS_TRANSFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VIDEO_PROCESS_TRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_VIDEO_SAMPLE {
    pub Width: u32,
    pub Height: u32,
    pub Format: D3D12_VIDEO_FORMAT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_VIDEO_SAMPLE {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_VIDEO_SAMPLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_SCALE_SUPPORT {
    pub OutputSizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub Flags: D3D12_VIDEO_SCALE_SUPPORT_FLAGS,
}
impl ::core::marker::Copy for D3D12_VIDEO_SCALE_SUPPORT {}
impl ::core::clone::Clone for D3D12_VIDEO_SCALE_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3D12_VIDEO_SIZE_RANGE {
    pub MaxWidth: u32,
    pub MaxHeight: u32,
    pub MinWidth: u32,
    pub MinHeight: u32,
}
impl ::core::marker::Copy for D3D12_VIDEO_SIZE_RANGE {}
impl ::core::clone::Clone for D3D12_VIDEO_SIZE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct D3DCONTENTPROTECTIONCAPS {
    pub Caps: u32,
    pub KeyExchangeType: ::windows_sys::core::GUID,
    pub BufferAlignmentStart: u32,
    pub BlockAlignmentSize: u32,
    pub ProtectedMemorySize: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for D3DCONTENTPROTECTIONCAPS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for D3DCONTENTPROTECTIONCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
#[cfg(target_arch = "x86")]
pub struct D3DCONTENTPROTECTIONCAPS {
    pub Caps: u32,
    pub KeyExchangeType: ::windows_sys::core::GUID,
    pub BufferAlignmentStart: u32,
    pub BlockAlignmentSize: u32,
    pub ProtectedMemorySize: u64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for D3DCONTENTPROTECTIONCAPS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for D3DCONTENTPROTECTIONCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct D3DOVERLAYCAPS {
    pub Caps: u32,
    pub MaxOverlayDisplayWidth: u32,
    pub MaxOverlayDisplayHeight: u32,
}
impl ::core::marker::Copy for D3DOVERLAYCAPS {}
impl ::core::clone::Clone for D3DOVERLAYCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DEVICE_INFO {
    pub pFriendlyDeviceName: ::windows_sys::core::BSTR,
    pub pUniqueDeviceName: ::windows_sys::core::BSTR,
    pub pManufacturerName: ::windows_sys::core::BSTR,
    pub pModelName: ::windows_sys::core::BSTR,
    pub pIconURL: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for DEVICE_INFO {}
impl ::core::clone::Clone for DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIRTYRECT_INFO {
    pub FrameNumber: u32,
    pub NumDirtyRects: u32,
    pub DirtyRects: [super::super::Foundation::RECT; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIRTYRECT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIRTYRECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_AES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
impl ::core::marker::Copy for DXVA2_AES_CTR_IV {}
impl ::core::clone::Clone for DXVA2_AES_CTR_IV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_AYUVSample16 {
    pub Cr: u16,
    pub Cb: u16,
    pub Y: u16,
    pub Alpha: u16,
}
impl ::core::marker::Copy for DXVA2_AYUVSample16 {}
impl ::core::clone::Clone for DXVA2_AYUVSample16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_AYUVSample8 {
    pub Cr: u8,
    pub Cb: u8,
    pub Y: u8,
    pub Alpha: u8,
}
impl ::core::marker::Copy for DXVA2_AYUVSample8 {}
impl ::core::clone::Clone for DXVA2_AYUVSample8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_ConfigPictureDecode {
    pub guidConfigBitstreamEncryption: ::windows_sys::core::GUID,
    pub guidConfigMBcontrolEncryption: ::windows_sys::core::GUID,
    pub guidConfigResidDiffEncryption: ::windows_sys::core::GUID,
    pub ConfigBitstreamRaw: u32,
    pub ConfigMBcontrolRasterOrder: u32,
    pub ConfigResidDiffHost: u32,
    pub ConfigSpatialResid8: u32,
    pub ConfigResid8Subtraction: u32,
    pub ConfigSpatialHost8or9Clipping: u32,
    pub ConfigSpatialResidInterleaved: u32,
    pub ConfigIntraResidUnsigned: u32,
    pub ConfigResidDiffAccelerator: u32,
    pub ConfigHostInverseScan: u32,
    pub ConfigSpecificIDCT: u32,
    pub Config4GroupedCoefs: u32,
    pub ConfigMinRenderTargetBuffCount: u16,
    pub ConfigDecoderSpecific: u16,
}
impl ::core::marker::Copy for DXVA2_ConfigPictureDecode {}
impl ::core::clone::Clone for DXVA2_ConfigPictureDecode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_DecodeBufferDesc {
    pub CompressedBufferType: DXVA2_BufferfType,
    pub BufferIndex: u32,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub FirstMBaddress: u32,
    pub NumMBsInBuffer: u32,
    pub Width: u32,
    pub Height: u32,
    pub Stride: u32,
    pub ReservedBits: u32,
    pub pvPVPState: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DXVA2_DecodeBufferDesc {}
impl ::core::clone::Clone for DXVA2_DecodeBufferDesc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_DecodeExecuteParams {
    pub NumCompBuffers: u32,
    pub pCompressedBuffers: *mut DXVA2_DecodeBufferDesc,
    pub pExtensionData: *mut DXVA2_DecodeExtensionData,
}
impl ::core::marker::Copy for DXVA2_DecodeExecuteParams {}
impl ::core::clone::Clone for DXVA2_DecodeExecuteParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_DecodeExtensionData {
    pub Function: u32,
    pub pPrivateInputData: *mut ::core::ffi::c_void,
    pub PrivateInputDataSize: u32,
    pub pPrivateOutputData: *mut ::core::ffi::c_void,
    pub PrivateOutputDataSize: u32,
}
impl ::core::marker::Copy for DXVA2_DecodeExtensionData {}
impl ::core::clone::Clone for DXVA2_DecodeExtensionData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_ExtendedFormat {
    pub Anonymous: DXVA2_ExtendedFormat_0,
}
impl ::core::marker::Copy for DXVA2_ExtendedFormat {}
impl ::core::clone::Clone for DXVA2_ExtendedFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union DXVA2_ExtendedFormat_0 {
    pub Anonymous: DXVA2_ExtendedFormat_0_0,
    pub value: u32,
}
impl ::core::marker::Copy for DXVA2_ExtendedFormat_0 {}
impl ::core::clone::Clone for DXVA2_ExtendedFormat_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_ExtendedFormat_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DXVA2_ExtendedFormat_0_0 {}
impl ::core::clone::Clone for DXVA2_ExtendedFormat_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_FilterValues {
    pub Level: DXVA2_Fixed32,
    pub Threshold: DXVA2_Fixed32,
    pub Radius: DXVA2_Fixed32,
}
impl ::core::marker::Copy for DXVA2_FilterValues {}
impl ::core::clone::Clone for DXVA2_FilterValues {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_Fixed32 {
    pub Anonymous: DXVA2_Fixed32_0,
}
impl ::core::marker::Copy for DXVA2_Fixed32 {}
impl ::core::clone::Clone for DXVA2_Fixed32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union DXVA2_Fixed32_0 {
    pub Anonymous: DXVA2_Fixed32_0_0,
    pub ll: i32,
}
impl ::core::marker::Copy for DXVA2_Fixed32_0 {}
impl ::core::clone::Clone for DXVA2_Fixed32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_Fixed32_0_0 {
    pub Fraction: u16,
    pub Value: i16,
}
impl ::core::marker::Copy for DXVA2_Fixed32_0_0 {}
impl ::core::clone::Clone for DXVA2_Fixed32_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_Frequency {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl ::core::marker::Copy for DXVA2_Frequency {}
impl ::core::clone::Clone for DXVA2_Frequency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_ProcAmpValues {
    pub Brightness: DXVA2_Fixed32,
    pub Contrast: DXVA2_Fixed32,
    pub Hue: DXVA2_Fixed32,
    pub Saturation: DXVA2_Fixed32,
}
impl ::core::marker::Copy for DXVA2_ProcAmpValues {}
impl ::core::clone::Clone for DXVA2_ProcAmpValues {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA2_ValueRange {
    pub MinValue: DXVA2_Fixed32,
    pub MaxValue: DXVA2_Fixed32,
    pub DefaultValue: DXVA2_Fixed32,
    pub StepSize: DXVA2_Fixed32,
}
impl ::core::marker::Copy for DXVA2_ValueRange {}
impl ::core::clone::Clone for DXVA2_ValueRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVA2_VideoDesc {
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub SampleFormat: DXVA2_ExtendedFormat,
    pub Format: super::super::Graphics::Direct3D9::D3DFORMAT,
    pub InputSampleFreq: DXVA2_Frequency,
    pub OutputFrameFreq: DXVA2_Frequency,
    pub UABProtectionLevel: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVA2_VideoDesc {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVA2_VideoDesc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVA2_VideoProcessBltParams {
    pub TargetFrame: i64,
    pub TargetRect: super::super::Foundation::RECT,
    pub ConstrictionSize: super::super::Foundation::SIZE,
    pub StreamingFlags: u32,
    pub BackgroundColor: DXVA2_AYUVSample16,
    pub DestFormat: DXVA2_ExtendedFormat,
    pub ProcAmpValues: DXVA2_ProcAmpValues,
    pub Alpha: DXVA2_Fixed32,
    pub NoiseFilterLuma: DXVA2_FilterValues,
    pub NoiseFilterChroma: DXVA2_FilterValues,
    pub DetailFilterLuma: DXVA2_FilterValues,
    pub DetailFilterChroma: DXVA2_FilterValues,
    pub DestData: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVA2_VideoProcessBltParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVA2_VideoProcessBltParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVA2_VideoProcessorCaps {
    pub DeviceCaps: u32,
    pub InputPool: super::super::Graphics::Direct3D9::D3DPOOL,
    pub NumForwardRefSamples: u32,
    pub NumBackwardRefSamples: u32,
    pub Reserved: u32,
    pub DeinterlaceTechnology: u32,
    pub ProcAmpControlCaps: u32,
    pub VideoProcessorOperations: u32,
    pub NoiseFilterTechnology: u32,
    pub DetailFilterTechnology: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVA2_VideoProcessorCaps {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVA2_VideoProcessorCaps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub struct DXVA2_VideoSample {
    pub Start: i64,
    pub End: i64,
    pub SampleFormat: DXVA2_ExtendedFormat,
    pub SrcSurface: super::super::Graphics::Direct3D9::IDirect3DSurface9,
    pub SrcRect: super::super::Foundation::RECT,
    pub DstRect: super::super::Foundation::RECT,
    pub Pal: [DXVA2_AYUVSample8; 16],
    pub PlanarAlpha: DXVA2_Fixed32,
    pub SampleData: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::marker::Copy for DXVA2_VideoSample {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::clone::Clone for DXVA2_VideoSample {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVABufferInfo {
    pub pCompSurface: *mut ::core::ffi::c_void,
    pub DataOffset: u32,
    pub DataSize: u32,
}
impl ::core::marker::Copy for DXVABufferInfo {}
impl ::core::clone::Clone for DXVABufferInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVACompBufferInfo {
    pub NumCompBuffers: u32,
    pub WidthToCreate: u32,
    pub HeightToCreate: u32,
    pub BytesToAllocate: u32,
    pub Usage: u32,
    pub Pool: super::super::Graphics::Direct3D9::D3DPOOL,
    pub Format: super::super::Graphics::Direct3D9::D3DFORMAT,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVACompBufferInfo {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVACompBufferInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHDETW_CREATEVIDEOPROCESSOR {
    pub pObject: u64,
    pub pD3D9Ex: u64,
    pub VPGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DXVAHDETW_CREATEVIDEOPROCESSOR {}
impl ::core::clone::Clone for DXVAHDETW_CREATEVIDEOPROCESSOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHDETW_DESTROYVIDEOPROCESSOR {
    pub pObject: u64,
}
impl ::core::marker::Copy for DXVAHDETW_DESTROYVIDEOPROCESSOR {}
impl ::core::clone::Clone for DXVAHDETW_DESTROYVIDEOPROCESSOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub struct DXVAHDETW_VIDEOPROCESSBLTHD {
    pub pObject: u64,
    pub pOutputSurface: u64,
    pub TargetRect: super::super::Foundation::RECT,
    pub OutputFormat: super::super::Graphics::Direct3D9::D3DFORMAT,
    pub ColorSpace: u32,
    pub OutputFrame: u32,
    pub StreamCount: u32,
    pub Enter: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::marker::Copy for DXVAHDETW_VIDEOPROCESSBLTHD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::clone::Clone for DXVAHDETW_VIDEOPROCESSBLTHD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub struct DXVAHDETW_VIDEOPROCESSBLTHD_STREAM {
    pub pObject: u64,
    pub pInputSurface: u64,
    pub SourceRect: super::super::Foundation::RECT,
    pub DestinationRect: super::super::Foundation::RECT,
    pub InputFormat: super::super::Graphics::Direct3D9::D3DFORMAT,
    pub FrameFormat: DXVAHD_FRAME_FORMAT,
    pub ColorSpace: u32,
    pub StreamNumber: u32,
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
    pub PastFrames: u32,
    pub FutureFrames: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::marker::Copy for DXVAHDETW_VIDEOPROCESSBLTHD_STREAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::clone::Clone for DXVAHDETW_VIDEOPROCESSBLTHD_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHDETW_VIDEOPROCESSBLTSTATE {
    pub pObject: u64,
    pub State: DXVAHD_BLT_STATE,
    pub DataSize: u32,
    pub SetState: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHDETW_VIDEOPROCESSBLTSTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHDETW_VIDEOPROCESSBLTSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHDETW_VIDEOPROCESSSTREAMSTATE {
    pub pObject: u64,
    pub StreamNumber: u32,
    pub State: DXVAHD_STREAM_STATE,
    pub DataSize: u32,
    pub SetState: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHDETW_VIDEOPROCESSSTREAMSTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHDETW_VIDEOPROCESSSTREAMSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub struct DXVAHDSW_CALLBACKS {
    pub CreateDevice: PDXVAHDSW_CreateDevice,
    pub ProposeVideoPrivateFormat: PDXVAHDSW_ProposeVideoPrivateFormat,
    pub GetVideoProcessorDeviceCaps: PDXVAHDSW_GetVideoProcessorDeviceCaps,
    pub GetVideoProcessorOutputFormats: PDXVAHDSW_GetVideoProcessorOutputFormats,
    pub GetVideoProcessorInputFormats: PDXVAHDSW_GetVideoProcessorInputFormats,
    pub GetVideoProcessorCaps: PDXVAHDSW_GetVideoProcessorCaps,
    pub GetVideoProcessorCustomRates: PDXVAHDSW_GetVideoProcessorCustomRates,
    pub GetVideoProcessorFilterRange: PDXVAHDSW_GetVideoProcessorFilterRange,
    pub DestroyDevice: PDXVAHDSW_DestroyDevice,
    pub CreateVideoProcessor: PDXVAHDSW_CreateVideoProcessor,
    pub SetVideoProcessBltState: PDXVAHDSW_SetVideoProcessBltState,
    pub GetVideoProcessBltStatePrivate: PDXVAHDSW_GetVideoProcessBltStatePrivate,
    pub SetVideoProcessStreamState: PDXVAHDSW_SetVideoProcessStreamState,
    pub GetVideoProcessStreamStatePrivate: PDXVAHDSW_GetVideoProcessStreamStatePrivate,
    pub VideoProcessBltHD: PDXVAHDSW_VideoProcessBltHD,
    pub DestroyVideoProcessor: PDXVAHDSW_DestroyVideoProcessor,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::marker::Copy for DXVAHDSW_CALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::clone::Clone for DXVAHDSW_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_BLT_STATE_ALPHA_FILL_DATA {
    pub Mode: DXVAHD_ALPHA_FILL_MODE,
    pub StreamNumber: u32,
}
impl ::core::marker::Copy for DXVAHD_BLT_STATE_ALPHA_FILL_DATA {}
impl ::core::clone::Clone for DXVAHD_BLT_STATE_ALPHA_FILL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_BLT_STATE_BACKGROUND_COLOR_DATA {
    pub YCbCr: super::super::Foundation::BOOL,
    pub BackgroundColor: DXVAHD_COLOR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_BLT_STATE_BACKGROUND_COLOR_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_BLT_STATE_BACKGROUND_COLOR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_BLT_STATE_CONSTRICTION_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub Size: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_BLT_STATE_CONSTRICTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_BLT_STATE_CONSTRICTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA {
    pub Anonymous: DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA_0,
}
impl ::core::marker::Copy for DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA {}
impl ::core::clone::Clone for DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA_0 {
    pub Anonymous: DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA_0_0,
    pub Value: u32,
}
impl ::core::marker::Copy for DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA_0 {}
impl ::core::clone::Clone for DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA_0_0 {}
impl ::core::clone::Clone for DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_BLT_STATE_PRIVATE_DATA {
    pub Guid: ::windows_sys::core::GUID,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DXVAHD_BLT_STATE_PRIVATE_DATA {}
impl ::core::clone::Clone for DXVAHD_BLT_STATE_PRIVATE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_BLT_STATE_TARGET_RECT_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub TargetRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_BLT_STATE_TARGET_RECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_BLT_STATE_TARGET_RECT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union DXVAHD_COLOR {
    pub RGB: DXVAHD_COLOR_RGBA,
    pub YCbCr: DXVAHD_COLOR_YCbCrA,
}
impl ::core::marker::Copy for DXVAHD_COLOR {}
impl ::core::clone::Clone for DXVAHD_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_COLOR_RGBA {
    pub R: f32,
    pub G: f32,
    pub B: f32,
    pub A: f32,
}
impl ::core::marker::Copy for DXVAHD_COLOR_RGBA {}
impl ::core::clone::Clone for DXVAHD_COLOR_RGBA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_COLOR_YCbCrA {
    pub Y: f32,
    pub Cb: f32,
    pub Cr: f32,
    pub A: f32,
}
impl ::core::marker::Copy for DXVAHD_COLOR_YCbCrA {}
impl ::core::clone::Clone for DXVAHD_COLOR_YCbCrA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_CONTENT_DESC {
    pub InputFrameFormat: DXVAHD_FRAME_FORMAT,
    pub InputFrameRate: DXVAHD_RATIONAL,
    pub InputWidth: u32,
    pub InputHeight: u32,
    pub OutputFrameRate: DXVAHD_RATIONAL,
    pub OutputWidth: u32,
    pub OutputHeight: u32,
}
impl ::core::marker::Copy for DXVAHD_CONTENT_DESC {}
impl ::core::clone::Clone for DXVAHD_CONTENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_CUSTOM_RATE_DATA {
    pub CustomRate: DXVAHD_RATIONAL,
    pub OutputFrames: u32,
    pub InputInterlaced: super::super::Foundation::BOOL,
    pub InputFramesOrFields: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_CUSTOM_RATE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_CUSTOM_RATE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_FILTER_RANGE_DATA {
    pub Minimum: i32,
    pub Maximum: i32,
    pub Default: i32,
    pub Multiplier: f32,
}
impl ::core::marker::Copy for DXVAHD_FILTER_RANGE_DATA {}
impl ::core::clone::Clone for DXVAHD_FILTER_RANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl ::core::marker::Copy for DXVAHD_RATIONAL {}
impl ::core::clone::Clone for DXVAHD_RATIONAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub struct DXVAHD_STREAM_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ppPastSurfaces: *mut super::super::Graphics::Direct3D9::IDirect3DSurface9,
    pub pInputSurface: super::super::Graphics::Direct3D9::IDirect3DSurface9,
    pub ppFutureSurfaces: *mut super::super::Graphics::Direct3D9::IDirect3DSurface9,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::marker::Copy for DXVAHD_STREAM_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::clone::Clone for DXVAHD_STREAM_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_STREAM_STATE_ALPHA_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub Alpha: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_ALPHA_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_ALPHA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub SourceAspectRatio: DXVAHD_RATIONAL,
    pub DestinationAspectRatio: DXVAHD_RATIONAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVAHD_STREAM_STATE_D3DFORMAT_DATA {
    pub Format: super::super::Graphics::Direct3D9::D3DFORMAT,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_D3DFORMAT_DATA {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_D3DFORMAT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub DestinationRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_STREAM_STATE_FILTER_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub Level: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_FILTER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_FILTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA {
    pub FrameFormat: DXVAHD_FRAME_FORMAT,
}
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA {}
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA {
    pub Anonymous: DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA_0,
}
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA {}
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA_0 {
    pub Anonymous: DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA_0_0,
    pub Value: u32,
}
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA_0 {}
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA_0_0 {}
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_STREAM_STATE_LUMA_KEY_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub Lower: f32,
    pub Upper: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_LUMA_KEY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_LUMA_KEY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA {
    pub RepeatFrame: super::super::Foundation::BOOL,
    pub OutputRate: DXVAHD_OUTPUT_RATE,
    pub CustomRate: DXVAHD_RATIONAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_STREAM_STATE_PALETTE_DATA {
    pub Count: u32,
    pub pEntries: *mut u32,
}
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_PALETTE_DATA {}
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_PALETTE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_STREAM_STATE_PRIVATE_DATA {
    pub Guid: ::windows_sys::core::GUID,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_PRIVATE_DATA {}
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_PRIVATE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub ITelecineFlags: u32,
    pub Frames: u32,
    pub InputField: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVAHD_STREAM_STATE_SOURCE_RECT_DATA {
    pub Enable: super::super::Foundation::BOOL,
    pub SourceRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVAHD_STREAM_STATE_SOURCE_RECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVAHD_STREAM_STATE_SOURCE_RECT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVAHD_VPCAPS {
    pub VPGuid: ::windows_sys::core::GUID,
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ProcessorCaps: u32,
    pub ITelecineCaps: u32,
    pub CustomRateCount: u32,
}
impl ::core::marker::Copy for DXVAHD_VPCAPS {}
impl ::core::clone::Clone for DXVAHD_VPCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVAHD_VPDEVCAPS {
    pub DeviceType: DXVAHD_DEVICE_TYPE,
    pub DeviceCaps: u32,
    pub FeatureCaps: u32,
    pub FilterCaps: u32,
    pub InputFormatCaps: u32,
    pub InputPool: super::super::Graphics::Direct3D9::D3DPOOL,
    pub OutputFormatCount: u32,
    pub InputFormatCount: u32,
    pub VideoProcessorCount: u32,
    pub MaxInputStreams: u32,
    pub MaxStreamStates: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVAHD_VPDEVCAPS {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVAHD_VPDEVCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVAUncompDataInfo {
    pub UncompWidth: u32,
    pub UncompHeight: u32,
    pub UncompFormat: super::super::Graphics::Direct3D9::D3DFORMAT,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVAUncompDataInfo {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVAUncompDataInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_AYUVsample2 {
    pub bCrValue: u8,
    pub bCbValue: u8,
    pub bY_Value: u8,
    pub bSampleAlpha8: u8,
}
impl ::core::marker::Copy for DXVA_AYUVsample2 {}
impl ::core::clone::Clone for DXVA_AYUVsample2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_BufferDescription {
    pub dwTypeIndex: u32,
    pub dwBufferIndex: u32,
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
    pub dwFirstMBaddress: u32,
    pub dwNumMBsInBuffer: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwStride: u32,
    pub dwReservedBits: u32,
}
impl ::core::marker::Copy for DXVA_BufferDescription {}
impl ::core::clone::Clone for DXVA_BufferDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_COPPCommand {
    pub macKDI: ::windows_sys::core::GUID,
    pub guidCommandID: ::windows_sys::core::GUID,
    pub dwSequence: u32,
    pub cbSizeData: u32,
    pub CommandData: [u8; 4056],
}
impl ::core::marker::Copy for DXVA_COPPCommand {}
impl ::core::clone::Clone for DXVA_COPPCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_COPPSignature {
    pub Signature: [u8; 256],
}
impl ::core::marker::Copy for DXVA_COPPSignature {}
impl ::core::clone::Clone for DXVA_COPPSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_COPPStatusInput {
    pub rApp: ::windows_sys::core::GUID,
    pub guidStatusRequestID: ::windows_sys::core::GUID,
    pub dwSequence: u32,
    pub cbSizeData: u32,
    pub StatusData: [u8; 4056],
}
impl ::core::marker::Copy for DXVA_COPPStatusInput {}
impl ::core::clone::Clone for DXVA_COPPStatusInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_COPPStatusOutput {
    pub macKDI: ::windows_sys::core::GUID,
    pub cbSizeData: u32,
    pub COPPStatus: [u8; 4076],
}
impl ::core::marker::Copy for DXVA_COPPStatusOutput {}
impl ::core::clone::Clone for DXVA_COPPStatusOutput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_ConfigPictureDecode {
    pub dwFunction: u32,
    pub dwReservedBits: [u32; 3],
    pub guidConfigBitstreamEncryption: ::windows_sys::core::GUID,
    pub guidConfigMBcontrolEncryption: ::windows_sys::core::GUID,
    pub guidConfigResidDiffEncryption: ::windows_sys::core::GUID,
    pub bConfigBitstreamRaw: u8,
    pub bConfigMBcontrolRasterOrder: u8,
    pub bConfigResidDiffHost: u8,
    pub bConfigSpatialResid8: u8,
    pub bConfigResid8Subtraction: u8,
    pub bConfigSpatialHost8or9Clipping: u8,
    pub bConfigSpatialResidInterleaved: u8,
    pub bConfigIntraResidUnsigned: u8,
    pub bConfigResidDiffAccelerator: u8,
    pub bConfigHostInverseScan: u8,
    pub bConfigSpecificIDCT: u8,
    pub bConfig4GroupedCoefs: u8,
}
impl ::core::marker::Copy for DXVA_ConfigPictureDecode {}
impl ::core::clone::Clone for DXVA_ConfigPictureDecode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVA_DeinterlaceBlt {
    pub Size: u32,
    pub Reserved: u32,
    pub rtTarget: i64,
    pub DstRect: super::super::Foundation::RECT,
    pub SrcRect: super::super::Foundation::RECT,
    pub NumSourceSurfaces: u32,
    pub Alpha: f32,
    pub Source: [DXVA_VideoSample; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVA_DeinterlaceBlt {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVA_DeinterlaceBlt {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVA_DeinterlaceBltEx {
    pub Size: u32,
    pub BackgroundColor: DXVA_AYUVsample2,
    pub rcTarget: super::super::Foundation::RECT,
    pub rtTarget: i64,
    pub NumSourceSurfaces: u32,
    pub Alpha: f32,
    pub Source: [DXVA_VideoSample2; 32],
    pub DestinationFormat: u32,
    pub DestinationFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVA_DeinterlaceBltEx {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVA_DeinterlaceBltEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVA_DeinterlaceBltEx32 {
    pub Size: u32,
    pub BackgroundColor: DXVA_AYUVsample2,
    pub rcTarget: super::super::Foundation::RECT,
    pub rtTarget: i64,
    pub NumSourceSurfaces: u32,
    pub Alpha: f32,
    pub Source: [DXVA_VideoSample32; 32],
    pub DestinationFormat: u32,
    pub DestinationFlags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVA_DeinterlaceBltEx32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVA_DeinterlaceBltEx32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVA_DeinterlaceCaps {
    pub Size: u32,
    pub NumPreviousOutputFrames: u32,
    pub InputPool: u32,
    pub NumForwardRefSamples: u32,
    pub NumBackwardRefSamples: u32,
    pub d3dOutputFormat: super::super::Graphics::Direct3D9::D3DFORMAT,
    pub VideoProcessingCaps: DXVA_VideoProcessCaps,
    pub DeinterlaceTechnology: DXVA_DeinterlaceTech,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVA_DeinterlaceCaps {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVA_DeinterlaceCaps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_DeinterlaceQueryAvailableModes {
    pub Size: u32,
    pub NumGuids: u32,
    pub Guids: [::windows_sys::core::GUID; 32],
}
impl ::core::marker::Copy for DXVA_DeinterlaceQueryAvailableModes {}
impl ::core::clone::Clone for DXVA_DeinterlaceQueryAvailableModes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVA_DeinterlaceQueryModeCaps {
    pub Size: u32,
    pub Guid: ::windows_sys::core::GUID,
    pub VideoDesc: DXVA_VideoDesc,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVA_DeinterlaceQueryModeCaps {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVA_DeinterlaceQueryModeCaps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_ExtendedFormat {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DXVA_ExtendedFormat {}
impl ::core::clone::Clone for DXVA_ExtendedFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_Frequency {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl ::core::marker::Copy for DXVA_Frequency {}
impl ::core::clone::Clone for DXVA_Frequency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_PictureParameters {
    pub wDecodedPictureIndex: u16,
    pub wDeblockedPictureIndex: u16,
    pub wForwardRefPictureIndex: u16,
    pub wBackwardRefPictureIndex: u16,
    pub wPicWidthInMBminus1: u16,
    pub wPicHeightInMBminus1: u16,
    pub bMacroblockWidthMinus1: u8,
    pub bMacroblockHeightMinus1: u8,
    pub bBlockWidthMinus1: u8,
    pub bBlockHeightMinus1: u8,
    pub bBPPminus1: u8,
    pub bPicStructure: u8,
    pub bSecondField: u8,
    pub bPicIntra: u8,
    pub bPicBackwardPrediction: u8,
    pub bBidirectionalAveragingMode: u8,
    pub bMVprecisionAndChromaRelation: u8,
    pub bChromaFormat: u8,
    pub bPicScanFixed: u8,
    pub bPicScanMethod: u8,
    pub bPicReadbackRequests: u8,
    pub bRcontrol: u8,
    pub bPicSpatialResid8: u8,
    pub bPicOverflowBlocks: u8,
    pub bPicExtrapolation: u8,
    pub bPicDeblocked: u8,
    pub bPicDeblockConfined: u8,
    pub bPic4MVallowed: u8,
    pub bPicOBMC: u8,
    pub bPicBinPB: u8,
    pub bMV_RPS: u8,
    pub bReservedBits: u8,
    pub wBitstreamFcodes: u16,
    pub wBitstreamPCEelements: u16,
    pub bBitstreamConcealmentNeed: u8,
    pub bBitstreamConcealmentMethod: u8,
}
impl ::core::marker::Copy for DXVA_PictureParameters {}
impl ::core::clone::Clone for DXVA_PictureParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVA_ProcAmpControlBlt {
    pub Size: u32,
    pub DstRect: super::super::Foundation::RECT,
    pub SrcRect: super::super::Foundation::RECT,
    pub Alpha: f32,
    pub Brightness: f32,
    pub Contrast: f32,
    pub Hue: f32,
    pub Saturation: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVA_ProcAmpControlBlt {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVA_ProcAmpControlBlt {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVA_ProcAmpControlCaps {
    pub Size: u32,
    pub InputPool: u32,
    pub d3dOutputFormat: super::super::Graphics::Direct3D9::D3DFORMAT,
    pub ProcAmpControlProps: u32,
    pub VideoProcessingCaps: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVA_ProcAmpControlCaps {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVA_ProcAmpControlCaps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVA_ProcAmpControlQueryRange {
    pub Size: u32,
    pub ProcAmpControlProp: DXVA_ProcAmpControlProp,
    pub VideoDesc: DXVA_VideoDesc,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVA_ProcAmpControlQueryRange {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVA_ProcAmpControlQueryRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct DXVA_VideoDesc {
    pub Size: u32,
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub SampleFormat: u32,
    pub d3dFormat: super::super::Graphics::Direct3D9::D3DFORMAT,
    pub InputSampleFreq: DXVA_Frequency,
    pub OutputFrameFreq: DXVA_Frequency,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for DXVA_VideoDesc {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for DXVA_VideoDesc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_VideoPropertyRange {
    pub MinValue: f32,
    pub MaxValue: f32,
    pub DefaultValue: f32,
    pub StepSize: f32,
}
impl ::core::marker::Copy for DXVA_VideoPropertyRange {}
impl ::core::clone::Clone for DXVA_VideoPropertyRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DXVA_VideoSample {
    pub rtStart: i64,
    pub rtEnd: i64,
    pub SampleFormat: DXVA_SampleFormat,
    pub lpDDSSrcSurface: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DXVA_VideoSample {}
impl ::core::clone::Clone for DXVA_VideoSample {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVA_VideoSample2 {
    pub Size: u32,
    pub Reserved: u32,
    pub rtStart: i64,
    pub rtEnd: i64,
    pub SampleFormat: u32,
    pub SampleFlags: u32,
    pub lpDDSSrcSurface: *mut ::core::ffi::c_void,
    pub rcSrc: super::super::Foundation::RECT,
    pub rcDst: super::super::Foundation::RECT,
    pub Palette: [DXVA_AYUVsample2; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVA_VideoSample2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVA_VideoSample2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVA_VideoSample2 {
    pub rtStart: i64,
    pub rtEnd: i64,
    pub SampleFormat: u32,
    pub SampleFlags: u32,
    pub lpDDSSrcSurface: *mut ::core::ffi::c_void,
    pub rcSrc: super::super::Foundation::RECT,
    pub rcDst: super::super::Foundation::RECT,
    pub Palette: [DXVA_AYUVsample2; 16],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVA_VideoSample2 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVA_VideoSample2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct DXVA_VideoSample32 {
    pub rtStart: i64,
    pub rtEnd: i64,
    pub SampleFormat: u32,
    pub SampleFlags: u32,
    pub lpDDSSrcSurface: u32,
    pub rcSrc: super::super::Foundation::RECT,
    pub rcDst: super::super::Foundation::RECT,
    pub Palette: [DXVA_AYUVsample2; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXVA_VideoSample32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXVA_VideoSample32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct DigitalWindowSetting {
    pub OriginX: f64,
    pub OriginY: f64,
    pub WindowSize: f64,
}
impl ::core::marker::Copy for DigitalWindowSetting {}
impl ::core::clone::Clone for DigitalWindowSetting {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MACROBLOCK_DATA {
    pub flags: u32,
    pub motionVectorX: i16,
    pub motionVectorY: i16,
    pub QPDelta: i32,
}
impl ::core::marker::Copy for MACROBLOCK_DATA {}
impl ::core::clone::Clone for MACROBLOCK_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFARGB {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbAlpha: u8,
}
impl ::core::marker::Copy for MFARGB {}
impl ::core::clone::Clone for MFARGB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFAYUVSample {
    pub bCrValue: u8,
    pub bCbValue: u8,
    pub bYValue: u8,
    pub bSampleAlpha8: u8,
}
impl ::core::marker::Copy for MFAYUVSample {}
impl ::core::clone::Clone for MFAYUVSample {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFAudioDecoderDegradationInfo {
    pub eDegradationReason: MFT_AUDIO_DECODER_DEGRADATION_REASON,
    pub eType: MFT_AUDIO_DECODER_DEGRADATION_TYPE,
}
impl ::core::marker::Copy for MFAudioDecoderDegradationInfo {}
impl ::core::clone::Clone for MFAudioDecoderDegradationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFBYTESTREAM_BUFFERING_PARAMS {
    pub cbTotalFileSize: u64,
    pub cbPlayableDataSize: u64,
    pub prgBuckets: *mut MF_LEAKY_BUCKET_PAIR,
    pub cBuckets: u32,
    pub qwNetBufferingTime: u64,
    pub qwExtraBufferingTimeDuringSeek: u64,
    pub qwPlayDuration: u64,
    pub dRate: f32,
}
impl ::core::marker::Copy for MFBYTESTREAM_BUFFERING_PARAMS {}
impl ::core::clone::Clone for MFBYTESTREAM_BUFFERING_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCLOCK_PROPERTIES {
    pub qwCorrelationRate: u64,
    pub guidClockId: ::windows_sys::core::GUID,
    pub dwClockFlags: u32,
    pub qwClockFrequency: u64,
    pub dwClockTolerance: u32,
    pub dwClockJitter: u32,
}
impl ::core::marker::Copy for MFCLOCK_PROPERTIES {}
impl ::core::clone::Clone for MFCLOCK_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    pub HWProtectionFunctionID: u32,
    pub PrivateDataByteCount: u32,
    pub HWProtectionDataByteCount: u32,
    pub Reserved: u32,
    pub InputData: [u8; 4],
}
impl ::core::marker::Copy for MFCONTENTPROTECTIONDEVICE_INPUT_DATA {}
impl ::core::clone::Clone for MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    pub PrivateDataByteCount: u32,
    pub MaxHWProtectionDataByteCount: u32,
    pub HWProtectionDataByteCount: u32,
    pub Status: ::windows_sys::core::HRESULT,
    pub TransportTimeInHundredsOfNanoseconds: i64,
    pub ExecutionTimeInHundredsOfNanoseconds: i64,
    pub OutputData: [u8; 4],
}
impl ::core::marker::Copy for MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {}
impl ::core::clone::Clone for MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    pub TaskIndex: u32,
    pub ClassName: [u16; 260],
    pub BasePriority: i32,
}
impl ::core::marker::Copy for MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {}
impl ::core::clone::Clone for MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCameraExtrinsic_CalibratedTransform {
    pub CalibrationId: ::windows_sys::core::GUID,
    pub Position: MF_FLOAT3,
    pub Orientation: MF_QUATERNION,
}
impl ::core::marker::Copy for MFCameraExtrinsic_CalibratedTransform {}
impl ::core::clone::Clone for MFCameraExtrinsic_CalibratedTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCameraExtrinsics {
    pub TransformCount: u32,
    pub CalibratedTransforms: [MFCameraExtrinsic_CalibratedTransform; 1],
}
impl ::core::marker::Copy for MFCameraExtrinsics {}
impl ::core::clone::Clone for MFCameraExtrinsics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCameraIntrinsic_CameraModel {
    pub FocalLength_x: f32,
    pub FocalLength_y: f32,
    pub PrincipalPoint_x: f32,
    pub PrincipalPoint_y: f32,
}
impl ::core::marker::Copy for MFCameraIntrinsic_CameraModel {}
impl ::core::clone::Clone for MFCameraIntrinsic_CameraModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCameraIntrinsic_DistortionModel {
    pub Radial_k1: f32,
    pub Radial_k2: f32,
    pub Radial_k3: f32,
    pub Tangential_p1: f32,
    pub Tangential_p2: f32,
}
impl ::core::marker::Copy for MFCameraIntrinsic_DistortionModel {}
impl ::core::clone::Clone for MFCameraIntrinsic_DistortionModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCameraIntrinsic_DistortionModel6KT {
    pub Radial_k1: f32,
    pub Radial_k2: f32,
    pub Radial_k3: f32,
    pub Radial_k4: f32,
    pub Radial_k5: f32,
    pub Radial_k6: f32,
    pub Tangential_p1: f32,
    pub Tangential_p2: f32,
}
impl ::core::marker::Copy for MFCameraIntrinsic_DistortionModel6KT {}
impl ::core::clone::Clone for MFCameraIntrinsic_DistortionModel6KT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCameraIntrinsic_DistortionModelArcTan {
    pub Radial_k0: f32,
    pub DistortionCenter_x: f32,
    pub DistortionCenter_y: f32,
    pub Tangential_x: f32,
    pub Tangential_y: f32,
}
impl ::core::marker::Copy for MFCameraIntrinsic_DistortionModelArcTan {}
impl ::core::clone::Clone for MFCameraIntrinsic_DistortionModelArcTan {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFCameraIntrinsic_PinholeCameraModel {
    pub FocalLength: MF_FLOAT2,
    pub PrincipalPoint: MF_FLOAT2,
}
impl ::core::marker::Copy for MFCameraIntrinsic_PinholeCameraModel {}
impl ::core::clone::Clone for MFCameraIntrinsic_PinholeCameraModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFExtendedCameraIntrinsic_IntrinsicModel {
    pub Width: u32,
    pub Height: u32,
    pub SplitFrameId: u32,
    pub CameraModel: MFCameraIntrinsic_CameraModel,
}
impl ::core::marker::Copy for MFExtendedCameraIntrinsic_IntrinsicModel {}
impl ::core::clone::Clone for MFExtendedCameraIntrinsic_IntrinsicModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFFOLDDOWN_MATRIX {
    pub cbSize: u32,
    pub cSrcChannels: u32,
    pub cDstChannels: u32,
    pub dwChannelMask: u32,
    pub Coeff: [i32; 64],
}
impl ::core::marker::Copy for MFFOLDDOWN_MATRIX {}
impl ::core::clone::Clone for MFFOLDDOWN_MATRIX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {
    pub Action: MFPOLICYMANAGER_ACTION,
    pub pbTicket: *mut u8,
    pub cbTicket: u32,
}
impl ::core::marker::Copy for MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {}
impl ::core::clone::Clone for MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {
    pub dwSize: u32,
    pub dwVer: u32,
    pub cbSignatureOffset: u32,
    pub cbSignatureSize: u32,
    pub cbExtensionOffset: u32,
    pub cbExtensionSize: u32,
    pub cActions: u32,
    pub rgOutputActions: [MFINPUTTRUSTAUTHORITY_ACCESS_ACTION; 1],
}
impl ::core::marker::Copy for MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {}
impl ::core::clone::Clone for MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MFMPEG2DLNASINKSTATS {
    pub cBytesWritten: u64,
    pub fPAL: super::super::Foundation::BOOL,
    pub fccVideo: u32,
    pub dwVideoWidth: u32,
    pub dwVideoHeight: u32,
    pub cVideoFramesReceived: u64,
    pub cVideoFramesEncoded: u64,
    pub cVideoFramesSkipped: u64,
    pub cBlackVideoFramesEncoded: u64,
    pub cVideoFramesDuplicated: u64,
    pub cAudioSamplesPerSec: u32,
    pub cAudioChannels: u32,
    pub cAudioBytesReceived: u64,
    pub cAudioFramesEncoded: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MFMPEG2DLNASINKSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MFMPEG2DLNASINKSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFMediaKeyStatus {
    pub pbKeyId: *mut u8,
    pub cbKeyId: u32,
    pub eMediaKeyStatus: MF_MEDIAKEY_STATUS,
}
impl ::core::marker::Copy for MFMediaKeyStatus {}
impl ::core::clone::Clone for MFMediaKeyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MFNetCredentialManagerGetParam {
    pub hrOp: ::windows_sys::core::HRESULT,
    pub fAllowLoggedOnUser: super::super::Foundation::BOOL,
    pub fClearTextPackage: super::super::Foundation::BOOL,
    pub pszUrl: ::windows_sys::core::PCWSTR,
    pub pszSite: ::windows_sys::core::PCWSTR,
    pub pszRealm: ::windows_sys::core::PCWSTR,
    pub pszPackage: ::windows_sys::core::PCWSTR,
    pub nRetries: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MFNetCredentialManagerGetParam {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MFNetCredentialManagerGetParam {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFOffset {
    pub fract: u16,
    pub value: i16,
}
impl ::core::marker::Copy for MFOffset {}
impl ::core::clone::Clone for MFOffset {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct MFP_ACQUIRE_USER_CREDENTIAL_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub dwUserData: usize,
    pub fProceedWithAuthentication: super::super::Foundation::BOOL,
    pub hrAuthenticationStatus: ::windows_sys::core::HRESULT,
    pub pwszURL: ::windows_sys::core::PCWSTR,
    pub pwszSite: ::windows_sys::core::PCWSTR,
    pub pwszRealm: ::windows_sys::core::PCWSTR,
    pub pwszPackage: ::windows_sys::core::PCWSTR,
    pub nRetries: i32,
    pub flags: u32,
    pub pCredential: IMFNetCredential,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for MFP_ACQUIRE_USER_CREDENTIAL_EVENT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for MFP_ACQUIRE_USER_CREDENTIAL_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_ERROR_EVENT {
    pub header: MFP_EVENT_HEADER,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_ERROR_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_ERROR_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_EVENT_HEADER {
    pub eEventType: MFP_EVENT_TYPE,
    pub hrEvent: ::windows_sys::core::HRESULT,
    pub pMediaPlayer: IMFPMediaPlayer,
    pub eState: MFP_MEDIAPLAYER_STATE,
    pub pPropertyStore: super::super::UI::Shell::PropertiesSystem::IPropertyStore,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_EVENT_HEADER {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_EVENT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_FRAME_STEP_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_FRAME_STEP_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_FRAME_STEP_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_MEDIAITEM_CLEARED_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_MEDIAITEM_CLEARED_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_MEDIAITEM_CLEARED_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_MEDIAITEM_CREATED_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
    pub dwUserData: usize,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_MEDIAITEM_CREATED_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_MEDIAITEM_CREATED_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_MEDIAITEM_SET_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_MEDIAITEM_SET_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_MEDIAITEM_SET_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_MF_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub MFEventType: u32,
    pub pMFMediaEvent: IMFMediaEvent,
    pub pMediaItem: IMFPMediaItem,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_MF_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_MF_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_PAUSE_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_PAUSE_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_PAUSE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_PLAYBACK_ENDED_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_PLAYBACK_ENDED_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_PLAYBACK_ENDED_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_PLAY_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_PLAY_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_PLAY_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_POSITION_SET_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_POSITION_SET_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_POSITION_SET_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_RATE_SET_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
    pub flRate: f32,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_RATE_SET_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_RATE_SET_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct MFP_STOP_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: IMFPMediaItem,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for MFP_STOP_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for MFP_STOP_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union MFPaletteEntry {
    pub ARGB: MFARGB,
    pub AYCbCr: MFAYUVSample,
}
impl ::core::marker::Copy for MFPaletteEntry {}
impl ::core::clone::Clone for MFPaletteEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFPinholeCameraIntrinsic_IntrinsicModel {
    pub Width: u32,
    pub Height: u32,
    pub CameraModel: MFCameraIntrinsic_PinholeCameraModel,
    pub DistortionModel: MFCameraIntrinsic_DistortionModel,
}
impl ::core::marker::Copy for MFPinholeCameraIntrinsic_IntrinsicModel {}
impl ::core::clone::Clone for MFPinholeCameraIntrinsic_IntrinsicModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFPinholeCameraIntrinsics {
    pub IntrinsicModelCount: u32,
    pub IntrinsicModels: [MFPinholeCameraIntrinsic_IntrinsicModel; 1],
}
impl ::core::marker::Copy for MFPinholeCameraIntrinsics {}
impl ::core::clone::Clone for MFPinholeCameraIntrinsics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFRR_COMPONENTS {
    pub dwRRInfoVersion: u32,
    pub dwRRComponents: u32,
    pub pRRComponents: *mut MFRR_COMPONENT_HASH_INFO,
}
impl ::core::marker::Copy for MFRR_COMPONENTS {}
impl ::core::clone::Clone for MFRR_COMPONENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFRR_COMPONENT_HASH_INFO {
    pub ulReason: u32,
    pub rgHeaderHash: [u16; 43],
    pub rgPublicKeyHash: [u16; 43],
    pub wszName: [u16; 260],
}
impl ::core::marker::Copy for MFRR_COMPONENT_HASH_INFO {}
impl ::core::clone::Clone for MFRR_COMPONENT_HASH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFRatio {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl ::core::marker::Copy for MFRatio {}
impl ::core::clone::Clone for MFRatio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFTOPONODE_ATTRIBUTE_UPDATE {
    pub NodeId: u64,
    pub guidAttributeKey: ::windows_sys::core::GUID,
    pub attrType: MF_ATTRIBUTE_TYPE,
    pub Anonymous: MFTOPONODE_ATTRIBUTE_UPDATE_0,
}
impl ::core::marker::Copy for MFTOPONODE_ATTRIBUTE_UPDATE {}
impl ::core::clone::Clone for MFTOPONODE_ATTRIBUTE_UPDATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub union MFTOPONODE_ATTRIBUTE_UPDATE_0 {
    pub u32: u32,
    pub u64: u64,
    pub d: f64,
}
impl ::core::marker::Copy for MFTOPONODE_ATTRIBUTE_UPDATE_0 {}
impl ::core::clone::Clone for MFTOPONODE_ATTRIBUTE_UPDATE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFT_INPUT_STREAM_INFO {
    pub hnsMaxLatency: i64,
    pub dwFlags: u32,
    pub cbSize: u32,
    pub cbMaxLookahead: u32,
    pub cbAlignment: u32,
}
impl ::core::marker::Copy for MFT_INPUT_STREAM_INFO {}
impl ::core::clone::Clone for MFT_INPUT_STREAM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFT_OUTPUT_DATA_BUFFER {
    pub dwStreamID: u32,
    pub pSample: IMFSample,
    pub dwStatus: u32,
    pub pEvents: IMFCollection,
}
impl ::core::marker::Copy for MFT_OUTPUT_DATA_BUFFER {}
impl ::core::clone::Clone for MFT_OUTPUT_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFT_OUTPUT_STREAM_INFO {
    pub dwFlags: u32,
    pub cbSize: u32,
    pub cbAlignment: u32,
}
impl ::core::marker::Copy for MFT_OUTPUT_STREAM_INFO {}
impl ::core::clone::Clone for MFT_OUTPUT_STREAM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFT_REGISTER_TYPE_INFO {
    pub guidMajorType: ::windows_sys::core::GUID,
    pub guidSubtype: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for MFT_REGISTER_TYPE_INFO {}
impl ::core::clone::Clone for MFT_REGISTER_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFT_REGISTRATION_INFO {
    pub clsid: ::windows_sys::core::GUID,
    pub guidCategory: ::windows_sys::core::GUID,
    pub uiFlags: u32,
    pub pszName: ::windows_sys::core::PCWSTR,
    pub cInTypes: u32,
    pub pInTypes: *mut MFT_REGISTER_TYPE_INFO,
    pub cOutTypes: u32,
    pub pOutTypes: *mut MFT_REGISTER_TYPE_INFO,
}
impl ::core::marker::Copy for MFT_REGISTRATION_INFO {}
impl ::core::clone::Clone for MFT_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFT_STREAM_STATE_PARAM {
    pub StreamId: u32,
    pub State: MF_STREAM_STATE,
}
impl ::core::marker::Copy for MFT_STREAM_STATE_PARAM {}
impl ::core::clone::Clone for MFT_STREAM_STATE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MFVIDEOFORMAT {
    pub dwSize: u32,
    pub videoInfo: MFVideoInfo,
    pub guidFormat: ::windows_sys::core::GUID,
    pub compressedInfo: MFVideoCompressedInfo,
    pub surfaceInfo: MFVideoSurfaceInfo,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MFVIDEOFORMAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MFVIDEOFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
pub struct MFVideoAlphaBitmap {
    pub GetBitmapFromDC: super::super::Foundation::BOOL,
    pub bitmap: MFVideoAlphaBitmap_0,
    pub params: MFVideoAlphaBitmapParams,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for MFVideoAlphaBitmap {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for MFVideoAlphaBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
pub union MFVideoAlphaBitmap_0 {
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub pDDS: super::super::Graphics::Direct3D9::IDirect3DSurface9,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for MFVideoAlphaBitmap_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for MFVideoAlphaBitmap_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MFVideoAlphaBitmapParams {
    pub dwFlags: u32,
    pub clrSrcKey: super::super::Foundation::COLORREF,
    pub rcSrc: super::super::Foundation::RECT,
    pub nrcDest: MFVideoNormalizedRect,
    pub fAlpha: f32,
    pub dwFilterMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MFVideoAlphaBitmapParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MFVideoAlphaBitmapParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MFVideoArea {
    pub OffsetX: MFOffset,
    pub OffsetY: MFOffset,
    pub Area: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MFVideoArea {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MFVideoArea {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFVideoCompressedInfo {
    pub AvgBitrate: i64,
    pub AvgBitErrorRate: i64,
    pub MaxKeyFrameSpacing: u32,
}
impl ::core::marker::Copy for MFVideoCompressedInfo {}
impl ::core::clone::Clone for MFVideoCompressedInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MFVideoInfo {
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub PixelAspectRatio: MFRatio,
    pub SourceChromaSubsampling: MFVideoChromaSubsampling,
    pub InterlaceMode: MFVideoInterlaceMode,
    pub TransferFunction: MFVideoTransferFunction,
    pub ColorPrimaries: MFVideoPrimaries,
    pub TransferMatrix: MFVideoTransferMatrix,
    pub SourceLighting: MFVideoLighting,
    pub FramesPerSecond: MFRatio,
    pub NominalRange: MFNominalRange,
    pub GeometricAperture: MFVideoArea,
    pub MinimumDisplayAperture: MFVideoArea,
    pub PanScanAperture: MFVideoArea,
    pub VideoFlags: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MFVideoInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MFVideoInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFVideoNormalizedRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl ::core::marker::Copy for MFVideoNormalizedRect {}
impl ::core::clone::Clone for MFVideoNormalizedRect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MFVideoSurfaceInfo {
    pub Format: u32,
    pub PaletteEntries: u32,
    pub Palette: [MFPaletteEntry; 1],
}
impl ::core::marker::Copy for MFVideoSurfaceInfo {}
impl ::core::clone::Clone for MFVideoSurfaceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MF_BYTE_STREAM_CACHE_RANGE {
    pub qwStartOffset: u64,
    pub qwEndOffset: u64,
}
impl ::core::marker::Copy for MF_BYTE_STREAM_CACHE_RANGE {}
impl ::core::clone::Clone for MF_BYTE_STREAM_CACHE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MF_FLOAT2 {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for MF_FLOAT2 {}
impl ::core::clone::Clone for MF_FLOAT2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MF_FLOAT3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ::core::marker::Copy for MF_FLOAT3 {}
impl ::core::clone::Clone for MF_FLOAT3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MF_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
impl ::core::marker::Copy for MF_LEAKY_BUCKET_PAIR {}
impl ::core::clone::Clone for MF_LEAKY_BUCKET_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MF_QUATERNION {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl ::core::marker::Copy for MF_QUATERNION {}
impl ::core::clone::Clone for MF_QUATERNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MF_SINK_WRITER_STATISTICS {
    pub cb: u32,
    pub llLastTimestampReceived: i64,
    pub llLastTimestampEncoded: i64,
    pub llLastTimestampProcessed: i64,
    pub llLastStreamTickReceived: i64,
    pub llLastSinkSampleRequest: i64,
    pub qwNumSamplesReceived: u64,
    pub qwNumSamplesEncoded: u64,
    pub qwNumSamplesProcessed: u64,
    pub qwNumStreamTicksReceived: u64,
    pub dwByteCountQueued: u32,
    pub qwByteCountProcessed: u64,
    pub dwNumOutstandingSinkSampleRequests: u32,
    pub dwAverageSampleRateReceived: u32,
    pub dwAverageSampleRateEncoded: u32,
    pub dwAverageSampleRateProcessed: u32,
}
impl ::core::marker::Copy for MF_SINK_WRITER_STATISTICS {}
impl ::core::clone::Clone for MF_SINK_WRITER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MF_TRANSCODE_SINK_INFO {
    pub dwVideoStreamID: u32,
    pub pVideoMediaType: IMFMediaType,
    pub dwAudioStreamID: u32,
    pub pAudioMediaType: IMFMediaType,
}
impl ::core::marker::Copy for MF_TRANSCODE_SINK_INFO {}
impl ::core::clone::Clone for MF_TRANSCODE_SINK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MF_VIDEO_SPHERICAL_VIEWDIRECTION {
    pub iHeading: i32,
    pub iPitch: i32,
    pub iRoll: i32,
}
impl ::core::marker::Copy for MF_VIDEO_SPHERICAL_VIEWDIRECTION {}
impl ::core::clone::Clone for MF_VIDEO_SPHERICAL_VIEWDIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MOVEREGION_INFO {
    pub FrameNumber: u32,
    pub NumMoveRegions: u32,
    pub MoveRegions: [MOVE_RECT; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MOVEREGION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MOVEREGION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MOVE_RECT {
    pub SourcePoint: super::super::Foundation::POINT,
    pub DestRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MOVE_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct MPEG1VIDEOINFO {
    pub hdr: VIDEOINFOHEADER,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub bSequenceHeader: [u8; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for MPEG1VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for MPEG1VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct MPEG2VIDEOINFO {
    pub hdr: VIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: MPEG2VIDEOINFO_FLAGS,
    pub dwSequenceHeader: [u32; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for MPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for MPEG2VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MT_ARBITRARY_HEADER {
    pub majortype: ::windows_sys::core::GUID,
    pub subtype: ::windows_sys::core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MT_ARBITRARY_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MT_ARBITRARY_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct MT_CUSTOM_VIDEO_PRIMARIES {
    pub fRx: f32,
    pub fRy: f32,
    pub fGx: f32,
    pub fGy: f32,
    pub fBx: f32,
    pub fBy: f32,
    pub fWx: f32,
    pub fWy: f32,
}
impl ::core::marker::Copy for MT_CUSTOM_VIDEO_PRIMARIES {}
impl ::core::clone::Clone for MT_CUSTOM_VIDEO_PRIMARIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_ACP_AND_CGMSA_SIGNALING {
    pub rnRandomNumber: OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulAvailableTVProtectionStandards: u32,
    pub ulActiveTVProtectionStandard: u32,
    pub ulReserved: u32,
    pub ulAspectRatioValidMask1: u32,
    pub ulAspectRatioData1: u32,
    pub ulAspectRatioValidMask2: u32,
    pub ulAspectRatioData2: u32,
    pub ulAspectRatioValidMask3: u32,
    pub ulAspectRatioData3: u32,
    pub ulReserved2: [u32; 4],
    pub ulReserved3: [u32; 4],
}
impl ::core::marker::Copy for OPM_ACP_AND_CGMSA_SIGNALING {}
impl ::core::clone::Clone for OPM_ACP_AND_CGMSA_SIGNALING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub struct OPM_ACTUAL_OUTPUT_FORMAT {
    pub rnRandomNumber: OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulDisplayWidth: u32,
    pub ulDisplayHeight: u32,
    pub dsfSampleInterleaveFormat: DXVA2_SampleFormat,
    pub d3dFormat: super::super::Graphics::Direct3D9::D3DFORMAT,
    pub ulFrequencyNumerator: u32,
    pub ulFrequencyDenominator: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::marker::Copy for OPM_ACTUAL_OUTPUT_FORMAT {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::clone::Clone for OPM_ACTUAL_OUTPUT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_CONFIGURE_PARAMETERS {
    pub omac: OPM_OMAC,
    pub guidSetting: ::windows_sys::core::GUID,
    pub ulSequenceNumber: u32,
    pub cbParametersSize: u32,
    pub abParameters: [u8; 4056],
}
impl ::core::marker::Copy for OPM_CONFIGURE_PARAMETERS {}
impl ::core::clone::Clone for OPM_CONFIGURE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_CONNECTED_HDCP_DEVICE_INFORMATION {
    pub rnRandomNumber: OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulHDCPFlags: u32,
    pub ksvB: OPM_HDCP_KEY_SELECTION_VECTOR,
    pub Reserved: [u8; 11],
    pub Reserved2: [u8; 16],
    pub Reserved3: [u8; 16],
}
impl ::core::marker::Copy for OPM_CONNECTED_HDCP_DEVICE_INFORMATION {}
impl ::core::clone::Clone for OPM_CONNECTED_HDCP_DEVICE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS {
    pub rnRandomNumber: OPM_RANDOM_NUMBER,
    pub guidInformation: ::windows_sys::core::GUID,
    pub ulSequenceNumber: u32,
    pub cbParametersSize: u32,
    pub abParameters: [u8; 4056],
}
impl ::core::marker::Copy for OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS {}
impl ::core::clone::Clone for OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_ENCRYPTED_INITIALIZATION_PARAMETERS {
    pub abEncryptedInitializationParameters: [u8; 256],
}
impl ::core::marker::Copy for OPM_ENCRYPTED_INITIALIZATION_PARAMETERS {}
impl ::core::clone::Clone for OPM_ENCRYPTED_INITIALIZATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_GET_CODEC_INFO_INFORMATION {
    pub rnRandomNumber: OPM_RANDOM_NUMBER,
    pub Merit: u32,
}
impl ::core::marker::Copy for OPM_GET_CODEC_INFO_INFORMATION {}
impl ::core::clone::Clone for OPM_GET_CODEC_INFO_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_GET_CODEC_INFO_PARAMETERS {
    pub cbVerifier: u32,
    pub Verifier: [u8; 4052],
}
impl ::core::marker::Copy for OPM_GET_CODEC_INFO_PARAMETERS {}
impl ::core::clone::Clone for OPM_GET_CODEC_INFO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_GET_INFO_PARAMETERS {
    pub omac: OPM_OMAC,
    pub rnRandomNumber: OPM_RANDOM_NUMBER,
    pub guidInformation: ::windows_sys::core::GUID,
    pub ulSequenceNumber: u32,
    pub cbParametersSize: u32,
    pub abParameters: [u8; 4056],
}
impl ::core::marker::Copy for OPM_GET_INFO_PARAMETERS {}
impl ::core::clone::Clone for OPM_GET_INFO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_HDCP_KEY_SELECTION_VECTOR {
    pub abKeySelectionVector: [u8; 5],
}
impl ::core::marker::Copy for OPM_HDCP_KEY_SELECTION_VECTOR {}
impl ::core::clone::Clone for OPM_HDCP_KEY_SELECTION_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_OMAC {
    pub abOMAC: [u8; 16],
}
impl ::core::marker::Copy for OPM_OMAC {}
impl ::core::clone::Clone for OPM_OMAC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_OUTPUT_ID_DATA {
    pub rnRandomNumber: OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub OutputId: u64,
}
impl ::core::marker::Copy for OPM_OUTPUT_ID_DATA {}
impl ::core::clone::Clone for OPM_OUTPUT_ID_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_RANDOM_NUMBER {
    pub abRandomNumber: [u8; 16],
}
impl ::core::marker::Copy for OPM_RANDOM_NUMBER {}
impl ::core::clone::Clone for OPM_RANDOM_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_REQUESTED_INFORMATION {
    pub omac: OPM_OMAC,
    pub cbRequestedInformationSize: u32,
    pub abRequestedInformation: [u8; 4076],
}
impl ::core::marker::Copy for OPM_REQUESTED_INFORMATION {}
impl ::core::clone::Clone for OPM_REQUESTED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS {
    pub ulNewTVProtectionStandard: u32,
    pub ulAspectRatioChangeMask1: u32,
    pub ulAspectRatioData1: u32,
    pub ulAspectRatioChangeMask2: u32,
    pub ulAspectRatioData2: u32,
    pub ulAspectRatioChangeMask3: u32,
    pub ulAspectRatioData3: u32,
    pub ulReserved: [u32; 4],
    pub ulReserved2: [u32; 4],
    pub ulReserved3: u32,
}
impl ::core::marker::Copy for OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS {}
impl ::core::clone::Clone for OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_SET_HDCP_SRM_PARAMETERS {
    pub ulSRMVersion: u32,
}
impl ::core::marker::Copy for OPM_SET_HDCP_SRM_PARAMETERS {}
impl ::core::clone::Clone for OPM_SET_HDCP_SRM_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_SET_PROTECTION_LEVEL_PARAMETERS {
    pub ulProtectionType: u32,
    pub ulProtectionLevel: u32,
    pub Reserved: u32,
    pub Reserved2: u32,
}
impl ::core::marker::Copy for OPM_SET_PROTECTION_LEVEL_PARAMETERS {}
impl ::core::clone::Clone for OPM_SET_PROTECTION_LEVEL_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct OPM_STANDARD_INFORMATION {
    pub rnRandomNumber: OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulInformation: u32,
    pub ulReserved: u32,
    pub ulReserved2: u32,
}
impl ::core::marker::Copy for OPM_STANDARD_INFORMATION {}
impl ::core::clone::Clone for OPM_STANDARD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ROI_AREA {
    pub rect: super::super::Foundation::RECT,
    pub QPDelta: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ROI_AREA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ROI_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct SENSORPROFILEID {
    pub Type: ::windows_sys::core::GUID,
    pub Index: u32,
    pub Unused: u32,
}
impl ::core::marker::Copy for SENSORPROFILEID {}
impl ::core::clone::Clone for SENSORPROFILEID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct STREAM_MEDIUM {
    pub gidMedium: ::windows_sys::core::GUID,
    pub unMediumInstance: u32,
}
impl ::core::marker::Copy for STREAM_MEDIUM {}
impl ::core::clone::Clone for STREAM_MEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct TOC_DESCRIPTOR {
    pub guidID: ::windows_sys::core::GUID,
    pub wStreamNumber: u16,
    pub guidType: ::windows_sys::core::GUID,
    pub wLanguageIndex: u16,
}
impl ::core::marker::Copy for TOC_DESCRIPTOR {}
impl ::core::clone::Clone for TOC_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub struct TOC_ENTRY_DESCRIPTOR {
    pub qwStartTime: u64,
    pub qwEndTime: u64,
    pub qwStartPacketOffset: u64,
    pub qwEndPacketOffset: u64,
    pub qwRepresentativeFrameTime: u64,
}
impl ::core::marker::Copy for TOC_ENTRY_DESCRIPTOR {}
impl ::core::clone::Clone for TOC_ENTRY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct VIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for VIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for VIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct VIDEOINFOHEADER2 {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub Anonymous: VIDEOINFOHEADER2_0,
    pub dwReserved2: u32,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for VIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for VIDEOINFOHEADER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union VIDEOINFOHEADER2_0 {
    pub dwControlFlags: u32,
    pub dwReserved1: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for VIDEOINFOHEADER2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for VIDEOINFOHEADER2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type MFPERIODICCALLBACK = ::core::option::Option<unsafe extern "system" fn(pcontext: ::windows_sys::core::IUnknown)>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PDXVAHDSW_CreateDevice = ::core::option::Option<unsafe extern "system" fn(pd3ddevice: super::super::Graphics::Direct3D9::IDirect3DDevice9Ex, phdevice: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_CreateVideoProcessor = ::core::option::Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, pvpguid: *const ::windows_sys::core::GUID, phvideoprocessor: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_DestroyDevice = ::core::option::Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_DestroyVideoProcessor = ::core::option::Option<unsafe extern "system" fn(hvideoprocessor: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_GetVideoProcessBltStatePrivate = ::core::option::Option<unsafe extern "system" fn(hvideoprocessor: super::super::Foundation::HANDLE, pdata: *mut DXVAHD_BLT_STATE_PRIVATE_DATA) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_GetVideoProcessStreamStatePrivate = ::core::option::Option<unsafe extern "system" fn(hvideoprocessor: super::super::Foundation::HANDLE, streamnumber: u32, pdata: *mut DXVAHD_STREAM_STATE_PRIVATE_DATA) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_GetVideoProcessorCaps = ::core::option::Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, pcontentdesc: *const DXVAHD_CONTENT_DESC, usage: DXVAHD_DEVICE_USAGE, count: u32, pcaps: *mut DXVAHD_VPCAPS) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_GetVideoProcessorCustomRates = ::core::option::Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, pvpguid: *const ::windows_sys::core::GUID, count: u32, prates: *mut DXVAHD_CUSTOM_RATE_DATA) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PDXVAHDSW_GetVideoProcessorDeviceCaps = ::core::option::Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, pcontentdesc: *const DXVAHD_CONTENT_DESC, usage: DXVAHD_DEVICE_USAGE, pcaps: *mut DXVAHD_VPDEVCAPS) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_GetVideoProcessorFilterRange = ::core::option::Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, filter: DXVAHD_FILTER, prange: *mut DXVAHD_FILTER_RANGE_DATA) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PDXVAHDSW_GetVideoProcessorInputFormats = ::core::option::Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, pcontentdesc: *const DXVAHD_CONTENT_DESC, usage: DXVAHD_DEVICE_USAGE, count: u32, pformats: *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PDXVAHDSW_GetVideoProcessorOutputFormats = ::core::option::Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, pcontentdesc: *const DXVAHD_CONTENT_DESC, usage: DXVAHD_DEVICE_USAGE, count: u32, pformats: *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`*"]
pub type PDXVAHDSW_Plugin = ::core::option::Option<unsafe extern "system" fn(size: u32, pcallbacks: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PDXVAHDSW_ProposeVideoPrivateFormat = ::core::option::Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, pformat: *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_SetVideoProcessBltState = ::core::option::Option<unsafe extern "system" fn(hvideoprocessor: super::super::Foundation::HANDLE, state: DXVAHD_BLT_STATE, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDXVAHDSW_SetVideoProcessStreamState = ::core::option::Option<unsafe extern "system" fn(hvideoprocessor: super::super::Foundation::HANDLE, streamnumber: u32, state: DXVAHD_STREAM_STATE, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PDXVAHDSW_VideoProcessBltHD = ::core::option::Option<unsafe extern "system" fn(hvideoprocessor: super::super::Foundation::HANDLE, poutputsurface: super::super::Graphics::Direct3D9::IDirect3DSurface9, outputframe: u32, streamcount: u32, pstreams: *const DXVAHD_STREAM_DATA) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_MediaFoundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PDXVAHD_CreateDevice = ::core::option::Option<unsafe extern "system" fn(pd3ddevice: super::super::Graphics::Direct3D9::IDirect3DDevice9Ex, pcontentdesc: *const DXVAHD_CONTENT_DESC, usage: DXVAHD_DEVICE_USAGE, pplugin: PDXVAHDSW_Plugin, ppdevice: *mut IDXVAHD_Device) -> ::windows_sys::core::HRESULT>;
