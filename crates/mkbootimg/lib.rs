use binrw::prelude::*;

pub const BOOT_MAGIC: &[u8; 8usize] = b"ANDROID!";
pub const BOOT_MAGIC_SIZE: u32 = 8;
pub const BOOT_NAME_SIZE: u32 = 16;
pub const BOOT_ARGS_SIZE: u32 = 512;
pub const BOOT_EXTRA_ARGS_SIZE: u32 = 1024;
pub const VENDOR_BOOT_MAGIC: &[u8; 8usize] = b"VNDRBOOT";
pub const VENDOR_BOOT_MAGIC_SIZE: u32 = 8;
pub const VENDOR_BOOT_ARGS_SIZE: u32 = 2048;
pub const VENDOR_BOOT_NAME_SIZE: u32 = 16;
pub const VENDOR_RAMDISK_TYPE_NONE: u32 = 0;
pub const VENDOR_RAMDISK_TYPE_PLATFORM: u32 = 1;
pub const VENDOR_RAMDISK_TYPE_RECOVERY: u32 = 2;
pub const VENDOR_RAMDISK_TYPE_DLKM: u32 = 3;
pub const VENDOR_RAMDISK_NAME_SIZE: u32 = 32;
pub const VENDOR_RAMDISK_TABLE_ENTRY_BOARD_ID_SIZE: u32 = 16;

#[derive(Debug, BinRead, BinWrite)]
#[brw(little, magic = b"ANDROID!")]
pub struct BootImgHdrV0 {
	pub magic: [u8; 8],
	pub kernel_size: u32,
	pub kernel_addr: u32,
	pub ramdisk_size: u32,
	pub ramdisk_addr: u32,
	pub second_size: u32,
	pub second_addr: u32,
	pub tags_addr: u32,
	pub page_size: u32,
	pub header_version: u32,
	pub os_version: u32,
	#[br(count = BOOT_NAME_SIZE)]
	pub name: Vec<u8>,
	#[br(count = BOOT_ARGS_SIZE)]
	pub cmdline: Vec<u8>,
	pub id: [u32; 8],
	#[br(count = BOOT_EXTRA_ARGS_SIZE)]
	pub extra_cmdline: Vec<u8>,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct BootImgHdrV1 {
	pub base: BootImgHdrV0,
	pub recovery_dtbo_size: u32,
	pub recovery_dtbo_offset: u64,
	pub header_size: u32,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct BootImgHdrV2 {
	pub base: BootImgHdrV1,
	pub dtb_size: u32,
	pub dtb_addr: u64,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct BootImgHdrV3 {
	pub magic: [u8; 8],
	pub kernel_size: u32,
	pub ramdisk_size: u32,
	pub os_version: u32,
	pub header_size: u32,
	pub reserved: [u32; 4],
	pub header_version: u32,
	#[br(count = 1536)]
	pub cmdline: Vec<u8>,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct VendorBootImgHdrV3 {
	pub magic: [u8; 8],
	pub header_version: u32,
	pub page_size: u32,
	pub kernel_addr: u32,
	pub ramdisk_addr: u32,
	pub vendor_ramdisk_size: u32,
	#[br(count = VENDOR_BOOT_ARGS_SIZE)]
	pub cmdline: Vec<u8>,
	pub tags_addr: u32,
	#[br(count = VENDOR_BOOT_NAME_SIZE)]
	pub name: Vec<u8>,
	pub header_size: u32,
	pub dtb_size: u32,
	pub dtb_addr: u64,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct BootImgHdrV4 {
	pub base: BootImgHdrV3,
	pub signature_size: u32,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct VendorBootImgHdrV4 {
	pub base: VendorBootImgHdrV3,
	pub vendor_ramdisk_table_size: u32,
	pub vendor_ramdisk_table_entry_num: u32,
	pub vendor_ramdisk_table_entry_size: u32,
	pub bootconfig_size: u32,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct VendorRamdiskTableEntryV4 {
	pub ramdisk_size: u32,
	pub ramdisk_offset: u32,
	pub ramdisk_type: u32,
	#[br(count = VENDOR_RAMDISK_NAME_SIZE)]
	pub ramdisk_name: Vec<u8>,
	pub board_id: [u32; 16],
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub enum VendorBootImageHeader {
	V3(VendorBootImgHdrV3),
	V4(VendorBootImgHdrV4),
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub enum BootImageHeader {
	Vendor(VendorBootImageHeader),
	V4(BootImgHdrV4),
	V3(BootImgHdrV3),
	V2(BootImgHdrV2),
	V1(BootImgHdrV1),
	V0(BootImgHdrV0),
}
