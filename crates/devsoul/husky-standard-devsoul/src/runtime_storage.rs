use crate::*;
use dashmap::DashMap;
use husky_devsoul::devsoul::IsRuntimeStorage;
use husky_devsoul_interface::{item_path::ItemPathIdInterface, IsLinketImpl};
use husky_entity_path::path::ItemPath;
use husky_ki::{version_stamp::KiVersionStamp, Ki};
use husky_linket_impl::standard::StandardLinketImplKiControlFlow;
use husky_standard_devsoul_interface::static_var::StandardStaticVarId;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct StandardDevRuntimeStorage {
    val_values: DashMap<
        StandardDevRuntimeValStorageKey,
        Arc<Mutex<Option<(ValVersionStamp, StandardLinketImplKiControlFlow)>>>,
    >,
    ki_values: DashMap<
        StandardDevRuntimeKiStorageKey,
        Arc<Mutex<Option<(KiVersionStamp, StandardLinketImplKiControlFlow)>>>,
    >,
    memo_field_values: DashMap<
        StandardDevRuntimeMemoizedFieldStorageKey,
        Arc<Mutex<Option<StandardLinketImplKiControlFlow>>>,
    >,
}

// ad hoc
type ValVersionStamp = ();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct StandardDevRuntimeValStorageKey {
    val_item_path_id_interface: ItemPathIdInterface,
    pedestal: StandardPedestal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct StandardDevRuntimeKiStorageKey {
    ki: Ki,
    pedestal: StandardPedestal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StandardDevRuntimeMemoizedFieldStorageKey {
    item_path_id_interface: ItemPathIdInterface,
    slf: AnyPointer,
}

// todo: make a safer key than AnyPointer
// a pointer might not be unique
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct AnyPointer(*const std::ffi::c_void);

unsafe impl Send for AnyPointer {}

impl IsRuntimeStorage<LinketImpl> for StandardDevRuntimeStorage {
    fn get_or_try_init_val_value(
        &self,
        val_item_path_id_interface: ItemPathIdInterface,
        pedestal: StandardPedestal,
        f: impl FnOnce() -> StandardLinketImplKiControlFlow,
        db: &salsa::Db,
    ) -> StandardLinketImplKiControlFlow {
        let key = StandardDevRuntimeValStorageKey {
            val_item_path_id_interface,
            pedestal,
        };
        let mu = self.val_values.entry(key.clone()).or_default().clone();
        let mut opt_stored_control_flow_store_guard = mu.lock().expect("todo");
        // ad hoc
        let new_version_stamp = (); // key.val.version_stamp(db);
        unsafe {
            match *opt_stored_control_flow_store_guard {
                Some((old_version_stamp, ref control_flow))
                    if old_version_stamp == new_version_stamp =>
                {
                    return control_flow.share_unchecked()
                }
                _ => *opt_stored_control_flow_store_guard = Some((new_version_stamp, f())),
            };
            opt_stored_control_flow_store_guard
                .as_ref()
                .expect("should be some")
                .1
                .share_unchecked()
        }
    }

    fn get_or_try_init_ki_value(
        &self,
        ki: Ki,
        pedestal: StandardPedestal,
        f: impl FnOnce() -> StandardLinketImplKiControlFlow,
        db: &::salsa::Db,
    ) -> StandardLinketImplKiControlFlow {
        use husky_devsoul_interface::pedestal::IsPedestal;

        let key = StandardDevRuntimeKiStorageKey { ki, pedestal };
        let mu = self.ki_values.entry(key.clone()).or_default().clone();
        let mut opt_stored_ki_control_flow_store_guard = mu.lock().expect("todo");
        let new_version_stamp = key.ki.version_stamp(db);
        unsafe {
            match *opt_stored_ki_control_flow_store_guard {
                Some((old_version_stamp, ref ki_control_flow))
                    if old_version_stamp == new_version_stamp =>
                {
                    return ki_control_flow.share_unchecked()
                }
                _ => *opt_stored_ki_control_flow_store_guard = Some((new_version_stamp, f())),
            };
            opt_stored_ki_control_flow_store_guard
                .as_ref()
                .expect("should be some")
                .1
                .share_unchecked()
        }
    }

    fn get_or_try_init_memo_field_value(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        slf: &'static std::ffi::c_void,
        f: impl FnOnce(&'static std::ffi::c_void) -> StandardLinketImplKiControlFlow,
    ) -> StandardLinketImplKiControlFlow {
        // todo: maybe add version stamp?
        let key = StandardDevRuntimeMemoizedFieldStorageKey {
            item_path_id_interface,
            slf: AnyPointer(slf as _),
        };
        let mu = self.memo_field_values.entry(key).or_default().clone();
        let mut opt_stored_ki_control_flow_store_guard = mu.lock().expect("todo");
        unsafe {
            match *opt_stored_ki_control_flow_store_guard {
                Some(ref ki_control_flow) => ki_control_flow.share_unchecked(),
                None => {
                    *opt_stored_ki_control_flow_store_guard = Some(f(slf));
                    opt_stored_ki_control_flow_store_guard
                        .as_ref()
                        .expect("should be some")
                        .share_unchecked()
                }
            }
        }
    }
}
