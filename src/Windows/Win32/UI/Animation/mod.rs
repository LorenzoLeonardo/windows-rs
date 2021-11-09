#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationInterpolator(pub ::windows::runtime::IUnknown);
impl IUIAnimationInterpolator {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: f64, initialvelocity: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvalue), ::core::mem::transmute(initialvelocity)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetDuration(&self, duration: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetDuration(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetFinalValue(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn InterpolateValue(&self, offset: f64) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn InterpolateVelocity(&self, offset: f64) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvaluedependencies), ::core::mem::transmute(initialvelocitydependencies), ::core::mem::transmute(durationdependencies)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationInterpolator {
    type Vtable = IUIAnimationInterpolator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2014694330, 56823, 18316, [164, 108, 123, 108, 115, 139, 121, 120]);
}
impl ::core::convert::From<IUIAnimationInterpolator> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationInterpolator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationInterpolator> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationInterpolator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationInterpolator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationInterpolator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationInterpolator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, initialvalue: f64, initialvelocity: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: f64, value: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: f64, velocity: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationInterpolator2(pub ::windows::runtime::IUnknown);
impl IUIAnimationInterpolator2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetDimension(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvalue), ::core::mem::transmute(initialvelocity), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetDuration(&self, duration: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetDuration(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetFinalValue(&self, value: *mut f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn InterpolateValue(&self, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset), ::core::mem::transmute(value), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn InterpolateVelocity(&self, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset), ::core::mem::transmute(velocity), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetPrimitiveInterpolation<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationPrimitiveInterpolation>>(&self, interpolation: Param0, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), interpolation.into_param().abi(), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvaluedependencies), ::core::mem::transmute(initialvelocitydependencies), ::core::mem::transmute(durationdependencies)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationInterpolator2 {
    type Vtable = IUIAnimationInterpolator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3933646840, 59938, 18979, [160, 239, 166, 169, 102, 112, 53, 24]);
}
impl ::core::convert::From<IUIAnimationInterpolator2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationInterpolator2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationInterpolator2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationInterpolator2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationInterpolator2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationInterpolator2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationInterpolator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dimension: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: f64, value: *mut f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interpolation: ::windows::runtime::RawPtr, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationLoopIterationChangeHandler2(pub ::windows::runtime::IUnknown);
impl IUIAnimationLoopIterationChangeHandler2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnLoopIterationChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard2>>(&self, storyboard: Param0, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(newiterationcount), ::core::mem::transmute(olditerationcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationLoopIterationChangeHandler2 {
    type Vtable = IUIAnimationLoopIterationChangeHandler2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(758846884, 18274, 18347, [160, 48, 178, 50, 33, 223, 58, 224]);
}
impl ::core::convert::From<IUIAnimationLoopIterationChangeHandler2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationLoopIterationChangeHandler2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationLoopIterationChangeHandler2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationLoopIterationChangeHandler2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationLoopIterationChangeHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationLoopIterationChangeHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationLoopIterationChangeHandler2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: ::windows::runtime::RawPtr, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationManager(pub ::windows::runtime::IUnknown);
impl IUIAnimationManager {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> ::windows::runtime::Result<IUIAnimationVariable> {
        let mut result__: <IUIAnimationVariable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvalue), &mut result__).from_abi::<IUIAnimationVariable>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn ScheduleTransition<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationTransition>>(&self, variable: Param0, transition: Param1, timenow: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), ::core::mem::transmute(timenow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateStoryboard(&self) -> ::windows::runtime::Result<IUIAnimationStoryboard> {
        let mut result__: <IUIAnimationStoryboard as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUIAnimationStoryboard>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(completiondeadline)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AbandonAllStoryboards(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Update(&self, timenow: f64) -> ::windows::runtime::Result<UI_ANIMATION_UPDATE_RESULT> {
        let mut result__: <UI_ANIMATION_UPDATE_RESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), &mut result__).from_abi::<UI_ANIMATION_UPDATE_RESULT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetVariableFromTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<IUIAnimationVariable> {
        let mut result__: <IUIAnimationVariable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id), &mut result__).from_abi::<IUIAnimationVariable>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetStoryboardFromTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<IUIAnimationStoryboard> {
        let mut result__: <IUIAnimationStoryboard as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id), &mut result__).from_abi::<IUIAnimationStoryboard>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__: <UI_ANIMATION_MANAGER_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<UI_ANIMATION_MANAGER_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Pause(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetManagerEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationManagerEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetCancelPriorityComparison<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationPriorityComparison>>(&self, comparison: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetTrimPriorityComparison<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationPriorityComparison>>(&self, comparison: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetCompressPriorityComparison<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationPriorityComparison>>(&self, comparison: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetConcludePriorityComparison<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationPriorityComparison>>(&self, comparison: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Shutdown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationManager {
    type Vtable = IUIAnimationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2439612780, 44173, 20093, [148, 229, 103, 250, 77, 194, 242, 232]);
}
impl ::core::convert::From<IUIAnimationManager> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationManager> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, initialvalue: f64, variable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr, timenow: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, completiondeadline: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32, variable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32, storyboard: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: UI_ANIMATION_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comparison: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comparison: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comparison: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comparison: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, delay: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationManager2(pub ::windows::runtime::IUnknown);
impl IUIAnimationManager2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateAnimationVectorVariable(&self, initialvalue: *const f64, cdimension: u32) -> ::windows::runtime::Result<IUIAnimationVariable2> {
        let mut result__: <IUIAnimationVariable2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvalue), ::core::mem::transmute(cdimension), &mut result__).from_abi::<IUIAnimationVariable2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> ::windows::runtime::Result<IUIAnimationVariable2> {
        let mut result__: <IUIAnimationVariable2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialvalue), &mut result__).from_abi::<IUIAnimationVariable2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn ScheduleTransition<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable2>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationTransition2>>(&self, variable: Param0, transition: Param1, timenow: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), ::core::mem::transmute(timenow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateStoryboard(&self) -> ::windows::runtime::Result<IUIAnimationStoryboard2> {
        let mut result__: <IUIAnimationStoryboard2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUIAnimationStoryboard2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(completiondeadline)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AbandonAllStoryboards(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Update(&self, timenow: f64) -> ::windows::runtime::Result<UI_ANIMATION_UPDATE_RESULT> {
        let mut result__: <UI_ANIMATION_UPDATE_RESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), &mut result__).from_abi::<UI_ANIMATION_UPDATE_RESULT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetVariableFromTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<IUIAnimationVariable2> {
        let mut result__: <IUIAnimationVariable2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id), &mut result__).from_abi::<IUIAnimationVariable2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetStoryboardFromTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<IUIAnimationStoryboard2> {
        let mut result__: <IUIAnimationStoryboard2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id), &mut result__).from_abi::<IUIAnimationStoryboard2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn EstimateNextEventTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__: <UI_ANIMATION_MANAGER_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<UI_ANIMATION_MANAGER_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Pause(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Animation`, `Win32_Foundation`*"]
    pub unsafe fn SetManagerEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationManagerEventHandler2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, handler: Param0, fregisterfornextanimationevent: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), handler.into_param().abi(), fregisterfornextanimationevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetCancelPriorityComparison<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationPriorityComparison2>>(&self, comparison: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetTrimPriorityComparison<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationPriorityComparison2>>(&self, comparison: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetCompressPriorityComparison<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationPriorityComparison2>>(&self, comparison: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetConcludePriorityComparison<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationPriorityComparison2>>(&self, comparison: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), comparison.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Shutdown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationManager2 {
    type Vtable = IUIAnimationManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3635869652, 16649, 19775, [172, 238, 135, 153, 38, 150, 140, 177]);
}
impl ::core::convert::From<IUIAnimationManager2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationManager2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationManager2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationManager2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, initialvalue: *const f64, cdimension: u32, variable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, initialvalue: f64, variable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr, timenow: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, completiondeadline: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32, variable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32, storyboard: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, seconds: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: UI_ANIMATION_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comparison: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comparison: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comparison: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, comparison: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, delay: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationManagerEventHandler(pub ::windows::runtime::IUnknown);
impl IUIAnimationManagerEventHandler {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationManagerEventHandler {
    type Vtable = IUIAnimationManagerEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2016616941, 30883, 17254, [181, 116, 106, 246, 7, 166, 71, 136]);
}
impl ::core::convert::From<IUIAnimationManagerEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationManagerEventHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationManagerEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationManagerEventHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationManagerEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationManagerEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManagerEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationManagerEventHandler2(pub ::windows::runtime::IUnknown);
impl IUIAnimationManagerEventHandler2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationManagerEventHandler2 {
    type Vtable = IUIAnimationManagerEventHandler2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4141884090, 49139, 17132, [144, 51, 224, 115, 243, 62, 131, 195]);
}
impl ::core::convert::From<IUIAnimationManagerEventHandler2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationManagerEventHandler2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationManagerEventHandler2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationManagerEventHandler2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationManagerEventHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationManagerEventHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManagerEventHandler2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationPrimitiveInterpolation(pub ::windows::runtime::IUnknown);
impl IUIAnimationPrimitiveInterpolation {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddCubic(&self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dimension), ::core::mem::transmute(beginoffset), ::core::mem::transmute(constantcoefficient), ::core::mem::transmute(linearcoefficient), ::core::mem::transmute(quadraticcoefficient), ::core::mem::transmute(cubiccoefficient)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddSinusoidal(&self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dimension), ::core::mem::transmute(beginoffset), ::core::mem::transmute(bias), ::core::mem::transmute(amplitude), ::core::mem::transmute(frequency), ::core::mem::transmute(phase)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationPrimitiveInterpolation {
    type Vtable = IUIAnimationPrimitiveInterpolation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3132231011, 17249, 17882, [162, 79, 171, 133, 8, 132, 107, 91]);
}
impl ::core::convert::From<IUIAnimationPrimitiveInterpolation> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationPrimitiveInterpolation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationPrimitiveInterpolation> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationPrimitiveInterpolation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationPrimitiveInterpolation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationPrimitiveInterpolation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPrimitiveInterpolation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationPriorityComparison(pub ::windows::runtime::IUnknown);
impl IUIAnimationPriorityComparison {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn HasPriority<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard>>(&self, scheduledstoryboard: Param0, newstoryboard: Param1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), scheduledstoryboard.into_param().abi(), newstoryboard.into_param().abi(), ::core::mem::transmute(priorityeffect)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationPriorityComparison {
    type Vtable = IUIAnimationPriorityComparison_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2214239092, 24454, 17944, [188, 106, 162, 250, 193, 155, 63, 68]);
}
impl ::core::convert::From<IUIAnimationPriorityComparison> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationPriorityComparison) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationPriorityComparison> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationPriorityComparison) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationPriorityComparison {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationPriorityComparison {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPriorityComparison_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheduledstoryboard: ::windows::runtime::RawPtr, newstoryboard: ::windows::runtime::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationPriorityComparison2(pub ::windows::runtime::IUnknown);
impl IUIAnimationPriorityComparison2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn HasPriority<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard2>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard2>>(&self, scheduledstoryboard: Param0, newstoryboard: Param1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), scheduledstoryboard.into_param().abi(), newstoryboard.into_param().abi(), ::core::mem::transmute(priorityeffect)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationPriorityComparison2 {
    type Vtable = IUIAnimationPriorityComparison2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1533901367, 17953, 18044, [139, 5, 112, 19, 29, 230, 45, 219]);
}
impl ::core::convert::From<IUIAnimationPriorityComparison2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationPriorityComparison2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationPriorityComparison2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationPriorityComparison2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationPriorityComparison2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationPriorityComparison2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPriorityComparison2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheduledstoryboard: ::windows::runtime::RawPtr, newstoryboard: ::windows::runtime::RawPtr, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationStoryboard(pub ::windows::runtime::IUnknown);
impl IUIAnimationStoryboard {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddTransition<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationTransition>>(&self, variable: Param0, transition: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddKeyframeAtOffset<'a, Param0: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, existingkeyframe: Param0, offset: f64) -> ::windows::runtime::Result<UI_ANIMATION_KEYFRAME> {
        let mut result__: <UI_ANIMATION_KEYFRAME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), existingkeyframe.into_param().abi(), ::core::mem::transmute(offset), &mut result__).from_abi::<UI_ANIMATION_KEYFRAME>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddKeyframeAfterTransition<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationTransition>>(&self, transition: Param0) -> ::windows::runtime::Result<UI_ANIMATION_KEYFRAME> {
        let mut result__: <UI_ANIMATION_KEYFRAME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), transition.into_param().abi(), &mut result__).from_abi::<UI_ANIMATION_KEYFRAME>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddTransitionAtKeyframe<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationTransition>, Param2: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, variable: Param0, transition: Param1, startkeyframe: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddTransitionBetweenKeyframes<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationTransition>, Param2: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param3: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, variable: Param0, transition: Param1, startkeyframe: Param2, endkeyframe: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi(), endkeyframe.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn RepeatBetweenKeyframes<'a, Param0: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param1: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, startkeyframe: Param0, endkeyframe: Param1, repetitioncount: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), startkeyframe.into_param().abi(), endkeyframe.into_param().abi(), ::core::mem::transmute(repetitioncount)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn HoldVariable<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable>>(&self, variable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), variable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Schedule(&self, timenow: f64) -> ::windows::runtime::Result<UI_ANIMATION_SCHEDULING_RESULT> {
        let mut result__: <UI_ANIMATION_SCHEDULING_RESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), &mut result__).from_abi::<UI_ANIMATION_SCHEDULING_RESULT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Conclude(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Finish(&self, completiondeadline: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(completiondeadline)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Abandon(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetTag(&self, object: *mut ::core::option::Option<::windows::runtime::IUnknown>, id: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__: <UI_ANIMATION_STORYBOARD_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<UI_ANIMATION_STORYBOARD_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetElapsedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetStoryboardEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboardEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationStoryboard {
    type Vtable = IUIAnimationStoryboard_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2835288719, 39929, 19185, [158, 103, 229, 228, 16, 222, 251, 132]);
}
impl ::core::convert::From<IUIAnimationStoryboard> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationStoryboard) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationStoryboard> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationStoryboard) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationStoryboard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationStoryboard {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboard_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, delay: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, completiondeadline: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: *mut ::windows::runtime::RawPtr, id: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, elapsedtime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationStoryboard2(pub ::windows::runtime::IUnknown);
impl IUIAnimationStoryboard2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddTransition<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable2>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationTransition2>>(&self, variable: Param0, transition: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddKeyframeAtOffset<'a, Param0: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, existingkeyframe: Param0, offset: f64) -> ::windows::runtime::Result<UI_ANIMATION_KEYFRAME> {
        let mut result__: <UI_ANIMATION_KEYFRAME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), existingkeyframe.into_param().abi(), ::core::mem::transmute(offset), &mut result__).from_abi::<UI_ANIMATION_KEYFRAME>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddKeyframeAfterTransition<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationTransition2>>(&self, transition: Param0) -> ::windows::runtime::Result<UI_ANIMATION_KEYFRAME> {
        let mut result__: <UI_ANIMATION_KEYFRAME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), transition.into_param().abi(), &mut result__).from_abi::<UI_ANIMATION_KEYFRAME>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddTransitionAtKeyframe<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable2>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationTransition2>, Param2: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, variable: Param0, transition: Param1, startkeyframe: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn AddTransitionBetweenKeyframes<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable2>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationTransition2>, Param2: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param3: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>>(&self, variable: Param0, transition: Param1, startkeyframe: Param2, endkeyframe: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi(), endkeyframe.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Animation`, `Win32_Foundation`*"]
    pub unsafe fn RepeatBetweenKeyframes<'a, Param0: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param1: ::windows::runtime::IntoParam<'a, UI_ANIMATION_KEYFRAME>, Param4: ::windows::runtime::IntoParam<'a, IUIAnimationLoopIterationChangeHandler2>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
        &self,
        startkeyframe: Param0,
        endkeyframe: Param1,
        crepetition: f64,
        repeatmode: UI_ANIMATION_REPEAT_MODE,
        piterationchangehandler: Param4,
        id: usize,
        fregisterfornextanimationevent: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), startkeyframe.into_param().abi(), endkeyframe.into_param().abi(), ::core::mem::transmute(crepetition), ::core::mem::transmute(repeatmode), piterationchangehandler.into_param().abi(), ::core::mem::transmute(id), fregisterfornextanimationevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn HoldVariable<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable2>>(&self, variable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), variable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetSkipDuration(&self, secondsduration: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(secondsduration)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Schedule(&self, timenow: f64) -> ::windows::runtime::Result<UI_ANIMATION_SCHEDULING_RESULT> {
        let mut result__: <UI_ANIMATION_SCHEDULING_RESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), &mut result__).from_abi::<UI_ANIMATION_SCHEDULING_RESULT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Conclude(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Finish(&self, completiondeadline: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(completiondeadline)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Abandon(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetTag(&self, object: *mut ::core::option::Option<::windows::runtime::IUnknown>, id: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__: <UI_ANIMATION_STORYBOARD_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<UI_ANIMATION_STORYBOARD_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetElapsedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Animation`, `Win32_Foundation`*"]
    pub unsafe fn SetStoryboardEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboardEventHandler2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, handler: Param0, fregisterstatuschangefornextanimationevent: Param1, fregisterupdatefornextanimationevent: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), handler.into_param().abi(), fregisterstatuschangefornextanimationevent.into_param().abi(), fregisterupdatefornextanimationevent.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationStoryboard2 {
    type Vtable = IUIAnimationStoryboard2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2921897170, 4820, 18757, [148, 25, 158, 65, 190, 3, 77, 242]);
}
impl ::core::convert::From<IUIAnimationStoryboard2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationStoryboard2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationStoryboard2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationStoryboard2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationStoryboard2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationStoryboard2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboard2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, transition: ::windows::runtime::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: ::windows::runtime::RawPtr, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, delay: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, secondsduration: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, completiondeadline: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: *mut ::windows::runtime::RawPtr, id: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, elapsedtime: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationStoryboardEventHandler(pub ::windows::runtime::IUnknown);
impl IUIAnimationStoryboardEventHandler {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnStoryboardStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard>>(&self, storyboard: Param0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnStoryboardUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard>>(&self, storyboard: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), storyboard.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationStoryboardEventHandler {
    type Vtable = IUIAnimationStoryboardEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029476360, 60540, 17252, [159, 138, 154, 243, 197, 140, 186, 230]);
}
impl ::core::convert::From<IUIAnimationStoryboardEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationStoryboardEventHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationStoryboardEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationStoryboardEventHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationStoryboardEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationStoryboardEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboardEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: ::windows::runtime::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationStoryboardEventHandler2(pub ::windows::runtime::IUnknown);
impl IUIAnimationStoryboardEventHandler2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnStoryboardStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard2>>(&self, storyboard: Param0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnStoryboardUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard2>>(&self, storyboard: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), storyboard.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationStoryboardEventHandler2 {
    type Vtable = IUIAnimationStoryboardEventHandler2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3133535578, 47740, 16716, [181, 153, 251, 248, 80, 245, 83, 198]);
}
impl ::core::convert::From<IUIAnimationStoryboardEventHandler2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationStoryboardEventHandler2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationStoryboardEventHandler2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationStoryboardEventHandler2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationStoryboardEventHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationStoryboardEventHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboardEventHandler2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: ::windows::runtime::RawPtr, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTimer(pub ::windows::runtime::IUnknown);
impl IUIAnimationTimer {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetTimerUpdateHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationTimerUpdateHandler>>(&self, updatehandler: Param0, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), updatehandler.into_param().abi(), ::core::mem::transmute(idlebehavior)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetTimerEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationTimerEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Enable(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn Disable(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn IsEnabled(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetFrameRateThreshold(&self, framespersecond: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(framespersecond)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTimer {
    type Vtable = IUIAnimationTimer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1796143825, 41043, 16854, [144, 133, 51, 166, 137, 20, 70, 101]);
}
impl ::core::convert::From<IUIAnimationTimer> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTimer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTimer> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTimer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updatehandler: ::windows::runtime::RawPtr, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, seconds: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, framespersecond: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTimerClientEventHandler(pub ::windows::runtime::IUnknown);
impl IUIAnimationTimerClientEventHandler {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnTimerClientStatusChanged(&self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(newstatus), ::core::mem::transmute(previousstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTimerClientEventHandler {
    type Vtable = IUIAnimationTimerClientEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3202043318, 38138, 19451, [164, 127, 239, 45, 158, 64, 140, 37]);
}
impl ::core::convert::From<IUIAnimationTimerClientEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTimerClientEventHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTimerClientEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTimerClientEventHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTimerClientEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTimerClientEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerClientEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTimerEventHandler(pub ::windows::runtime::IUnknown);
impl IUIAnimationTimerEventHandler {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnPreUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnPostUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnRenderingTooSlow(&self, framespersecond: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(framespersecond)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTimerEventHandler {
    type Vtable = IUIAnimationTimerEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(659193322, 55153, 16533, [171, 189, 141, 247, 171, 210, 60, 227]);
}
impl ::core::convert::From<IUIAnimationTimerEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTimerEventHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTimerEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTimerEventHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTimerEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTimerEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, framespersecond: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTimerUpdateHandler(pub ::windows::runtime::IUnknown);
impl IUIAnimationTimerUpdateHandler {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnUpdate(&self, timenow: f64) -> ::windows::runtime::Result<UI_ANIMATION_UPDATE_RESULT> {
        let mut result__: <UI_ANIMATION_UPDATE_RESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(timenow), &mut result__).from_abi::<UI_ANIMATION_UPDATE_RESULT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetTimerClientEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationTimerClientEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn ClearTimerClientEventHandler(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTimerUpdateHandler {
    type Vtable = IUIAnimationTimerUpdateHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(425003447, 23902, 20030, [178, 120, 238, 55, 89, 179, 103, 173]);
}
impl ::core::convert::From<IUIAnimationTimerUpdateHandler> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTimerUpdateHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTimerUpdateHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTimerUpdateHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTimerUpdateHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTimerUpdateHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerUpdateHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTransition(pub ::windows::runtime::IUnknown);
impl IUIAnimationTransition {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetInitialValue(&self, value: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn IsDurationKnown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetDuration(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTransition {
    type Vtable = IUIAnimationTransition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3698123346, 63281, 16847, [182, 16, 97, 75, 108, 160, 73, 173]);
}
impl ::core::convert::From<IUIAnimationTransition> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTransition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTransition> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTransition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTransition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTransition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocity: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: *mut f64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTransition2(pub ::windows::runtime::IUnknown);
impl IUIAnimationTransition2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetDimension(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetInitialValue(&self, value: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetInitialVectorValue(&self, value: *const f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetInitialVectorVelocity(&self, velocity: *const f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(velocity), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn IsDurationKnown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetDuration(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTransition2 {
    type Vtable = IUIAnimationTransition2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1660916003, 43098, 20123, [162, 24, 67, 90, 147, 226, 104, 253]);
}
impl ::core::convert::From<IUIAnimationTransition2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTransition2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTransition2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTransition2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTransition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTransition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransition2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dimension: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *const f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocity: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, velocity: *const f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: *mut f64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTransitionFactory(pub ::windows::runtime::IUnknown);
impl IUIAnimationTransitionFactory {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateTransition<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationInterpolator>>(&self, interpolator: Param0) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), interpolator.into_param().abi(), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTransitionFactory {
    type Vtable = IUIAnimationTransitionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4242087427, 15931, 17837, [187, 177, 109, 252, 129, 83, 116, 61]);
}
impl ::core::convert::From<IUIAnimationTransitionFactory> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTransitionFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTransitionFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTransitionFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTransitionFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTransitionFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interpolator: ::windows::runtime::RawPtr, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTransitionFactory2(pub ::windows::runtime::IUnknown);
impl IUIAnimationTransitionFactory2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateTransition<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationInterpolator2>>(&self, interpolator: Param0) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), interpolator.into_param().abi(), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTransitionFactory2 {
    type Vtable = IUIAnimationTransitionFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2474461462, 49574, 17109, [136, 216, 48, 52, 77, 110, 254, 49]);
}
impl ::core::convert::From<IUIAnimationTransitionFactory2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTransitionFactory2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTransitionFactory2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTransitionFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTransitionFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTransitionFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interpolator: ::windows::runtime::RawPtr, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTransitionLibrary(pub ::windows::runtime::IUnknown);
impl IUIAnimationTransitionLibrary {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay), ::core::mem::transmute(finalvalue), ::core::mem::transmute(hold), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(speed), ::core::mem::transmute(finalvalue), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(period), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(minimumvalue), ::core::mem::transmute(maximumvalue), ::core::mem::transmute(period), ::core::mem::transmute(slope), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(accelerationratio), ::core::mem::transmute(decelerationratio), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(finalvelocity), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(maximumduration), ::core::mem::transmute(finalvalue), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows::runtime::Result<IUIAnimationTransition> {
        let mut result__: <IUIAnimationTransition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), ::core::mem::transmute(finalvelocity), ::core::mem::transmute(acceleration), &mut result__).from_abi::<IUIAnimationTransition>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTransitionLibrary {
    type Vtable = IUIAnimationTransitionLibrary_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3394901169, 53839, 18616, [143, 228, 199, 129, 105, 186, 149, 78]);
}
impl ::core::convert::From<IUIAnimationTransitionLibrary> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTransitionLibrary) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTransitionLibrary> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTransitionLibrary) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTransitionLibrary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTransitionLibrary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionLibrary_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, speed: f64, finalvalue: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, period: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationTransitionLibrary2(pub ::windows::runtime::IUnknown);
impl IUIAnimationTransitionLibrary2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateInstantaneousVectorTransition(&self, finalvalue: *const f64, cdimension: u32) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), ::core::mem::transmute(cdimension), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay), ::core::mem::transmute(finalvalue), ::core::mem::transmute(hold), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateDiscreteVectorTransition(&self, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(delay), ::core::mem::transmute(finalvalue), ::core::mem::transmute(cdimension), ::core::mem::transmute(hold), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateLinearVectorTransition(&self, duration: f64, finalvalue: *const f64, cdimension: u32) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(cdimension), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(speed), ::core::mem::transmute(finalvalue), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateLinearVectorTransitionFromSpeed(&self, speed: f64, finalvalue: *const f64, cdimension: u32) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(speed), ::core::mem::transmute(finalvalue), ::core::mem::transmute(cdimension), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(period), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(minimumvalue), ::core::mem::transmute(maximumvalue), ::core::mem::transmute(period), ::core::mem::transmute(slope), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(accelerationratio), ::core::mem::transmute(decelerationratio), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(finalvelocity), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateCubicVectorTransition(&self, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(finalvelocity), ::core::mem::transmute(cdimension), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(maximumduration), ::core::mem::transmute(finalvalue), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), ::core::mem::transmute(finalvelocity), ::core::mem::transmute(acceleration), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateCubicBezierLinearTransition(&self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(x1), ::core::mem::transmute(y1), ::core::mem::transmute(x2), ::core::mem::transmute(y2), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn CreateCubicBezierLinearVectorTransition(&self, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows::runtime::Result<IUIAnimationTransition2> {
        let mut result__: <IUIAnimationTransition2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(duration), ::core::mem::transmute(finalvalue), ::core::mem::transmute(cdimension), ::core::mem::transmute(x1), ::core::mem::transmute(y1), ::core::mem::transmute(x2), ::core::mem::transmute(y2), &mut result__).from_abi::<IUIAnimationTransition2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationTransitionLibrary2 {
    type Vtable = IUIAnimationTransitionLibrary2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(63942227, 38272, 20195, [179, 99, 46, 206, 81, 180, 175, 106]);
}
impl ::core::convert::From<IUIAnimationTransitionLibrary2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationTransitionLibrary2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationTransitionLibrary2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationTransitionLibrary2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationTransitionLibrary2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationTransitionLibrary2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionLibrary2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, delay: f64, finalvalue: f64, hold: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, speed: f64, finalvalue: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, period: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maximumduration: f64, finalvalue: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationVariable(pub ::windows::runtime::IUnknown);
impl IUIAnimationVariable {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetValue(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetFinalValue(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetPreviousValue(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetIntegerValue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetFinalIntegerValue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetPreviousIntegerValue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetCurrentStoryboard(&self) -> ::windows::runtime::Result<IUIAnimationStoryboard> {
        let mut result__: <IUIAnimationStoryboard as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUIAnimationStoryboard>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetLowerBound(&self, bound: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetUpperBound(&self, bound: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetTag(&self, object: *mut ::core::option::Option<::windows::runtime::IUnknown>, id: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetVariableChangeHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariableChangeHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetVariableIntegerChangeHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariableIntegerChangeHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationVariable {
    type Vtable = IUIAnimationVariable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2364453205, 10313, 19685, [148, 72, 145, 255, 112, 225, 228, 217]);
}
impl ::core::convert::From<IUIAnimationVariable> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationVariable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationVariable> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationVariable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationVariable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationVariable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, previousvalue: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, previousvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bound: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bound: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: *mut ::windows::runtime::RawPtr, id: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationVariable2(pub ::windows::runtime::IUnknown);
impl IUIAnimationVariable2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetDimension(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetValue(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetVectorValue(&self, value: *mut f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(cdimension)).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    #[doc = "*Required features: `Win32_UI_Animation`, `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn GetCurve<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::DirectComposition::IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    #[doc = "*Required features: `Win32_UI_Animation`, `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn GetVectorCurve(&self, animation: *const ::core::option::Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(animation), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetFinalValue(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetFinalVectorValue(&self, finalvalue: *mut f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetPreviousValue(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetPreviousVectorValue(&self, previousvalue: *mut f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(previousvalue), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetIntegerValue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetIntegerVectorValue(&self, value: *mut i32, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetFinalIntegerValue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetFinalIntegerVectorValue(&self, finalvalue: *mut i32, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(finalvalue), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetPreviousIntegerValue(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetPreviousIntegerVectorValue(&self, previousvalue: *mut i32, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(previousvalue), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetCurrentStoryboard(&self) -> ::windows::runtime::Result<IUIAnimationStoryboard2> {
        let mut result__: <IUIAnimationStoryboard2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUIAnimationStoryboard2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetLowerBound(&self, bound: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetLowerBoundVector(&self, bound: *const f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetUpperBound(&self, bound: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetUpperBoundVector(&self, bound: *const f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(bound), ::core::mem::transmute(cdimension)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn GetTag(&self, object: *mut ::core::option::Option<::windows::runtime::IUnknown>, id: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Animation`, `Win32_Foundation`*"]
    pub unsafe fn SetVariableChangeHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariableChangeHandler2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, handler: Param0, fregisterfornextanimationevent: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), handler.into_param().abi(), fregisterfornextanimationevent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Animation`, `Win32_Foundation`*"]
    pub unsafe fn SetVariableIntegerChangeHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariableIntegerChangeHandler2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, handler: Param0, fregisterfornextanimationevent: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), handler.into_param().abi(), fregisterfornextanimationevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn SetVariableCurveChangeHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariableCurveChangeHandler2>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), handler.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationVariable2 {
    type Vtable = IUIAnimationVariable2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1226093316, 38571, 17625, [158, 119, 213, 16, 155, 126, 116, 102]);
}
impl ::core::convert::From<IUIAnimationVariable2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationVariable2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationVariable2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationVariable2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationVariable2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationVariable2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariable2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dimension: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectComposition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, animation: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))] usize,
    #[cfg(feature = "Win32_Graphics_DirectComposition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, animation: *const ::windows::runtime::RawPtr, cdimension: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: *mut f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, previousvalue: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, previousvalue: *mut f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, finalvalue: *mut i32, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, previousvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, previousvalue: *mut i32, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bound: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bound: *const f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bound: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bound: *const f64, cdimension: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: *mut ::windows::runtime::RawPtr, id: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationVariableChangeHandler(pub ::windows::runtime::IUnknown);
impl IUIAnimationVariableChangeHandler {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnValueChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationVariable>>(&self, storyboard: Param0, variable: Param1, newvalue: f64, previousvalue: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), variable.into_param().abi(), ::core::mem::transmute(newvalue), ::core::mem::transmute(previousvalue)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationVariableChangeHandler {
    type Vtable = IUIAnimationVariableChangeHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1666758586, 34770, 17109, [191, 113, 130, 233, 25, 221, 88, 98]);
}
impl ::core::convert::From<IUIAnimationVariableChangeHandler> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationVariableChangeHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationVariableChangeHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationVariableChangeHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationVariableChangeHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationVariableChangeHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableChangeHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, newvalue: f64, previousvalue: f64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationVariableChangeHandler2(pub ::windows::runtime::IUnknown);
impl IUIAnimationVariableChangeHandler2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnValueChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard2>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationVariable2>>(&self, storyboard: Param0, variable: Param1, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), variable.into_param().abi(), ::core::mem::transmute(newvalue), ::core::mem::transmute(previousvalue), ::core::mem::transmute(cdimension)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationVariableChangeHandler2 {
    type Vtable = IUIAnimationVariableChangeHandler2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1672267986, 28334, 19376, [184, 121, 88, 109, 216, 207, 190, 66]);
}
impl ::core::convert::From<IUIAnimationVariableChangeHandler2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationVariableChangeHandler2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationVariableChangeHandler2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationVariableChangeHandler2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationVariableChangeHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationVariableChangeHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableChangeHandler2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationVariableCurveChangeHandler2(pub ::windows::runtime::IUnknown);
impl IUIAnimationVariableCurveChangeHandler2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnCurveChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationVariable2>>(&self, variable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), variable.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationVariableCurveChangeHandler2 {
    type Vtable = IUIAnimationVariableCurveChangeHandler2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1921605265, 325, 19489, [145, 146, 90, 171, 64, 237, 223, 128]);
}
impl ::core::convert::From<IUIAnimationVariableCurveChangeHandler2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationVariableCurveChangeHandler2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationVariableCurveChangeHandler2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationVariableCurveChangeHandler2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationVariableCurveChangeHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationVariableCurveChangeHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableCurveChangeHandler2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationVariableIntegerChangeHandler(pub ::windows::runtime::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnIntegerValueChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationVariable>>(&self, storyboard: Param0, variable: Param1, newvalue: i32, previousvalue: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), variable.into_param().abi(), ::core::mem::transmute(newvalue), ::core::mem::transmute(previousvalue)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationVariableIntegerChangeHandler {
    type Vtable = IUIAnimationVariableIntegerChangeHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3141408080, 13678, 17584, [153, 218, 133, 172, 96, 23, 134, 94]);
}
impl ::core::convert::From<IUIAnimationVariableIntegerChangeHandler> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationVariableIntegerChangeHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationVariableIntegerChangeHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationVariableIntegerChangeHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationVariableIntegerChangeHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationVariableIntegerChangeHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableIntegerChangeHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, newvalue: i32, previousvalue: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIAnimationVariableIntegerChangeHandler2(pub ::windows::runtime::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler2 {
    #[doc = "*Required features: `Win32_UI_Animation`*"]
    pub unsafe fn OnIntegerValueChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IUIAnimationStoryboard2>, Param1: ::windows::runtime::IntoParam<'a, IUIAnimationVariable2>>(&self, storyboard: Param0, variable: Param1, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), storyboard.into_param().abi(), variable.into_param().abi(), ::core::mem::transmute(newvalue), ::core::mem::transmute(previousvalue), ::core::mem::transmute(cdimension)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIAnimationVariableIntegerChangeHandler2 {
    type Vtable = IUIAnimationVariableIntegerChangeHandler2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2191224049, 20282, 17426, [174, 9, 178, 67, 235, 76, 107, 88]);
}
impl ::core::convert::From<IUIAnimationVariableIntegerChangeHandler2> for ::windows::runtime::IUnknown {
    fn from(value: IUIAnimationVariableIntegerChangeHandler2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIAnimationVariableIntegerChangeHandler2> for ::windows::runtime::IUnknown {
    fn from(value: &IUIAnimationVariableIntegerChangeHandler2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIAnimationVariableIntegerChangeHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIAnimationVariableIntegerChangeHandler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableIntegerChangeHandler2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storyboard: ::windows::runtime::RawPtr, variable: ::windows::runtime::RawPtr, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows::runtime::HRESULT,
);
pub const UIAnimationManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1277150778, 26972, 18408, [163, 57, 26, 25, 75, 227, 208, 184]);
pub const UIAnimationManager2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3529345090, 34948, 19018, [179, 33, 9, 19, 20, 55, 155, 221]);
pub const UIAnimationTimer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3217902092, 1718, 17284, [183, 104, 13, 170, 121, 44, 56, 14]);
pub const UIAnimationTransitionFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2325421277, 64727, 16796, [139, 68, 66, 253, 23, 219, 24, 135]);
pub const UIAnimationTransitionFactory2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2217750423, 32635, 16448, [177, 144, 114, 172, 157, 24, 228, 32]);
pub const UIAnimationTransitionLibrary: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(493036205, 43653, 20213, [168, 40, 134, 215, 16, 103, 209, 69]);
pub const UIAnimationTransitionLibrary2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2167379018, 50632, 19673, [176, 166, 179, 218, 128, 47, 34, 141]);
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_DEPENDENCIES(pub u32);
pub const UI_ANIMATION_DEPENDENCY_NONE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(0u32);
pub const UI_ANIMATION_DEPENDENCY_INTERMEDIATE_VALUES: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(1u32);
pub const UI_ANIMATION_DEPENDENCY_FINAL_VALUE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(2u32);
pub const UI_ANIMATION_DEPENDENCY_FINAL_VELOCITY: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(4u32);
pub const UI_ANIMATION_DEPENDENCY_DURATION: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(8u32);
impl ::core::convert::From<u32> for UI_ANIMATION_DEPENDENCIES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_DEPENDENCIES {
    type Abi = Self;
}
impl ::core::ops::BitOr for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_IDLE_BEHAVIOR(pub i32);
pub const UI_ANIMATION_IDLE_BEHAVIOR_CONTINUE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(0i32);
pub const UI_ANIMATION_IDLE_BEHAVIOR_DISABLE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(1i32);
impl ::core::convert::From<i32> for UI_ANIMATION_IDLE_BEHAVIOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_IDLE_BEHAVIOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct UI_ANIMATION_KEYFRAME(pub isize);
impl ::core::default::Default for UI_ANIMATION_KEYFRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for UI_ANIMATION_KEYFRAME {}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_KEYFRAME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_MANAGER_STATUS(pub i32);
pub const UI_ANIMATION_MANAGER_IDLE: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(0i32);
pub const UI_ANIMATION_MANAGER_BUSY: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(1i32);
impl ::core::convert::From<i32> for UI_ANIMATION_MANAGER_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_MANAGER_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_MODE(pub i32);
pub const UI_ANIMATION_MODE_DISABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(0i32);
pub const UI_ANIMATION_MODE_SYSTEM_DEFAULT: UI_ANIMATION_MODE = UI_ANIMATION_MODE(1i32);
pub const UI_ANIMATION_MODE_ENABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(2i32);
impl ::core::convert::From<i32> for UI_ANIMATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_PRIORITY_EFFECT(pub i32);
pub const UI_ANIMATION_PRIORITY_EFFECT_FAILURE: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(0i32);
pub const UI_ANIMATION_PRIORITY_EFFECT_DELAY: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(1i32);
impl ::core::convert::From<i32> for UI_ANIMATION_PRIORITY_EFFECT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_PRIORITY_EFFECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START: i32 = -2i32;
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_REPEAT_MODE(pub i32);
pub const UI_ANIMATION_REPEAT_MODE_NORMAL: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(0i32);
pub const UI_ANIMATION_REPEAT_MODE_ALTERNATE: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(1i32);
impl ::core::convert::From<i32> for UI_ANIMATION_REPEAT_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_REPEAT_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_ROUNDING_MODE(pub i32);
pub const UI_ANIMATION_ROUNDING_NEAREST: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(0i32);
pub const UI_ANIMATION_ROUNDING_FLOOR: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(1i32);
pub const UI_ANIMATION_ROUNDING_CEILING: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(2i32);
impl ::core::convert::From<i32> for UI_ANIMATION_ROUNDING_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_ROUNDING_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_SCHEDULING_RESULT(pub i32);
pub const UI_ANIMATION_SCHEDULING_UNEXPECTED_FAILURE: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(0i32);
pub const UI_ANIMATION_SCHEDULING_INSUFFICIENT_PRIORITY: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(1i32);
pub const UI_ANIMATION_SCHEDULING_ALREADY_SCHEDULED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(2i32);
pub const UI_ANIMATION_SCHEDULING_SUCCEEDED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(3i32);
pub const UI_ANIMATION_SCHEDULING_DEFERRED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(4i32);
impl ::core::convert::From<i32> for UI_ANIMATION_SCHEDULING_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_SCHEDULING_RESULT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_SECONDS_EVENTUALLY: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Animation`*"]
pub const UI_ANIMATION_SECONDS_INFINITE: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_SLOPE(pub i32);
pub const UI_ANIMATION_SLOPE_INCREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(0i32);
pub const UI_ANIMATION_SLOPE_DECREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(1i32);
impl ::core::convert::From<i32> for UI_ANIMATION_SLOPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_SLOPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_STORYBOARD_STATUS(pub i32);
pub const UI_ANIMATION_STORYBOARD_BUILDING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(0i32);
pub const UI_ANIMATION_STORYBOARD_SCHEDULED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(1i32);
pub const UI_ANIMATION_STORYBOARD_CANCELLED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(2i32);
pub const UI_ANIMATION_STORYBOARD_PLAYING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(3i32);
pub const UI_ANIMATION_STORYBOARD_TRUNCATED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(4i32);
pub const UI_ANIMATION_STORYBOARD_FINISHED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(5i32);
pub const UI_ANIMATION_STORYBOARD_READY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(6i32);
pub const UI_ANIMATION_STORYBOARD_INSUFFICIENT_PRIORITY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(7i32);
impl ::core::convert::From<i32> for UI_ANIMATION_STORYBOARD_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_STORYBOARD_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_TIMER_CLIENT_STATUS(pub i32);
pub const UI_ANIMATION_TIMER_CLIENT_IDLE: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(0i32);
pub const UI_ANIMATION_TIMER_CLIENT_BUSY: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(1i32);
impl ::core::convert::From<i32> for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_TIMER_CLIENT_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Animation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_ANIMATION_UPDATE_RESULT(pub i32);
pub const UI_ANIMATION_UPDATE_NO_CHANGE: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(0i32);
pub const UI_ANIMATION_UPDATE_VARIABLES_CHANGED: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(1i32);
impl ::core::convert::From<i32> for UI_ANIMATION_UPDATE_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UI_ANIMATION_UPDATE_RESULT {
    type Abi = Self;
}
