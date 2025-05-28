impl :: bincode :: Encode for SVERelocDescriptor
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.reloc_type, encoder) ?; ::
        bincode :: Encode :: encode(&self.sym_offset, encoder) ?; core ::
        result :: Result :: Ok(())
    }
}