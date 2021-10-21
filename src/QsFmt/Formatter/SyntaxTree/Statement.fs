﻿// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

namespace Microsoft.Quantum.QsFmt.Formatter.SyntaxTree

type ParameterDeclaration = { Name: Terminal; Type: TypeAnnotation }

type ParameterBinding =
    | ParameterDeclaration of ParameterDeclaration
    | ParameterTuple of ParameterBinding Tuple

type SymbolBinding =
    | SymbolDeclaration of Terminal
    | SymbolTuple of SymbolBinding Tuple

module SymbolBinding =
    let mapPrefix mapper =
        function
        | SymbolDeclaration terminal -> terminal |> Terminal.mapPrefix mapper |> SymbolDeclaration
        | SymbolTuple tuple -> tuple |> Tuple.mapPrefix mapper |> SymbolTuple

type SingleQubit =
    {
        Qubit: Terminal
        OpenParen: Terminal
        CloseParen: Terminal
    }

type QubitArray =
    {
        Qubit: Terminal
        OpenBracket: Terminal
        Length: Expression
        CloseBracket: Terminal
    }

type QubitInitializer =
    | SingleQubit of SingleQubit
    | QubitArray of QubitArray
    | QubitTuple of QubitInitializer Tuple

type QubitBinding =
    {
        Name: SymbolBinding
        Equals: Terminal
        Initializer: QubitInitializer
    }

module QubitBinding =
    let mapPrefix mapper (binding: QubitBinding) =
        { binding with Name = SymbolBinding.mapPrefix mapper binding.Name }

type QubitDeclarationKind =
    | Use
    | Borrow

type ForBinding =
    {
        Name: SymbolBinding
        In: Terminal
        Value: Expression
    }

module ForBinding =
    let mapPrefix mapper (binding: ForBinding) =
        { binding with Name = SymbolBinding.mapPrefix mapper binding.Name }

// Expression Statement

type ExpressionStatement =
    {
        Value: Expression
        Semicolon: Terminal
    }

// Return Statement

type Return =
    {
        ReturnKeyword: Terminal
        Expression: Expression
        Semicolon: Terminal
    }

// Fail Statement

// ToDo

// Let Statement

type Let =
    {
        LetKeyword: Terminal
        Binding: SymbolBinding
        Equals: Terminal
        Value: Expression
        Semicolon: Terminal
    }

// Mutable Statement

// ToDo

// Set Statement

// ToDo

// Set and Update Statement

// ToDo

// Set-With Statement

// ToDo

// If Statement

type If =
    {
        IfKeyword: Terminal
        Condition: Expression
        Block: Statement Block
    }

// Elif Statement

// ToDo

// Else Statement

and Else = { ElseKeyword: Terminal; Block: Statement Block }

// For Statement

and For =
    {
        ForKeyword: Terminal
        OpenParen: Terminal option
        Binding: ForBinding
        CloseParen: Terminal option
        Block: Statement Block
    }

// While Statement

// ToDo

// Repeat Statement

// ToDo

// Until Statement

// ToDo

// Within Statement

// ToDo

// Apply Statement

// ToDo

// Qubit Declaration Statement

and QubitDeclarationCoda =
    | Semicolon of Terminal
    | Block of Statement Block

and QubitDeclaration =
    {
        Kind: QubitDeclarationKind
        Keyword: Terminal
        OpenParen: Terminal option
        Binding: QubitBinding
        CloseParen: Terminal option
        Coda: QubitDeclarationCoda
    }

// Statement

and Statement =
    | ExpressionStatement of ExpressionStatement
    | Let of Let
    | Return of Return
    | QubitDeclaration of QubitDeclaration
    | If of If
    | Else of Else
    | For of For
    | Unknown of Terminal

module Statement =
    let mapPrefix mapper =
        function
        | ExpressionStatement expr -> { expr with Value = expr.Value |> Expression.mapPrefix mapper } |> ExpressionStatement
        | Let lets -> { lets with LetKeyword = lets.LetKeyword |> Terminal.mapPrefix mapper } |> Let
        | Return returns ->
            { returns with ReturnKeyword = returns.ReturnKeyword |> Terminal.mapPrefix mapper } |> Return
        | QubitDeclaration decl -> { decl with Keyword = decl.Keyword |> Terminal.mapPrefix mapper } |> QubitDeclaration
        | If ifs -> { ifs with IfKeyword = ifs.IfKeyword |> Terminal.mapPrefix mapper } |> If
        | Else elses -> { elses with ElseKeyword = elses.ElseKeyword |> Terminal.mapPrefix mapper } |> Else
        | For loop -> { loop with ForKeyword = loop.ForKeyword |> Terminal.mapPrefix mapper } |> For
        | Unknown terminal -> Terminal.mapPrefix mapper terminal |> Unknown
