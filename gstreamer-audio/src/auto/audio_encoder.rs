// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gst;
use gst_audio_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use AudioInfo;

glib_wrapper! {
    pub struct AudioEncoder(Object<gst_audio_sys::GstAudioEncoder, gst_audio_sys::GstAudioEncoderClass, AudioEncoderClass>) @extends gst::Element, gst::Object;

    match fn {
        get_type => || gst_audio_sys::gst_audio_encoder_get_type(),
    }
}

unsafe impl Send for AudioEncoder {}
unsafe impl Sync for AudioEncoder {}

pub const NONE_AUDIO_ENCODER: Option<&AudioEncoder> = None;

pub trait AudioEncoderExt: 'static {
    fn allocate_output_buffer(&self, size: usize) -> Result<gst::Buffer, glib::BoolError>;

    fn get_audio_info(&self) -> Option<AudioInfo>;

    fn get_drainable(&self) -> bool;

    fn get_frame_max(&self) -> i32;

    fn get_frame_samples_max(&self) -> i32;

    fn get_frame_samples_min(&self) -> i32;

    fn get_hard_min(&self) -> bool;

    fn get_hard_resync(&self) -> bool;

    fn get_lookahead(&self) -> i32;

    fn get_mark_granule(&self) -> bool;

    fn get_perfect_timestamp(&self) -> bool;

    fn get_tolerance(&self) -> gst::ClockTime;

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode);

    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_allocation_caps(&self, allocation_caps: Option<&gst::Caps>);

    fn set_drainable(&self, enabled: bool);

    fn set_frame_max(&self, num: i32);

    fn set_frame_samples_max(&self, num: i32);

    fn set_frame_samples_min(&self, num: i32);

    fn set_hard_min(&self, enabled: bool);

    fn set_hard_resync(&self, enabled: bool);

    fn set_headers(&self, headers: &[&gst::Buffer]);

    fn set_latency(&self, min: gst::ClockTime, max: gst::ClockTime);

    fn set_lookahead(&self, num: i32);

    fn set_mark_granule(&self, enabled: bool);

    fn set_perfect_timestamp(&self, enabled: bool);

    fn set_tolerance(&self, tolerance: gst::ClockTime);

    fn connect_property_hard_resync_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_mark_granule_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_perfect_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AudioEncoder>> AudioEncoderExt for O {
    fn allocate_output_buffer(&self, size: usize) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_audio_sys::gst_audio_encoder_allocate_output_buffer(
                self.as_ref().to_glib_none().0,
                size,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to allocate output buffer"))
        }
    }

    fn get_audio_info(&self) -> Option<AudioInfo> {
        unsafe {
            from_glib_full(gst_audio_sys::gst_audio_encoder_get_audio_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_drainable(&self) -> bool {
        unsafe {
            from_glib(gst_audio_sys::gst_audio_encoder_get_drainable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_frame_max(&self) -> i32 {
        unsafe { gst_audio_sys::gst_audio_encoder_get_frame_max(self.as_ref().to_glib_none().0) }
    }

    fn get_frame_samples_max(&self) -> i32 {
        unsafe {
            gst_audio_sys::gst_audio_encoder_get_frame_samples_max(self.as_ref().to_glib_none().0)
        }
    }

    fn get_frame_samples_min(&self) -> i32 {
        unsafe {
            gst_audio_sys::gst_audio_encoder_get_frame_samples_min(self.as_ref().to_glib_none().0)
        }
    }

    fn get_hard_min(&self) -> bool {
        unsafe {
            from_glib(gst_audio_sys::gst_audio_encoder_get_hard_min(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_hard_resync(&self) -> bool {
        unsafe {
            from_glib(gst_audio_sys::gst_audio_encoder_get_hard_resync(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_lookahead(&self) -> i32 {
        unsafe { gst_audio_sys::gst_audio_encoder_get_lookahead(self.as_ref().to_glib_none().0) }
    }

    fn get_mark_granule(&self) -> bool {
        unsafe {
            from_glib(gst_audio_sys::gst_audio_encoder_get_mark_granule(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_perfect_timestamp(&self) -> bool {
        unsafe {
            from_glib(gst_audio_sys::gst_audio_encoder_get_perfect_timestamp(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_tolerance(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_audio_sys::gst_audio_encoder_get_tolerance(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            from_glib_full(gst_audio_sys::gst_audio_encoder_proxy_getcaps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_allocation_caps(&self, allocation_caps: Option<&gst::Caps>) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_allocation_caps(
                self.as_ref().to_glib_none().0,
                allocation_caps.to_glib_none().0,
            );
        }
    }

    fn set_drainable(&self, enabled: bool) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_drainable(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_frame_max(&self, num: i32) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_frame_max(self.as_ref().to_glib_none().0, num);
        }
    }

    fn set_frame_samples_max(&self, num: i32) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_frame_samples_max(
                self.as_ref().to_glib_none().0,
                num,
            );
        }
    }

    fn set_frame_samples_min(&self, num: i32) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_frame_samples_min(
                self.as_ref().to_glib_none().0,
                num,
            );
        }
    }

    fn set_hard_min(&self, enabled: bool) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_hard_min(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_hard_resync(&self, enabled: bool) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_hard_resync(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_headers(&self, headers: &[&gst::Buffer]) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_headers(
                self.as_ref().to_glib_none().0,
                headers.to_glib_full(),
            );
        }
    }

    fn set_latency(&self, min: gst::ClockTime, max: gst::ClockTime) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_latency(
                self.as_ref().to_glib_none().0,
                min.to_glib(),
                max.to_glib(),
            );
        }
    }

    fn set_lookahead(&self, num: i32) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_lookahead(self.as_ref().to_glib_none().0, num);
        }
    }

    fn set_mark_granule(&self, enabled: bool) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_mark_granule(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_perfect_timestamp(&self, enabled: bool) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_perfect_timestamp(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_tolerance(&self, tolerance: gst::ClockTime) {
        unsafe {
            gst_audio_sys::gst_audio_encoder_set_tolerance(
                self.as_ref().to_glib_none().0,
                tolerance.to_glib(),
            );
        }
    }

    fn connect_property_hard_resync_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_hard_resync_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_audio_sys::GstAudioEncoder,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioEncoder>,
        {
            let f: &F = &*(f as *const F);
            f(&AudioEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hard-resync\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hard_resync_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mark_granule_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mark_granule_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_audio_sys::GstAudioEncoder,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioEncoder>,
        {
            let f: &F = &*(f as *const F);
            f(&AudioEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mark-granule\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mark_granule_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_perfect_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_perfect_timestamp_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_audio_sys::GstAudioEncoder,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioEncoder>,
        {
            let f: &F = &*(f as *const F);
            f(&AudioEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::perfect-timestamp\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_perfect_timestamp_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tolerance_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_audio_sys::GstAudioEncoder,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AudioEncoder>,
        {
            let f: &F = &*(f as *const F);
            f(&AudioEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tolerance\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tolerance_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
