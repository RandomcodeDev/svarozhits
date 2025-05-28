impl < __Context > :: bincode :: Decode < __Context > for SVERelocType
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result :: Ok(Self ::RISCV_NONE {}), 1u32
            =>core :: result :: Result :: Ok(Self ::RISCV_32 {}), 2u32 =>core
            :: result :: Result :: Ok(Self ::RISCV_64 {}), 3u32 =>core ::
            result :: Result :: Ok(Self ::RISCV_RELATIVE {}), 4u32 =>core ::
            result :: Result :: Ok(Self ::RISCV_COPY {}), 5u32 =>core ::
            result :: Result :: Ok(Self ::RISCV_IMPORT {}), 6u32 =>core ::
            result :: Result :: Ok(Self ::RISCV_TLS_DTPMOD32 {}), 7u32 =>core
            :: result :: Result :: Ok(Self ::RISCV_TLS_DTPMOD64 {}), 8u32
            =>core :: result :: Result :: Ok(Self ::RISCV_TLS_DTPREL32 {}),
            9u32 =>core :: result :: Result ::
            Ok(Self ::RISCV_TLS_DTPREL64 {}), 10u32 =>core :: result :: Result
            :: Ok(Self ::RISCV_TLS_TPREL32 {}), 11u32 =>core :: result ::
            Result :: Ok(Self ::RISCV_TLS_TPREL64 {}), 12u32 =>core :: result
            :: Result :: Ok(Self ::RISCV_IRELATIVE {}), variant =>core ::
            result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "SVERelocType", allowed : &::
                bincode :: error :: AllowedEnumVariants :: Range
                { min: 0, max: 12 }
            })
        }
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for SVERelocType
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result :: Ok(Self ::RISCV_NONE {}), 1u32
            =>core :: result :: Result :: Ok(Self ::RISCV_32 {}), 2u32 =>core
            :: result :: Result :: Ok(Self ::RISCV_64 {}), 3u32 =>core ::
            result :: Result :: Ok(Self ::RISCV_RELATIVE {}), 4u32 =>core ::
            result :: Result :: Ok(Self ::RISCV_COPY {}), 5u32 =>core ::
            result :: Result :: Ok(Self ::RISCV_IMPORT {}), 6u32 =>core ::
            result :: Result :: Ok(Self ::RISCV_TLS_DTPMOD32 {}), 7u32 =>core
            :: result :: Result :: Ok(Self ::RISCV_TLS_DTPMOD64 {}), 8u32
            =>core :: result :: Result :: Ok(Self ::RISCV_TLS_DTPREL32 {}),
            9u32 =>core :: result :: Result ::
            Ok(Self ::RISCV_TLS_DTPREL64 {}), 10u32 =>core :: result :: Result
            :: Ok(Self ::RISCV_TLS_TPREL32 {}), 11u32 =>core :: result ::
            Result :: Ok(Self ::RISCV_TLS_TPREL64 {}), 12u32 =>core :: result
            :: Result :: Ok(Self ::RISCV_IRELATIVE {}), variant =>core ::
            result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "SVERelocType", allowed : &::
                bincode :: error :: AllowedEnumVariants :: Range
                { min: 0, max: 12 }
            })
        }
    }
}