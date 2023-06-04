use std::io::Cursor;

use anyhow::anyhow;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use object::elf::FileHeader32;
use object::read::elf::ElfFile32;
use object::read::elf::ElfFile64;
use object::read::elf::ElfSection32;
use object::SectionIndex;
use object::{read::elf::ElfFile, Object};
use object::{Endianness, ObjectSection, ObjectSegment};

use crate::il2cpp::Il2CppCodeRegistration;

use crate::utils::*;

use super::Type;

pub fn vaddr(elf: &ElfFile32, vaddr: u64) -> Result<u64, anyhow::Error> {
    for segment in elf.segments() {
        //println!("t - {:x}", segment.address());
        if segment.address() <= vaddr {
            let offset = vaddr - segment.address();
            if offset < segment.size() {
                let out = segment.file_range().0 + offset;
                return Ok(out);
            }
        }
    }
    Err(anyhow!(
        "Failed to convert virtual address value: {}",
        vaddr
    ))
}

pub fn map_virtual_address_to_real(elf: &ElfFile32, vaddr: u64) -> Option<u64> {
    let e_shnum = elf.sections().count();
    println!("count of elf file{}", e_shnum);

    let first_section = elf.section_by_index(SectionIndex(0)).unwrap();

    if vaddr < first_section.address() {
        return Some(0);
    }

    for i in 1..e_shnum {
        let section = elf.section_by_index(SectionIndex(i)).unwrap();
        if section.address() > vaddr {
            println!(
                "MapVirtualAddressToReal: Yes, section is greater than address: {}",
                section.address()
            );
            let ret_sec = elf.section_by_index(SectionIndex(i - 1)).unwrap();
            let res_offset = vaddr - ret_sec.address();

            let ret_sec_shoffset = ret_sec.file_range().unwrap().0;
            return Some((ret_sec_shoffset + res_offset) as u64);
        }
    }

    let ret_sec = elf.section_by_index(SectionIndex(e_shnum - 1)).unwrap();
    println!(
        "MapVirtualAddressToReal: uiAddr (vaddr) is larger: Now the last section is: {:?}",
        ret_sec
    );
    let res_offset = vaddr - ret_sec.address();
    let ret_sec_shoffset = ret_sec.file_range().unwrap().0;
    Some((ret_sec_shoffset + res_offset) as u64)
}

pub type Elf<'a> = ElfFile32<'a, Endianness>;

pub struct CodeRegistration {
    pub method_pointers: Vec<u64>,
}

impl CodeRegistration {
    pub fn read(elf: &Elf, addr: u64) -> Result<Self, anyhow::Error> {
        let mut cur = Cursor::new(elf.data());
        cur.set_position(addr);
        let method_ptrs_len = cur.read_u32::<LittleEndian>()?;
        let method_ptrs_addr = cur.read_u32::<LittleEndian>()?;

        println!("method_ptrs_len: {}", method_ptrs_len);
        println!("method_ptrs_addr: {:x}", method_ptrs_addr);

        cur.set_position(method_ptrs_addr as u64);
        let mut method_pointers = Vec::with_capacity(method_ptrs_len as usize);
        for _ in 0..method_ptrs_len {
            method_pointers.push(cur.read_u32::<LittleEndian>()? as u64);
        }

        return Ok(Self { method_pointers });
    }
}

pub struct MetadataRegistration {
    pub types: Vec<Type>, // Il2Cpptypes
    pub field_offset_addrs: Vec<u32>,
}

impl MetadataRegistration {
    pub fn read(elf: &Elf, addr: u64) -> Result<Self, anyhow::Error> {
        let mut cur = Cursor::new(elf.data());
        let addr = vaddr(elf, addr)?;
        cur.set_position(addr + 4 * 6); // 8 -> 4 (32bit)
        let types_len = cur.read_u32::<LittleEndian>()?;
        let types_addr = cur.read_u32::<LittleEndian>()?;
        println!("types_len {}", types_len);
        println!("types_addr {:x}", types_addr);

        cur.set_position(vaddr(elf, types_addr as u64)?);
        let type_addrs: Vec<_> = (0..types_len)
            .map(|_| cur.read_u32::<LittleEndian>())
            .collect();

        //println!("{:x?}", type_addrs);
        let mut types = Vec::with_capacity(types_len as usize);
        for type_addr in type_addrs {
            // parse Il2Cpptype
            cur.set_position(vaddr(elf, type_addr? as u64)?);
            let data: u32 = cur.read_u32::<LittleEndian>()?;
            let attrs = cur.read_u16::<LittleEndian>()? as u32;
            let ty = cur.read_u8()?;
            let bitfield = cur.read_u8()?;
            let by_ref = (bitfield >> 7) != 0;

            types.push(Type {
                data,
                ty,
                by_ref,
                attrs,
            })
        }
        //println!("done parse type");

        cur.set_position(addr + 4 * 10); // fieldOffsetscount?
        let offsets_len = cur.read_u32::<LittleEndian>()?;
        let offsets_addr = cur.read_u32::<LittleEndian>()?;
        //println!("offsets_len: {}", offsets_len);
        //println!("offsets_addr: {:?}", offsets_addr);

        cur.set_position(vaddr(elf, offsets_addr as u64)?);
        let field_offset_addrs = (0..offsets_len)
            .map(|_| cur.read_u32::<LittleEndian>())
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(Self {
            types,
            field_offset_addrs,
        })
    }
}
