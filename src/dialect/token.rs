//! All terminal symbols.

/// Token classes
// Generated by lemon (parse.h).
// Renamed manually.
// To be keep in sync.
#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
#[repr(u16)]
pub enum TokenType {
    TK_EOF = 0,
    TK_SEMI = 1,
    TK_EXPLAIN = 2,
    TK_QUERY = 3,
    TK_PLAN = 4,
    TK_BEGIN = 5,
    TK_TRANSACTION = 6,
    TK_DEFERRED = 7,
    TK_IMMEDIATE = 8,
    TK_EXCLUSIVE = 9,
    TK_COMMIT = 10,
    TK_END = 11,
    TK_ROLLBACK = 12,
    TK_SAVEPOINT = 13,
    TK_RELEASE = 14,
    TK_TO = 15,
    TK_TABLE = 16,
    TK_CREATE = 17,
    TK_IF = 18,
    TK_NOT = 19,
    TK_EXISTS = 20,
    TK_TEMP = 21,
    TK_LP = 22,
    TK_RP = 23,
    TK_AS = 24,
    TK_WITHOUT = 25,
    TK_COMMA = 26,
    TK_ABORT = 27,
    TK_ACTION = 28,
    TK_AFTER = 29,
    TK_ANALYZE = 30,
    TK_ASC = 31,
    TK_ATTACH = 32,
    TK_BEFORE = 33,
    TK_BY = 34,
    TK_CASCADE = 35,
    TK_CAST = 36,
    TK_CONFLICT = 37,
    TK_DATABASE = 38,
    TK_DESC = 39,
    TK_DETACH = 40,
    TK_EACH = 41,
    TK_FAIL = 42,
    TK_OR = 43,
    TK_AND = 44,
    TK_IS = 45,
    TK_MATCH = 46,
    TK_LIKE_KW = 47,
    TK_BETWEEN = 48,
    TK_IN = 49,
    TK_ISNULL = 50,
    TK_NOTNULL = 51,
    TK_NE = 52,
    TK_EQ = 53,
    TK_GT = 54,
    TK_LE = 55,
    TK_LT = 56,
    TK_GE = 57,
    TK_ESCAPE = 58,
    TK_ID = 59,
    TK_COLUMNKW = 60,
    TK_DO = 61,
    TK_FOR = 62,
    TK_IGNORE = 63,
    TK_INITIALLY = 64,
    TK_INSTEAD = 65,
    TK_NO = 66,
    TK_KEY = 67,
    TK_OF = 68,
    TK_OFFSET = 69,
    TK_PRAGMA = 70,
    TK_RAISE = 71,
    TK_RECURSIVE = 72,
    TK_REPLACE = 73,
    TK_RESTRICT = 74,
    TK_ROW = 75,
    TK_ROWS = 76,
    TK_TRIGGER = 77,
    TK_VACUUM = 78,
    TK_VIEW = 79,
    TK_VIRTUAL = 80,
    TK_WITH = 81,
    TK_NULLS = 82,
    TK_FIRST = 83,
    TK_LAST = 84,
    TK_CURRENT = 85,
    TK_FOLLOWING = 86,
    TK_PARTITION = 87,
    TK_PRECEDING = 88,
    TK_RANGE = 89,
    TK_UNBOUNDED = 90,
    TK_EXCLUDE = 91,
    TK_GROUPS = 92,
    TK_OTHERS = 93,
    TK_TIES = 94,
    TK_GENERATED = 95,
    TK_ALWAYS = 96,
    TK_MATERIALIZED = 97,
    TK_REINDEX = 98,
    TK_RENAME = 99,
    TK_CTIME_KW = 100,
    TK_ANY = 101,
    TK_BITAND = 102,
    TK_BITOR = 103,
    TK_LSHIFT = 104,
    TK_RSHIFT = 105,
    TK_PLUS = 106,
    TK_MINUS = 107,
    TK_STAR = 108,
    TK_SLASH = 109,
    TK_REM = 110,
    TK_CONCAT = 111,
    TK_COLLATE = 112,
    TK_BITNOT = 113,
    TK_ON = 114,
    TK_INDEXED = 115,
    TK_STRING = 116,
    TK_JOIN_KW = 117,
    TK_CONSTRAINT = 118,
    TK_DEFAULT = 119,
    TK_NULL = 120,
    TK_PRIMARY = 121,
    TK_UNIQUE = 122,
    TK_CHECK = 123,
    TK_REFERENCES = 124,
    TK_AUTOINCR = 125,
    TK_INSERT = 126,
    TK_DELETE = 127,
    TK_UPDATE = 128,
    TK_SET = 129,
    TK_DEFERRABLE = 130,
    TK_FOREIGN = 131,
    TK_DROP = 132,
    TK_UNION = 133,
    TK_ALL = 134,
    TK_EXCEPT = 135,
    TK_INTERSECT = 136,
    TK_SELECT = 137,
    TK_VALUES = 138,
    TK_DISTINCT = 139,
    TK_DOT = 140,
    TK_FROM = 141,
    TK_JOIN = 142,
    TK_USING = 143,
    TK_ORDER = 144,
    TK_GROUP = 145,
    TK_HAVING = 146,
    TK_LIMIT = 147,
    TK_WHERE = 148,
    TK_RETURNING = 149,
    TK_INTO = 150,
    TK_NOTHING = 151,
    TK_BLOB = 152,
    TK_FLOAT = 153,
    TK_INTEGER = 154,
    TK_VARIABLE = 155,
    TK_CASE = 156,
    TK_WHEN = 157,
    TK_THEN = 158,
    TK_ELSE = 159,
    TK_INDEX = 160,
    TK_ALTER = 161,
    TK_ADD = 162,
    TK_WINDOW = 163,
    TK_OVER = 164,
    TK_FILTER = 165,
}
