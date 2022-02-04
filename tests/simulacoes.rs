
/* aqui serão realizados testes unitários
 * referentes ao ramo de simulações das
 * partidas antigas realizadas. */


// importando lib-externa...
use bate_bola::{banco_de_dados::carrega, simulacao};


#[test]
#[ignore]
fn simulando_primeira_partida() {
   // carregando o BD...
   let mut bd = carrega().unwrap();
   // primeira partida.
   let primeira = bd.remove(&1).unwrap();
   // descartando os demais.
   drop(bd);

   // execução os dados colhidos.
   simulacao(primeira);

   // funcionou o teste.
   assert!(true);
}

#[test]
#[ignore]
fn replay_em_todas_partidas() {
   // carregando o BD...
   let bd = carrega().unwrap();

   // execução os dados colhidos.
   let mut contador:u32 = 1;
   for partida in bd.into_values() { 
      // visualizando demais informações.
      println!(
         "\n\ntestando ... {}ª partida ...
         \r--- --- BARRA --- ---\n{}
         \r--- --- BOLA --- ---\n{}", 
         contador, partida.0, partida.1
      );
      contador += 1;

      // rodando partida em sí.
      simulacao(partida); 
   }

   // funcionou o teste.
   assert!(true);
}

#[test]
fn ultima_jogada_replay() {
   // carregando o BD...
   let mut bd = carrega().unwrap();

   // execução os dados colhidos.
   let ultimo:u16 = (bd.len()-1) as u16;
   let partida = bd.remove(&ultimo).unwrap();
   // descartando demais.
   drop(bd);

   // visualizando demais informações.
   println!(
      "\n\ntestando ... {}ª partida ...
      \r--- --- BARRA --- ---\n{}
      \r--- --- BOLA --- ---\n{}", 
      ultimo, partida.0, partida.1
   );

   // rodando partida em sí.
   simulacao(partida); 

   // funcionou o teste.
   assert!(true);
}
