/* automatically generated by rust-bindgen 0.59.1 */

pub type ULONG = ::std::os::raw::c_ulong;
pub type PULONG = *mut ULONG;
pub type USHORT = ::std::os::raw::c_ushort;
pub type UCHAR = ::std::os::raw::c_uchar;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type SHORT = ::std::os::raw::c_short;
pub const _VIGEM_TARGET_TYPE_Xbox360Wired: _VIGEM_TARGET_TYPE = 0;
pub const _VIGEM_TARGET_TYPE_DualShock4Wired: _VIGEM_TARGET_TYPE = 2;
pub type _VIGEM_TARGET_TYPE = ::std::os::raw::c_int;
pub use self::_VIGEM_TARGET_TYPE as VIGEM_TARGET_TYPE;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_UP: _XUSB_BUTTON = 1;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_DOWN: _XUSB_BUTTON = 2;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_LEFT: _XUSB_BUTTON = 4;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_DPAD_RIGHT: _XUSB_BUTTON = 8;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_START: _XUSB_BUTTON = 16;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_BACK: _XUSB_BUTTON = 32;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_LEFT_THUMB: _XUSB_BUTTON = 64;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_RIGHT_THUMB: _XUSB_BUTTON = 128;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_LEFT_SHOULDER: _XUSB_BUTTON = 256;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_RIGHT_SHOULDER: _XUSB_BUTTON = 512;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_GUIDE: _XUSB_BUTTON = 1024;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_A: _XUSB_BUTTON = 4096;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_B: _XUSB_BUTTON = 8192;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_X: _XUSB_BUTTON = 16384;
pub const _XUSB_BUTTON_XUSB_GAMEPAD_Y: _XUSB_BUTTON = 32768;
pub type _XUSB_BUTTON = ::std::os::raw::c_int;
pub use self::_XUSB_BUTTON as XUSB_BUTTON;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XUSB_REPORT {
    pub wButtons: USHORT,
    pub bLeftTrigger: BYTE,
    pub bRightTrigger: BYTE,
    pub sThumbLX: SHORT,
    pub sThumbLY: SHORT,
    pub sThumbRX: SHORT,
    pub sThumbRY: SHORT,
}
#[test]
fn bindgen_test_layout__XUSB_REPORT() {
    assert_eq!(::std::mem::size_of::<_XUSB_REPORT>(), 12usize, concat!("Size of: ", stringify!(_XUSB_REPORT)));
    assert_eq!(::std::mem::align_of::<_XUSB_REPORT>(), 2usize, concat!("Alignment of ", stringify!(_XUSB_REPORT)));
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).wButtons as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(_XUSB_REPORT), "::", stringify!(wButtons))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).bLeftTrigger as *const _ as usize },
        2usize,
        concat!("Offset of field: ", stringify!(_XUSB_REPORT), "::", stringify!(bLeftTrigger))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).bRightTrigger as *const _ as usize },
        3usize,
        concat!("Offset of field: ", stringify!(_XUSB_REPORT), "::", stringify!(bRightTrigger))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).sThumbLX as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(_XUSB_REPORT), "::", stringify!(sThumbLX))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).sThumbLY as *const _ as usize },
        6usize,
        concat!("Offset of field: ", stringify!(_XUSB_REPORT), "::", stringify!(sThumbLY))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).sThumbRX as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(_XUSB_REPORT), "::", stringify!(sThumbRX))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).sThumbRY as *const _ as usize },
        10usize,
        concat!("Offset of field: ", stringify!(_XUSB_REPORT), "::", stringify!(sThumbRY))
    );
}
pub type XUSB_REPORT = _XUSB_REPORT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DS4_LIGHTBAR_COLOR {
    pub Red: UCHAR,
    pub Green: UCHAR,
    pub Blue: UCHAR,
}
#[test]
fn bindgen_test_layout__DS4_LIGHTBAR_COLOR() {
    assert_eq!(
        ::std::mem::size_of::<_DS4_LIGHTBAR_COLOR>(),
        3usize,
        concat!("Size of: ", stringify!(_DS4_LIGHTBAR_COLOR))
    );
    assert_eq!(
        ::std::mem::align_of::<_DS4_LIGHTBAR_COLOR>(),
        1usize,
        concat!("Alignment of ", stringify!(_DS4_LIGHTBAR_COLOR))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_LIGHTBAR_COLOR>())).Red as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(_DS4_LIGHTBAR_COLOR), "::", stringify!(Red))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_LIGHTBAR_COLOR>())).Green as *const _ as usize },
        1usize,
        concat!("Offset of field: ", stringify!(_DS4_LIGHTBAR_COLOR), "::", stringify!(Green))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_LIGHTBAR_COLOR>())).Blue as *const _ as usize },
        2usize,
        concat!("Offset of field: ", stringify!(_DS4_LIGHTBAR_COLOR), "::", stringify!(Blue))
    );
}
pub type DS4_LIGHTBAR_COLOR = _DS4_LIGHTBAR_COLOR;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DS4_REPORT {
    pub bThumbLX: BYTE,
    pub bThumbLY: BYTE,
    pub bThumbRX: BYTE,
    pub bThumbRY: BYTE,
    pub wButtons: USHORT,
    pub bSpecial: BYTE,
    pub bTriggerL: BYTE,
    pub bTriggerR: BYTE,
}
#[test]
fn bindgen_test_layout__DS4_REPORT() {
    assert_eq!(::std::mem::size_of::<_DS4_REPORT>(), 10usize, concat!("Size of: ", stringify!(_DS4_REPORT)));
    assert_eq!(::std::mem::align_of::<_DS4_REPORT>(), 2usize, concat!("Alignment of ", stringify!(_DS4_REPORT)));
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bThumbLX as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(_DS4_REPORT), "::", stringify!(bThumbLX))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bThumbLY as *const _ as usize },
        1usize,
        concat!("Offset of field: ", stringify!(_DS4_REPORT), "::", stringify!(bThumbLY))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bThumbRX as *const _ as usize },
        2usize,
        concat!("Offset of field: ", stringify!(_DS4_REPORT), "::", stringify!(bThumbRX))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bThumbRY as *const _ as usize },
        3usize,
        concat!("Offset of field: ", stringify!(_DS4_REPORT), "::", stringify!(bThumbRY))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).wButtons as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(_DS4_REPORT), "::", stringify!(wButtons))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bSpecial as *const _ as usize },
        6usize,
        concat!("Offset of field: ", stringify!(_DS4_REPORT), "::", stringify!(bSpecial))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bTriggerL as *const _ as usize },
        7usize,
        concat!("Offset of field: ", stringify!(_DS4_REPORT), "::", stringify!(bTriggerL))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bTriggerR as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(_DS4_REPORT), "::", stringify!(bTriggerR))
    );
}
pub type DS4_REPORT = _DS4_REPORT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DS4_TOUCH {
    pub bPacketCounter: BYTE,
    pub bIsUpTrackingNum1: BYTE,
    pub bTouchData1: [BYTE; 3usize],
    pub bIsUpTrackingNum2: BYTE,
    pub bTouchData2: [BYTE; 3usize],
}
#[test]
fn bindgen_test_layout__DS4_TOUCH() {
    assert_eq!(::std::mem::size_of::<_DS4_TOUCH>(), 9usize, concat!("Size of: ", stringify!(_DS4_TOUCH)));
    assert_eq!(::std::mem::align_of::<_DS4_TOUCH>(), 1usize, concat!("Alignment of ", stringify!(_DS4_TOUCH)));
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_TOUCH>())).bPacketCounter as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(_DS4_TOUCH), "::", stringify!(bPacketCounter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_TOUCH>())).bIsUpTrackingNum1 as *const _ as usize },
        1usize,
        concat!("Offset of field: ", stringify!(_DS4_TOUCH), "::", stringify!(bIsUpTrackingNum1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_TOUCH>())).bTouchData1 as *const _ as usize },
        2usize,
        concat!("Offset of field: ", stringify!(_DS4_TOUCH), "::", stringify!(bTouchData1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_TOUCH>())).bIsUpTrackingNum2 as *const _ as usize },
        5usize,
        concat!("Offset of field: ", stringify!(_DS4_TOUCH), "::", stringify!(bIsUpTrackingNum2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_TOUCH>())).bTouchData2 as *const _ as usize },
        6usize,
        concat!("Offset of field: ", stringify!(_DS4_TOUCH), "::", stringify!(bTouchData2))
    );
}
pub type DS4_TOUCH = _DS4_TOUCH;
pub const _VIGEM_ERRORS_VIGEM_ERROR_NONE: _VIGEM_ERRORS = 536870912;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_NOT_FOUND: _VIGEM_ERRORS = -536870911;
pub const _VIGEM_ERRORS_VIGEM_ERROR_NO_FREE_SLOT: _VIGEM_ERRORS = -536870910;
pub const _VIGEM_ERRORS_VIGEM_ERROR_INVALID_TARGET: _VIGEM_ERRORS = -536870909;
pub const _VIGEM_ERRORS_VIGEM_ERROR_REMOVAL_FAILED: _VIGEM_ERRORS = -536870908;
pub const _VIGEM_ERRORS_VIGEM_ERROR_ALREADY_CONNECTED: _VIGEM_ERRORS = -536870907;
pub const _VIGEM_ERRORS_VIGEM_ERROR_TARGET_UNINITIALIZED: _VIGEM_ERRORS = -536870906;
pub const _VIGEM_ERRORS_VIGEM_ERROR_TARGET_NOT_PLUGGED_IN: _VIGEM_ERRORS = -536870905;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_VERSION_MISMATCH: _VIGEM_ERRORS = -536870904;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_ACCESS_FAILED: _VIGEM_ERRORS = -536870903;
pub const _VIGEM_ERRORS_VIGEM_ERROR_CALLBACK_ALREADY_REGISTERED: _VIGEM_ERRORS = -536870896;
pub const _VIGEM_ERRORS_VIGEM_ERROR_CALLBACK_NOT_FOUND: _VIGEM_ERRORS = -536870895;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_ALREADY_CONNECTED: _VIGEM_ERRORS = -536870894;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_INVALID_HANDLE: _VIGEM_ERRORS = -536870893;
pub const _VIGEM_ERRORS_VIGEM_ERROR_XUSB_USERINDEX_OUT_OF_RANGE: _VIGEM_ERRORS = -536870892;
pub const _VIGEM_ERRORS_VIGEM_ERROR_INVALID_PARAMETER: _VIGEM_ERRORS = -536870891;
pub const _VIGEM_ERRORS_VIGEM_ERROR_NOT_SUPPORTED: _VIGEM_ERRORS = -536870890;
#[doc = " Values that represent ViGEm errors"]
pub type _VIGEM_ERRORS = ::std::os::raw::c_int;
#[doc = " Values that represent ViGEm errors"]
pub use self::_VIGEM_ERRORS as VIGEM_ERROR;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VIGEM_CLIENT_T {
    _unused: [u8; 0],
}
#[doc = " Defines an alias representing a driver connection object"]
pub type PVIGEM_CLIENT = *mut _VIGEM_CLIENT_T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VIGEM_TARGET_T {
    _unused: [u8; 0],
}
#[doc = " Defines an alias representing a target device object"]
pub type PVIGEM_TARGET = *mut _VIGEM_TARGET_T;
pub type PFN_VIGEM_TARGET_ADD_RESULT =
    ::std::option::Option<unsafe extern "stdcall" fn(arg1: PVIGEM_CLIENT, arg2: PVIGEM_TARGET, arg3: VIGEM_ERROR)>;
pub type PFN_VIGEM_X360_NOTIFICATION = ::std::option::Option<
    unsafe extern "stdcall" fn(
        arg1: PVIGEM_CLIENT,
        arg2: PVIGEM_TARGET,
        arg3: UCHAR,
        arg4: UCHAR,
        arg5: UCHAR,
        arg6: LPVOID,
    ),
>;
pub type PFN_VIGEM_DS4_NOTIFICATION = ::std::option::Option<
    unsafe extern "stdcall" fn(
        arg1: PVIGEM_CLIENT,
        arg2: PVIGEM_TARGET,
        arg3: UCHAR,
        arg4: UCHAR,
        arg5: DS4_LIGHTBAR_COLOR,
        arg6: LPVOID,
    ),
>;
extern "C" {
    #[doc = "  Allocates an object representing a driver connection"]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger-Stelzer"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @returns\tA PVIGEM_CLIENT object."]
    pub fn vigem_alloc() -> PVIGEM_CLIENT;
}
extern "C" {
    #[doc = " Frees up memory used by the driver connection object"]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger-Stelzer"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem\tThe PVIGEM_CLIENT object."]
    pub fn vigem_free(vigem: PVIGEM_CLIENT);
}
extern "C" {
    #[doc = " Initializes the driver object and establishes a connection to the emulation bus"]
    #[doc = "          driver. Returns an error if no compatible bus device has been found."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger-Stelzer"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem\tThe PVIGEM_CLIENT object."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_ERROR."]
    pub fn vigem_connect(vigem: PVIGEM_CLIENT) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Disconnects from the bus device and resets the driver object state. The driver object"]
    #[doc = "           may be reused again after calling this function. When called, all targets which may"]
    #[doc = "           still be connected will be destroyed automatically. Be aware, that allocated target"]
    #[doc = "           objects won't be automatically freed, this has to be taken care of by the caller."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger-Stelzer"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem\tThe PVIGEM_CLIENT object."]
    pub fn vigem_disconnect(vigem: PVIGEM_CLIENT);
}
extern "C" {
    #[doc = " Allocates an object representing an Xbox 360 Controller device."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger-Stelzer"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @returns\tA PVIGEM_TARGET representing an Xbox 360 Controller device."]
    pub fn vigem_target_x360_alloc() -> PVIGEM_TARGET;
}
extern "C" {
    #[doc = " Allocates an object representing a DualShock 4 Controller device."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger-Stelzer"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @returns\tA PVIGEM_TARGET representing a DualShock 4 Controller device."]
    pub fn vigem_target_ds4_alloc() -> PVIGEM_TARGET;
}
extern "C" {
    #[doc = " Frees up memory used by the target device object. This does not automatically remove"]
    #[doc = "          the associated device from the bus, if present. If the target device doesn't get"]
    #[doc = "          removed before this call, the device becomes orphaned until the owning process is"]
    #[doc = "          terminated."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger-Stelzer"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    pub fn vigem_target_free(target: PVIGEM_TARGET);
}
extern "C" {
    #[doc = " Adds a provided target device to the bus driver, which is equal to a device plug-in"]
    #[doc = "          event of a physical hardware device. This function blocks until the target device is"]
    #[doc = "          in full operational mode."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger-Stelzer"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem \tThe driver connection object."]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_add(vigem: PVIGEM_CLIENT, target: PVIGEM_TARGET) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Adds a provided target device to the bus driver, which is equal to a device plug-in"]
    #[doc = "          event of a physical hardware device. This function immediately returns. An optional"]
    #[doc = "          callback may be registered which gets called on error or if the target device has"]
    #[doc = "          become fully operational."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger-Stelzer"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem \tThe driver connection object."]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = " @param \tresult\tAn optional function getting called when the target device becomes available."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_add_async(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        result: PFN_VIGEM_TARGET_ADD_RESULT,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Removes a provided target device from the bus driver, which is equal to a device"]
    #[doc = "           unplug event of a physical hardware device. The target device object may be reused"]
    #[doc = "           after this function is called. If this function is never called on target device"]
    #[doc = "           objects, they will be removed from the bus when the owning process terminates."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem \tThe driver connection object."]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_remove(vigem: PVIGEM_CLIENT, target: PVIGEM_TARGET) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Registers a function which gets called, when LED index or vibration state changes"]
    #[doc = "                 occur on the provided target device. This function fails if the provided"]
    #[doc = "                 target device isn't fully operational or in an erroneous state."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem\t\t\tThe driver connection object."]
    #[doc = " @param \ttarget\t\t\tThe target device object."]
    #[doc = " @param \tnotification\tThe notification callback."]
    #[doc = " @param \tuserData\t\tThe user data passed to the notification callback."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_x360_register_notification(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        notification: PFN_VIGEM_X360_NOTIFICATION,
        userData: LPVOID,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Registers a function which gets called, when LightBar or vibration state changes"]
    #[doc = "                 occur on the provided target device. This function fails if the provided"]
    #[doc = "                 target device isn't fully operational or in an erroneous state."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem\t\t\tThe driver connection object."]
    #[doc = " @param \ttarget\t\t\tThe target device object."]
    #[doc = " @param \tnotification\tThe notification callback."]
    #[doc = " @param \tuserData\t\tThe user data passed to the notification callback."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_ds4_register_notification(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        notification: PFN_VIGEM_DS4_NOTIFICATION,
        userData: LPVOID,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Removes a previously registered callback function from the provided target object."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    pub fn vigem_target_x360_unregister_notification(target: PVIGEM_TARGET);
}
extern "C" {
    #[doc = " Removes a previously registered callback function from the provided target object."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    pub fn vigem_target_ds4_unregister_notification(target: PVIGEM_TARGET);
}
extern "C" {
    #[doc = " Overrides the default Vendor ID value with the provided one."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = " @param \tvid   \tThe Vendor ID to set."]
    pub fn vigem_target_set_vid(target: PVIGEM_TARGET, vid: USHORT);
}
extern "C" {
    #[doc = " Overrides the default Product ID value with the provided one."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = " @param \tpid   \tThe Product ID to set."]
    pub fn vigem_target_set_pid(target: PVIGEM_TARGET, pid: USHORT);
}
extern "C" {
    #[doc = " Returns the Vendor ID of the provided target device object."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = ""]
    #[doc = " @returns\tThe Vendor ID."]
    pub fn vigem_target_get_vid(target: PVIGEM_TARGET) -> USHORT;
}
extern "C" {
    #[doc = " Returns the Product ID of the provided target device object."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = ""]
    #[doc = " @returns\tThe Product ID."]
    pub fn vigem_target_get_pid(target: PVIGEM_TARGET) -> USHORT;
}
extern "C" {
    #[doc = " Sends a state report to the provided target device."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem \tThe driver connection object."]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = " @param \treport\tThe report to send to the target device."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_x360_update(vigem: PVIGEM_CLIENT, target: PVIGEM_TARGET, report: XUSB_REPORT) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Sends a state report to the provided target device."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \tvigem \tThe driver connection object."]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = " @param \treport\tThe report to send to the target device."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_ds4_update(vigem: PVIGEM_CLIENT, target: PVIGEM_TARGET, report: DS4_REPORT) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Returns the internal index (serial number) the bus driver assigned to the provided"]
    #[doc = "               target device object. Note that this value is specific to the inner workings of"]
    #[doc = "               the bus driver, it does not reflect related values like player index or device"]
    #[doc = "               arrival order experienced by other APIs. It may be used to identify the target"]
    #[doc = "               device object for its lifetime. This value becomes invalid once the target"]
    #[doc = "               device is removed from the bus and may change on the next addition of the"]
    #[doc = "               device."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = ""]
    #[doc = " @returns\tThe internally used index of the target device."]
    pub fn vigem_target_get_index(target: PVIGEM_TARGET) -> ULONG;
}
extern "C" {
    #[doc = " Returns the type of the provided target device object."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t28.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_TARGET_TYPE."]
    pub fn vigem_target_get_type(target: PVIGEM_TARGET) -> VIGEM_TARGET_TYPE;
}
extern "C" {
    #[doc = " Returns TRUE if the provided target device object is currently attached to the bus,"]
    #[doc = "              FALSE otherwise."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t30.08.2017"]
    #[doc = ""]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = ""]
    #[doc = " @returns\tTRUE if device is attached to the bus, FALSE otherwise."]
    pub fn vigem_target_is_attached(target: PVIGEM_TARGET) -> BOOL;
}
extern "C" {
    #[doc = " Returns the user index of the emulated Xenon device. This value correspondents to the"]
    #[doc = "                (zero-based) index number representing the player number via LED present on a"]
    #[doc = "                physical controller and is compatible to the dwUserIndex property of the"]
    #[doc = "                XInput* APIs."]
    #[doc = ""]
    #[doc = " @author\tBenjamin \"Nefarius\" H�glinger"]
    #[doc = " @date\t10.05.2018"]
    #[doc = ""]
    #[doc = " @param \tvigem \tThe driver connection object."]
    #[doc = " @param \ttarget\tThe target device object."]
    #[doc = " @param \tindex \tThe (zero-based) user index of the Xenon device."]
    #[doc = ""]
    #[doc = " @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_x360_get_user_index(vigem: PVIGEM_CLIENT, target: PVIGEM_TARGET, index: PULONG) -> VIGEM_ERROR;
}