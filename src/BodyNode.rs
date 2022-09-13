use malvolio::prelude::BodyNode;
use malvolio::utility_enum;

utility_enum!(
    pub enum YewMuiBodyNode {
        //imput
        Autocomplete(Autocomplete),
        Button(Button),
        ButtonGroup(ButtonGroup),
    }
);
