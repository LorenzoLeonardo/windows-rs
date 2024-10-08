pub trait IDMOQualityControl_Impl: Sized + windows_core::IUnknownImpl {
    fn SetNow(&self, rtnow: i64) -> windows_core::Result<()>;
    fn SetStatus(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IDMOQualityControl {}
impl IDMOQualityControl_Vtbl {
    pub const fn new<Identity: IDMOQualityControl_Impl, const OFFSET: isize>() -> IDMOQualityControl_Vtbl {
        unsafe extern "system" fn SetNow<Identity: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtnow: i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMOQualityControl_Impl::SetNow(this, core::mem::transmute_copy(&rtnow)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMOQualityControl_Impl::SetStatus(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDMOQualityControl_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetNow: SetNow::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMOQualityControl as windows_core::Interface>::IID
    }
}
pub trait IDMOVideoOutputOptimizations_Impl: Sized + windows_core::IUnknownImpl {
    fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> windows_core::Result<u32>;
    fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> windows_core::Result<()>;
    fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> windows_core::Result<u32>;
    fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IDMOVideoOutputOptimizations {}
impl IDMOVideoOutputOptimizations_Vtbl {
    pub const fn new<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>() -> IDMOVideoOutputOptimizations_Vtbl {
        unsafe extern "system" fn QueryOperationModePreferences<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDMOVideoOutputOptimizations_Impl::QueryOperationModePreferences(this, core::mem::transmute_copy(&uloutputstreamindex)) {
                Ok(ok__) => {
                    pdwrequestedcapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationMode<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDMOVideoOutputOptimizations_Impl::SetOperationMode(this, core::mem::transmute_copy(&uloutputstreamindex), core::mem::transmute_copy(&dwenabledfeatures)).into()
        }
        unsafe extern "system" fn GetCurrentOperationMode<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDMOVideoOutputOptimizations_Impl::GetCurrentOperationMode(this, core::mem::transmute_copy(&uloutputstreamindex)) {
                Ok(ok__) => {
                    pdwenabledfeatures.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSampleRequirements<Identity: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDMOVideoOutputOptimizations_Impl::GetCurrentSampleRequirements(this, core::mem::transmute_copy(&uloutputstreamindex)) {
                Ok(ok__) => {
                    pdwrequestedfeatures.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryOperationModePreferences: QueryOperationModePreferences::<Identity, OFFSET>,
            SetOperationMode: SetOperationMode::<Identity, OFFSET>,
            GetCurrentOperationMode: GetCurrentOperationMode::<Identity, OFFSET>,
            GetCurrentSampleRequirements: GetCurrentSampleRequirements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDMOVideoOutputOptimizations as windows_core::Interface>::IID
    }
}
pub trait IEnumDMO_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, citemstofetch: u32, pclsid: *mut windows_core::GUID, names: *mut windows_core::PWSTR, pcitemsfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, citemstoskip: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDMO>;
}
impl windows_core::RuntimeName for IEnumDMO {}
impl IEnumDMO_Vtbl {
    pub const fn new<Identity: IEnumDMO_Impl, const OFFSET: isize>() -> IEnumDMO_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, citemstofetch: u32, pclsid: *mut windows_core::GUID, names: *mut windows_core::PWSTR, pcitemsfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDMO_Impl::Next(this, core::mem::transmute_copy(&citemstofetch), core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&names), core::mem::transmute_copy(&pcitemsfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, citemstoskip: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDMO_Impl::Skip(this, core::mem::transmute_copy(&citemstoskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDMO_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumDMO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDMO_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDMO as windows_core::Interface>::IID
    }
}
pub trait IMediaBuffer_Impl: Sized + windows_core::IUnknownImpl {
    fn SetLength(&self, cblength: u32) -> windows_core::Result<()>;
    fn GetMaxLength(&self) -> windows_core::Result<u32>;
    fn GetBufferAndLength(&self, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMediaBuffer {}
impl IMediaBuffer_Vtbl {
    pub const fn new<Identity: IMediaBuffer_Impl, const OFFSET: isize>() -> IMediaBuffer_Vtbl {
        unsafe extern "system" fn SetLength<Identity: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cblength: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaBuffer_Impl::SetLength(this, core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetMaxLength<Identity: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbmaxlength: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaBuffer_Impl::GetMaxLength(this) {
                Ok(ok__) => {
                    pcbmaxlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaBuffer_Impl::GetBufferAndLength(this, core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pcblength)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLength: SetLength::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaBuffer as windows_core::Interface>::IID
    }
}
pub trait IMediaObject_Impl: Sized + windows_core::IUnknownImpl {
    fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::Result<()>;
    fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> windows_core::Result<u32>;
    fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> windows_core::Result<u32>;
    fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn SetInputType(&self, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::Result<()>;
    fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::Result<()>;
    fn GetInputCurrentType(&self, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetOutputCurrentType(&self, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()>;
    fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> windows_core::Result<()>;
    fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> windows_core::Result<i64>;
    fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: i64) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
    fn Discontinuity(&self, dwinputstreamindex: u32) -> windows_core::Result<()>;
    fn AllocateStreamingResources(&self) -> windows_core::Result<()>;
    fn FreeStreamingResources(&self) -> windows_core::Result<()>;
    fn GetInputStatus(&self, dwinputstreamindex: u32) -> windows_core::Result<u32>;
    fn ProcessInput(&self, dwinputstreamindex: u32, pbuffer: Option<&IMediaBuffer>, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> windows_core::Result<()>;
    fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::Result<()>;
    fn Lock(&self, block: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMediaObject {}
impl IMediaObject_Vtbl {
    pub const fn new<Identity: IMediaObject_Impl, const OFFSET: isize>() -> IMediaObject_Vtbl {
        unsafe extern "system" fn GetStreamCount<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::GetStreamCount(this, core::mem::transmute_copy(&pcinputstreams), core::mem::transmute_copy(&pcoutputstreams)).into()
        }
        unsafe extern "system" fn GetInputStreamInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaObject_Impl::GetInputStreamInfo(this, core::mem::transmute_copy(&dwinputstreamindex)) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaObject_Impl::GetOutputStreamInfo(this, core::mem::transmute_copy(&dwoutputstreamindex)) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::GetInputType(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&dwtypeindex), core::mem::transmute_copy(&pmt)).into()
        }
        unsafe extern "system" fn GetOutputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::GetOutputType(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&dwtypeindex), core::mem::transmute_copy(&pmt)).into()
        }
        unsafe extern "system" fn SetInputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::SetInputType(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pmt), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetOutputType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::SetOutputType(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&pmt), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::GetInputCurrentType(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pmt)).into()
        }
        unsafe extern "system" fn GetOutputCurrentType<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::GetOutputCurrentType(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&pmt)).into()
        }
        unsafe extern "system" fn GetInputSizeInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::GetInputSizeInfo(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pcbsize), core::mem::transmute_copy(&pcbmaxlookahead), core::mem::transmute_copy(&pcbalignment)).into()
        }
        unsafe extern "system" fn GetOutputSizeInfo<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::GetOutputSizeInfo(this, core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&pcbsize), core::mem::transmute_copy(&pcbalignment)).into()
        }
        unsafe extern "system" fn GetInputMaxLatency<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaObject_Impl::GetInputMaxLatency(this, core::mem::transmute_copy(&dwinputstreamindex)) {
                Ok(ok__) => {
                    prtmaxlatency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputMaxLatency<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::SetInputMaxLatency(this, core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&rtmaxlatency)).into()
        }
        unsafe extern "system" fn Flush<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::Flush(this).into()
        }
        unsafe extern "system" fn Discontinuity<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::Discontinuity(this, core::mem::transmute_copy(&dwinputstreamindex)).into()
        }
        unsafe extern "system" fn AllocateStreamingResources<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::AllocateStreamingResources(this).into()
        }
        unsafe extern "system" fn FreeStreamingResources<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::FreeStreamingResources(this).into()
        }
        unsafe extern "system" fn GetInputStatus<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaObject_Impl::GetInputStatus(this, core::mem::transmute_copy(&dwinputstreamindex)) {
                Ok(ok__) => {
                    dwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputstreamindex: u32, pbuffer: *mut core::ffi::c_void, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::ProcessInput(this, core::mem::transmute_copy(&dwinputstreamindex), windows_core::from_raw_borrowed(&pbuffer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&rttimestamp), core::mem::transmute_copy(&rttimelength)).into()
        }
        unsafe extern "system" fn ProcessOutput<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::ProcessOutput(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&coutputbuffercount), core::mem::transmute_copy(&poutputbuffers), core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn Lock<Identity: IMediaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObject_Impl::Lock(this, core::mem::transmute_copy(&block)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetInputStreamInfo: GetInputStreamInfo::<Identity, OFFSET>,
            GetOutputStreamInfo: GetOutputStreamInfo::<Identity, OFFSET>,
            GetInputType: GetInputType::<Identity, OFFSET>,
            GetOutputType: GetOutputType::<Identity, OFFSET>,
            SetInputType: SetInputType::<Identity, OFFSET>,
            SetOutputType: SetOutputType::<Identity, OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Identity, OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Identity, OFFSET>,
            GetInputSizeInfo: GetInputSizeInfo::<Identity, OFFSET>,
            GetOutputSizeInfo: GetOutputSizeInfo::<Identity, OFFSET>,
            GetInputMaxLatency: GetInputMaxLatency::<Identity, OFFSET>,
            SetInputMaxLatency: SetInputMaxLatency::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            Discontinuity: Discontinuity::<Identity, OFFSET>,
            AllocateStreamingResources: AllocateStreamingResources::<Identity, OFFSET>,
            FreeStreamingResources: FreeStreamingResources::<Identity, OFFSET>,
            GetInputStatus: GetInputStatus::<Identity, OFFSET>,
            ProcessInput: ProcessInput::<Identity, OFFSET>,
            ProcessOutput: ProcessOutput::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaObject as windows_core::Interface>::IID
    }
}
pub trait IMediaObjectInPlace_Impl: Sized + windows_core::IUnknownImpl {
    fn Process(&self, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IMediaObjectInPlace>;
    fn GetLatency(&self) -> windows_core::Result<i64>;
}
impl windows_core::RuntimeName for IMediaObjectInPlace {}
impl IMediaObjectInPlace_Vtbl {
    pub const fn new<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>() -> IMediaObjectInPlace_Vtbl {
        unsafe extern "system" fn Process<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaObjectInPlace_Impl::Process(this, core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&reftimestart), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Clone<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediaobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaObjectInPlace_Impl::Clone(this) {
                Ok(ok__) => {
                    ppmediaobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatency<Identity: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, platencytime: *mut i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaObjectInPlace_Impl::GetLatency(this) {
                Ok(ok__) => {
                    platencytime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Process: Process::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetLatency: GetLatency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaObjectInPlace as windows_core::Interface>::IID
    }
}
