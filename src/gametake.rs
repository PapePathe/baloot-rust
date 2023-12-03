#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub struct Take (pub u8);

impl Take {
  pub const TOUT: Take = Take(6);
  pub const CENT: Take = Take(5);
  pub const SPADE: Take = Take(4);
  pub const HEART: Take = Take(3);
  pub const DIAMOND: Take = Take(2);
  pub const CLUBS: Take = Take(2);
  pub const SKIP: Take = Take(0);
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameTake {
    Tout(Take),
    Cent(Take),
    Spade(Take),
    Heart(Take),
    Diamond(Take),
    Club(Take),
    Skip(Take),
}
