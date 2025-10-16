; Locals query for EmmyLuaDoc
; 用于作用域分析的查询文件

; 定义作用域
(class_annotation) @scope

; 类定义
(class_annotation
  name: (identifier) @definition.type)

; 字段定义
(field_annotation
  name: (field_name) @definition.field)

; 参数定义
(param_annotation
  name: (param_name) @definition.parameter)

; 泛型定义
(generic_annotation
  name: (identifier) @definition.type)

; 别名定义
(alias_annotation
  name: (identifier) @definition.type)

; 枚举定义
(enum_annotation
  name: (identifier) @definition.type)

; 类型引用
(basic_type
  (identifier) @reference)

; 参数引用
(param_def
  name: (identifier) @reference)
