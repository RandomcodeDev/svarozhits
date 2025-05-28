impl :: bincode :: Encode for SVETableType
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        match self
        {
            Self ::Relocation
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (0u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::Symbol
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (1u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::String
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (2u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::Export
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (3u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::Import
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (4u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::ImportPointer
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (5u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            }, Self ::MaxTableCount
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (6u32), encoder) ?
                ; core :: result :: Result :: Ok(())
            },
        }
    }
}