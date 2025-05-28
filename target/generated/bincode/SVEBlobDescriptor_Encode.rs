impl :: bincode :: Encode for SVEBlobDescriptor
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.offset, encoder) ?; :: bincode ::
        Encode :: encode(&self.size, encoder) ?; core :: result :: Result ::
        Ok(())
    }
}