/// Low-level operations that get translated directly into e.g. LLVM instructions.
/// These are always wrapped when exposed to end users, and can only make it
/// into an Expr when added directly by can::builtins
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LowLevel {
    StrConcat,
    StrIsEmpty,
    StrSplit,
    ListLen,
    ListGetUnsafe,
    ListSet,
    ListSetInPlace,
    ListSingle,
    ListRepeat,
    ListReverse,
    ListConcat,
    ListAppend,
    ListPrepend,
    ListJoin,
    ListMap,
    ListKeepIf,
    ListWalkRight,
    NumAdd,
    NumAddWrap,
    NumAddChecked,
    NumSub,
    NumMul,
    NumGt,
    NumGte,
    NumLt,
    NumLte,
    NumCompare,
    NumDivUnchecked,
    NumRemUnchecked,
    NumAbs,
    NumNeg,
    NumSin,
    NumCos,
    NumSqrtUnchecked,
    NumRound,
    NumToFloat,
    NumPow,
    NumCeiling,
    NumPowInt,
    NumFloor,
    NumIsFinite,
    NumAtan,
    Eq,
    NotEq,
    And,
    Or,
    Not,
}
