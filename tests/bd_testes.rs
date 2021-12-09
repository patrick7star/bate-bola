

extern crate bate_bola;
use bate_bola::banco_de_dados::carrega;

fn coloca_cabecario_na_str(mut s:String, cabecario:&str) -> String {
   // descobrindo a linha com mais caractéres.
   let mut maior = 0; 
   for ls in s.lines() {
      if ls.len() > maior 
         { maior = ls.len(); }
   }
   let ts = maior-cabecario.len();
   let mut sii = String::new();
   sii += &"=".repeat(ts/2);
   sii += cabecario;
   sii += &"=".repeat(ts/2);
   sii += "\n";
   s.insert_str(0, sii.as_str());
   return s;
}

#[test]
fn visualizando_bd() {
   match carrega() {
      Ok(tabela) => {
         for (k, tupla) in tabela.values().enumerate() {
            println!("\n--- --- --- {}ª inserção --- --- ---",k+1);
            println!(
               "{}\n{}", 
               coloca_cabecario_na_str(tupla.0.to_string(), "BARRA"), 
               coloca_cabecario_na_str(tupla.1.to_string(), "BOLHA")
            );
         }
      },
      Err(_) => println!("erro ao carregar dados do BD"),
   }
}
