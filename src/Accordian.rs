use std::{collections::HashMap, borrow::Cow};
use malvolio::{
    attributes::IntoAttribute,
    into_attribute_for_grouping_enum, into_grouping_union,
    prelude::{Id, Style},
    utility_enum,
};

use ammonia::clean;

#[derive(Debug, Clone, Default)]
#[must_use]
pub struct Accordian {
    attrs: HashMap<Cow<'static,str>,Cow<'static,str>>,
    text: Cow<'static,str>,
}

impl Accordian {
    pub fn new() -> Accordian {
        Default::default()
    }
}

pub fn accordian() -> Accordian {
    Accordian::new()
}

impl Accordian {

    pub fn text<S>(mut self, text: S) -> Self
    where
        S: Into<Cow<'static,str>>,
    {
        self.text = clean(&text.into()).into();
        self
    }

    pub fn text_unsanitized<S>(mut self, text: S) -> Self
    where
        S: Into<Cow<'static,str>>,
    {
            self.text = text.into();
            self
    }

    pub fn attribute<I>(mut self, attribute: I ) -> Self
        where Into<AccordianAttr>,
    {
        let res = attribute.into().into_attribute();
        self.attrs.insert(res.0,res.1);
        self
    }
    
    into_grouping_union!(Accordian,BodyNode);

    utility_enum!(
        /// An attribute for Attribute tag.
        #[allow(missing_docs)]
        pub enum AccordianAttr {
            Classes(Classes),
            DefaultExpanded(DefaultExpanded),
            Disabled(Disabled),
            DisabledGutters(DisabledGutters),
            Expanded(Expanded),
            OnChange(OnChange),
            Square(Square),
            Sx(Sx),
            TransitionComponent(TransitionComponent),
            TransitionProps(TransitionProps),
        }
     );
    into_attribute_for_grouping_enum!(Classes,DefaultExpanded,Disabled,DisabledGutters,Expanded,OnChange,Square,Sx,TransitionComponent,TransitionProps);

#[derive(Debug, Clone)]
pub struct Classes {
    class: malvolio::attributes::Class
}

#[derive(Debug,Clone)]
pub struct DefaultExpanded(Cow<'static,bool>);

impl DefaultExpanded {
    pub fn new<bool: V>(bool: V) -> Self
    where
    V: Into<Cow<'static, str>>
    {
        let string = "false";
        if V == true { string = "true" } else { string = "false" };

        Self(string.into())
    }
}
   into_grouping_union!(DefaultExpanded,AccordianAttr);

impl Disabled {
    pub fn new<bool: V>(bool: V) -> Self
    where
    V: Into<Cow<'static,str>>
    {
        let string = "false";
        if V == true { string = "true" } else { string = "false" };

        Self(string.into())
    }
}
    into_grouping_union!(Disabled, AccordianAttr);

impl DisabledGutters {
    pub fn new<bool: V>(bool: V) -> Self
    where
    V: Into<Cow<'static,str>>
    {
        let string = "false";
        if V == true { string = "true" } else { string = "false" }

        Self(string.into())
    }
}

    into_grouping_union!(DisabledGutters,AccordianAttr);


}
