; Highlights query for EmmyLuaDoc
; 用于语法高亮的查询文件

; ============================================
; Lua 注释前缀 (Comment Prefix)
; ============================================

(comment_prefix) @comment

; ============================================
; 注解关键字 (Annotation Keywords)
; ============================================

; 使用匿名节点匹配注解标记
"@class" @keyword
"@field" @keyword
"@type" @keyword
"@param" @keyword
"@return" @keyword
"@generic" @keyword
"@vararg" @keyword
"@overload" @keyword
"@see" @keyword
"@alias" @keyword
"@enum" @keyword
"@module" @keyword
"@cast" @keyword
"@version" @keyword
"@diagnostic" @keyword
"@operator" @keyword

; 特殊注解节点（这些没有 @ 前缀的文本节点）
(deprecated_annotation) @keyword
(private_annotation) @keyword
(protected_annotation) @keyword
(public_annotation) @keyword
(package_annotation) @keyword
(async_annotation) @keyword
(nodiscard_annotation) @keyword
(meta_annotation) @keyword

; ============================================
; 类型关键字 (Type Keywords)
; ============================================

[
  "fun"
  "table"
] @keyword.type

; ============================================
; 基础类型 (Basic Types)
; ============================================

((identifier) @type.builtin
  (#match? @type.builtin "^(string|number|integer|boolean|table|function|thread|userdata|nil|any|unknown|self)$"))

; 自定义类型
(basic_type
  (identifier) @type)

; ============================================
; 类定义 (Class Definitions)
; ============================================

(class_annotation
  name: (identifier) @type.definition)

(class_annotation
  parent: (type_list
    (type
      (primary_type
        (basic_type
          (identifier) @type)))))

; ============================================
; 字段和参数 (Fields and Parameters)
; ============================================

(field_annotation
  name: (field_name) @variable.member)

(param_annotation
  name: (param_name) @variable.parameter)

(return_annotation
  name: (identifier)? @variable.parameter)

(param_def
  name: (identifier) @variable.parameter)

; ============================================
; 泛型 (Generics)
; ============================================

(generic_annotation
  name: (identifier) @type.parameter)

(generic_type
  base: (identifier) @type)

; ============================================
; 别名和枚举 (Aliases and Enums)
; ============================================

(alias_annotation
  name: (identifier) @type.definition)

(enum_annotation
  name: (identifier) @type.definition)

; ============================================
; 可见性修饰符 (Visibility Modifiers)
; ============================================

[
  "public"
  "private"
  "protected"
  "package"
] @keyword.modifier

; ============================================
; 操作符 (Operators)
; ============================================

[
  "call"
  "add" "sub" "mul" "div" "mod" "pow"
  "concat"
  "len"
  "eq" "lt" "le"
  "unm"
  "bnot" "band" "bor" "bxor" "shl" "shr"
] @operator

; ============================================
; 字面量 (Literals)
; ============================================

(string) @string

(number) @number

(boolean) @boolean

; ============================================
; 标点符号 (Punctuation)
; ============================================

[
  ":"
  "|"
  ","
  "?"
] @punctuation.delimiter

; 类型续行中的 |
(type_continuation
  "|" @punctuation.delimiter)

[
  "("
  ")"
  "["
  "]"
  "<"
  ">"
] @punctuation.bracket

; ============================================
; 引用和诊断 (References and Diagnostics)
; ============================================

(see_annotation
  reference: (identifier) @variable)

(diagnostic_annotation
  action: [
    "disable"
    "enable"
    "disable-next-line"
    "disable-line"
  ] @keyword.directive)

(diagnostic_list
  (identifier) @constant)

; ============================================
; 模块名 (Module Names)
; ============================================

(module_annotation
  name: (string) @module)

; ============================================
; 版本 (Version)
; ============================================

(version_annotation
  version: [
    (identifier) @constant
    (string) @string
  ])

; ============================================
; 数组类型标记 (Array Type Markers)
; ============================================

(array_type
  "[" @punctuation.bracket
  "]" @punctuation.bracket)

; ============================================
; 函数类型 (Function Types)
; ============================================

(function_type
  "fun" @keyword.function
  ":" @punctuation.delimiter)

; ============================================
; 表类型 (Table Types)
; ============================================

(table_type
  "table" @type.builtin)

; 表字面量类型 (Table Literal Types)
(table_literal_type
  "{" @punctuation.bracket
  "}" @punctuation.bracket)

(table_field
  name: (identifier) @property
  ":" @punctuation.delimiter
  type: (type_list
    (type
      (primary_type
        (basic_type
          (identifier) @type)))))

; 表字段中的逗号
(table_literal_type
  "," @punctuation.delimiter)

; ============================================
; 泛型参数 (Generic Parameters)
; ============================================

; @class 和 @alias 的泛型参数定义
(generic_params
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

(generic_params
  params: (identifier) @type.parameter)

; ============================================
; nil 字面量 (nil Literal)
; ============================================

"nil" @constant.builtin
