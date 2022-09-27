use malvolio::prelude::BodyNode;
use malvolio::utility_enum;

utility_enum!(
    pub enum YewMuiBodyNode {
        //imput
        Autocomplete(Autocomplete),
        Button(Button),
        ButtonGroup(ButtonGroup),
        Box(Box),
        Checkbox(Checkbox),
        FormGroup(FormGroup),
        FormLable(FormLable),
        FormControlLable(FormControlLable),
        Fab(Fab),
        Radio(Radio),
        RadioGroup(RadioGroup),
        Rating(Rating),
        Typography(Typography),
        StyledRating(StyledRating),
        InputLable(InputLable),
        Stack(Stack),
        Slider(Slider),
        Switch(Switch),

    }
);
