// biblioteca padrão do Rust:
use std::fmt::{Display, Debug, Formatter, Result as R};
use std::cmp::PartialEq;

/// enum todas direções de movimentos.
#[derive(Copy,Clone, Debug)]
pub enum Direcao { 
   Norte, 
   Oeste, 
   Leste, 
   Sul, 
   Noroeste, 
   Sudoeste, 
   Nordeste, 
   Sudeste 
}
impl PartialEq for Direcao {
   // verifica se são iguais.
   fn eq(&self, other:&Self) -> bool {
      // qual direção o argumento da direita retorna.
      let direita:&str = match *self {
         Direcao::Norte => "N",
         Direcao::Leste => "L",
         Direcao::Sul => "S",
         Direcao::Oeste => "O",
         Direcao::Noroeste => "NO",
         Direcao::Nordeste => "ND",
         Direcao::Sudeste => "SD",
         Direcao::Sudoeste => "SO",
      };
      // qual direção o argumento da esquerda retorna.
      let esquerda:&str = match other {
         Direcao::Norte => "N",
         Direcao::Leste => "L",
         Direcao::Sul => "S",
         Direcao::Oeste => "O",
         Direcao::Noroeste => "NO",
         Direcao::Nordeste => "ND",
         Direcao::Sudeste => "SD",
         Direcao::Sudoeste => "SO",
      };
      // ele são iguais?   
      return direita == esquerda;
      /*
      match *self {
         other => true,
         _ => false,
      }*/

   }
   // retorna a negação do primeiro método.
   fn ne(&self, other:&Self) -> bool {
      ! self.eq(other)
   }
}

/** estrutura para localizar qualquer coisa 
 na tela, ou a abstração de tela.  
*/
#[derive(Copy,Clone)]
pub struct Ponto { pub y:u16, pub x: u16 }
impl Display for Ponto {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> R{
      return write!(formatador, "linha={0} coluna={1}", 
                     self.y, self.x);
   }
}
impl Debug for Ponto {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> R{
      return write!(formatador, "y={0} x={1}", self.y, self.x);
   }
}
impl PartialEq for Ponto {
   fn eq(&self, ponto:&Ponto) -> bool {
      self.y == ponto.y && self.x == ponto.x
   }
   fn ne(&self, ponto:&Ponto) -> bool {
      return !(self.eq(ponto));
   }
}

/// objeto que se movimenta na tela. 
#[derive(Clone,Copy)]
pub struct Seta {
   // onde o direcionador está tendendo...
   pub sentido:Direcao,
   // atual posição na tela(y,x).
   pub posicao:Ponto,
   // símbolo do direcionador.
   pub forma:char,
   // sentido marcado após alteração do primeiro sentido.
   pub antiga_posicao:Ponto,
}
impl Seta {
   // cria joystick...
   pub fn cria(dir:Direcao, pt:Ponto, simb:char) -> Seta {
      // retorna estrutura criada.
      Seta { 
         sentido: dir,
         posicao:pt,
         forma: simb,
         antiga_posicao:Ponto{y:0, x:0},
      }
   }

   // faz um movimento...
   pub fn faz_passo(&mut self, novo:Direcao) {
      // registrando antiga posição.
      self.antiga_posicao.y = self.posicao.y;
      self.antiga_posicao.x = self.posicao.x;
      // alterando orientação para novo-sentido.
      self.sentido = novo;
      // movendo de acordo com a direção dada.
      match novo {
         Direcao::Norte => { 
            if self.posicao.y >= 1
               { self.posicao.y -= 1; }
         },
         Direcao::Sul => 
            {self.posicao.y += 1;}, 
         Direcao::Oeste => { 
            if self.posicao.x >= 1
               { self.posicao.x -= 1; }
         },
         Direcao::Leste => 
            {self.posicao.x += 1;},
         // todos passos compostos:
         Direcao::Noroeste => {
            if self.posicao.y >= 1 && self.posicao.x >= 1 { 
               // um passo para esquerda.
               self.posicao.x -= 1;
               // e outro para cima.
               self.posicao.y -= 1; 
            }
         },
         Direcao::Nordeste => {
            // um passo à direita.
            self.posicao.x += 1;
            // só passos válidos para evitar 'overflow'.
            if self.posicao.y >= 1 { 
               // e outro para cima.
               self.posicao.y -= 1; 
            }
         },
         Direcao::Sudeste => {
            // um passo para direita.
            self.posicao.x += 1;
            // e outro para baixo.
            self.posicao.y += 1; 
         },
         Direcao::Sudoeste => {
            // e outro para baixo.
            self.posicao.y += 1; 
            // só passos válidos para evitar 'overflow'.
            if self.posicao.x >= 1 { 
               // um passo para esquerda.
               self.posicao.x -= 1;
            }
         },
      };
   }
}


/// barra que flutua de um lado para outro.
pub struct Barra {
   // comprimento da barra.
   pub comprimento:u16,
   /* "Seta" que já se movimenta, baseado em 
    * posição, direção, e também define um 
    * formato dado. */
   pub esqueleto:Seta,
   // sua dimensão, foque na largura de tal.
   pub area: Dimensao
}
impl Barra {
   // cria nova forma dado comprimento e formato.
   pub fn nova(cmpr:u16, molde:char, pi:Ponto, dim:Dimensao) -> Self {
      Self {
         esqueleto: Seta::cria(Direcao::Leste, pi, molde),
         comprimento: cmpr,
         area: dim
      }
   }

   // move a barra-flutuante dada a direção.
   pub fn r#move(&mut self, dir:Direcao) { 
      // se estiver entre os limites da tela, então mover.
      if !self.colidiu_na_parede() {
         match dir {
            Direcao::Leste | Direcao::Oeste => {
               // se o comando é na mesma direção, então acelerar.
               self.esqueleto.faz_passo(dir);
            },
            _ => self
                 .esqueleto
                 .faz_passo(self.esqueleto.sentido),
         };
      }
      // se estiver no limite das telas, aceitar apenas
      // o oposto.
      else {
         // se colidiu com a parede, dependendo de qual
         // ir ao lado oposto depois disso.
         if self.esqueleto.sentido == Direcao::Leste
            { self.esqueleto.faz_passo(Direcao::Oeste); }
         else 
            { self.esqueleto.faz_passo(Direcao::Leste); }
      }
   }

   // verifica se a barra colidiu com as paredes.
   pub fn colidiu_na_parede(&self) -> bool {
      // alias para legibilidade da expressão lógica.
      let c1 = self.comprimento + self.esqueleto.posicao.x;
      let c2 = self.area.largura;
      // verifica se é igual, ou posição é zero.
      (c1 == c2) || self.esqueleto.posicao.x == 0
   }

   /* verifica se dada posição de demais objeto
    * colide com o corpo da 'barra'. */
   pub fn foi_acertada(&self, p:Ponto) -> bool {
      // todos a pele de contato da barra.
      let mut campo_contato:Vec<Ponto> = Vec::new();
      // renomeando valores trabalhados para legibilidade.
      let px = self.esqueleto.posicao.x;
      let py = self.esqueleto.posicao.y;
      let cmp = self.comprimento;
      let sentido = self.esqueleto.sentido;
      // se a direção for sul ao acertar discartar.
      if sentido == Direcao::Sudeste
         || sentido == Direcao::Sudoeste 
         || sentido == Direcao::Sul 
         || sentido == Direcao::Noroeste 
         || sentido == Direcao::Nordeste && 
         ( 
            (px == p.x && py-1 == p.y) || 
            (px+cmp == p.x  && py-1 == p.y) ||
            (px == p.x && py == p.y ) || 
            (px+cmp == p.x && py == p.y)
         )
            { return false; }
      for k in 0..self.comprimento {
         campo_contato.push(Ponto{x:px+k, y:py+1});
         // se for o caso dos extremos-inferiores, então descartar.
         // parte inferior da barra.
         campo_contato.push(Ponto{x:px+k, y:py-1});
      }
      // se ponto for o mesmo dos "pertencentes"
      // a barra, quer dizer que ela foi atinginda.
      for ponto in campo_contato.into_iter() {
         if ponto == p { return true; }
      }
      // este é o padrão assumido inicialmente.
      false
   }
}


/** para ter a dimensão da tela que se está trabalhando
 e como a tela do jogo é quase inteiramente a do 
 terminal, então varia bastante, variando assim
 também o tempo, importanto!  
*/
#[derive(Copy, Clone)]
pub struct Dimensao { 
   pub largura:u16, 
   pub altura:u16 
}
// implementando sua formatação...
impl Display for Dimensao {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> R {
      write!(formatador, "{}x{}", self.altura, self.largura)
   }
}

/** objeto que rebate nas paredes, e volta,
 e se também rebate na barra, quica de volta
 para parede. 
*/
pub struct Bola { 
   // objeto que se move.
   pub esqueleto:Seta,
   // seus limites de movimento no plano.
   pub area:Dimensao,
}
impl Bola {
   // cria a instância da bola que caí/move-se.
   pub fn nova(di:Direcao, pi:Ponto, a:Dimensao) -> Self {
      // cria um vetor móvel.
      return Bola {
         // instância do objeto móvel.
         esqueleto: Seta::cria(di, pi, 'O'),
         // dimensão da tela que este move-se.
         area: a,
      }
   }

   // faz o movimento da bola, este, que é em 
   // todos sentidos possíveis.
   pub fn r#move(&mut self) {
      let nova_dir = self.direcao_pos_colisao();
      self.esqueleto.faz_passo(nova_dir);
   }

   /* verifica se a bola passou os limites
    * da tela, assim dá o alerta se ele 
    * pode ser desenhado ou não. */
   pub fn furou_tela(&self) -> bool {
      // obtendo ambos valores.
      let y1 = self.esqueleto.posicao.y; 
      let y2 = self.area.altura;
      // se o valor 'y' for maior que a altura
      // da dimensão, quer dizer que passou limite.
      if y1 >= y2 { true }
      // caso contrário, não.
      else { false }
   }

   /* verifica se houve uma colisão com as
    * paredes superior e laterais. */
   pub fn colidiu(&self) -> (bool, Parede) {
      // alias dos valores:
      let y = self.esqueleto.posicao.y;
      let x = self.esqueleto.posicao.x;
      // fim dos cantos inferior e esquerdo.
      let a = self.area.altura-1;
      let c = self.area.largura-1;
      // descobre parede.
      let qual_parede:Parede = {
         if (y >= 1 && y <= a-1) && x == 0
            { Parede::LateralEsquerda }

         else if (y >= 1 && y <= a-1)  && x == c
            { Parede::LateralDireita }

         else if (x >= 1 && x <= c-1) && y == 0
            { Parede::Teto }

         else if (x >= 1 && x <= c-1) && y == a 
            { Parede::Chao }
         
         else if (x == 0 && y == 0) ||
         (x == c && y == a) ||
         (x == 0 && y == a) ||
         (x == c && y == 0) { Parede::Canto }

         else 
            { Parede::SemContato }
      };

      // na lateral esquerda.
      if  y <= a && x == 0 
         { (true, qual_parede) }
      // no teto.
      else if y == 0 && x <= c 
         { (true, qual_parede) }
      // na lateral direita.
      else if y <= a && x == c 
         { (true, qual_parede) }
      else if y == a && x <= c
         { (true, qual_parede) }
      // nenhum caso acima, então sem colisão.
      else
         { (false, qual_parede) }
   }

   /* dado uma colisão, computar direção
    * de rebote que o objeto irá seguir. */
   fn direcao_pos_colisao(&mut self) -> Direcao {
      // verifica se houve colisão primeiramente. 
      let (_, wall):
      (bool, Parede) = { self.colidiu() };
      // atual sentido.
      let sentido = self.esqueleto.sentido;
      
      // baseado na parede acertada...
      match wall {
         Parede::LateralEsquerda => {
            match sentido {
               Direcao::Noroeste =>  Direcao::Nordeste,
               Direcao::Sudoeste =>  Direcao::Sudeste,
               Direcao::Oeste | _ => Direcao::Leste,
            }
         },
         Parede::Teto => {
            match sentido {
               Direcao::Nordeste => Direcao::Sudeste,
               Direcao::Noroeste => Direcao::Sudoeste,
               Direcao::Norte | _ => Direcao::Sul,
            }
         },
         Parede::LateralDireita => {
            match sentido {
               Direcao::Sudeste => Direcao::Sudoeste,
               Direcao::Nordeste => Direcao::Noroeste,
               Direcao::Leste | _ => Direcao::Oeste,
            }
         },
         Parede::Chao => {
            match sentido  {
               Direcao::Sudeste => Direcao::Nordeste,
               Direcao::Sudoeste => Direcao::Noroeste,
               Direcao::Sul => Direcao::Norte,
               _ => Direcao::Norte,
            }
         },
         Parede::Canto => {
            match sentido {
               Direcao::Nordeste => Direcao::Sudoeste,
               Direcao::Noroeste => Direcao::Sudeste,
               Direcao::Sudeste => Direcao::Noroeste,
               Direcao::Sudoeste => Direcao::Nordeste,
               // vindo dos cantos replica sentido anti-horario.
               _ => Direcao::Leste
            }
         },
         Parede::SemContato =>
            self.esqueleto.sentido,
      }
   }
}


// enum representando todas paredes:
#[derive(Debug)]
pub enum Parede {
   LateralEsquerda,
   LateralDireita,
   Teto,
   Chao,
   Canto, // não têm uma parede definida.
   SemContato, // não atingiu a parede.
}
