use malvolio::body::body_node::BodyNode::BodyNode;
use malvolio::utility_enum;

utility_enum!(

    pub enum yew_mui_body_node {
        //imput
        Autocomplete(Autocomplete),
        Button(Button),
        ButtonGroup(ButtonGroup),
    }
);
