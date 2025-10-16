; Tags query for EmmyLuaDoc
; 用于生成符号标签（用于代码导航）

; 类定义
(class_annotation
  name: (identifier) @name) @definition.class

; 字段定义
(field_annotation
  name: (field_name) @name) @definition.field

; 枚举定义
(enum_annotation
  name: (identifier) @name) @definition.enum

; 别名定义
(alias_annotation
  name: (identifier) @name) @definition.type

; 函数参数
(param_annotation
  name: (param_name) @name) @definition.parameter

; 泛型参数
(generic_annotation
  name: (identifier) @name) @definition.type
