use binrw::{BinReaderExt, Endian};
use mkbootimg::{
	BootImageHeader, VendorBootImageHeader, BOOT_MAGIC, VENDOR_BOOT_MAGIC,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut file = std::fs::File::open("boot.img")?;

	let boot_image_header =
		file.read_type::<BootImageHeader>(Endian::Little)?;

	println!("{:?}", boot_image_header);

	match boot_image_header {
		BootImageHeader::Vendor(vendor_header) => match vendor_header {
			VendorBootImageHeader::V3(v3) => {
				assert_eq!(&v3.magic, VENDOR_BOOT_MAGIC);
			}
			VendorBootImageHeader::V4(v4) => {
				assert_eq!(&v4.base.magic, VENDOR_BOOT_MAGIC);
			}
		},
		BootImageHeader::V3(v) => {
			assert_eq!(&v.magic, BOOT_MAGIC);
		}
		BootImageHeader::V2(v) => {
			assert_eq!(&v.base.base.magic, BOOT_MAGIC);
		}
		BootImageHeader::V1(v) => {
			assert_eq!(&v.base.magic, BOOT_MAGIC);
		}
		BootImageHeader::V0(v) => {
			assert_eq!(&v.magic, BOOT_MAGIC);
		}
		_ => todo!(),
	}

	Ok(())
}
