use std::ptr;

use flat_message::FlatMessage;

// Recursive expansion of FlatMessage macro
// =========================================

unsafe fn ptr_read_unaligned_as_usize<T: Into<usize>>(x: *const T) -> usize {
    let x = ptr::read_unaligned(x);
    x.into()
}

fn de<T: Into<usize>>(
    buffer: *const u8,
    mut ptr_it: *const u32,
    p_end: *const u32,
    ref_table_offset: usize,
    hash_table_offset: usize,
    data_buffer: &[u8],
    unique_id: u64,
    timestamp: u64,
) -> core::result::Result<ProcessCreated, flat_message::Error> {
    let mut p_ofs = unsafe { buffer.add(ref_table_offset) as *const T };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 139902734u32 {
                break *ptr_it == 139902734u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_container_id_139902734 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_container_id_139902734 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_container_id_139902734
    } else {
        None
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(184324878u32));
            }
            if *ptr_it == 184324878u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_integrity_level_184324878): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(184324878u32));
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(228234254u32));
            }
            if *ptr_it == 228234254u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_logon_id_228234254): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(228234254u32));
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 452458254u32 {
                break *ptr_it == 452458254u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_parent_command_line_452458254 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_parent_command_line_452458254 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_parent_command_line_452458254
    } else {
        None
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(717936782u32));
            }
            if *ptr_it == 717936782u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_environment_variables_717936782): Option<Vec<String>> =
        flat_message::SerDeVec::<'_, Vec<String>>::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(717936782u32));
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(718071299u32));
            }
            if *ptr_it == 718071299u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_terminal_session_id_718071299): Option<u32> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(718071299u32));
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 879704846u32 {
                break *ptr_it == 879704846u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_description_879704846 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_description_879704846 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_description_879704846
    } else {
        None
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(1330094094u32));
            }
            if *ptr_it == 1330094094u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_image_path_1330094094): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(1330094094u32));
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 1330908430u32 {
                break *ptr_it == 1330908430u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_file_version_1330908430 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_file_version_1330908430 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_file_version_1330908430
    } else {
        None
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(1618501134u32));
            }
            if *ptr_it == 1618501134u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_user_1618501134): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(1618501134u32));
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(1765866755u32));
            }
            if *ptr_it == 1765866755u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_pid_1765866755): Option<u32> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(1765866755u32));
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(1928553987u32));
            }
            if *ptr_it == 1928553987u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_creation_flags_1928553987): Option<u32> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(1928553987u32));
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 2070555662u32 {
                break *ptr_it == 2070555662u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_sha1_2070555662 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_sha1_2070555662 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_sha1_2070555662
    } else {
        None
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 2222618382u32 {
                break *ptr_it == 2222618382u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_signature_status_2222618382 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_signature_status_2222618382 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_signature_status_2222618382
    } else {
        None
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 2308439054u32 {
                break *ptr_it == 2308439054u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_imphash_2308439054 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_imphash_2308439054 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_imphash_2308439054
    } else {
        None
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 2333202190u32 {
                break *ptr_it == 2333202190u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_original_file_name_2333202190 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_original_file_name_2333202190 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_original_file_name_2333202190
    } else {
        None
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(2369371406u32));
            }
            if *ptr_it == 2369371406u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_name_2369371406): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(2369371406u32));
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 2393554446u32 {
                break *ptr_it == 2393554446u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_md5_2393554446 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_md5_2393554446 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_md5_2393554446
    } else {
        None
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(2473020931u32));
            }
            if *ptr_it == 2473020931u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_parent_pid_2473020931): Option<u32> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(2473020931u32));
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 2631152910u32 {
                break *ptr_it == 2631152910u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_sha256_2631152910 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_sha256_2631152910 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_sha256_2631152910
    } else {
        None
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 2674982158u32 {
                break *ptr_it == 2674982158u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_signer_2674982158 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_signer_2674982158 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_signer_2674982158
    } else {
        None
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 2858608654u32 {
                break *ptr_it == 2858608654u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_company_2858608654 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_company_2858608654 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_company_2858608654
    } else {
        None
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(2934375182u32));
            }
            if *ptr_it == 2934375182u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_current_directory_2934375182): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(2934375182u32));
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(3122048526u32));
            }
            if *ptr_it == 3122048526u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_parent_image_path_3122048526): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(3122048526u32));
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(3310367747u32));
            }
            if *ptr_it == 3310367747u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_main_thread_id_3310367747): Option<u32> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(3310367747u32));
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 3355738638u32 {
                break *ptr_it == 3355738638u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_logon_guid_3355738638 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_logon_guid_3355738638 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_logon_guid_3355738638
    } else {
        None
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(3593476622u32));
            }
            if *ptr_it == 3593476622u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_sid_3593476622): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(3593476622u32));
    };
    let create_field = loop {
        unsafe {
            if ptr_it == p_end {
                break false;
            }
            if *ptr_it >= 3846717710u32 {
                break *ptr_it == 3846717710u32;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    };
    let inner_var_product_3846717710 = if create_field {
        let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
        unsafe {
            p_ofs = p_ofs.add(1);
        }
        unsafe {
            ptr_it = ptr_it.add(1);
        }
        let inner_var_product_3846717710 = if offset < 8 || offset >= hash_table_offset {
            if offset == 0 {
                None
            } else {
                return Err(flat_message::Error::InvalidFieldOffset((
                    offset as u32,
                    hash_table_offset as u32,
                )));
            }
        } else {
            let tmp: Option<String> = flat_message::SerDe::from_buffer(data_buffer, offset);
            if tmp.is_none() { None } else { tmp }
        };
        inner_var_product_3846717710
    } else {
        None
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(3939367950u32));
            }
            if *ptr_it == 3939367950u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_parent_3939367950): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(3939367950u32));
    };
    unsafe {
        loop {
            if ptr_it == p_end {
                return Err(flat_message::Error::FieldIsMissing(4001438990u32));
            }
            if *ptr_it == 4001438990u32 {
                ptr_it = ptr_it.add(1);
                break;
            }
            p_ofs = p_ofs.add(1);
            ptr_it = ptr_it.add(1);
        }
    }
    let offset = unsafe { ptr_read_unaligned_as_usize(p_ofs) };
    unsafe {
        p_ofs = p_ofs.add(1);
    }
    if offset < 8 || offset >= hash_table_offset {
        return Err(flat_message::Error::InvalidFieldOffset((
            offset as u32,
            hash_table_offset as u32,
        )));
    }
    let Some(inner_var_command_line_4001438990): Option<String> =
        flat_message::SerDe::from_buffer(data_buffer, offset)
    else {
        return Err(flat_message::Error::FailToDeserialize(4001438990u32));
    };
    Ok(ProcessCreated {
        container_id: inner_var_container_id_139902734,
        integrity_level: inner_var_integrity_level_184324878,
        logon_id: inner_var_logon_id_228234254,
        parent_command_line: inner_var_parent_command_line_452458254,
        environment_variables: inner_var_environment_variables_717936782,
        terminal_session_id: inner_var_terminal_session_id_718071299,
        description: inner_var_description_879704846,
        image_path: inner_var_image_path_1330094094,
        file_version: inner_var_file_version_1330908430,
        user: inner_var_user_1618501134,
        pid: inner_var_pid_1765866755,
        creation_flags: inner_var_creation_flags_1928553987,
        sha1: inner_var_sha1_2070555662,
        signature_status: inner_var_signature_status_2222618382,
        imphash: inner_var_imphash_2308439054,
        original_file_name: inner_var_original_file_name_2333202190,
        name: inner_var_name_2369371406,
        md5: inner_var_md5_2393554446,
        parent_pid: inner_var_parent_pid_2473020931,
        sha256: inner_var_sha256_2631152910,
        signer: inner_var_signer_2674982158,
        company: inner_var_company_2858608654,
        current_directory: inner_var_current_directory_2934375182,
        parent_image_path: inner_var_parent_image_path_3122048526,
        main_thread_id: inner_var_main_thread_id_3310367747,
        logon_guid: inner_var_logon_guid_3355738638,
        sid: inner_var_sid_3593476622,
        product: inner_var_product_3846717710,
        parent: inner_var_parent_3939367950,
        command_line: inner_var_command_line_4001438990,
        unique_id: flat_message::UniqueID::with_value(unique_id),
        timestamp: flat_message::Timestamp::with_value(timestamp),
    })
}

impl flat_message::FlatMessage<'_> for ProcessCreated {
    fn serialize_to(
        &self,
        output: &mut ::flat_message::Storage,
        config: flat_message::Config,
    ) -> core::result::Result<(), flat_message::Error> {
        todo!();
    }

    fn deserialize_from_ref(
        input: &flat_message::StorageRef,
    ) -> core::result::Result<Self, flat_message::Error> {
        use ::std::ptr;
        enum RefOffsetSize {
            U8,
            U16,
            U32,
        }
        let input = input.as_slice();
        let len = input.len();
        if len < 8 {
            return Err(flat_message::Error::InvalidHeaderLength(len));
        }
        let buffer = input.as_ptr();
        let header: flat_message::headers::HeaderV1 =
            unsafe { ptr::read_unaligned(buffer as *const flat_message::headers::HeaderV1) };
        if header.magic != 21843014u32 {
            return Err(flat_message::Error::InvalidMagic);
        }
        let mut metadata_size = 0usize;
        if header.flags & 4u8 != 0 {
            metadata_size += 4;
        }
        if header.flags & 8u8 != 0 {
            metadata_size += 4;
        }
        if header.flags & 16u8 != 0 {
            metadata_size += 8;
        }
        if header.flags & 32u8 != 0 {
            metadata_size += 8;
        }
        let ref_offset_size = match header.flags & 0b0000_0011 {
            0 => RefOffsetSize::U8,
            1 => RefOffsetSize::U16,
            2 => RefOffsetSize::U32,
            _ => return Err(flat_message::Error::InvalidOffsetSize),
        };
        let ref_table_size = match ref_offset_size {
            RefOffsetSize::U8 => header.fields_count as usize,
            RefOffsetSize::U16 => header.fields_count as usize * 2,
            RefOffsetSize::U32 => header.fields_count as usize * 4,
        };
        let hash_table_size = header.fields_count as usize * 4;
        let min_size = 8 + metadata_size + hash_table_size + ref_table_size;
        if min_size > len {
            return Err(flat_message::Error::InvalidSizeToStoreFieldsTable((
                len as u32,
                min_size as u32,
            )));
        }
        let mut metadata_ptr = unsafe { buffer.add(len - metadata_size) as *const u64 };
        let timestamp = if header.flags & 16u8 != 0 {
            let value = unsafe { ptr::read_unaligned(metadata_ptr) };
            unsafe {
                metadata_ptr = metadata_ptr.add(1);
            }
            value
        } else {
            0
        };
        let unique_id = if header.flags & 32u8 != 0 {
            unsafe { ptr::read_unaligned(metadata_ptr) }
        } else {
            0
        };
        let hash_table_offset = len - ref_table_size - metadata_size - hash_table_size;
        let ref_table_offset = hash_table_offset + hash_table_size;
        let data_buffer = &input[..hash_table_offset];
        let mut ptr_it = unsafe { buffer.add(hash_table_offset) as *const u32 };
        let p_end = unsafe { ptr_it.add(header.fields_count as usize) };
        if header.flags & 4u8 != 0 {
            let checksum = flat_message::crc32(&input[..len - 4]);
            if checksum != unsafe { ptr::read_unaligned(buffer.add(len - 4) as *const u32) } {
                return Err(flat_message::Error::InvalidChecksum((checksum, unsafe {
                    ptr::read_unaligned(buffer.add(len - 4) as *const u32)
                })));
            }
        }
        match ref_offset_size {
            RefOffsetSize::U8 => de::<u8>(
                buffer,
                ptr_it,
                p_end,
                ref_table_offset,
                hash_table_offset,
                data_buffer,
                unique_id,
                timestamp,
            ),
            RefOffsetSize::U16 => de::<u16>(
                buffer,
                ptr_it,
                p_end,
                ref_table_offset,
                hash_table_offset,
                data_buffer,
                unique_id,
                timestamp,
            ),
            RefOffsetSize::U32 => de::<u8>(
                buffer,
                ptr_it,
                p_end,
                ref_table_offset,
                hash_table_offset,
                data_buffer,
                unique_id,
                timestamp,
            ),
        }
    }
    unsafe fn deserialize_from_ref_unchecked(
        input: &flat_message::StorageRef,
    ) -> core::result::Result<Self, flat_message::Error> {
        Self::deserialize_from_ref(input)
    }
}

// #[derive(FlatMessage)]
// #[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

fn main() {
    println!("Hello, world!");
}
