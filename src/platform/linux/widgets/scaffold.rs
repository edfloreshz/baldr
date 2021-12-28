use crate::Widget;
use crate::widgets::scaffold::Scaffold;
use crate::widgets::text::Text;
use crate::widgets::appbar::AppBar;

impl<'a> Default for Scaffold<AppBar<Text<'a>>, Text<'a>> {
    fn default() -> Self {
        todo!()
    }
}

impl<'a> Widget for Scaffold<AppBar<Text<'a>>, Text<'a>> {

}