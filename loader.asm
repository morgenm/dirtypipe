; From https://blog.stalkr.net/2014/10/tiny-elf-3264-with-nasm.html
BITS 64
    org 0x400000
 
ehdr:           ; Elf64_Ehdr
    db 0x7f, "ELF", 2, 1, 1, 0 ; e_ident
    times 8 db 0
    dw  2         ; e_type
    dw  0x3e      ; e_machine
    dd  1         ; e_version
    dq  _start    ; e_entry
    dq  phdr - $$ ; e_phoff
    dq  0         ; e_shoff
    dd  0         ; e_flags
    dw  ehdrsize  ; e_ehsize
    dw  phdrsize  ; e_phentsize
    dw  1         ; e_phnum
    dw  0         ; e_shentsize
    dw  0         ; e_shnum
    dw  0         ; e_shstrndx
    ehdrsize  equ  $ - ehdr

phdr:           ; Elf64_Phdr
    dd  1         ; p_type
    dd  5         ; p_flags
    dq  0         ; p_offset
    dq  $$        ; p_vaddr
    dq  $$        ; p_paddr
    dq  filesize  ; p_filesz
    dq  filesize  ; p_memsz
    dq  0x1000    ; p_align
    phdrsize  equ  $ - phdr

_start:
    call shellcode

shellcode:
    incbin "shellcode.bin"

filesize equ $ - $$
