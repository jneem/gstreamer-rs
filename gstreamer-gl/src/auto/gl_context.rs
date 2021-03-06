// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_gl_sys;
use std::mem;
use std::ptr;
use GLDisplay;
use GLPlatform;
use GLSLProfile;
use GLSLVersion;
use GLWindow;
use GLAPI;

glib_wrapper! {
    pub struct GLContext(Object<gst_gl_sys::GstGLContext, gst_gl_sys::GstGLContextClass, GLContextClass>) @extends gst::Object;

    match fn {
        get_type => || gst_gl_sys::gst_gl_context_get_type(),
    }
}

impl GLContext {
    pub fn new<P: IsA<GLDisplay>>(display: &P) -> GLContext {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(gst_gl_sys::gst_gl_context_new(
                display.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn get_current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gst_gl_sys::gst_gl_context_get_current()) }
    }

    pub fn get_current_gl_api(platform: GLPlatform) -> (GLAPI, u32, u32) {
        assert_initialized_main_thread!();
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            let ret = from_glib(gst_gl_sys::gst_gl_context_get_current_gl_api(
                platform.to_glib(),
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            ));
            let major = major.assume_init();
            let minor = minor.assume_init();
            (ret, major, minor)
        }
    }
}

unsafe impl Send for GLContext {}
unsafe impl Sync for GLContext {}

pub const NONE_GL_CONTEXT: Option<&GLContext> = None;

pub trait GLContextExt: 'static {
    fn activate(&self, activate: bool) -> Result<(), glib::error::BoolError>;

    fn can_share<P: IsA<GLContext>>(&self, other_context: &P) -> bool;

    fn check_feature(&self, feature: &str) -> bool;

    fn check_framebuffer_status(&self, fbo_target: u32) -> bool;

    fn check_gl_version(&self, api: GLAPI, maj: i32, min: i32) -> bool;

    fn clear_framebuffer(&self);

    fn clear_shader(&self);

    fn create<P: IsA<GLContext>>(&self, other_context: Option<&P>) -> Result<(), glib::Error>;

    fn destroy(&self);

    fn fill_info(&self) -> Result<(), glib::Error>;

    fn get_display(&self) -> GLDisplay;

    fn get_gl_api(&self) -> GLAPI;

    fn get_gl_platform(&self) -> GLPlatform;

    fn get_gl_platform_version(&self) -> (i32, i32);

    fn get_gl_version(&self) -> (i32, i32);

    fn get_window(&self) -> Option<GLWindow>;

    fn is_shared(&self) -> bool;

    fn set_shared_with<P: IsA<GLContext>>(&self, share: &P);

    fn set_window<P: IsA<GLWindow>>(&self, window: &P) -> Result<(), glib::error::BoolError>;

    fn supports_glsl_profile_version(&self, version: GLSLVersion, profile: GLSLProfile) -> bool;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn supports_precision(&self, version: GLSLVersion, profile: GLSLProfile) -> bool;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn supports_precision_highp(&self, version: GLSLVersion, profile: GLSLProfile) -> bool;

    fn swap_buffers(&self);
}

impl<O: IsA<GLContext>> GLContextExt for O {
    fn activate(&self, activate: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_gl_sys::gst_gl_context_activate(
                    self.as_ref().to_glib_none().0,
                    activate.to_glib()
                ),
                "Failed to activate OpenGL context"
            )
        }
    }

    fn can_share<P: IsA<GLContext>>(&self, other_context: &P) -> bool {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_can_share(
                self.as_ref().to_glib_none().0,
                other_context.as_ref().to_glib_none().0,
            ))
        }
    }

    fn check_feature(&self, feature: &str) -> bool {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_check_feature(
                self.as_ref().to_glib_none().0,
                feature.to_glib_none().0,
            ))
        }
    }

    fn check_framebuffer_status(&self, fbo_target: u32) -> bool {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_check_framebuffer_status(
                self.as_ref().to_glib_none().0,
                fbo_target,
            ))
        }
    }

    fn check_gl_version(&self, api: GLAPI, maj: i32, min: i32) -> bool {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_check_gl_version(
                self.as_ref().to_glib_none().0,
                api.to_glib(),
                maj,
                min,
            ))
        }
    }

    fn clear_framebuffer(&self) {
        unsafe {
            gst_gl_sys::gst_gl_context_clear_framebuffer(self.as_ref().to_glib_none().0);
        }
    }

    fn clear_shader(&self) {
        unsafe {
            gst_gl_sys::gst_gl_context_clear_shader(self.as_ref().to_glib_none().0);
        }
    }

    fn create<P: IsA<GLContext>>(&self, other_context: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gst_gl_sys::gst_gl_context_create(
                self.as_ref().to_glib_none().0,
                other_context.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn destroy(&self) {
        unsafe {
            gst_gl_sys::gst_gl_context_destroy(self.as_ref().to_glib_none().0);
        }
    }

    fn fill_info(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ =
                gst_gl_sys::gst_gl_context_fill_info(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_display(&self) -> GLDisplay {
        unsafe {
            from_glib_full(gst_gl_sys::gst_gl_context_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_gl_api(&self) -> GLAPI {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_get_gl_api(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_gl_platform(&self) -> GLPlatform {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_get_gl_platform(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_gl_platform_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            gst_gl_sys::gst_gl_context_get_gl_platform_version(
                self.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            let major = major.assume_init();
            let minor = minor.assume_init();
            (major, minor)
        }
    }

    fn get_gl_version(&self) -> (i32, i32) {
        unsafe {
            let mut maj = mem::MaybeUninit::uninit();
            let mut min = mem::MaybeUninit::uninit();
            gst_gl_sys::gst_gl_context_get_gl_version(
                self.as_ref().to_glib_none().0,
                maj.as_mut_ptr(),
                min.as_mut_ptr(),
            );
            let maj = maj.assume_init();
            let min = min.assume_init();
            (maj, min)
        }
    }

    fn get_window(&self) -> Option<GLWindow> {
        unsafe {
            from_glib_full(gst_gl_sys::gst_gl_context_get_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_shared(&self) -> bool {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_is_shared(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_shared_with<P: IsA<GLContext>>(&self, share: &P) {
        unsafe {
            gst_gl_sys::gst_gl_context_set_shared_with(
                self.as_ref().to_glib_none().0,
                share.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_window<P: IsA<GLWindow>>(&self, window: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_gl_sys::gst_gl_context_set_window(
                    self.as_ref().to_glib_none().0,
                    window.as_ref().to_glib_full()
                ),
                "Failed to set window"
            )
        }
    }

    fn supports_glsl_profile_version(&self, version: GLSLVersion, profile: GLSLProfile) -> bool {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_supports_glsl_profile_version(
                self.as_ref().to_glib_none().0,
                version.to_glib(),
                profile.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn supports_precision(&self, version: GLSLVersion, profile: GLSLProfile) -> bool {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_supports_precision(
                self.as_ref().to_glib_none().0,
                version.to_glib(),
                profile.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn supports_precision_highp(&self, version: GLSLVersion, profile: GLSLProfile) -> bool {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_context_supports_precision_highp(
                self.as_ref().to_glib_none().0,
                version.to_glib(),
                profile.to_glib(),
            ))
        }
    }

    fn swap_buffers(&self) {
        unsafe {
            gst_gl_sys::gst_gl_context_swap_buffers(self.as_ref().to_glib_none().0);
        }
    }
}
