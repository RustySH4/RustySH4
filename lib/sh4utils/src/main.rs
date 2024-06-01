use elf::endian::AnyEndian;
use elf::note::Note;
use elf::note::NoteGnuBuildId;
use elf::section::SectionHeader;
use elf::ElfBytes;

fn main() {
    let path = std::path::PathBuf::from("../../assets/c_hello/main.o");
    let file_data = std::fs::read(path).expect("Could not read file.");
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Open test1");

    // Get the ELF file's build-id
    let abi_shdr: SectionHeader = file
        .section_header_by_name(".note.gnu.build-id")
        .expect("section table should be parseable")
        .expect("file should have a .note.ABI-tag section");

    let notes: Vec<Note> = file
        .section_data_as_notes(&abi_shdr)
        .expect("Should be able to get note section data")
        .collect();

    // Find lazy-parsing types for the common ELF sections (we want .dynsym, .dynstr, .hash)
    let common = file.find_common_data().expect("shdrs should parse");
    let (dynsyms, strtab) = (common.dynsyms.unwrap(), common.dynsyms_strs.unwrap());
    let hash_table = common.sysv_hash.unwrap();

    println!("{:#?}", hash_table);
}
