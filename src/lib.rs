
// cor transparente:
pub const TRANSPARENTE:i16 = -1;
// velocidade(tempo em miliseg de cada novo quadro).
pub const VELOCIDADE:i32 = 100;
// quantidade limite de toques no chão.
pub const TOQUES_LIMITE:u8 = 3;

// importando modelos:
mod modelos;
pub use self::modelos::*;
pub mod estatisticas;
pub use self::estatisticas::*;
pub mod banco_de_dados;

// biblioteca externa:
extern crate pancurses;
extern crate fastrand;

// oculta documentação da "exportação".
#[doc(hidden)]
pub use pancurses::*;

/* adicionando o módulo que cuida da simulação
 * dos antigos jogos. */
mod simulacao;
pub use simulacao::simulacao;

/* referente a implementação de gráficos 
 * do programa. Trabalha na coloração e
 * visualização de todos elementos que
 * aparecem na tela. */
mod graficos;
pub use graficos::*;

