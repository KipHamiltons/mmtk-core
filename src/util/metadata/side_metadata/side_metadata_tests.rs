#[cfg(test)]
mod tests {
    use crate::util::constants;
    use crate::util::heap::layout::vm_layout_constants;
    use crate::util::metadata::side_metadata::address_to_meta_address;
    use crate::util::metadata::side_metadata::bzero_metadata;
    use crate::util::metadata::side_metadata::ensure_metadata_is_mapped;
    use crate::util::metadata::side_metadata::fetch_add_atomic;
    use crate::util::metadata::side_metadata::fetch_sub_atomic;
    use crate::util::metadata::side_metadata::load_atomic;
    use crate::util::metadata::side_metadata::meta_byte_lshift;
    use crate::util::metadata::side_metadata::meta_byte_mask;
    #[cfg(target_pointer_width = "32")]
    use crate::util::metadata::side_metadata::meta_bytes_per_chunk;
    use crate::util::metadata::side_metadata::metadata_address_range_size;
    use crate::util::metadata::side_metadata::sanity;
    use crate::util::metadata::side_metadata::SideMetadata;
    use crate::util::metadata::side_metadata::SideMetadataContext;
    use crate::util::metadata::side_metadata::SideMetadataSanity;
    use crate::util::metadata::MetadataScope;
    use crate::util::metadata::MetadataSpec;
    use crate::util::metadata::GLOBAL_SIDE_METADATA_BASE_ADDRESS;
    // #[cfg(target_pointer_width = "64")]
    use crate::util::metadata::LOCAL_SIDE_METADATA_BASE_ADDRESS;
    use crate::util::test_util::{serial_test, with_cleanup};
    use crate::util::Address;

    #[test]
    fn test_side_metadata_address_to_meta_address() {
        let mut gspec = MetadataSpec {
            is_on_side: true,
            scope: MetadataScope::Global,
            offset: GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize(),
            num_of_bits: 1,
            log_min_obj_size: 0,
        };
        #[cfg(target_pointer_width = "64")]
        let mut lspec = MetadataSpec {
            is_on_side: true,
            scope: MetadataScope::PolicySpecific,
            offset: LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize(),
            num_of_bits: 1,
            log_min_obj_size: 0,
        };

        #[cfg(target_pointer_width = "32")]
        let mut lspec = MetadataSpec {
            is_on_side: true,
            scope: MetadataScope::PolicySpecific,
            offset: 0,
            num_of_bits: 1,
            log_min_obj_size: 0,
        };

        assert_eq!(
            address_to_meta_address(gspec, unsafe { Address::from_usize(0) }).as_usize(),
            GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize()
        );
        assert_eq!(
            address_to_meta_address(lspec, unsafe { Address::from_usize(0) }).as_usize(),
            LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize()
        );

        assert_eq!(
            address_to_meta_address(gspec, unsafe { Address::from_usize(7) }).as_usize(),
            GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize()
        );
        assert_eq!(
            address_to_meta_address(lspec, unsafe { Address::from_usize(7) }).as_usize(),
            LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize()
        );

        assert_eq!(
            address_to_meta_address(gspec, unsafe { Address::from_usize(27) }).as_usize(),
            GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 3
        );
        assert_eq!(
            address_to_meta_address(lspec, unsafe { Address::from_usize(129) }).as_usize(),
            LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 16
        );

        gspec.log_min_obj_size = 2;
        lspec.log_min_obj_size = 1;

        assert_eq!(
            address_to_meta_address(gspec, unsafe { Address::from_usize(0) }).as_usize(),
            GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize()
        );
        assert_eq!(
            address_to_meta_address(lspec, unsafe { Address::from_usize(0) }).as_usize(),
            LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize()
        );

        assert_eq!(
            address_to_meta_address(gspec, unsafe { Address::from_usize(32) }).as_usize(),
            GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 1
        );
        assert_eq!(
            address_to_meta_address(lspec, unsafe { Address::from_usize(32) }).as_usize(),
            LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 2
        );

        assert_eq!(
            address_to_meta_address(gspec, unsafe { Address::from_usize(316) }).as_usize(),
            GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 9
        );
        assert_eq!(
            address_to_meta_address(lspec, unsafe { Address::from_usize(316) }).as_usize(),
            LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 19
        );

        gspec.num_of_bits = 2;
        lspec.num_of_bits = 8;

        assert_eq!(
            address_to_meta_address(gspec, unsafe { Address::from_usize(0) }).as_usize(),
            GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize()
        );
        assert_eq!(
            address_to_meta_address(lspec, unsafe { Address::from_usize(0) }).as_usize(),
            LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize()
        );

        assert_eq!(
            address_to_meta_address(gspec, unsafe { Address::from_usize(32) }).as_usize(),
            GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 2
        );
        assert_eq!(
            address_to_meta_address(lspec, unsafe { Address::from_usize(32) }).as_usize(),
            LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 16
        );

        assert_eq!(
            address_to_meta_address(gspec, unsafe { Address::from_usize(316) }).as_usize(),
            GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 19
        );
        assert_eq!(
            address_to_meta_address(lspec, unsafe { Address::from_usize(318) }).as_usize(),
            LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize() + 159
        );
    }

    #[test]
    fn test_side_metadata_meta_byte_mask() {
        let mut spec = MetadataSpec {
            is_on_side: true,
            scope: MetadataScope::Global,
            offset: GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize(),
            num_of_bits: 1,
            log_min_obj_size: 0,
        };

        assert_eq!(meta_byte_mask(spec), 1);

        spec.num_of_bits = 2;
        assert_eq!(meta_byte_mask(spec), 3);
        spec.num_of_bits = 4;
        assert_eq!(meta_byte_mask(spec), 15);
        spec.num_of_bits = 8;
        assert_eq!(meta_byte_mask(spec), 255);
    }

    #[test]
    fn test_side_metadata_meta_byte_lshift() {
        let mut spec = MetadataSpec {
            is_on_side: true,
            scope: MetadataScope::Global,
            offset: GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize(),
            num_of_bits: 1,
            log_min_obj_size: 0,
        };

        assert_eq!(meta_byte_lshift(spec, unsafe { Address::from_usize(0) }), 0);
        assert_eq!(meta_byte_lshift(spec, unsafe { Address::from_usize(5) }), 5);
        assert_eq!(
            meta_byte_lshift(spec, unsafe { Address::from_usize(15) }),
            7
        );

        spec.num_of_bits = 4;

        assert_eq!(meta_byte_lshift(spec, unsafe { Address::from_usize(0) }), 0);
        assert_eq!(meta_byte_lshift(spec, unsafe { Address::from_usize(5) }), 4);
        assert_eq!(
            meta_byte_lshift(spec, unsafe { Address::from_usize(15) }),
            4
        );
        assert_eq!(
            meta_byte_lshift(spec, unsafe { Address::from_usize(0x10010) }),
            0
        );
    }

    #[test]
    fn test_side_metadata_try_mmap_metadata() {
        serial_test(|| {
            with_cleanup(
                || {
                    // We need to do this because of the static NO_METADATA
                    // sanity::reset();
                    let mut gspec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::Global,
                        offset: GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize(),
                        num_of_bits: 1,
                        log_min_obj_size: 0,
                    };
                    #[cfg(target_pointer_width = "64")]
                    let mut lspec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::PolicySpecific,
                        offset: LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize(),
                        num_of_bits: 2,
                        log_min_obj_size: 1,
                    };
                    #[cfg(target_pointer_width = "32")]
                    let mut lspec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::PolicySpecific,
                        offset: 0,
                        num_of_bits: 2,
                        log_min_obj_size: 1,
                    };

                    let metadata = SideMetadata::new(SideMetadataContext {
                        global: vec![gspec],
                        local: vec![lspec],
                    });

                    let mut metadata_sanity = SideMetadataSanity::new();
                    metadata_sanity.verify_metadata_context("NoPolicy", metadata.get_context());

                    assert!(metadata
                        .try_map_metadata_space(
                            vm_layout_constants::HEAP_START,
                            constants::BYTES_IN_PAGE,
                        )
                        .is_ok());

                    ensure_metadata_is_mapped(gspec, vm_layout_constants::HEAP_START);
                    ensure_metadata_is_mapped(lspec, vm_layout_constants::HEAP_START);
                    ensure_metadata_is_mapped(
                        gspec,
                        vm_layout_constants::HEAP_START + constants::BYTES_IN_PAGE - 1,
                    );
                    ensure_metadata_is_mapped(
                        lspec,
                        vm_layout_constants::HEAP_START + constants::BYTES_IN_PAGE - 1,
                    );

                    metadata.ensure_unmap_metadata_space(
                        vm_layout_constants::HEAP_START,
                        constants::BYTES_IN_PAGE,
                    );

                    gspec.log_min_obj_size = 3;
                    gspec.num_of_bits = 4;
                    lspec.log_min_obj_size = 4;
                    lspec.num_of_bits = 4;

                    metadata_sanity.reset();

                    let metadata = SideMetadata::new(SideMetadataContext {
                        global: vec![gspec],
                        local: vec![lspec],
                    });

                    metadata_sanity.verify_metadata_context("NoPolicy", metadata.get_context());
                    metadata_sanity.reset();

                    assert!(metadata
                        .try_map_metadata_space(
                            vm_layout_constants::HEAP_START + vm_layout_constants::BYTES_IN_CHUNK,
                            vm_layout_constants::BYTES_IN_CHUNK,
                        )
                        .is_ok());

                    ensure_metadata_is_mapped(
                        gspec,
                        vm_layout_constants::HEAP_START + vm_layout_constants::BYTES_IN_CHUNK,
                    );
                    ensure_metadata_is_mapped(
                        lspec,
                        vm_layout_constants::HEAP_START + vm_layout_constants::BYTES_IN_CHUNK,
                    );
                    ensure_metadata_is_mapped(
                        gspec,
                        vm_layout_constants::HEAP_START + vm_layout_constants::BYTES_IN_CHUNK * 2
                            - 8,
                    );
                    ensure_metadata_is_mapped(
                        lspec,
                        vm_layout_constants::HEAP_START + vm_layout_constants::BYTES_IN_CHUNK * 2
                            - 16,
                    );

                    metadata.ensure_unmap_metadata_space(
                        vm_layout_constants::HEAP_START + vm_layout_constants::BYTES_IN_CHUNK,
                        vm_layout_constants::BYTES_IN_CHUNK,
                    );
                },
                || {
                    sanity::reset();
                },
            );
        })
    }

    #[test]
    fn test_side_metadata_atomic_fetch_add_sub_ge8bits() {
        serial_test(|| {
            with_cleanup(
                || {
                    // We need to do this because of the static NO_METADATA
                    // sanity::reset();
                    let data_addr = vm_layout_constants::HEAP_START;

                    let metadata_1_spec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::Global,
                        offset: GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize(),
                        num_of_bits: 16,
                        log_min_obj_size: 6,
                    };

                    let metadata_2_spec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::Global,
                        offset: metadata_1_spec.offset
                            + metadata_address_range_size(metadata_1_spec),
                        num_of_bits: 8,
                        log_min_obj_size: 7,
                    };

                    let metadata = SideMetadata::new(SideMetadataContext {
                        global: vec![metadata_1_spec, metadata_2_spec],
                        local: vec![],
                    });

                    let mut metadata_sanity = SideMetadataSanity::new();
                    metadata_sanity.verify_metadata_context("NoPolicy", metadata.get_context());

                    assert!(metadata
                        .try_map_metadata_space(data_addr, constants::BYTES_IN_PAGE,)
                        .is_ok());

                    let zero = fetch_add_atomic(metadata_1_spec, data_addr, 5);
                    assert_eq!(zero, 0);

                    let five = load_atomic(metadata_1_spec, data_addr);
                    assert_eq!(five, 5);

                    let zero = fetch_add_atomic(metadata_2_spec, data_addr, 5);
                    assert_eq!(zero, 0);

                    let five = load_atomic(metadata_2_spec, data_addr);
                    assert_eq!(five, 5);

                    let another_five = fetch_sub_atomic(metadata_1_spec, data_addr, 2);
                    assert_eq!(another_five, 5);

                    let three = load_atomic(metadata_1_spec, data_addr);
                    assert_eq!(three, 3);

                    let another_five = fetch_sub_atomic(metadata_2_spec, data_addr, 2);
                    assert_eq!(another_five, 5);

                    let three = load_atomic(metadata_2_spec, data_addr);
                    assert_eq!(three, 3);

                    metadata.ensure_unmap_metadata_space(data_addr, constants::BYTES_IN_PAGE);
                    metadata_sanity.reset();
                },
                || {
                    sanity::reset();
                },
            );
        });
    }

    #[test]
    fn test_side_metadata_atomic_fetch_add_sub_2bits() {
        serial_test(|| {
            with_cleanup(
                || {
                    // We need to do this because of the static NO_METADATA
                    // sanity::reset();
                    let data_addr = vm_layout_constants::HEAP_START
                        + (vm_layout_constants::BYTES_IN_CHUNK << 1);

                    let metadata_1_spec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::Global,
                        offset: GLOBAL_SIDE_METADATA_BASE_ADDRESS.as_usize(),
                        num_of_bits: 2,
                        log_min_obj_size: constants::LOG_BYTES_IN_WORD as usize,
                    };

                    let metadata = SideMetadata::new(SideMetadataContext {
                        global: vec![metadata_1_spec],
                        local: vec![],
                    });

                    let mut metadata_sanity = SideMetadataSanity::new();
                    metadata_sanity.verify_metadata_context("NoPolicy", metadata.get_context());

                    assert!(metadata
                        .try_map_metadata_space(data_addr, constants::BYTES_IN_PAGE,)
                        .is_ok());

                    let zero = fetch_add_atomic(metadata_1_spec, data_addr, 2);
                    assert_eq!(zero, 0);

                    let two = load_atomic(metadata_1_spec, data_addr);
                    assert_eq!(two, 2);

                    let another_two = fetch_sub_atomic(metadata_1_spec, data_addr, 1);
                    assert_eq!(another_two, 2);

                    let one = load_atomic(metadata_1_spec, data_addr);
                    assert_eq!(one, 1);

                    metadata.ensure_unmap_metadata_space(data_addr, constants::BYTES_IN_PAGE);

                    metadata_sanity.reset();
                },
                || {
                    sanity::reset();
                },
            );
        });
    }

    #[test]
    fn test_side_metadata_bzero_metadata() {
        serial_test(|| {
            with_cleanup(
                || {
                    // We need to do this because of the static NO_METADATA
                    // sanity::reset();
                    let data_addr = vm_layout_constants::HEAP_START
                        + (vm_layout_constants::BYTES_IN_CHUNK << 2);

                    #[cfg(target_pointer_width = "64")]
                    let metadata_1_spec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::PolicySpecific,
                        offset: LOCAL_SIDE_METADATA_BASE_ADDRESS.as_usize(),
                        num_of_bits: 16,
                        log_min_obj_size: 9,
                    };
                    #[cfg(target_pointer_width = "64")]
                    let metadata_2_spec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::PolicySpecific,
                        offset: metadata_1_spec.offset
                            + metadata_address_range_size(metadata_1_spec),
                        num_of_bits: 8,
                        log_min_obj_size: 7,
                    };

                    #[cfg(target_pointer_width = "32")]
                    let metadata_1_spec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::PolicySpecific,
                        offset: 0,
                        num_of_bits: 16,
                        log_min_obj_size: 9,
                    };
                    #[cfg(target_pointer_width = "32")]
                    let metadata_2_spec = MetadataSpec {
                        is_on_side: true,
                        scope: MetadataScope::PolicySpecific,
                        offset: metadata_1_spec.offset
                            + meta_bytes_per_chunk(
                                metadata_1_spec.log_min_obj_size,
                                metadata_1_spec.num_of_bits,
                            ),
                        num_of_bits: 8,
                        log_min_obj_size: 7,
                    };

                    let metadata = SideMetadata::new(SideMetadataContext {
                        global: vec![],
                        local: vec![metadata_1_spec, metadata_2_spec],
                    });

                    let mut metadata_sanity = SideMetadataSanity::new();
                    metadata_sanity.verify_metadata_context("NoPolicy", metadata.get_context());

                    assert!(metadata
                        .try_map_metadata_space(data_addr, constants::BYTES_IN_PAGE,)
                        .is_ok());

                    let zero = fetch_add_atomic(metadata_1_spec, data_addr, 5);
                    assert_eq!(zero, 0);

                    let five = load_atomic(metadata_1_spec, data_addr);
                    assert_eq!(five, 5);

                    let zero = fetch_add_atomic(metadata_2_spec, data_addr, 5);
                    assert_eq!(zero, 0);

                    let five = load_atomic(metadata_2_spec, data_addr);
                    assert_eq!(five, 5);

                    bzero_metadata(metadata_2_spec, data_addr, constants::BYTES_IN_PAGE);

                    let five = load_atomic(metadata_1_spec, data_addr);
                    assert_eq!(five, 5);
                    let five = load_atomic(metadata_2_spec, data_addr);
                    assert_eq!(five, 0);

                    bzero_metadata(metadata_1_spec, data_addr, constants::BYTES_IN_PAGE);

                    let five = load_atomic(metadata_1_spec, data_addr);
                    assert_eq!(five, 0);
                    let five = load_atomic(metadata_2_spec, data_addr);
                    assert_eq!(five, 0);

                    metadata.ensure_unmap_metadata_space(data_addr, constants::BYTES_IN_PAGE);

                    metadata_sanity.reset();
                },
                || {
                    sanity::reset();
                },
            );
        });
    }
}
