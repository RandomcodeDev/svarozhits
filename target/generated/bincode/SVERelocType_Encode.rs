impl :: bincode :: Encode for SVERelocType
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        match self
        {
            Self ::RISCV_NONE
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (0u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_32
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (1u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_64
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (2u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_RELATIVE
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (3u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_COPY
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (4u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_IMPORT
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (5u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_TLS_DTPMOD32
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (6u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_TLS_DTPMOD64
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (7u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_TLS_DTPREL32
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (8u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_TLS_DTPREL64
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (9u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_TLS_TPREL32
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (10u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_TLS_TPREL64
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (11u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::RISCV_IRELATIVE
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (12u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            },
        }
    }
}