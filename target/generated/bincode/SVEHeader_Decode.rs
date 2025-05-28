impl < __Context > :: bincode :: Decode < __Context > for SVEHeader
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            magic : :: bincode :: Decode :: decode(decoder) ?, revision : ::
            bincode :: Decode :: decode(decoder) ?, machine : :: bincode ::
            Decode :: decode(decoder) ?, code_blob : :: bincode :: Decode ::
            decode(decoder) ?, data_blob : :: bincode :: Decode ::
            decode(decoder) ?, rodata_blob : :: bincode :: Decode ::
            decode(decoder) ?, zdata_size : :: bincode :: Decode ::
            decode(decoder) ?, table_count : :: bincode :: Decode ::
            decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for SVEHeader
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            magic : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, revision : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, machine : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, code_blob : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, data_blob : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, rodata_blob : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, zdata_size : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, table_count : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?,
        })
    }
}