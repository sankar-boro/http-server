// macro_rules! factory_tuple ({ $(($n:tt, $T:ident)),+} => {
//     impl<Func, $($T,)+ Res> Factory<($($T,)+), Res> for Func
//     where Func: Fn($($T,)+) -> Res + Clone + 'static,
//           Res: Responder,
//     {
//         fn call(&self, param: ($($T,)+)) -> Res {
//             (self)($(param.$n,)+)
//         }
//     }
// });

// // #[rustfmt::skip]
// mod m {
//     use super::*;

//   factory_tuple!((0, String));
//   factory_tuple!((0, String), (1, FormDataExtractor));
//   // factory_tuple!((0, A));
//   // factory_tuple!((0, A), (1, B), (2, C));
//   // factory_tuple!((0, A), (1, B), (2, C), (3, D));
//   // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E));
//   // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
//   // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
//   // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H));
//   // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I));
//   // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J));
// }