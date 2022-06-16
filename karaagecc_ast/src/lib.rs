// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use karaagecc_source::Source;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File<'a> {
    pub source: &'a Source,
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    IntLiteral(i64),
}

impl<'a> File<'a> {
    pub fn new(source: &'a Source) -> Self {
        Self {
            source,
            stmts: Vec::new(),
        }
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = &Source::inline("");
        let f = File::new(s);
        assert_eq!(f.source, s);
        assert!(f.stmts.is_empty());
    }
}

#[macro_export]
macro_rules! ast {
    // root
    (
        $source:expr => [
            $(
                { $( $stmt:tt )* }
            ),+ $(,)?
        ]
    ) => {
        $crate::File{
            source: $source,
            stmts: vec![
                $( $crate::stmt!($( $stmt )*) ),+
            ]
        }
    };
}

#[macro_export]
macro_rules! stmt {
    ($( $input:tt )* ) => {
        $crate::Stmt::Expr($crate::expr!($( $input )*))
    };
}

#[macro_export]
macro_rules! expr {
    (int ( $expr:expr )) => {
        $crate::Expr::IntLiteral($expr)
    };
}

#[cfg(test)]
mod macro_tests {
    use super::ast;
    use crate as ast;
    use karaagecc_source::Source;

    #[test]
    fn test_macro() {
        let s = &Source::inline("1\n2");
        assert_eq!(
            ast! {
                s => [
                    { int(1) },
                    { int(2) },
                ]
            },
            ast::File {
                source: s,
                stmts: vec![
                    ast::Stmt::Expr(ast::Expr::IntLiteral(1)),
                    ast::Stmt::Expr(ast::Expr::IntLiteral(2)),
                ],
            }
        );
    }
}