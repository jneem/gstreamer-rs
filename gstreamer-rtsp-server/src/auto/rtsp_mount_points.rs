// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use gst_rtsp;
use gst_rtsp_server_sys;
use std::mem;
use RTSPMediaFactory;

glib_wrapper! {
    pub struct RTSPMountPoints(Object<gst_rtsp_server_sys::GstRTSPMountPoints, gst_rtsp_server_sys::GstRTSPMountPointsClass, RTSPMountPointsClass>);

    match fn {
        get_type => || gst_rtsp_server_sys::gst_rtsp_mount_points_get_type(),
    }
}

impl RTSPMountPoints {
    pub fn new() -> RTSPMountPoints {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_rtsp_server_sys::gst_rtsp_mount_points_new()) }
    }
}

impl Default for RTSPMountPoints {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPMountPoints {}
unsafe impl Sync for RTSPMountPoints {}

pub const NONE_RTSP_MOUNT_POINTS: Option<&RTSPMountPoints> = None;

pub trait RTSPMountPointsExt: 'static {
    fn add_factory<P: IsA<RTSPMediaFactory>>(&self, path: &str, factory: &P);

    fn make_path(&self, url: &gst_rtsp::RTSPUrl) -> Result<GString, glib::BoolError>;

    fn match_(&self, path: &str) -> (RTSPMediaFactory, i32);

    fn remove_factory(&self, path: &str);
}

impl<O: IsA<RTSPMountPoints>> RTSPMountPointsExt for O {
    fn add_factory<P: IsA<RTSPMediaFactory>>(&self, path: &str, factory: &P) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_mount_points_add_factory(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                factory.as_ref().to_glib_full(),
            );
        }
    }

    fn make_path(&self, url: &gst_rtsp::RTSPUrl) -> Result<GString, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_rtsp_server_sys::gst_rtsp_mount_points_make_path(
                self.as_ref().to_glib_none().0,
                url.to_glib_none().0,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to make path"))
        }
    }

    fn match_(&self, path: &str) -> (RTSPMediaFactory, i32) {
        unsafe {
            let mut matched = mem::MaybeUninit::uninit();
            let ret = from_glib_full(gst_rtsp_server_sys::gst_rtsp_mount_points_match(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                matched.as_mut_ptr(),
            ));
            let matched = matched.assume_init();
            (ret, matched)
        }
    }

    fn remove_factory(&self, path: &str) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_mount_points_remove_factory(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }
}
