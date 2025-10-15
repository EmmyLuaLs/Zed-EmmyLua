;; Function snippets
(snippet
  prefix: "fn"
  body: "function ${1:name}(${2:args})\n\t${3:-- body}\nend"
  description: "Define a function")

(snippet
  prefix: "lf"
  body: "local function ${1:name}(${2:args})\n\t${3:-- body}\nend"
  description: "Define a local function")

(snippet
  prefix: "af"
  body: "local ${1:name} = function(${2:args})\n\t${3:-- body}\nend"
  description: "Anonymous function assignment")

;; Control flow snippets
(snippet
  prefix: "if"
  body: "if ${1:condition} then\n\t${2:-- body}\nend"
  description: "If statement")

(snippet
  prefix: "ife"
  body: "if ${1:condition} then\n\t${2:-- if body}\nelse\n\t${3:-- else body}\nend"
  description: "If-else statement")

(snippet
  prefix: "elif"
  body: "elseif ${1:condition} then\n\t${2:-- body}"
  description: "Elseif clause")

(snippet
  prefix: "for"
  body: "for ${1:i} = ${2:1}, ${3:10} do\n\t${4:-- body}\nend"
  description: "Numeric for loop")

(snippet
  prefix: "fori"
  body: "for ${1:i}, ${2:v} in ipairs(${3:table}) do\n\t${4:-- body}\nend"
  description: "For-in loop with ipairs")

(snippet
  prefix: "forp"
  body: "for ${1:k}, ${2:v} in pairs(${3:table}) do\n\t${4:-- body}\nend"
  description: "For-in loop with pairs")

(snippet
  prefix: "while"
  body: "while ${1:condition} do\n\t${2:-- body}\nend"
  description: "While loop")

(snippet
  prefix: "repeat"
  body: "repeat\n\t${1:-- body}\nuntil ${2:condition}"
  description: "Repeat-until loop")

;; Variable declarations
(snippet
  prefix: "local"
  body: "local ${1:name} = ${2:value}"
  description: "Local variable")

(snippet
  prefix: "req"
  body: "local ${1:module} = require('${2:module_name}')"
  description: "Require module")

;; Table snippets
(snippet
  prefix: "tab"
  body: "local ${1:table} = {\n\t${2:-- content}\n}"
  description: "Create a table")

;; EmmyLua annotation snippets
(snippet
  prefix: "---@class"
  body: "---@class ${1:ClassName}\n---@field ${2:fieldName} ${3:type} ${4:description}"
  description: "EmmyLua class annotation")

(snippet
  prefix: "---@param"
  body: "---@param ${1:paramName} ${2:type} ${3:description}"
  description: "EmmyLua parameter annotation")

(snippet
  prefix: "---@return"
  body: "---@return ${1:type} ${2:description}"
  description: "EmmyLua return annotation")

(snippet
  prefix: "---@type"
  body: "---@type ${1:type}"
  description: "EmmyLua type annotation")

(snippet
  prefix: "---@generic"
  body: "---@generic ${1:T}"
  description: "EmmyLua generic type")

(snippet
  prefix: "---@field"
  body: "---@field ${1:fieldName} ${2:type} ${3:description}"
  description: "EmmyLua field annotation")

(snippet
  prefix: "---@alias"
  body: "---@alias ${1:AliasName} ${2:type}"
  description: "EmmyLua type alias")

(snippet
  prefix: "---@enum"
  body: "---@enum ${1:EnumName}\nlocal ${1:EnumName} = {\n\t${2:-- values}\n}"
  description: "EmmyLua enum annotation")

(snippet
  prefix: "---@overload"
  body: "---@overload fun(${1:params}):${2:returnType}"
  description: "EmmyLua overload annotation")

(snippet
  prefix: "---doc"
  body: "---${1:description}\n---@param ${2:paramName} ${3:type} ${4:param description}\n---@return ${5:type} ${6:return description}\nfunction ${7:name}(${2:paramName})\n\t${8:-- body}\nend"
  description: "Documented function with EmmyLua annotations")

;; Common patterns
(snippet
  prefix: "pcall"
  body: "local success, result = pcall(${1:function}, ${2:args})\nif not success then\n\t${3:-- error handling}\nend"
  description: "Protected call with error handling")

(snippet
  prefix: "mt"
  body: "local ${1:mt} = {\n\t__index = ${2:table}\n}\nsetmetatable(${3:instance}, ${1:mt})"
  description: "Create metatable")

(snippet
  prefix: "class"
  body: "---@class ${1:ClassName}\nlocal ${1:ClassName} = {}\n${1:ClassName}.__index = ${1:ClassName}\n\n---@return ${1:ClassName}\nfunction ${1:ClassName}:new()\n\tlocal instance = setmetatable({}, ${1:ClassName})\n\t${2:-- initialization}\n\treturn instance\nend\n\nreturn ${1:ClassName}"
  description: "Create a class pattern with EmmyLua annotations")
