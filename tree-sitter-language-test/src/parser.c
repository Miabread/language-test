#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 12
#define STATE_COUNT 95
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 39
#define ALIAS_COUNT 0
#define TOKEN_COUNT 18
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 8
#define MAX_ALIAS_SEQUENCE_LENGTH 7

enum {
  anon_sym_import = 1,
  anon_sym_LBRACE = 2,
  anon_sym_COMMA = 3,
  anon_sym_RBRACE = 4,
  anon_sym_export = 5,
  anon_sym_struct = 6,
  anon_sym_SEMI = 7,
  anon_sym_LPAREN = 8,
  anon_sym_RPAREN = 9,
  anon_sym_COLON = 10,
  anon_sym_func = 11,
  anon_sym_DASH_GT = 12,
  anon_sym_let = 13,
  anon_sym_EQ = 14,
  sym_identifier = 15,
  sym_number = 16,
  sym_comment = 17,
  sym_source_file = 18,
  sym__item = 19,
  sym_import_item = 20,
  sym_export_item = 21,
  sym_struct_item = 22,
  sym_struct_parameter_list = 23,
  sym_struct_parameter = 24,
  sym_function_item = 25,
  sym_func_parameter_list = 26,
  sym_func_parameter = 27,
  sym__type = 28,
  sym_block = 29,
  sym__statement = 30,
  sym_let_statement = 31,
  sym__expression = 32,
  sym__pattern = 33,
  aux_sym_source_file_repeat1 = 34,
  aux_sym_import_item_repeat1 = 35,
  aux_sym_struct_parameter_list_repeat1 = 36,
  aux_sym_func_parameter_list_repeat1 = 37,
  aux_sym_block_repeat1 = 38,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_import] = "import",
  [anon_sym_LBRACE] = "{",
  [anon_sym_COMMA] = ",",
  [anon_sym_RBRACE] = "}",
  [anon_sym_export] = "export",
  [anon_sym_struct] = "struct",
  [anon_sym_SEMI] = ";",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_COLON] = ":",
  [anon_sym_func] = "func",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_let] = "let",
  [anon_sym_EQ] = "=",
  [sym_identifier] = "identifier",
  [sym_number] = "number",
  [sym_comment] = "comment",
  [sym_source_file] = "source_file",
  [sym__item] = "_item",
  [sym_import_item] = "import_item",
  [sym_export_item] = "export_item",
  [sym_struct_item] = "struct_item",
  [sym_struct_parameter_list] = "struct_parameter_list",
  [sym_struct_parameter] = "struct_parameter",
  [sym_function_item] = "function_item",
  [sym_func_parameter_list] = "func_parameter_list",
  [sym_func_parameter] = "func_parameter",
  [sym__type] = "_type",
  [sym_block] = "block",
  [sym__statement] = "_statement",
  [sym_let_statement] = "let_statement",
  [sym__expression] = "_expression",
  [sym__pattern] = "_pattern",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_import_item_repeat1] = "import_item_repeat1",
  [aux_sym_struct_parameter_list_repeat1] = "struct_parameter_list_repeat1",
  [aux_sym_func_parameter_list_repeat1] = "func_parameter_list_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_export] = anon_sym_export,
  [anon_sym_struct] = anon_sym_struct,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_func] = anon_sym_func,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_let] = anon_sym_let,
  [anon_sym_EQ] = anon_sym_EQ,
  [sym_identifier] = sym_identifier,
  [sym_number] = sym_number,
  [sym_comment] = sym_comment,
  [sym_source_file] = sym_source_file,
  [sym__item] = sym__item,
  [sym_import_item] = sym_import_item,
  [sym_export_item] = sym_export_item,
  [sym_struct_item] = sym_struct_item,
  [sym_struct_parameter_list] = sym_struct_parameter_list,
  [sym_struct_parameter] = sym_struct_parameter,
  [sym_function_item] = sym_function_item,
  [sym_func_parameter_list] = sym_func_parameter_list,
  [sym_func_parameter] = sym_func_parameter,
  [sym__type] = sym__type,
  [sym_block] = sym_block,
  [sym__statement] = sym__statement,
  [sym_let_statement] = sym_let_statement,
  [sym__expression] = sym__expression,
  [sym__pattern] = sym__pattern,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_import_item_repeat1] = aux_sym_import_item_repeat1,
  [aux_sym_struct_parameter_list_repeat1] = aux_sym_struct_parameter_list_repeat1,
  [aux_sym_func_parameter_list_repeat1] = aux_sym_func_parameter_list_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_export] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_struct] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_func] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_let] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__item] = {
    .visible = false,
    .named = true,
  },
  [sym_import_item] = {
    .visible = true,
    .named = true,
  },
  [sym_export_item] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_item] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_parameter_list] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_parameter] = {
    .visible = true,
    .named = true,
  },
  [sym_function_item] = {
    .visible = true,
    .named = true,
  },
  [sym_func_parameter_list] = {
    .visible = true,
    .named = true,
  },
  [sym_func_parameter] = {
    .visible = true,
    .named = true,
  },
  [sym__type] = {
    .visible = false,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym__statement] = {
    .visible = false,
    .named = true,
  },
  [sym_let_statement] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym__pattern] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_import_item_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_struct_parameter_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_func_parameter_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_body = 1,
  field_name = 2,
  field_parameters = 3,
  field_pattern = 4,
  field_return_type = 5,
  field_trailing = 6,
  field_type = 7,
  field_value = 8,
};

static const char *ts_field_names[] = {
  [0] = NULL,
  [field_body] = "body",
  [field_name] = "name",
  [field_parameters] = "parameters",
  [field_pattern] = "pattern",
  [field_return_type] = "return_type",
  [field_trailing] = "trailing",
  [field_type] = "type",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[11] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 1},
  [3] = {.index = 3, .length = 2},
  [4] = {.index = 5, .length = 2},
  [5] = {.index = 7, .length = 4},
  [6] = {.index = 11, .length = 3},
  [7] = {.index = 14, .length = 1},
  [8] = {.index = 15, .length = 1},
  [9] = {.index = 16, .length = 2},
  [10] = {.index = 18, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 1},
    {field_parameters, 2},
  [2] =
    {field_type, 1},
  [3] =
    {field_name, 0},
    {field_type, 2},
  [5] =
    {field_pattern, 0},
    {field_type, 2},
  [7] =
    {field_body, 5},
    {field_name, 1},
    {field_parameters, 2},
    {field_return_type, 4},
  [11] =
    {field_name, 0},
    {field_pattern, 1},
    {field_type, 3},
  [14] =
    {field_trailing, 1},
  [15] =
    {field_trailing, 2},
  [16] =
    {field_name, 1},
    {field_value, 3},
  [18] =
    {field_name, 1},
    {field_type, 2},
    {field_type, 3},
    {field_value, 5},
};

static TSSymbol ts_alias_sequences[11][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(28);
      if (lookahead == '(') ADVANCE(36);
      if (lookahead == ')') ADVANCE(37);
      if (lookahead == '+') ADVANCE(27);
      if (lookahead == ',') ADVANCE(31);
      if (lookahead == '-') ADVANCE(6);
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == ':') ADVANCE(38);
      if (lookahead == ';') ADVANCE(35);
      if (lookahead == '=') ADVANCE(43);
      if (lookahead == 'e') ADVANCE(26);
      if (lookahead == 'f') ADVANCE(24);
      if (lookahead == 'i') ADVANCE(10);
      if (lookahead == 'l') ADVANCE(9);
      if (lookahead == 's') ADVANCE(19);
      if (lookahead == '{') ADVANCE(30);
      if (lookahead == '}') ADVANCE(32);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(47);
      END_STATE();
    case 1:
      if (lookahead == ')') ADVANCE(37);
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == ':') ADVANCE(38);
      if (lookahead == '}') ADVANCE(32);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(27);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(47);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(46);
      END_STATE();
    case 2:
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(49);
      END_STATE();
    case 3:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(48);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == 'l') ADVANCE(44);
      if (lookahead == '}') ADVANCE(32);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(27);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(5)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(47);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(46);
      END_STATE();
    case 6:
      if (lookahead == '>') ADVANCE(40);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(47);
      END_STATE();
    case 7:
      if (lookahead == 'c') ADVANCE(39);
      END_STATE();
    case 8:
      if (lookahead == 'c') ADVANCE(23);
      END_STATE();
    case 9:
      if (lookahead == 'e') ADVANCE(20);
      END_STATE();
    case 10:
      if (lookahead == 'm') ADVANCE(15);
      END_STATE();
    case 11:
      if (lookahead == 'n') ADVANCE(7);
      END_STATE();
    case 12:
      if (lookahead == 'o') ADVANCE(17);
      END_STATE();
    case 13:
      if (lookahead == 'o') ADVANCE(18);
      END_STATE();
    case 14:
      if (lookahead == 'p') ADVANCE(12);
      END_STATE();
    case 15:
      if (lookahead == 'p') ADVANCE(13);
      END_STATE();
    case 16:
      if (lookahead == 'r') ADVANCE(25);
      END_STATE();
    case 17:
      if (lookahead == 'r') ADVANCE(21);
      END_STATE();
    case 18:
      if (lookahead == 'r') ADVANCE(22);
      END_STATE();
    case 19:
      if (lookahead == 't') ADVANCE(16);
      END_STATE();
    case 20:
      if (lookahead == 't') ADVANCE(41);
      END_STATE();
    case 21:
      if (lookahead == 't') ADVANCE(33);
      END_STATE();
    case 22:
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 23:
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 24:
      if (lookahead == 'u') ADVANCE(11);
      END_STATE();
    case 25:
      if (lookahead == 'u') ADVANCE(8);
      END_STATE();
    case 26:
      if (lookahead == 'x') ADVANCE(14);
      END_STATE();
    case 27:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(47);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_export);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_struct);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_func);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_let);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_let);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(46);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(45);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(46);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(42);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(46);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(46);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(47);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(49);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 5},
  [5] = {.lex_state = 5},
  [6] = {.lex_state = 5},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 1},
  [20] = {.lex_state = 1},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 5},
  [25] = {.lex_state = 1},
  [26] = {.lex_state = 5},
  [27] = {.lex_state = 1},
  [28] = {.lex_state = 1},
  [29] = {.lex_state = 1},
  [30] = {.lex_state = 1},
  [31] = {.lex_state = 5},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 1},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 1},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 1},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 1},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 1},
  [50] = {.lex_state = 1},
  [51] = {.lex_state = 0},
  [52] = {.lex_state = 1},
  [53] = {.lex_state = 1},
  [54] = {.lex_state = 1},
  [55] = {.lex_state = 0},
  [56] = {.lex_state = 0},
  [57] = {.lex_state = 0},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 1},
  [61] = {.lex_state = 1},
  [62] = {.lex_state = 1},
  [63] = {.lex_state = 1},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 1},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 1},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 1},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 0},
  [77] = {.lex_state = 0},
  [78] = {.lex_state = 0},
  [79] = {.lex_state = 0},
  [80] = {.lex_state = 1},
  [81] = {.lex_state = 0},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 0},
  [84] = {.lex_state = 0},
  [85] = {.lex_state = 0},
  [86] = {.lex_state = 0},
  [87] = {.lex_state = 1},
  [88] = {.lex_state = 0},
  [89] = {.lex_state = 0},
  [90] = {.lex_state = 0},
  [91] = {.lex_state = 0},
  [92] = {.lex_state = 1},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 0},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_export] = ACTIONS(1),
    [anon_sym_struct] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_func] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_let] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(84),
    [sym__item] = STATE(2),
    [sym_import_item] = STATE(2),
    [sym_export_item] = STATE(2),
    [sym_struct_item] = STATE(2),
    [sym_function_item] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_import] = ACTIONS(7),
    [anon_sym_export] = ACTIONS(9),
    [anon_sym_struct] = ACTIONS(11),
    [anon_sym_func] = ACTIONS(13),
    [sym_comment] = ACTIONS(3),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_import,
    ACTIONS(9), 1,
      anon_sym_export,
    ACTIONS(11), 1,
      anon_sym_struct,
    ACTIONS(13), 1,
      anon_sym_func,
    ACTIONS(15), 1,
      ts_builtin_sym_end,
    STATE(3), 6,
      sym__item,
      sym_import_item,
      sym_export_item,
      sym_struct_item,
      sym_function_item,
      aux_sym_source_file_repeat1,
  [27] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(17), 1,
      ts_builtin_sym_end,
    ACTIONS(19), 1,
      anon_sym_import,
    ACTIONS(22), 1,
      anon_sym_export,
    ACTIONS(25), 1,
      anon_sym_struct,
    ACTIONS(28), 1,
      anon_sym_func,
    STATE(3), 6,
      sym__item,
      sym_import_item,
      sym_export_item,
      sym_struct_item,
      sym_function_item,
      aux_sym_source_file_repeat1,
  [54] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(31), 1,
      anon_sym_RBRACE,
    ACTIONS(33), 1,
      anon_sym_let,
    ACTIONS(35), 1,
      sym_identifier,
    ACTIONS(37), 1,
      sym_number,
    STATE(66), 1,
      sym__expression,
    STATE(5), 3,
      sym__statement,
      sym_let_statement,
      aux_sym_block_repeat1,
  [78] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      anon_sym_let,
    ACTIONS(39), 1,
      anon_sym_RBRACE,
    ACTIONS(41), 1,
      sym_identifier,
    ACTIONS(43), 1,
      sym_number,
    STATE(51), 1,
      sym__expression,
    STATE(6), 3,
      sym__statement,
      sym_let_statement,
      aux_sym_block_repeat1,
  [102] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    ACTIONS(47), 1,
      anon_sym_let,
    ACTIONS(50), 1,
      sym_identifier,
    ACTIONS(53), 1,
      sym_number,
    STATE(88), 1,
      sym__expression,
    STATE(6), 3,
      sym__statement,
      sym_let_statement,
      aux_sym_block_repeat1,
  [126] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(56), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [137] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(58), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [148] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(60), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [159] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(62), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [170] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(64), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [181] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(66), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [192] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(68), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [203] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(70), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [214] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(72), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [225] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(74), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [236] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(76), 1,
      anon_sym_RPAREN,
    ACTIONS(78), 1,
      sym_identifier,
    ACTIONS(80), 1,
      sym_number,
    STATE(64), 1,
      sym_func_parameter,
    STATE(86), 1,
      sym__pattern,
  [255] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(82), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [266] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(78), 1,
      sym_identifier,
    ACTIONS(80), 1,
      sym_number,
    ACTIONS(84), 1,
      anon_sym_RPAREN,
    STATE(37), 1,
      sym_func_parameter,
    STATE(86), 1,
      sym__pattern,
  [285] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(78), 1,
      sym_identifier,
    ACTIONS(80), 1,
      sym_number,
    ACTIONS(86), 1,
      anon_sym_RPAREN,
    STATE(64), 1,
      sym_func_parameter,
    STATE(86), 1,
      sym__pattern,
  [304] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(88), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [315] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(90), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [326] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(92), 5,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_export,
      anon_sym_struct,
      anon_sym_func,
  [337] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(94), 2,
      anon_sym_RBRACE,
      sym_number,
    ACTIONS(96), 2,
      anon_sym_let,
      sym_identifier,
  [349] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(98), 1,
      anon_sym_RPAREN,
    ACTIONS(100), 1,
      anon_sym_COLON,
    ACTIONS(102), 1,
      sym_identifier,
    STATE(48), 1,
      sym_struct_parameter,
  [365] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(104), 2,
      anon_sym_RBRACE,
      sym_number,
    ACTIONS(106), 2,
      anon_sym_let,
      sym_identifier,
  [377] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(100), 1,
      anon_sym_COLON,
    ACTIONS(102), 1,
      sym_identifier,
    ACTIONS(108), 1,
      anon_sym_RPAREN,
    STATE(35), 1,
      sym_struct_parameter,
  [393] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(110), 1,
      anon_sym_COLON,
    STATE(85), 1,
      sym__pattern,
    ACTIONS(112), 2,
      sym_identifier,
      sym_number,
  [407] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(78), 1,
      sym_identifier,
    ACTIONS(80), 1,
      sym_number,
    STATE(64), 1,
      sym_func_parameter,
    STATE(86), 1,
      sym__pattern,
  [423] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(100), 1,
      anon_sym_COLON,
    ACTIONS(102), 1,
      sym_identifier,
    ACTIONS(114), 1,
      anon_sym_RPAREN,
    STATE(48), 1,
      sym_struct_parameter,
  [439] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(116), 2,
      anon_sym_RBRACE,
      sym_number,
    ACTIONS(118), 2,
      anon_sym_let,
      sym_identifier,
  [451] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(120), 1,
      anon_sym_COMMA,
    ACTIONS(122), 1,
      anon_sym_RBRACE,
    STATE(42), 1,
      aux_sym_import_item_repeat1,
  [464] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(114), 1,
      anon_sym_RPAREN,
    ACTIONS(124), 1,
      anon_sym_COMMA,
    STATE(46), 1,
      aux_sym_struct_parameter_list_repeat1,
  [477] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(93), 1,
      sym__expression,
    ACTIONS(126), 2,
      sym_identifier,
      sym_number,
  [488] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(128), 1,
      anon_sym_COMMA,
    ACTIONS(130), 1,
      anon_sym_RPAREN,
    STATE(33), 1,
      aux_sym_struct_parameter_list_repeat1,
  [501] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(90), 1,
      sym__expression,
    ACTIONS(132), 2,
      sym_identifier,
      sym_number,
  [512] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(134), 1,
      anon_sym_COMMA,
    ACTIONS(136), 1,
      anon_sym_RPAREN,
    STATE(44), 1,
      aux_sym_func_parameter_list_repeat1,
  [525] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(57), 1,
      sym__pattern,
    ACTIONS(138), 2,
      sym_identifier,
      sym_number,
  [536] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(140), 1,
      anon_sym_COMMA,
    ACTIONS(142), 1,
      anon_sym_RBRACE,
    STATE(32), 1,
      aux_sym_import_item_repeat1,
  [549] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(144), 1,
      anon_sym_COMMA,
    ACTIONS(146), 1,
      anon_sym_RBRACE,
    STATE(45), 1,
      aux_sym_import_item_repeat1,
  [562] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(100), 1,
      anon_sym_COLON,
    ACTIONS(102), 1,
      sym_identifier,
    STATE(48), 1,
      sym_struct_parameter,
  [575] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(148), 1,
      anon_sym_COMMA,
    ACTIONS(151), 1,
      anon_sym_RBRACE,
    STATE(42), 1,
      aux_sym_import_item_repeat1,
  [588] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(153), 1,
      anon_sym_COMMA,
    ACTIONS(156), 1,
      anon_sym_RPAREN,
    STATE(43), 1,
      aux_sym_func_parameter_list_repeat1,
  [601] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(76), 1,
      anon_sym_RPAREN,
    ACTIONS(158), 1,
      anon_sym_COMMA,
    STATE(43), 1,
      aux_sym_func_parameter_list_repeat1,
  [614] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(160), 1,
      anon_sym_COMMA,
    ACTIONS(162), 1,
      anon_sym_RBRACE,
    STATE(42), 1,
      aux_sym_import_item_repeat1,
  [627] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(164), 1,
      anon_sym_COMMA,
    ACTIONS(167), 1,
      anon_sym_RPAREN,
    STATE(46), 1,
      aux_sym_struct_parameter_list_repeat1,
  [640] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(169), 1,
      anon_sym_LPAREN,
    STATE(75), 1,
      sym_func_parameter_list,
  [650] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(167), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [658] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 1,
      sym_identifier,
    STATE(58), 1,
      sym__type,
  [668] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(173), 1,
      sym_identifier,
    STATE(89), 1,
      sym__type,
  [678] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 1,
      anon_sym_RBRACE,
    ACTIONS(177), 1,
      anon_sym_SEMI,
  [688] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(179), 1,
      anon_sym_RBRACE,
    ACTIONS(181), 1,
      sym_identifier,
  [698] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(183), 1,
      anon_sym_RBRACE,
    ACTIONS(185), 1,
      sym_identifier,
  [708] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 1,
      sym_identifier,
    STATE(67), 1,
      sym__type,
  [718] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(189), 1,
      anon_sym_LBRACE,
    STATE(16), 1,
      sym_block,
  [728] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_LPAREN,
    STATE(76), 1,
      sym_struct_parameter_list,
  [738] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(193), 1,
      anon_sym_COLON,
    ACTIONS(195), 1,
      anon_sym_EQ,
  [748] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(197), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [756] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(199), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [764] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 1,
      sym_identifier,
    STATE(55), 1,
      sym__type,
  [774] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(162), 1,
      anon_sym_RBRACE,
    ACTIONS(203), 1,
      sym_identifier,
  [784] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(205), 1,
      sym_identifier,
    STATE(69), 1,
      sym__type,
  [794] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(207), 1,
      sym_identifier,
    STATE(59), 1,
      sym__type,
  [804] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(156), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [812] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(122), 1,
      anon_sym_RBRACE,
    ACTIONS(203), 1,
      sym_identifier,
  [822] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(177), 1,
      anon_sym_SEMI,
    ACTIONS(209), 1,
      anon_sym_RBRACE,
  [832] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(211), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [840] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(203), 1,
      sym_identifier,
    ACTIONS(213), 1,
      anon_sym_RBRACE,
  [850] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(215), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [858] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(203), 1,
      sym_identifier,
    ACTIONS(217), 1,
      anon_sym_RBRACE,
  [868] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(151), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [876] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(219), 1,
      anon_sym_SEMI,
  [883] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(221), 1,
      anon_sym_DASH_GT,
  [890] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_SEMI,
  [897] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(225), 1,
      anon_sym_DASH_GT,
  [904] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(227), 1,
      anon_sym_SEMI,
  [911] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(229), 1,
      anon_sym_SEMI,
  [918] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(231), 1,
      anon_sym_COLON,
  [925] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(233), 1,
      anon_sym_DASH_GT,
  [932] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(203), 1,
      sym_identifier,
  [939] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(235), 1,
      anon_sym_SEMI,
  [946] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(237), 1,
      anon_sym_DASH_GT,
  [953] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(239), 1,
      anon_sym_LBRACE,
  [960] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(241), 1,
      ts_builtin_sym_end,
  [967] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 1,
      anon_sym_COLON,
  [974] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(245), 1,
      anon_sym_COLON,
  [981] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(247), 1,
      sym_identifier,
  [988] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(177), 1,
      anon_sym_SEMI,
  [995] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(249), 1,
      anon_sym_EQ,
  [1002] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(251), 1,
      anon_sym_SEMI,
  [1009] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(253), 1,
      anon_sym_DASH_GT,
  [1016] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(255), 1,
      sym_identifier,
  [1023] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(257), 1,
      anon_sym_SEMI,
  [1030] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(259), 1,
      anon_sym_LBRACE,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 27,
  [SMALL_STATE(4)] = 54,
  [SMALL_STATE(5)] = 78,
  [SMALL_STATE(6)] = 102,
  [SMALL_STATE(7)] = 126,
  [SMALL_STATE(8)] = 137,
  [SMALL_STATE(9)] = 148,
  [SMALL_STATE(10)] = 159,
  [SMALL_STATE(11)] = 170,
  [SMALL_STATE(12)] = 181,
  [SMALL_STATE(13)] = 192,
  [SMALL_STATE(14)] = 203,
  [SMALL_STATE(15)] = 214,
  [SMALL_STATE(16)] = 225,
  [SMALL_STATE(17)] = 236,
  [SMALL_STATE(18)] = 255,
  [SMALL_STATE(19)] = 266,
  [SMALL_STATE(20)] = 285,
  [SMALL_STATE(21)] = 304,
  [SMALL_STATE(22)] = 315,
  [SMALL_STATE(23)] = 326,
  [SMALL_STATE(24)] = 337,
  [SMALL_STATE(25)] = 349,
  [SMALL_STATE(26)] = 365,
  [SMALL_STATE(27)] = 377,
  [SMALL_STATE(28)] = 393,
  [SMALL_STATE(29)] = 407,
  [SMALL_STATE(30)] = 423,
  [SMALL_STATE(31)] = 439,
  [SMALL_STATE(32)] = 451,
  [SMALL_STATE(33)] = 464,
  [SMALL_STATE(34)] = 477,
  [SMALL_STATE(35)] = 488,
  [SMALL_STATE(36)] = 501,
  [SMALL_STATE(37)] = 512,
  [SMALL_STATE(38)] = 525,
  [SMALL_STATE(39)] = 536,
  [SMALL_STATE(40)] = 549,
  [SMALL_STATE(41)] = 562,
  [SMALL_STATE(42)] = 575,
  [SMALL_STATE(43)] = 588,
  [SMALL_STATE(44)] = 601,
  [SMALL_STATE(45)] = 614,
  [SMALL_STATE(46)] = 627,
  [SMALL_STATE(47)] = 640,
  [SMALL_STATE(48)] = 650,
  [SMALL_STATE(49)] = 658,
  [SMALL_STATE(50)] = 668,
  [SMALL_STATE(51)] = 678,
  [SMALL_STATE(52)] = 688,
  [SMALL_STATE(53)] = 698,
  [SMALL_STATE(54)] = 708,
  [SMALL_STATE(55)] = 718,
  [SMALL_STATE(56)] = 728,
  [SMALL_STATE(57)] = 738,
  [SMALL_STATE(58)] = 748,
  [SMALL_STATE(59)] = 756,
  [SMALL_STATE(60)] = 764,
  [SMALL_STATE(61)] = 774,
  [SMALL_STATE(62)] = 784,
  [SMALL_STATE(63)] = 794,
  [SMALL_STATE(64)] = 804,
  [SMALL_STATE(65)] = 812,
  [SMALL_STATE(66)] = 822,
  [SMALL_STATE(67)] = 832,
  [SMALL_STATE(68)] = 840,
  [SMALL_STATE(69)] = 850,
  [SMALL_STATE(70)] = 858,
  [SMALL_STATE(71)] = 868,
  [SMALL_STATE(72)] = 876,
  [SMALL_STATE(73)] = 883,
  [SMALL_STATE(74)] = 890,
  [SMALL_STATE(75)] = 897,
  [SMALL_STATE(76)] = 904,
  [SMALL_STATE(77)] = 911,
  [SMALL_STATE(78)] = 918,
  [SMALL_STATE(79)] = 925,
  [SMALL_STATE(80)] = 932,
  [SMALL_STATE(81)] = 939,
  [SMALL_STATE(82)] = 946,
  [SMALL_STATE(83)] = 953,
  [SMALL_STATE(84)] = 960,
  [SMALL_STATE(85)] = 967,
  [SMALL_STATE(86)] = 974,
  [SMALL_STATE(87)] = 981,
  [SMALL_STATE(88)] = 988,
  [SMALL_STATE(89)] = 995,
  [SMALL_STATE(90)] = 1002,
  [SMALL_STATE(91)] = 1009,
  [SMALL_STATE(92)] = 1016,
  [SMALL_STATE(93)] = 1023,
  [SMALL_STATE(94)] = 1030,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [19] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(83),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(94),
  [25] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(92),
  [28] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(87),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(38),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(66),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [41] = {.entry = {.count = 1, .reusable = false}}, SHIFT(51),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2),
  [47] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(38),
  [50] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(88),
  [53] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(88),
  [56] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_export_item, 6),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_item, 6),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_item, 4, .production_id = 1),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 7),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_item, 3),
  [70] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_item, 5),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_export_item, 3),
  [74] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_item, 6, .production_id = 5),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [78] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [82] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 8),
  [84] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [86] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_export_item, 4),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_item, 4),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_export_item, 5),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_let_statement, 5, .production_id = 9),
  [96] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_let_statement, 5, .production_id = 9),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_let_statement, 7, .production_id = 10),
  [106] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_let_statement, 7, .production_id = 10),
  [108] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__pattern, 1),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__statement, 2),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__statement, 2),
  [120] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [122] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [124] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [126] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [128] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [130] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [132] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [134] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [136] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [138] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [142] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [144] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [146] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [148] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_import_item_repeat1, 2), SHIFT_REPEAT(80),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_import_item_repeat1, 2),
  [153] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_parameter_list_repeat1, 2), SHIFT_REPEAT(29),
  [156] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_parameter_list_repeat1, 2),
  [158] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [160] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [162] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [164] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_struct_parameter_list_repeat1, 2), SHIFT_REPEAT(41),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_struct_parameter_list_repeat1, 2),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [177] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [185] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [195] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_parameter, 3, .production_id = 3),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_parameter, 2, .production_id = 2),
  [201] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_parameter, 3, .production_id = 4),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_parameter, 4, .production_id = 6),
  [217] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_parameter_list, 4),
  [221] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_parameter_list, 5),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_parameter_list, 5),
  [225] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [227] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [229] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_parameter_list, 3),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [233] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_parameter_list, 4),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_parameter_list, 2),
  [237] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_parameter_list, 3),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [241] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [253] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_parameter_list, 2),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [257] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [259] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_language_test(void) {
  static TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .parse_table = (const uint16_t *)ts_parse_table,
    .parse_actions = ts_parse_actions,
    .lex_modes = ts_lex_modes,
    .alias_sequences = (const TSSymbol *)ts_alias_sequences,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .lex_fn = ts_lex,
    .field_count = FIELD_COUNT,
    .field_map_slices = (const TSFieldMapSlice *)ts_field_map_slices,
    .field_map_entries = (const TSFieldMapEntry *)ts_field_map_entries,
    .field_names = ts_field_names,
    .large_state_count = LARGE_STATE_COUNT,
    .small_parse_table = (const uint16_t *)ts_small_parse_table,
    .small_parse_table_map = (const uint32_t *)ts_small_parse_table_map,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .state_count = STATE_COUNT,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
