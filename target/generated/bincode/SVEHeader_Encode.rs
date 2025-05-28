impl :: bincode :: Encode for SVEHeader
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.magic, encoder) ?; :: bincode ::
        Encode :: encode(&self.revision, encoder) ?; :: bincode :: Encode ::
        encode(&self.machine, encoder) ?; :: bincode :: Encode ::
        encode(&self.code_blob, encoder) ?; :: bincode :: Encode ::
        encode(&self.data_blob, encoder) ?; :: bincode :: Encode ::
        encode(&self.rodata_blob, encoder) ?; :: bincode :: Encode ::
        encode(&self.zdata_size, encoder) ?; :: bincode :: Encode ::
        encode(&self.table_count, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}