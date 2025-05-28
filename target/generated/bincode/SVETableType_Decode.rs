impl < __Context > :: bincode :: Decode < __Context > for SVETableType
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result :: Ok(Self ::Relocation {}), 1u32
            =>core :: result :: Result :: Ok(Self ::Symbol {}), 2u32 =>core ::
            result :: Result :: Ok(Self ::String {}), 3u32 =>core :: result ::
            Result :: Ok(Self ::Export {}), 4u32 =>core :: result :: Result ::
            Ok(Self ::Import {}), 5u32 =>core :: result :: Result ::
            Ok(Self ::ImportPointer {}), 6u32 =>core :: result :: Result ::
            Ok(Self ::MaxTableCount {}), variant =>core :: result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "SVETableType", allowed : &::
                bincode :: error :: AllowedEnumVariants :: Range
                { min: 0, max: 6 }
            })
        }
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for SVETableType
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result :: Ok(Self ::Relocation {}), 1u32
            =>core :: result :: Result :: Ok(Self ::Symbol {}), 2u32 =>core ::
            result :: Result :: Ok(Self ::String {}), 3u32 =>core :: result ::
            Result :: Ok(Self ::Export {}), 4u32 =>core :: result :: Result ::
            Ok(Self ::Import {}), 5u32 =>core :: result :: Result ::
            Ok(Self ::ImportPointer {}), 6u32 =>core :: result :: Result ::
            Ok(Self ::MaxTableCount {}), variant =>core :: result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "SVETableType", allowed : &::
                bincode :: error :: AllowedEnumVariants :: Range
                { min: 0, max: 6 }
            })
        }
    }
}