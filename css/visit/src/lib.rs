use swc_atoms::JsWord;
use swc_common::Span;
use swc_css_ast::*;
use swc_visit::define;

/// Visitable nodes.
pub trait Node {}

impl<T: ?Sized> Node for T {}

define!({
    pub struct Text {
        pub span: Span,
        pub value: JsWord,
    }

    pub struct Str {
        pub span: Span,
        pub value: JsWord,
    }

    pub struct Num {
        pub span: Span,
        pub value: f64,
    }

    pub struct Property {
        pub span: Span,
        pub name: Text,
        pub values: Vec<Value>,
        pub important: Option<Span>,
    }

    pub struct StyleRule {
        pub span: Span,
        pub selectors: Vec<ComplexSelector>,
        pub block: DeclBlock,
    }

    pub struct DeclBlock {
        pub span: Span,
        pub properties: Vec<Property>,
    }

    pub struct Tokens {
        pub span: Span,
        pub tokens: Vec<TokenAndSpan>,
    }

    pub struct TokenAndSpan {
        pub span: Span,
        pub token: Token,
    }

    pub struct Unit {
        pub span: Span,
        pub kind: UnitKind,
    }

    pub enum Value {
        Paren(ParenValue),

        Unit(UnitValue),

        Number(Num),

        Percent(PercentValue),

        Hash(HashValue),

        Text(Text),

        Str(Str),

        Fn(FnValue),

        Bin(BinValue),

        Array(ArrayValue),

        Space(SpaceValues),

        Brace(BraceValue),

        Lazy(Tokens),

        AtText(AtTextValue),

        Url(UrlValue),
    }

    pub struct SpaceValues {
        pub span: Span,
        pub values: Vec<Value>,
    }

    pub struct BinValue {
        pub span: Span,

        pub op: BinOp,

        pub left: Box<Value>,

        pub right: Box<Value>,
    }

    pub struct FnValue {
        pub span: Span,

        pub name: Text,

        pub args: Vec<Value>,
    }

    pub struct ParenValue {
        pub span: Span,

        pub value: Option<Box<Value>>,
    }

    pub struct ArrayValue {
        pub span: Span,

        pub values: Vec<Value>,
    }

    pub struct HashValue {
        pub span: Span,
        pub value: JsWord,
    }

    pub struct UnitValue {
        pub span: Span,
        pub value: Num,
        pub unit: Unit,
    }

    pub struct PercentValue {
        pub span: Span,
        pub value: Num,
    }

    pub struct BraceValue {
        pub span: Span,
        pub value: Box<Value>,
    }

    pub struct AtTextValue {
        pub span: Span,
        pub name: Text,
        pub block: Option<BraceValue>,
    }

    pub struct UrlValue {
        pub span: Span,

        pub url: JsWord,
    }

    pub struct ComplexSelector {
        pub span: Span,
        pub selectors: Vec<CompoundSelector>,
    }

    pub struct CompoundSelector {
        pub span: Span,

        pub has_nest_prefix: bool,

        pub combinator: Option<SelectorCombinator>,

        pub type_selector: Option<NamespacedName>,

        pub subclass_selectors: Vec<SubclassSelector>,
    }

    pub struct NamespacedName {
        pub span: Span,
        pub prefix: Option<Text>,
        pub name: Text,
    }

    pub enum SubclassSelector {
        Id(IdSelector),

        Class(ClassSelector),

        Attr(AttrSelector),

        Pseudo(PseudoSelector),

        At(AtSelector),
    }

    pub struct AttrSelector {
        pub span: Span,
        pub name: NamespacedName,
        pub op: Option<AttrSelectorOp>,
        pub value: Option<Str>,
        pub modifier: Option<char>,
    }

    pub struct PseudoSelector {
        pub span: Span,
        pub is_element: bool,
        pub name: Text,
        pub args: Tokens,
    }

    pub struct UniversalSelector {
        pub span: Span,
    }

    pub struct IdSelector {
        pub span: Span,
        pub text: Text,
    }

    pub struct ClassSelector {
        pub span: Span,
        pub text: Text,
    }

    pub struct TagSelector {
        pub span: Span,
        pub text: Text,
    }

    pub struct AtSelector {
        pub span: Span,
        pub text: Text,
    }

    pub struct Stylesheet {
        pub span: Span,
        pub rules: Vec<Rule>,
    }

    pub enum Rule {
        Style(StyleRule),

        AtRule(AtRule),
    }

    pub struct Invalid {
        pub span: Span,
    }

    pub enum AtRule {
        Charset(CharsetRule),
        Import(ImportRule),
        FontFace(FontFaceRule),
        Keyframes(KeyframesRule),
        Media(MediaRule),
        Supports(SupportsRule),
        Page(PageRule),
        Namespace(NamespaceRule),
        Viewport(ViewportRule),
        Document(DocumentRule),
        Unknown(UnknownAtRule),
    }

    pub struct CharsetRule {
        pub span: Span,
        pub charset: Str,
    }

    pub struct ImportRule {
        pub span: Span,
        pub src: Str,
        pub condition: Option<MediaQuery>,
    }

    pub struct FontFaceRule {
        pub span: Span,
        pub block: DeclBlock,
    }

    pub struct NamespaceRule {
        pub span: Span,
        pub prefix: Text,
        pub value: Str,
    }

    pub struct ViewportRule {
        pub span: Span,
        pub block: DeclBlock,
    }

    pub struct UnknownAtRule {
        pub span: Span,
        pub name: Text,
        pub tokens: Tokens,
    }

    pub struct DocumentRule {
        pub span: Span,
        pub selectors: Vec<FnValue>,
        pub block: Vec<Rule>,
    }

    pub struct KeyframesRule {
        pub span: Span,
        pub id: Text,
        pub blocks: Vec<KeyframeBlock>,
    }

    pub struct KeyframeBlock {
        pub span: Span,
        pub selector: Vec<KeyframeSelector>,
        pub rule: KeyframeBlockRule,
    }

    pub enum KeyframeSelector {
        Id(Text),
        Percent(PercentValue),
    }

    pub enum KeyframeBlockRule {
        Decl(Box<DeclBlock>),
        AtRule(Box<AtRule>),
    }

    pub struct MediaRule {
        pub span: Span,

        pub query: Box<MediaQuery>,

        pub rules: Vec<Rule>,
    }

    pub enum MediaQuery {
        Text(Text),
        And(AndMediaQuery),
        Or(OrMediaQuery),
        Not(NotMediaQuery),
        Only(OnlyMediaQuery),
        Property(Property),
        Comma(CommaMediaQuery),
    }

    pub struct AndMediaQuery {
        pub span: Span,
        pub left: Box<MediaQuery>,
        pub right: Box<MediaQuery>,
    }

    pub struct OrMediaQuery {
        pub span: Span,
        pub left: Box<MediaQuery>,
        pub right: Box<MediaQuery>,
    }

    pub struct NotMediaQuery {
        pub span: Span,
        pub query: Box<MediaQuery>,
    }

    pub struct OnlyMediaQuery {
        pub span: Span,
        pub query: Box<MediaQuery>,
    }

    pub struct CommaMediaQuery {
        pub span: Span,
        pub queries: Vec<MediaQuery>,
    }

    pub struct PageRule {
        pub span: Span,

        pub prelude: Vec<PageSelector>,

        pub block: PageRuleBlock,
    }

    pub struct PageSelector {
        pub span: Span,

        pub ident: Option<Text>,

        pub pseudo: Option<Text>,
    }

    pub struct PageRuleBlock {
        pub span: Span,
        pub items: Vec<PageRuleBlockItem>,
    }

    pub enum PageRuleBlockItem {
        Property(Box<Property>),
        Nested(Box<NestedPageRule>),
    }

    pub struct NestedPageRule {
        pub span: Span,

        pub prelude: Vec<ComplexSelector>,

        pub block: PageRuleBlock,
    }

    pub struct SupportsRule {
        pub span: Span,

        pub query: SupportQuery,

        pub rules: Vec<Rule>,
    }

    pub enum SupportQuery {
        Not(NotSupportQuery),
        And(AndSupportQuery),
        Or(OrSupportQuery),
        Property(Property),
        Paren(ParenSupportQuery),
    }

    pub struct NotSupportQuery {
        pub span: Span,
        pub query: Box<SupportQuery>,
    }

    pub struct AndSupportQuery {
        pub span: Span,
        pub left: Box<SupportQuery>,
        pub right: Box<SupportQuery>,
    }

    pub struct OrSupportQuery {
        pub span: Span,
        pub left: Box<SupportQuery>,
        pub right: Box<SupportQuery>,
    }

    pub struct ParenSupportQuery {
        pub span: Span,
        pub query: Box<SupportQuery>,
    }
});
