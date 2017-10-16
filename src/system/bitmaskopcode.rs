// extern crate rand;
//
// trait ByteGenerator {
//     fn generate_byte() -> u8;
// }
//
// struct RandomByteGenerator;
//
// impl ByteGenerator for RandomByteGenerator {
//     /** Return a random 8 bit integer. */
//     fn generate_byte() -> u8 {
//         rand::random()
//     }
// }
//
// struct BitMaskOpcode<G> {
//     mask: u8,
//     generator: G
// }
//
// impl<G: ByteGenerator> BitMaskOpcode<G> {
//     fn new(mask: u8) -> Self {
//         BitMaskOpcode {
//             mask,
//             generator: RandomByteGenerator
//         }
//     }
//
//     fn set_byte_generator(&mut self, generator: G) {
//         self.generator = generator;
//     }
//
//     fn invoke(&self) -> u8 {
//         self.mask & self.generator::generate_byte()
//     }
// }
