// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_gl_sys;
use GLContext;

glib_wrapper! {
    pub struct GLColorConvert(Object<gst_gl_sys::GstGLColorConvert, gst_gl_sys::GstGLColorConvertClass, GLColorConvertClass>) @extends gst::Object;

    match fn {
        get_type => || gst_gl_sys::gst_gl_color_convert_get_type(),
    }
}

impl GLColorConvert {
    pub fn new<P: IsA<GLContext>>(context: &P) -> GLColorConvert {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gst_gl_sys::gst_gl_color_convert_new(
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn set_caps(
        &self,
        in_caps: &gst::Caps,
        out_caps: &gst::Caps,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_gl_sys::gst_gl_color_convert_set_caps(
                    self.to_glib_none().0,
                    in_caps.to_glib_none().0,
                    out_caps.to_glib_none().0
                ),
                "Failed to set caps"
            )
        }
    }

    pub fn transform_caps<P: IsA<GLContext>>(
        context: &P,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: &gst::Caps,
    ) -> Option<gst::Caps> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gst_gl_sys::gst_gl_color_convert_transform_caps(
                context.as_ref().to_glib_none().0,
                direction.to_glib(),
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for GLColorConvert {}
unsafe impl Sync for GLColorConvert {}
