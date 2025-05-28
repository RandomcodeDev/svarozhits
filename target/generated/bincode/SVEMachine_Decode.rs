impl < __Context > :: bincode :: Decode < __Context > for SVEMachine
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result :: Ok(Self ::RISCV {}), 1u32
            =>core :: result :: Result :: Ok(Self ::AMD64 {}), 2u32 =>core ::
            result :: Result :: Ok(Self ::Unknown {}), variant =>core ::
            result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "SVEMachine", allowed : &::
                bincode :: error :: AllowedEnumVariants ::
                Allowed(& [0u32, 1u32, 2u32])
            })
        }
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for SVEMachine
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result :: Ok(Self ::RISCV {}), 1u32
            =>core :: result :: Result :: Ok(Self ::AMD64 {}), 2u32 =>core ::
            result :: Result :: Ok(Self ::Unknown {}), variant =>core ::
            result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "SVEMachine", allowed : &::
                bincode :: error :: AllowedEnumVariants ::
                Allowed(& [0u32, 1u32, 2u32])
            })
        }
    }
}