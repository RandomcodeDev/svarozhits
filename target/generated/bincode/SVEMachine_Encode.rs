impl :: bincode :: Encode for SVEMachine
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        match self
        {
            Self ::RISCV
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (0u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::AMD64
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (1u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::Unknown
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (2u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            },
        }
    }
}