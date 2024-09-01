use lubalia_utils::transcriber::{cursor::TranscriberCursor, error::TranscriptionException};

use crate::{syntax::node::NodeParserTickResult, token::Token};

use super::manifest::ProgramManifest;

pub struct ParsingContext<'a> {
    pub manifest: &'a mut ProgramManifest,
}

impl<'a> ParsingContext<'a> {
    pub fn new(manifest: &'a mut ProgramManifest) -> Self {
        ParsingContext { manifest }
    }
}

impl ParsingContext<'_> {
    pub fn intent<T>(&mut self, cursor: &mut TranscriberCursor<Token>, mut intent: impl FnMut(&mut TranscriberCursor<Token>, &mut Self) -> NodeParserTickResult<T>) -> ParsingIntent<T> {
        ParsingIntent(cursor.intent(move |cursor| intent(cursor, self)).0)
    }
}

pub struct ParsingIntent<T>(pub NodeParserTickResult<T>);

impl<T> ParsingIntent<T> {
    pub fn map<U, F>(self, f: F) -> ParsingIntent<U> where F: FnOnce(T) -> U {
        ParsingIntent(self.0.map(|v| v.map(f)))
    }

    pub fn alt_with_map<U>(
        self,
        cursor: &mut TranscriberCursor<Token>,
        ctx: &mut ParsingContext,
        intent: impl Fn(&mut TranscriberCursor<Token>, &mut ParsingContext) -> NodeParserTickResult<U>,
        map: impl Fn(U) -> T,
    ) -> Self {
        if let Err(TranscriptionException::NotFound(_)) = self.0 {
            Self(intent(cursor, ctx).map(|v| v.map(map)))
        } else {
            self
        }
    }

    pub fn tag(self, tag: String) -> NodeParserTickResult<T> {
        if let Err(TranscriptionException::NotFound(_)) = self.0 {
            Err(TranscriptionException::NotFound(tag))
        } else {
            self.0
        }
    }
}
