mod generated;

use biome_rowan::{Language, RawSyntaxKind, SyntaxKind};
use generated::OscFile;

use self::generated::OscDslSyntaxKind;

impl From<u16> for OscDslSyntaxKind {
    fn from(v: u16) -> Self {
        assert!(v <= OscDslSyntaxKind::__LAST as u16);
        unsafe { std::mem::transmute::<u16, OscDslSyntaxKind>(v) }
    }
}

impl From<OscDslSyntaxKind> for u16 {
    fn from(v: OscDslSyntaxKind) -> Self {
        v as u16
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OscDslLanguage;

impl Language for OscDslLanguage {
    type Kind = OscDslSyntaxKind;

    type Root = OscFile;
}

impl SyntaxKind for OscDslSyntaxKind {
    const TOMBSTONE: Self = OscDslSyntaxKind::TOMBSTONE;

    const EOF: Self = OscDslSyntaxKind::EOF;

    fn is_bogus(&self) -> bool {
        *self == OscDslSyntaxKind::BOGUS
    }

    fn to_bogus(&self) -> Self {
        OscDslSyntaxKind::BOGUS
    }

    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        *self == OscDslSyntaxKind::OSC_FILE
    }

    fn is_list(&self) -> bool {
        OscDslSyntaxKind::is_list(*self)
    }

    fn to_string(&self) -> Option<&'static str> {
        OscDslSyntaxKind::to_string(*self)
    }
}
