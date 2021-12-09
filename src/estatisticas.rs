/*!
   Aqui ficaram estrutura de dados que 
obtem os dados de forma organizada dos 
objetos do jogo, assim como ele como um
todo. 
*/

// bibiotecas:
// padrão:
use std::time::{Instant, Duration};
use std::fmt::{Formatter, Display, Result as Rst};
// minha própria.
use super::modelos::{Barra,Ponto, Direcao, Parede};


/// metadados da barra.
pub struct BarraMetadados {
   /* O tempo que ela leva, sem qualquer
    * interrupção, para ir de um lado 
    * para o outro. */
   pub tempo_travessia:Duration,
   /* a quantia total de rebatidas que 
    * ela efetuou. */
   pub qtd_rebatidas:u16,
   // quantia total de passos involuntários.
   pub total_comandos_dados:u16,
   // fila de todo seu rastro durante o jogo.
   pub rastro:Vec<(Ponto, Direcao)>,
   // comprimento da partida.
   pub comprimento:u8,
   // conta a quantia de colisões com a parede.
   pub colisoes_nas_paredes:u16,
   // atributos privados para processamento.
   ti:Instant,
   vm:f32,
   xo:u16,
}

impl BarraMetadados {
   // gera uma nova instância.
   pub fn gera(c:u8) -> Self {
      /* A maioria dos dados aqui serão 
       * inicializados com valores que, 
       * ao longo o percurso serão mudados. */
      BarraMetadados {
         comprimento: c,
         rastro: Vec::new(),
         total_comandos_dados: 0,
         tempo_travessia: Duration::new(0, 0),
         qtd_rebatidas: 0,
         colisoes_nas_paredes: 0,
         // variáveis para processamento, inicializando...
         ti: Instant::now(),
         vm: 0f32,
         xo: 0,
      }
   }
   // efetua uma atualização dos dados atuais.
   pub fn atualiza(&mut self, brr:&Barra, r:(Ponto, Direcao)) {
      /* vê se colidiu para marcar tempo, e também
       * se novo tempo é maior que o anterior.
       */
      // contando colisão na parede.
      if  brr.colidiu_na_parede() 
         { self.colisoes_nas_paredes += 1; }
      /* partindo do presuposto que o primeiro
       * espaço da array(índice zero) é o primeiro
       * lugar da fila, então o índice 't-1',
       * onde 't' é o comprimento da array é 
       * seu final; vamos colocar o próximo 
       * entrante no final da fila, ou seja, 
       * no fim da array. */
      self.rastro.push(r);
      // medindo a velocidade-média à cada meio-segundo.
      if self.ti.elapsed() > Duration::from_millis(400) {
         let (xi, xf):(i32, i32) = (
            self.xo as i32, 
            brr.esqueleto.posicao.x as i32
         );
         // grande escalar é chars/miliseg.
         self.vm = ((xi-xf).abs() as f32)/400_f32;
         // computando tempo de travessia...
         let c:f32 = (brr.area.largura-brr.comprimento) as f32;
         let t = c / self.vm;
         self.tempo_travessia = Duration::from_millis(t as u64);
         // resetando contador.
         self.ti = Instant::now();
         // marcando nova posição referêncial.
         self.xo = brr.esqueleto.posicao.x;
      }
   }
   /* atalho para gera instância, dando argumentos a todos
    * parâmetros que são públicos e, valores arbitrários 
    * para atributos privados. Tem que ser feito aqui, pois
    * fora do escopo onde fica a estrutura original, não é 
    * aceito tal implementação. */
   pub fn gera_via_parametros(tempo_travessia:Duration, 
   qtd_rebatidas:u16, total_comandos_dados:u16,
   rastro:Vec<(Ponto, Direcao)>, comprimento:u8,
   colisoes_nas_paredes:u16) -> Self {
      // criando instância, apenas anexando o nome do 
      // parâmetro que é o mesmo que o atributo, o 
      // Rust faz o resto.
      BarraMetadados {
         tempo_travessia, qtd_rebatidas,
         total_comandos_dados, rastro,
         comprimento, colisoes_nas_paredes,
         xo:0, vm:0f32, ti:Instant::now()
      }
   }
}

// implementação visualização...
impl Display for BarraMetadados {
   fn fmt(&self, formatacao:&mut Formatter<'_>) -> Rst {
      write!(formatacao, 
         "\rtempo de travesia: {:.1} seg
         \rrebatidas: {}
         \rteclas pressionadas: {}
         \rpercurso total: {} chars
         \rcomprimento da barra: {}
         \rrebotes da parede:{}",
         self.tempo_travessia.as_secs_f32(),
         self.qtd_rebatidas,
         self.total_comandos_dados,
         self.rastro.len(),
         self.comprimento,
         self.colisoes_nas_paredes,
      )
   }
}


/// metadados da bola em movimento.
pub struct BolaMetadados {
   // fila com todo seu rastro desde o ínicio.
   pub rastro:Vec<(Ponto, Direcao)>,
   /* fila com todas suas colisões registradas:
    * o ponto de colisão, a parede onde foi 
    * batida, e, a direção que estáva. */
   pub colisoes:Vec<(Ponto, Parede, Direcao)>,
}

impl BolaMetadados {
   // cria uma nova instância do modelo agregador
   // de dados.
   pub fn gera() -> Self {
      // apenas instância as arrays.
      BolaMetadados {
         rastro: Vec::new(),
         colisoes: Vec::new(),
      }
   }
   // atualiza filas, que são os únicos dados aqui:
   pub fn atualiza( &mut self, 
   r:Option<(Ponto, Direcao)>, 
   c:Option<(Ponto, Parede, Direcao)>) {
      /* presumindo em ambos que, o ínicio
       * da array é dado como o ínicio da 
       * fila, então, para adicionar um novo
       * item nela, apenas colocando os dados
       * no final de ambas. */
      // verificando se o dado é válido.
      match r {
         Some(rastro) => 
            { self.rastro.push(rastro); },
         None => (),
      }
      // veriificando um input válido.
      match c {
         Some(colisao) => 
            { self.colisoes.push(colisao); }
         None => (),
      }
   }
}

impl Display for BolaMetadados {
   fn fmt(&self, formatacao:&mut Formatter<'_>) -> Rst {
      /* contador de colisões em geral, e a quantia
       * de colisões em cada parede; o nome aqui 
       * referente a parede está abreviado. */
      let qtd:f32 = self.colisoes.len() as f32;
      let mut le:f32 = 0_f32;
      let mut ld:f32 = 0_f32;
      let mut ps:f32 = 0_f32;
      let mut pi:f32 = 0_f32;
      let mut c:f32 = 0_f32;
      for x in self.colisoes.iter() {
         // trabalhando respeitando o tipo de parede.
         match x.1 {
            /* calculando a percentagem de colisões as 
             * paredes, e os cantos. */
            Parede::LateralEsquerda => 
               { le += 1.0; },
            Parede::LateralDireita => 
               { ld += 1.0; },
            Parede::Teto => 
               { ps += 1f32; },
            Parede::Chao => 
               { pi += 1f32; },
            Parede::Canto => 
               { c += 1.0; },
            Parede::SemContato => (),
         };
      }
      /* quantias referentes as direções predoninantes
       * durante a partida. */
      let (mut n, mut l, mut o, mut s, mut no, mut ne, mut so, mut se) = 
         (0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
      let qtd_c = self.rastro.len() as f32;
      // computando direções predominantes...
      for tupla in self.rastro.iter() {
         match tupla.1 {
            Direcao::Leste => 
               { l += 1.0/qtd_c },
            Direcao::Oeste =>
               { o += 1.0/qtd_c },
            Direcao::Norte =>
               { n += 1.0/qtd_c },
            Direcao::Sul =>
               { s += 1.0/qtd_c },
            Direcao::Sudeste =>
               { se += 1.0/qtd_c },
            Direcao::Sudoeste =>
               { so += 1.0/qtd_c },
            Direcao::Noroeste =>
               { no += 1.0/qtd_c },
            Direcao::Nordeste =>
               { ne += 1.0/qtd_c },
         }
      }
      write!(formatacao, 
         "\rrebotes na parede: {}
         \rdistribuição de colisões:
         \r   Esquerda... {:.1}%
         \r   Topo... {:.1}%
         \r   Direita... {:.1}%
         \r   Chão... {:.1}%
         \r   Cantos... {:.1}%
         \rdireções dominante: 
         \r   NO({13:0.0}%)\tNE({7:0.0}%)
         \r   O({12:0.0}%)\tL({8:0.0}%) 
         \r   SO({11:0.0}%)\tSE({9:0.0}%) 
         \r   N({6:0.0}%)\tS({10:0.0}%)", 
         self.colisoes.len(),
         // computando fatia das percentagens:
         (le / qtd)*100f32,
         (ps / qtd)*100f32,
         (ld / qtd)*100f32,
         (pi / qtd)*100f32,
         (c  / qtd)*100f32,
         n*100f32,
         ne*100f32,
         l*100f32,
         se*100f32,
         s*100f32,
         so*100f32,
         o*100f32,
         no*100f32
      )
   }
}

