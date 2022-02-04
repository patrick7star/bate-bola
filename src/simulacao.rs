

/**
 Aqui vamos criar um simulador que roda
 como se desenvolveu as partidas realizadas
 anteriormente.
 */


/* bibliotecas necessárias, tanto 
 * interna como externas com padrão
 * no Rust. */
use super::modelos::{Ponto, Dimensao, Bola, Barra};
use super::estatisticas::{BarraMetadados, BolaMetadados};
use crate::VELOCIDADE;
use pancurses::{
   curs_set, resize_term, noecho, initscr,
   start_color, use_default_colors, newwin, 
   Input, napms, endwin
};

/// tanto metadados da 'Barra' e da 'Bola'.
type Metadados = (BarraMetadados, BolaMetadados);


/* Cede um método para o tipo de dado, onde
 * ele segue a posição injetada, independente 
 * como interaje com os demais objetos da tela. */
trait MovimentoRestrito {
   /* faz o movimento, independente da colisão com 
    * qualquer outro objeto em tela. */
   fn move_a_posicao(&mut self, nova_posicao:Ponto); 
}

// implementando o trait para o tipo 'Bola'.
impl MovimentoRestrito for Bola {
   fn move_a_posicao(&mut self, nova_posicao:Ponto) { 
      self.esqueleto.posicao = nova_posicao;
   }
}

// mesma implementação para o tipo 'Barra'.
impl MovimentoRestrito for Barra {
   fn move_a_posicao(&mut self, nova_posicao:Ponto) { 
      self.esqueleto.posicao = nova_posicao;
   }
}


// computa a possível dimensão do "tabuleiro" do jogo.
fn acha_dimensao_tabuleiro(dados_i:&BarraMetadados, dados_ii:&BolaMetadados)
-> Dimensao {
   // váriaveis que guardam máximo e mínimo.
   let (mut min_y, mut max_y):(u16, u16) = (u16::MAX, u16::MIN);
   let (mut min_x, mut max_x):(u16, u16) = (u16::MAX, u16::MIN);

   // primeiro, buscando nos dados da barra.
   let coordenadas_barra = {
      dados_i.rastro
      .iter()
      .map(|tupla| tupla.0)
   };
   //obtem apenas os valores referentes 'Ponto's da tupla.
   let coordenadas_bola = {
      dados_ii.rastro
      .iter()
      .map(|tupla| tupla.0)
   };
   // mesclando ambos iteradores para avaliar em apenas um laço.
   let coordenadas = coordenadas_barra.chain(coordenadas_bola);
   
   /* primeiro busca computar 'Dimensão baseado no
    * rastro da 'Barra', posteriormente, melhora
    * à aproximação mastigando dados da 'Bola'. */
   for ponto in coordenadas {
      // pega um máximo 'x' se houver.
      if max_x < ponto.x 
         { max_x = ponto.x; } 
      // pega um máximo 'y' se houver.
      if max_y < ponto.y 
         { max_y = ponto.y; }
      // pega um mínimo 'y' se houver.
      if min_y > ponto.y 
         { min_y = ponto.y; }
      // pega um mínimo 'x' se houver.
      if min_x > ponto.x 
         { min_x = ponto.x; }
   }

   /* baseado na diferença do menor e maior
    * de cada coordenada, calculamos as 
    * proporções da 'Dimensão' de cada 
    * eixo aqui. */
   Dimensao { 
      altura: max_y - min_y,
      largura: max_x - min_x
   }
}

/** executa a simulação de uma partida antiga 
 dado todos suas posições iniciais e finais,
 direções a cada instante de tempo, em cada
 posição. */
pub fn simulacao(dados:Metadados) {
   // computando a área do tabuleiro, baseado nos dados.
   let area:Dimensao = acha_dimensao_tabuleiro(&dados.0, &dados.1);
   // dados iniciais da 'Bola'.
   let parametros = dados.1;
   let mut bola = Bola::nova(
      parametros.rastro[0].1, 
      parametros.rastro[0].0 , 
      area
   );
   // todas posições da 'Bola' em cada instante.
   let rastros_bola = {
      parametros
      .rastro[1..]
      .iter()
      .map(|t| t.0)
   };
   // criando uma instância inicial.
   let parametros = dados.0;
   let mut barra = Barra::nova(
      parametros.comprimento as u16, 
      '^', parametros.rastro[0].0,
      area
   );
   // todas posições da 'Barra' em cada instante.
   let rastros_barra = {
      parametros
      .rastro[1..]
      .iter()
      .map(|t| t.0)
   };
   // tem que ter o mesmo tamanho
   assert_eq!(rastros_barra.len(), rastros_bola.len());

   /* criando tela para demonstração. Todo
    * o ambiente do ncurses será refeito aqui
    * especificamente para a simulação, algumas
    * coisas serão diferentes do layout original,
    * mas o tabuleiro o mesmo no final. */
   // instanciando elementos da janela.
   let janela = initscr();
   // criando tabuleiro com dimensão exatada da partida.
   let tabuleiro = newwin(
      area.altura as i32, 
      area.largura as i32, 
      0, 0
   );
   // configuração da janela:
   tabuleiro.keypad(true);
   tabuleiro.nodelay(true);
   janela.nodelay(true);
   curs_set(0);
   noecho();
   // inicia coloração.
   start_color();
   use_default_colors();

   /* executa o jogo até que todas posições dos
    * objetos tenham sido escritas pelos lugares
    * já destinados na tela. */
   for (p1, p2) in rastros_barra.zip(rastros_bola) { 
      // apaga "frame" anterior.
      tabuleiro.clear();
      // desenha as bordas do tabuleiro.
      let at = tabuleiro.get_max_y();
      let lt = tabuleiro.get_max_x();
      /* se a largura ou altura tiver sido
       * automaticamente redimensionada, então
       * voltar ela para proporção normal antes
       * de redesenhar a borda novamente. */
      if  (at > area.altura as i32) ||
      (lt > area.largura as i32) { 
         resize_term(area.altura as i32, area.largura as i32); 
      }
      tabuleiro.border(0, 0, 0, 0, 0, 0, 0, 0);

      // se detectar redimensionamento de janela...
      if let Some(Input::KeyResize) = janela.getch() {
         // variável com nomes curtos para legibilidade.
         let aj = janela.get_max_y();
         let lj = janela.get_max_x();
         /* se espaçar muito da janela principal
          * então centralizar menorzinha. */
         if aj - at > 5 || lj - lt > 3 {
            /* computando coordenadas do centro-superior-esquerdo
             * da janelinha para centralizá-la. */
            let yc = (aj/2) - (at/2);
            let xc = (lj/2) - (lt/2);
            tabuleiro.mvwin(yc, xc);
            // limpa janela maior para não deixar rastros.
            janela.clear();
         }
      }

      // coloca bola na posição tal.
      bola.move_a_posicao(p2.clone());
      // coloca barra na posição tal.
      barra.move_a_posicao(p1.clone());

      // desenha bola e barra:
      bola.plota_bola(&tabuleiro);
      barra.plota_barra(&tabuleiro);

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(VELOCIDADE);
   }

   // termina ambiente gráfico.
   endwin();
}
